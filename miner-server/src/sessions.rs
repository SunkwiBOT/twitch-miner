// ! The cookie carries a random value; the server keeps only its SHA-256, so a
// memory dump never yields a usable cookie.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use base64::Engine;
use miner_core::Manager;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use tokio::sync::{Notify, RwLock};
use tokio_util::sync::CancellationToken;

const MAX_SESSIONS: usize = 50;
// ! Its own cap, never shared with MAX_SESSIONS: every device an SSO signs
// in is a header session, and must not be able to evict a real cookie or
// bearer session by flooding the header with distinct values.
const MAX_HEADER_SESSIONS: usize = 50;
const SESSION_BYTES: usize = 32;
// ! Kept short, out of the in-app log buffer, and reused by the Settings
// card's "signed in as" text: the header value names a real person, shown
// back only where the server itself already logs or displays it.
pub(crate) const LOGGED_VALUE_MAX_CHARS: usize = 64;
// ! Bounded so a flood of distinct values cannot grow this forever: once
// full, the oldest logged value is forgotten first, and may be logged again.
const LOGGED_VALUES_MAX: usize = 1_000;

pub const SESSION_COOKIE: &str = "tm_session";
// ! `__Host-` makes the browser refuse the cookie unless it is `Secure`, on
// `Path=/` and without `Domain`: a page served over plain HTTP on the same
// host can neither read, plant nor overwrite it.
pub const PUBLIC_SESSION_COOKIE: &str = "__Host-tm_session";
// ! The cookie's `Max-Age`, enforced here too: a stolen cookie dies with it.
pub const SESSION_MAX_AGE: Duration = Duration::from_secs(30 * 24 * 60 * 60);
// ! Short on purpose: the header is re-checked on every request, so a push
// channel just forces a reconnect through the proxy at this deadline rather
// than outliving a visitor the SSO has since dropped for up to 30 days.
pub(crate) const HEADER_SESSION_TTL: Duration = Duration::from_secs(5 * 60);

type ActiveAt = Arc<Mutex<Instant>>;

// ! A bearer password brings no cookie: every script holding it shares one
// session, renewed by each request since each one gives the password again.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Holder {
    Cookie([u8; 32]),
    Bearer,
    // ! Hashed like the cookie: a session is shared only by requests naming
    // the same signed-in person, never by every visitor of the proxy.
    Header([u8; 32]),
}

struct Session {
    holder: Holder,
    created_at: Instant,
    active_at: ActiveAt,
    revoked: CancellationToken,
}

fn read(active_at: &ActiveAt) -> Instant {
    *miner_core::util::locked(active_at)
}

impl Session {
    fn new(holder: Holder, now: Instant) -> Self {
        Self {
            holder,
            created_at: now,
            active_at: Arc::new(Mutex::new(now)),
            revoked: CancellationToken::new(),
        }
    }

    fn expires_at(&self) -> Instant {
        let ttl = match self.holder {
            Holder::Header(_) => HEADER_SESSION_TTL,
            Holder::Cookie(_) | Holder::Bearer => SESSION_MAX_AGE,
        };

        self.created_at + ttl
    }

    fn idle_at(&self, idle_limit: Option<Duration>) -> Option<Instant> {
        idle_limit.map(|limit| read(&self.active_at) + limit)
    }

    fn alive_at(&self, now: Instant, idle_limit: Option<Duration>) -> bool {
        now < self.expires_at() && self.idle_at(idle_limit).is_none_or(|idle_at| now < idle_at)
    }

    fn lease(&self) -> SessionLease {
        SessionLease {
            revoked: self.revoked.clone(),
            expires_at: self.expires_at(),
            active_at: self.active_at.clone(),
            renewable: matches!(self.holder, Holder::Header(_)),
        }
    }
}

