// ! A browser never sends a `Secure` cookie over plain HTTP: the local mode
// cannot set one, and the public mode must.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::sync::Once;
use std::time::Duration;

use axum::http::{header, HeaderMap, HeaderName, HeaderValue, Method};
use ipnet::IpNet;
use miner_core::api::ForwardedHeader;
use miner_core::error::ErrorCode;
use url::Url;

use crate::sessions::{PUBLIC_SESSION_COOKIE, SESSION_COOKIE};

pub const HTTPS: &str = "https";

const X_FORWARDED_FOR: HeaderName = HeaderName::from_static(ForwardedHeader::For.name());
const X_FORWARDED_PROTO: HeaderName = HeaderName::from_static(ForwardedHeader::Proto.name());
const SEC_FETCH_SITE: HeaderName = HeaderName::from_static("sec-fetch-site");
const SAME_ORIGIN: &str = "same-origin";
// ! Values a proxy sends for a request it could not authenticate itself:
// never a real sign-in, whatever the header name.
const PLACEHOLDER_VALUES: [&str; 9] = [
    "(null)",
    "null",
    "anonymous",
    "-",
    "undefined",
    "none",
    "guest",
    "--",
    ",",
];

static HOST_MISMATCH_WARNING: Once = Once::new();
static DOUBLED_HEADER_WARNING: Once = Once::new();
static ALLOW_LIST_REFUSAL_WARNING: Once = Once::new();

pub enum Access {
    Local {
        password: String,
    },
    Public {
        // ! Boxed: Url itself is 88 bytes, more than three times Access::Local
        // as a whole, which would otherwise trip clippy's large_enum_variant.
        url: Box<Url>,
        password: String,
        trusted_hops: usize,
        proxy_networks: Vec<IpNet>,
        // ! Only ever `Some` with an explicit `MINER_PROXY_NETWORKS`: env.rs
        // refuses to start otherwise, so every device of the local network
        // cannot forge the header.
        auth_header: Option<HeaderName>,
        // ! The same header's spelling as typed in MINER_AUTH_HEADER:
        // HeaderName itself always lowercases, so this is kept only to show
        // the operator's own proxy header name back in the Settings card.
        auth_header_display: Option<String>,
        // ! `None` keeps every signed-in value, with a startup warning:
        // env.rs refuses an explicit but empty list.
        auth_users: Option<Vec<String>>,
    },
}

impl Access {
    pub fn password(&self) -> &str {
        match self {
            Self::Local { password } | Self::Public { password, .. } => password,
        }
    }

    pub fn is_public(&self) -> bool {
        matches!(self, Self::Public { .. })
    }

    // ! The configured header's own name, in the spelling MINER_AUTH_HEADER
    // was given: shown back to the signed-in user in the Settings card, so
    // they know which proxy header it reads.
    pub fn auth_header_name(&self) -> Option<&str> {
        match self {
            Self::Local { .. } => None,
            Self::Public {
                auth_header_display,
                ..
            } => auth_header_display.as_deref(),
        }
    }

