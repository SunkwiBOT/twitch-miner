use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::{Notify, RwLock};

use crate::error::{Error, ErrorCode, Result};
use crate::http::read_bounded;
use crate::requests::{address_target, RequestKind, RequestLog};
use crate::store::Store;
use crate::util::locked;

// ! Badges' anonymous request: one every 10 minutes, or 5 for a manual one. The
// catalogue keeps its own interval in `catalog.rs`.
pub const REFRESH_INTERVAL: Duration = Duration::from_secs(600);
pub const MANUAL_REFRESH_GAP: Duration = Duration::from_secs(300);

const CACHE_MAX_AGE: chrono::Duration = chrono::Duration::days(7);

const MAX_BYTES: usize = 16 * 1024 * 1024;

pub trait FeedAnswer: Default + Serialize + DeserializeOwned + Send + Sync + 'static {
    const NAME: &'static str;
    const CACHE_KEY: &'static str;
    const KIND: RequestKind;
    const REFRESH_INTERVAL: Duration;
    const MANUAL_REFRESH_GAP: Duration;

    fn parse(payload: &Value) -> Result<Self>;
    fn count(&self) -> usize;
    fn fetched_at(&self) -> Option<DateTime<Utc>>;
    fn mark_fetched(&mut self, at: DateTime<Utc>);
    fn set_problem(&mut self, problem: Option<ErrorCode>);
    fn mark_next_refresh(&mut self, _at: Option<DateTime<Utc>>) {}
    // ! Badges have no finer-grained state to compare against, so every new
    // answer counts as changed; `Catalog` overrides this since its embedded
    // live lists refresh every cycle without a real change.
    fn meaningfully_changed(&self, _previous: &Self) -> bool {
        true
    }
}

pub struct FeedClient<T> {
    client: reqwest::Client,
    url: String,
    requests: RequestLog,
    headers: Vec<(&'static str, String)>,
    // ! A 304 answer costs no request of the quota.
    tag: Mutex<Option<String>>,
    answer: PhantomData<fn() -> T>,
}

pub enum Fetched<T> {
    New(T),
    Unchanged,
}

pub struct Fetch<T> {
    pub result: Result<Fetched<T>>,
    pub retry_after: Duration,
}

impl<T: FeedAnswer> FeedClient<T> {
    pub(crate) fn with_url(url: &str, requests: RequestLog, agent: &str) -> Result<Self> {
        Self::with_headers(url, requests, agent, &[])
    }

    pub(crate) fn with_headers(
        url: &str,
        requests: RequestLog,
        agent: &str,
        headers: &[(&'static str, String)],
    ) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(agent)
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .gzip(true)
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;

        Ok(Self {
            client,
            url: url.to_owned(),
            requests,
            headers: headers.to_vec(),
            tag: Mutex::new(None),
            answer: PhantomData,
        })
    }

    pub async fn fetch(&self) -> Fetch<T> {
        let mut retry_after = Duration::ZERO;
        let result = async {
            let tag = locked(&self.tag).clone();
            let mut request = self
                .client
                .get(&self.url)
                .header("Accept", "application/json");

            for (name, value) in &self.headers {
                request = request.header(*name, value);
            }

            if let Some(tag) = &tag {
                request = request.header("If-None-Match", tag);
            }

            let response = self
                .requests
                .start(T::KIND, address_target(&self.url))
                .expecting(&[reqwest::StatusCode::NOT_MODIFIED])
                .send(request)
                .await?;

            for name in ["ratelimit-reset", "retry-after"] {
                if let Some(seconds) = response
                    .headers()
                    .get(name)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.trim().parse::<u64>().ok())
                {
                    retry_after = retry_after.max(Duration::from_secs(seconds.min(3600)));
                }
            }

            let status = response.status();

            if status == reqwest::StatusCode::NOT_MODIFIED && tag.is_some() {
                return Ok(Fetched::Unchanged);
            }

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(Error::CatalogRateLimited);
            }

            if !status.is_success() {
                return Err(Error::Status(status));
            }

            let tag = response
                .headers()
                .get(reqwest::header::ETAG)
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned);
            let body = read_bounded(response, MAX_BYTES).await?;
            let payload: Value = serde_json::from_slice(&body)?;
            let answer = T::parse(&payload)?;

            *locked(&self.tag) = tag;