#[derive(Clone)]
pub struct SessionLease {
    revoked: CancellationToken,
    expires_at: Instant,
    active_at: ActiveAt,
    // ! A header lease ends on its own short deadline: its close code lets
    // the UI quietly reconnect through the proxy, instead of showing a
    // sign-in screen like a real session end does.
    renewable: bool,
}

impl SessionLease {
    pub async fn ended(&self) {
        tokio::select! {
            _ = self.revoked.cancelled() => {}
            _ = tokio::time::sleep_until(self.expires_at.into()) => {}
        }
    }

    // ! Only the activity route calls it: what the interface asks on its own (the
    // logs every five seconds, a snapshot) keeps no session open.
    pub fn touch(&self) {
        *miner_core::util::locked(&self.active_at) = Instant::now();
    }

    pub fn renewable(&self) -> bool {
        self.renewable
    }
}

#[derive(Default)]
pub struct Sessions {
    entries: RwLock<Vec<Session>>,
    created: Notify,
    // ! Every value ever signed in through the header during this process,
    // not tied to any one session: its short TTL would otherwise log the
    // same person again at every renewal.
    logged_header_values: Mutex<Vec<[u8; 32]>>,
}

fn hash(session: &str) -> [u8; 32] {
    Sha256::digest(session.as_bytes()).into()
}

// ! Each kind keeps its own cap (`MAX_SESSIONS` for cookies and the bearer,
// `MAX_HEADER_SESSIONS` for the header): flooding one kind evicts only its
// own oldest entry, never another visitor's or script's session.
fn add(entries: &mut Vec<Session>, holder: Holder, now: Instant) -> SessionLease {
    entries.retain(|entry| {
        let alive = now < entry.expires_at();

        if !alive {
            entry.revoked.cancel();
        }

        alive
    });

    let session = Session::new(holder, now);
    let lease = session.lease();

    entries.push(session);

    let is_header = matches!(holder, Holder::Header(_));
    let limit = if is_header {
        MAX_HEADER_SESSIONS
    } else {
        MAX_SESSIONS
    };
    let of_same_kind = |entry: &Session| matches!(entry.holder, Holder::Header(_)) == is_header;

    if entries.iter().filter(|entry| of_same_kind(entry)).count() > limit {
        let index = entries
            .iter()
            .position(of_same_kind)
            .expect("just counted at least one");

        entries.remove(index).revoked.cancel();
    }

    lease
}

impl Sessions {
    pub async fn create(&self) -> String {
        let mut raw = [0u8; SESSION_BYTES];
        rand::fill(&mut raw);
        let session = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(raw);

        add(
            &mut *self.entries.write().await,
            Holder::Cookie(hash(&session)),
            Instant::now(),
        );
        self.created.notify_waiters();

        session
    }

    pub async fn bearer(&self, idle_limit: Option<Duration>) -> SessionLease {
        let now = Instant::now();
        let mut entries = self.entries.write().await;
        let alive = entries
            .iter()
            .find(|entry| entry.holder == Holder::Bearer && entry.alive_at(now, idle_limit));

        if let Some(entry) = alive {
            *miner_core::util::locked(&entry.active_at) = now;

            return entry.lease();
        }

        let lease = add(&mut entries, Holder::Bearer, now);

        drop(entries);
        self.created.notify_waiters();

        lease
    }

    // ! Behind the trusted proxy the header is checked on every request, so
    // nothing here needs revoking by hand: a short, per-value hard deadline
    // (not 30 days) forces the next request through the proxy again, ending
    // a push channel for anyone the SSO has since dropped and never sharing
    // one person's session with another.
    pub async fn header(&self, value: &str, idle_limit: Option<Duration>) -> SessionLease {
        let now = Instant::now();
        let key = hash(value);
        let mut entries = self.entries.write().await;
        let alive = entries.iter().find(|entry| {
            matches!(entry.holder, Holder::Header(stored) if stored == key)
                && entry.alive_at(now, idle_limit)
        });

        if let Some(entry) = alive {
            *miner_core::util::locked(&entry.active_at) = now;

            return entry.lease();
        }

        // ! Logged once per value for the whole process, not once per
        // session: the short TTL would otherwise renew this line every few
        // minutes for the same person. Its own target, read by neither
        // `/api/logs` nor the "Copy for a bug report" export (they keep only
        // `twitch_miner`/`miner_core` lines): the value names a real person,
        // not an error to triage.
        self.log_header_value_once(value, key);

        let lease = add(&mut entries, Holder::Header(key), now);

        drop(entries);
        self.created.notify_waiters();

        lease
    }