    // ! `header_trusted` comes from `admit`: only a request from inside the
    // explicit `MINER_PROXY_NETWORKS` (loopback alone is not enough, unlike
    // for the password path) with a `Host` matching `MINER_PUBLIC_URL` earns
    // it, so a weaker admission path can never smuggle a forged header in.
    pub fn trusted_header_value<'h>(
        &self,
        headers: &'h HeaderMap,
        header_trusted: bool,
    ) -> Option<&'h str> {
        if !header_trusted {
            return None;
        }

        let Self::Public {
            auth_header,
            auth_users,
            ..
        } = self
        else {
            return None;
        };
        let name = auth_header.as_ref()?;
        let mut values = headers.get_all(name).iter();
        let raw = values.next()?;

        // ! More than one line is ambiguous: a proxy that adds its own
        // alongside a client-sent one, instead of replacing it, must not let
        // either win.
        if values.next().is_some() {
            DOUBLED_HEADER_WARNING.call_once(|| {
                tracing::warn!(
                    target: "miner_server",
                    "a request carried {name} more than once: neither line was trusted"
                );
            });

            return None;
        }

        let value = raw.to_str().ok()?.trim();

        // ! `to_str` already refuses a value that is not valid UTF-8, but a
        // horizontal tab or another control character survives it (and
        // `trim` only strips the ends): neither is a name a human typed.
        if value.is_empty() || is_placeholder(value) || value.chars().any(char::is_control) {
            return None;
        }

        match auth_users {
            Some(allowed) => {
                let found = allowed.iter().any(|user| user == value);

                if !found {
                    ALLOW_LIST_REFUSAL_WARNING.call_once(|| {
                        tracing::warn!(
                            target: "miner_server",
                            "a request's {name} value is not in MINER_AUTH_USERS: refused"
                        );
                    });
                }

                found.then_some(value)
            }
            None => Some(value),
        }
    }

    pub fn cookie_name(&self) -> &'static str {
        match self {
            Self::Local { .. } => SESSION_COOKIE,
            Self::Public { .. } => PUBLIC_SESSION_COOKIE,
        }
    }

    pub fn session_cookie(&self, value: &str, max_age: Duration) -> HeaderValue {
        let name = self.cookie_name();
        let secure = if self.is_public() { "; Secure" } else { "" };
        let cookie = format!(
            "{name}={value}; Path=/; HttpOnly; SameSite=Strict; Max-Age={}{secure}",
            max_age.as_secs()
        );

        HeaderValue::from_str(&cookie).expect("the session value is URL-safe base64")
    }

    // ! A request without `Origin` does not come from a browser page.
    pub fn allows_origin(&self, headers: &HeaderMap) -> bool {
        let Some(origin) = headers.get(header::ORIGIN).and_then(|v| v.to_str().ok()) else {
            return true;
        };

        match self {
            Self::Local { .. } => origin_matches_host(origin, headers),
            // ! The proxy may rewrite `Host` to an internal name; the public
            // address is the one the browser shows.
            Self::Public { url, .. } => {
                Url::parse(origin).is_ok_and(|origin| origin.origin() == url.origin())
            }
        }
    }

    // ! Another port of this host is the same site to the browser, so
    // `SameSite=Strict` still sends the cookie: only reads are let through from
    // another site or port. An older browser names the source in `Origin` only, not
    // in `Sec-Fetch-Site`.
    pub fn allows_write(&self, method: &Method, headers: &HeaderMap) -> bool {
        if method.is_safe() {
            return true;
        }

        match headers.get(SEC_FETCH_SITE).and_then(|v| v.to_str().ok()) {
            Some(site) => site.eq_ignore_ascii_case(SAME_ORIGIN),
            None => self.allows_origin(headers),
        }
    }

    // ! Without MINER_PUBLIC_URL every visitor of a proxy shares the proxy's
    // address: one person's wrong passwords lock the sign-in for all.
    pub fn behind_unknown_proxy(&self, headers: &HeaderMap) -> bool {
        matches!(self, Self::Local { .. })
            && (headers.contains_key(X_FORWARDED_FOR) || headers.contains_key(X_FORWARDED_PROTO))
    }

    // ! Behind the proxy every connection comes from the proxy itself: a request
    // without what the proxy adds reached the published port directly, or the proxy
    // is set up wrong, and the password and the cookie must not travel over it.
    // ! The second element is whether the header may be trusted: loopback
    // alone admits the password path (unless it is itself listed), but never
    // the header, and a `Host` not matching `MINER_PUBLIC_URL` never does
    // either (a DNS-rebound page, another container on the same network).
    pub fn admit(&self, headers: &HeaderMap, socket: IpAddr) -> Result<(IpAddr, bool), ErrorCode> {
        match self {
            Self::Local { .. } => Ok((socket, false)),
            Self::Public {
                trusted_hops,
                proxy_networks,
                auth_header,
                url,
                ..
            } => {
                let proxy = socket.to_canonical();
                let in_networks = proxy_networks
                    .iter()
                    .any(|network| network.contains(&proxy));
                let trusted = if proxy_networks.is_empty() {
                    from_local_network(proxy)
                } else {
                    proxy.is_loopback() || in_networks
                };

                if !trusted {
                    return Err(ErrorCode::ProxyNotLocal);
                }

                if !over_https(headers, *trusted_hops) {
                    return Err(ErrorCode::HttpsRequired);
                }

                let visitor =
                    forwarded_for(headers, *trusted_hops).ok_or(ErrorCode::ForwardedForRequired)?;
                let host_ok = host_matches(headers, url);

                if let Some(name) = auth_header {
                    if in_networks && headers.contains_key(name) && !host_ok {
                        HOST_MISMATCH_WARNING.call_once(|| {
                            tracing::warn!(
                                target: "miner_server",
                                "a request from the trusted proxy network carried {name} with a Host that does not match MINER_PUBLIC_URL: keep the proxy's original Host (nginx: proxy_set_header Host $host;, Apache: ProxyPreserveHost On)"
                            );
                        });
                    }
                }

                let header_trusted = auth_header.is_some() && in_networks && host_ok;

                Ok((visitor, header_trusted))
            }
        }
    }
}