            Ok(Fetched::New(answer))
        }
        .await;

        Fetch {
            result,
            retry_after,
        }
    }

    fn forget_tag(&self) {
        *locked(&self.tag) = None;
    }
}
pub struct Refresh {
    pub result: Result<(usize, bool)>,
    pub retry_after: Duration,
}

impl Refresh {
    // ! The automatic loop's next try: honours `retry-after` (even on a non-429 error, as
    // a floor under the regular interval), else the regular interval.
    pub fn wait(&self, refresh_interval: Duration) -> Duration {
        match self.result {
            Err(Error::CatalogRateLimited) if !self.retry_after.is_zero() => self.retry_after,
            _ => refresh_interval.max(self.retry_after),
        }
    }

    // ! The manual button's own cooldown: honours `retry-after` when the API gives one,
    // otherwise floors at the regular manual gap (v1.11.0's unconditional 300 s) rather
    // than the longer automatic interval.
    fn manual_wait(&self, manual_refresh_gap: Duration) -> Duration {
        match self.result {
            Err(Error::CatalogRateLimited) if !self.retry_after.is_zero() => self.retry_after,
            Err(Error::CatalogRateLimited) => manual_refresh_gap,
            Ok(_) => manual_refresh_gap.max(self.retry_after),
            Err(_) => Duration::ZERO,
        }
    }
}

pub struct Feed<T> {
    pub state: Arc<RwLock<T>>,
    pub(crate) client: FeedClient<T>,
    fetching: tokio::sync::Mutex<()>,
    // ! A monotonic deadline: wall time never advances under a paused test clock, which
    // would make the background loop spin on `tick` forever once the wall clock itself
    // is frozen.
    next_manual: Mutex<tokio::time::Instant>,
    resync: Mutex<Duration>,
    notify: Notify,
    changed: Arc<Notify>,
}

impl<T: FeedAnswer> Feed<T> {
    pub fn new(kept: T, client: FeedClient<T>, changed: Arc<Notify>) -> Self {
        let next_manual = kept
            .fetched_at()
            .map_or_else(tokio::time::Instant::now, |at| {
                let due =
                    at + chrono::Duration::from_std(T::MANUAL_REFRESH_GAP).unwrap_or_default();
                let left = (due - Utc::now()).to_std().unwrap_or(Duration::ZERO);

                tokio::time::Instant::now() + left
            });

        Self {
            state: Arc::new(RwLock::new(kept)),
            client,
            fetching: tokio::sync::Mutex::new(()),
            next_manual: Mutex::new(next_manual),
            resync: Mutex::new(T::REFRESH_INTERVAL),
            notify: Notify::new(),
            changed,
        }
    }

    pub async fn first_wait(&self) -> Duration {
        self.state
            .read()
            .await
            .fetched_at()
            .map_or(Duration::ZERO, |at| {
                let due = at + chrono::Duration::from_std(T::REFRESH_INTERVAL).unwrap_or_default();

                (due - Utc::now()).to_std().unwrap_or(Duration::ZERO)
            })
    }

    // ! One clock read: the duration left on the manual deadline, zero once it is due.
    fn time_left(&self) -> Duration {
        locked(&self.next_manual).saturating_duration_since(tokio::time::Instant::now())
    }

    pub fn next_manual_refresh(&self) -> DateTime<Utc> {
        Utc::now() + chrono::Duration::from_std(self.time_left()).unwrap_or_default()
    }

    // ! Woken by a manual refresh elsewhere: the wait it yields is the automatic
    // schedule's own, so the background loop resyncs to it instead of the manual gap.
    pub async fn resynced(&self) -> Duration {
        self.notify.notified().await;

        *locked(&self.resync)
    }

    async fn refresh(&self, store: &Store) -> Refresh {
        let fetch = self.client.fetch().await;
        let refresh = Refresh {
            result: self.apply(fetch.result, store).await,
            retry_after: fetch.retry_after,
        };

        match &refresh.result {
            Ok((count, true)) => tracing::info!(count, "{} refreshed", T::NAME),
            Ok(_) => {}
            Err(Error::CatalogRateLimited) => tracing::info!(
                retry_in_seconds = refresh.wait(T::REFRESH_INTERVAL).as_secs(),
                "{} refresh postponed: the API asks to wait, the last one is kept",
                T::NAME
            ),
            Err(error) => {
                tracing::warn!(%error, "{} refresh failed, the last one is kept", T::NAME)
            }
        }

        refresh
    }