    fn log_header_value_once(&self, value: &str, key: [u8; 32]) {
        let mut logged = miner_core::util::locked(&self.logged_header_values);

        if logged.contains(&key) {
            return;
        }

        if logged.len() >= LOGGED_VALUES_MAX {
            logged.remove(0);
        }

        logged.push(key);
        drop(logged);

        let shown: String = value.chars().take(LOGGED_VALUE_MAX_CHARS).collect();
        tracing::info!(target: "miner_server", user = shown.as_str(), "signed in through the trusted proxy header");
    }

    pub async fn find(&self, session: &str, idle_limit: Option<Duration>) -> Option<SessionLease> {
        self.find_at(session, Instant::now(), idle_limit).await
    }

    async fn find_at(
        &self,
        session: &str,
        now: Instant,
        idle_limit: Option<Duration>,
    ) -> Option<SessionLease> {
        let wanted = hash(session);

        self.entries
            .read()
            .await
            .iter()
            .find(|entry| match entry.holder {
                Holder::Cookie(stored) => bool::from(stored.ct_eq(&wanted)),
                Holder::Bearer | Holder::Header(_) => false,
            })
            .filter(|entry| entry.alive_at(now, idle_limit))
            .map(Session::lease)
    }

    async fn end_idle_at(&self, now: Instant, idle_limit: Option<Duration>) -> Option<Instant> {
        idle_limit?;

        let idle = |entry: &Session| entry.idle_at(idle_limit).is_some_and(|at| at <= now);

        if self.entries.read().await.iter().any(idle) {
            self.entries.write().await.retain(|entry| {
                let ended = idle(entry);

                if ended {
                    entry.revoked.cancel();
                }

                !ended
            });
        }

        self.entries
            .read()
            .await
            .iter()
            .filter_map(|entry| entry.idle_at(idle_limit))
            .min()
    }

    pub async fn end_idle(&self, manager: &Manager) {
        let changed = manager.changed.notified();
        tokio::pin!(changed);
        changed.as_mut().enable();

        let created = self.created.notified();
        tokio::pin!(created);
        created.as_mut().enable();

        loop {
            let idle_limit = manager.global_settings().await.auto_lock.idle_limit();
            let next = self.end_idle_at(Instant::now(), idle_limit).await;
            let deadline = async {
                match next {
                    Some(at) => tokio::time::sleep_until(at.into()).await,
                    None => std::future::pending().await,
                }
            };

            tokio::select! {
                _ = deadline => {}
                _ = &mut changed => {
                    changed.set(manager.changed.notified());
                    changed.as_mut().enable();
                }
                _ = &mut created => {
                    created.set(self.created.notified());
                    created.as_mut().enable();
                }
            }
        }
    }

    pub async fn revoke(&self, session: &str) {
        let wanted = hash(session);

        self.end(|holder| match holder {
            Holder::Cookie(stored) => bool::from(stored.ct_eq(&wanted)),
            Holder::Bearer | Holder::Header(_) => false,
        })
        .await;
    }

    pub async fn revoke_bearer(&self) {
        self.end(|holder| holder == Holder::Bearer).await;
    }

    async fn end(&self, ended: impl Fn(Holder) -> bool) {
        self.entries.write().await.retain(|entry| {
            let kept = !ended(entry.holder);

            if !kept {
                entry.revoked.cancel();
            }

            kept
        });
    }
}