fn is_placeholder(value: &str) -> bool {
    PLACEHOLDER_VALUES
        .iter()
        .any(|placeholder| value.eq_ignore_ascii_case(placeholder))
}

// ! A browser cannot forge `Host`: a visitor whose page runs on the server
// itself (DNS rebinding) or on another container of the same proxy network
// still names its own origin there, never the dashboard's public one.
// ! No `String` built per request: the port (if any) is compared on its
// own instead of formatting "{host}:{port}" to match against.
fn host_matches(headers: &HeaderMap, url: &Url) -> bool {
    let Some(host) = headers.get(header::HOST).and_then(|v| v.to_str().ok()) else {
        return false;
    };
    let Some(expected_host) = url.host_str() else {
        return false;
    };

    match url.port() {
        Some(port) => host.rsplit_once(':').is_some_and(|(name, given_port)| {
            name.eq_ignore_ascii_case(expected_host) && given_port.parse() == Ok(port)
        }),
        None => host.eq_ignore_ascii_case(expected_host),
    }
}

// ! Every device of the local network can reach the port and write its own `X-Forwarded-*`, and Docker's userland proxy relays IPv6 visitors from its bridge gateway: MINER_PROXY_NETWORKS narrows this to the real proxy.
fn from_local_network(socket: IpAddr) -> bool {
    match socket.to_canonical() {
        IpAddr::V4(ip) => {
            ip.is_loopback() || ip.is_private() || ip.is_link_local() || shared_space(ip)
        }
        IpAddr::V6(ip) => ip.is_loopback() || unique_local(ip) || link_local(ip),
    }
}

// ! 100.64.0.0/10: carrier-grade NAT, and the addresses Tailscale hands out.
fn shared_space(ip: Ipv4Addr) -> bool {
    let [first, second, ..] = ip.octets();

    first == 100 && second & 0xc0 == 0x40
}

fn unique_local(ip: Ipv6Addr) -> bool {
    ip.segments()[0] & 0xfe00 == 0xfc00
}

fn link_local(ip: Ipv6Addr) -> bool {
    ip.segments()[0] & 0xffc0 == 0xfe80
}

fn entries<'a>(headers: &'a HeaderMap, name: &HeaderName) -> Option<Vec<&'a str>> {
    let mut entries = Vec::new();

    for value in headers.get_all(name) {
        entries.extend(value.to_str().ok()?.split(',').map(str::trim));
    }

    Some(entries)
}

// ! Proxies usually replace `X-Forwarded-Proto` instead of adding to it: a list
// shorter than the trusted hops is read from its first entry. Traefik (and
// others) write `wss` instead of `https` for a WebSocket upgrade behind a TLS
// entrypoint (traefik/traefik#6388): the push channel must not be refused for
// writing the TLS-equivalent, correct value under its own name.
fn over_https(headers: &HeaderMap, trusted_hops: usize) -> bool {
    const WSS: &str = "wss";

    entries(headers, &X_FORWARDED_PROTO).is_some_and(|protocols| {
        protocols
            .get(protocols.len().saturating_sub(trusted_hops))
            .is_some_and(|protocol| {
                protocol.eq_ignore_ascii_case(HTTPS) || protocol.eq_ignore_ascii_case(WSS)
            })
    })
}

fn origin_matches_host(origin: &str, headers: &HeaderMap) -> bool {
    let host = headers.get(header::HOST).and_then(|v| v.to_str().ok());
    let origin_host = Url::parse(origin).ok().and_then(|url| {
        let host = url.host_str()?.to_owned();

        Some(match url.port() {
            Some(port) => format!("{host}:{port}"),
            None => host,
        })
    });

    matches!((origin_host, host), (Some(a), Some(b)) if a.eq_ignore_ascii_case(b))
}

// ! Each proxy appends the address it received the request from; whatever
// stands before the trusted hops was written by the client and can be forged.
fn forwarded_for(headers: &HeaderMap, trusted_hops: usize) -> Option<IpAddr> {
    let addresses = entries(headers, &X_FORWARDED_FOR)?;

    addresses
        .get(addresses.len().checked_sub(trusted_hops)?)?
        .parse()
        .ok()
}