    // One fetch under the lock: the manual cooldown and a 429's retry-after share this
    // deadline, so the background loop never skips the retry the API asked for.
    async fn fetch_locked(&self, store: &Store) -> Refresh {
        let refresh = self.refresh(store).await;

        *locked(&self.next_manual) =
            tokio::time::Instant::now() + refresh.manual_wait(T::MANUAL_REFRESH_GAP);

        self.state.write().await.mark_next_refresh(
            chrono::Duration::from_std(refresh.wait(T::REFRESH_INTERVAL))
                .ok()
                .map(|left| Utc::now() + left),
        );

        self.changed.notify_waiters();

        refresh
    }

    // The background loop's tick: skipped while the deadline above still holds, whether
    // a manual refresh or a rate limit set it.
    pub async fn tick(&self, store: &Store) -> (Duration, Option<Refresh>) {
        let _one_at_a_time = self.fetching.lock().await;
        let left = self.time_left();

        if !left.is_zero() {
            return (left, None);
        }

        let refresh = self.fetch_locked(store).await;
        let wait = refresh.wait(T::REFRESH_INTERVAL);

        (wait, Some(refresh))
    }

    // The manual refresh button or command: refused under the same deadline, and
    // resyncs the background loop to the automatic schedule this refresh set.
    pub async fn refresh_manual(&self, store: &Store) -> Result<(usize, bool)> {
        let _one_at_a_time = self.fetching.lock().await;

        if !self.time_left().is_zero() {
            return Err(Error::CatalogRateLimited);
        }

        let refresh = self.fetch_locked(store).await;

        *locked(&self.resync) = refresh.wait(T::REFRESH_INTERVAL);
        self.notify.notify_one();

        refresh.result
    }
    async fn apply(&self, fetched: Result<Fetched<T>>, store: &Store) -> Result<(usize, bool)> {
        let mut state = self.state.write().await;

        match fetched {
            Ok(Fetched::New(next)) if next.count() == 0 && state.count() > 0 => {
                // ! Forgotten with the empty answer, or the next read would get a 304 for it.
                self.client.forget_tag();

                let refused = Error::InvalidResponse(format!("an empty {} came back", T::NAME));

                state.set_problem(Some(refused.code()));

                Err(refused)
            }
            Ok(Fetched::New(next)) => {
                let count = next.count();
                let changed = next.meaningfully_changed(&state);
                let json = serde_json::to_string(&next);

                *state = next;
                drop(state);

                if changed {
                    keep(store, T::CACHE_KEY, json, T::NAME).await;
                }

                Ok((count, changed))
            }
            Ok(Fetched::Unchanged) => {
                state.mark_fetched(Utc::now());
                state.set_problem(None);

                Ok((state.count(), false))
            }
            Err(error) => {
                state.set_problem(Some(error.code()));

                Err(error)
            }
        }
    }
}

pub fn cached<T: FeedAnswer>(store: &Store) -> T {
    let kept = match store.get_setting(T::CACHE_KEY) {
        Ok(json) => json.and_then(|json| {
            serde_json::from_str::<T>(&json)
                .inspect_err(|error| {
                    tracing::warn!(%error, "kept {} unreadable, fetched again", T::NAME);
                })
                .ok()
        }),
        Err(error) => {
            tracing::warn!(%error, "kept {} unreadable", T::NAME);
            None
        }
    };
    let now = Utc::now();

    match kept {
        Some(list)
            if list
                .fetched_at()
                .is_some_and(|at| at <= now && now - at < CACHE_MAX_AGE) =>
        {
            tracing::info!(count = list.count(), "{} kept from the last start", T::NAME);
            list
        }
        _ => T::default(),
    }
}

async fn keep(
    store: &Store,
    key: &'static str,
    json: serde_json::Result<String>,
    what: &'static str,
) {
    let kept = async {
        let json = json?;
        let store = store.clone();

        tokio::task::spawn_blocking(move || store.set_setting(key, &json))
            .await
            .map_err(|error| Error::Storage(error.to_string()))?
    }
    .await;

    if let Err(error) = kept {
        tracing::warn!(%error, what, "list not kept for the next start");
    }
}
