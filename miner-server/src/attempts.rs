use std::collections::HashMap;
use std::net::{IpAddr, Ipv6Addr};
use std::sync::Mutex;
use std::time::{Duration, Instant};

const MAX_FAILURES: u32 = 5;
pub const LOCKOUT: Duration = Duration::from_secs(30);
const MAX_CLIENTS: usize = 1_000;
pub const THROTTLED_GAP: Duration = Duration::from_secs(2);
const CONNECTION_BURST: Duration = Duration::from_secs(60);
// ! An IPv6 subscriber holds a whole /64: counted per address, it could take
// a new one after every few tries.
const IPV6_CLIENT_MASK: u128 = u128::MAX << 64;

pub enum SignIn {
    Accepted,
    Refused,
    LockedOut { retry_after: Duration },
}

struct Client {
    failures: u32,
    locked_until: Option<Instant>,
    last_try: Instant,
}

#[derive(Default)]
pub struct SignInAttempts {
    clients: Mutex<HashMap<IpAddr, Client>>,
    connections: Mutex<HashMap<IpAddr, Instant>>,
}

// ! A dual-stack socket shows an IPv4 visitor as `::ffff:a.b.c.d`: it stays that
// one IPv4 address, not the /64 every IPv4 visitor shares.
pub(crate) fn client_of(address: IpAddr) -> IpAddr {
    match address.to_canonical() {
        IpAddr::V6(address) => IpAddr::V6(Ipv6Addr::from(u128::from(address) & IPV6_CLIENT_MASK)),
        address => address,
    }
}

impl SignInAttempts {
    pub async fn attempt(
        &self,
        visitor: IpAddr,
        connection: IpAddr,
        password_matches: bool,
    ) -> SignIn {
        let reserved = self.throttle_at(connection, Instant::now());
        if let Some(wait) = reserved {
            tokio::time::sleep(wait).await;
        }

        let now = Instant::now();
        let verdict = self.attempt_at(visitor, password_matches, now);
        if matches!(verdict, SignIn::Refused) && reserved.is_none() {
            self.connection_failed_at(connection, now);
        }

        verdict
    }

    // ! A forged `X-Forwarded-For` makes each try a new visitor: past 30 failures a minute the connection's address gets one try every 2 s, slowed and never locked, so the visitors behind a shared proxy still get in.
    pub(crate) fn throttle_at(&self, connection: IpAddr, now: Instant) -> Option<Duration> {
        let mut connections = miner_core::util::locked(&self.connections);
        let due = connections.get_mut(&client_of(connection))?;
        let ahead = due.saturating_duration_since(now) + THROTTLED_GAP;
        if ahead <= CONNECTION_BURST {
            return None;
        }

        *due += THROTTLED_GAP;

        Some(ahead - CONNECTION_BURST)
    }

    pub(crate) fn connection_failed_at(&self, connection: IpAddr, now: Instant) {
        let connection = client_of(connection);
        let mut connections = miner_core::util::locked(&self.connections);
        if !connections.contains_key(&connection) && connections.len() >= MAX_CLIENTS {
            connections.retain(|_, due| *due > now);
            if connections.len() >= MAX_CLIENTS {
                return;
            }
        }

        let due = connections.entry(connection).or_insert(now);
        *due = (*due).max(now) + THROTTLED_GAP;
    }

    // ! While the client is locked out the password is not looked at, so a
    // right guess cannot be told from a wrong one.
    pub(crate) fn attempt_at(
        &self,
        client: IpAddr,
        password_matches: bool,
        now: Instant,
    ) -> SignIn {
        let client = client_of(client);
        let mut clients = miner_core::util::locked(&self.clients);

        if let Some(until) = clients.get(&client).and_then(|known| known.locked_until) {
            if now < until {
                return SignIn::LockedOut {
                    retry_after: until - now,
                };
            }
        }

        if password_matches {
            clients.remove(&client);

            return SignIn::Accepted;
        }

        // ! A running lockout goes last, or a flood of new addresses would lift it.
        if !clients.contains_key(&client) && clients.len() >= MAX_CLIENTS {
            let oldest = clients
                .iter()
                .min_by_key(|(_, known)| {
                    let locked = known.locked_until.is_some_and(|until| now < until);

                    (locked, known.last_try)
                })
                .map(|(address, _)| *address);

            if let Some(oldest) = oldest {
                clients.remove(&oldest);
            }
        }

        let known = clients.entry(client).or_insert(Client {
            failures: 0,
            locked_until: None,
            last_try: now,
        });

        if known.locked_until.is_some() {
            known.failures = 0;
            known.locked_until = None;
        }

        known.failures += 1;
        known.last_try = now;

        if known.failures >= MAX_FAILURES {
            known.locked_until = Some(now + LOCKOUT);
        }

        SignIn::Refused
    }
}
