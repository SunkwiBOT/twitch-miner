use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use axum::http::HeaderName;
use ipnet::IpNet;
use miner_core::api::InstallKind;
use url::Url;

use crate::access::{Access, HTTPS};

const DEFAULT_DATA_DIR: &str = "data";
const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;
// ! A shorter password is said at start, not refused: a running server keeps
// starting.
pub const PASSWORD_MIN_CHARS: usize = 12;
const FILE_SUFFIX: &str = "_FILE";
const PASSWORD: &str = "MINER_PASSWORD";
// ! The value docker-compose.yml and the README show: anyone who read them knows it.
const PASSWORD_EXAMPLE: &str = "change-me";
const PUBLIC_URL: &str = "MINER_PUBLIC_URL";
const VAULT_PASSPHRASE: &str = "MINER_VAULT_PASSPHRASE";
const TRUSTED_HOPS: &str = "MINER_TRUSTED_HOPS";
const TRUSTED_HOPS_DEFAULT: usize = 1;
const TRUSTED_HOPS_MAX: usize = 10;
const PROXY_NETWORKS: &str = "MINER_PROXY_NETWORKS";
const AUTH_HEADER: &str = "MINER_AUTH_HEADER";
const AUTH_USERS: &str = "MINER_AUTH_USERS";
// ! Set by the Docker image only: the same server run by hand is the bare binary.
const CONTAINER: &str = "MINER_CONTAINER";

pub struct ServerEnv {
    pub data_dir: PathBuf,
    pub log_dir: PathBuf,
    pub address: SocketAddr,
    pub access: Access,
    pub vault_passphrase: Option<String>,
    pub install_kind: InstallKind,
    // ! Computed once at start, from values already validated: main.rs only
    // logs them, so a test can assert on this list the way it asserts on
    // `short_password()`, instead of re-deriving the same condition.
    pub warnings: Vec<String>,
}

impl ServerEnv {
    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_lookup(
            |name| std::env::var(name).ok(),
            |path| std::fs::read_to_string(path),
        )
    }

    fn from_lookup(
        lookup: impl Fn(&str) -> Option<String>,
        read: impl Fn(&Path) -> std::io::Result<String>,
    ) -> anyhow::Result<Self> {
        let set = |name: &str| lookup(name).filter(|value| !value.is_empty());
        let secret = |name: &str| read_secret(name, &set, &read);
        let data_dir = set("DATA_DIR").map(PathBuf::from).unwrap_or_else(|| {
            PathBuf::from(miner_core::data_folder_name(
                DEFAULT_DATA_DIR,
                miner_core::release_build(),
            ))
        });
        let log_dir = set("LOG_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| data_dir.join("logs"));
        let host: IpAddr = set("HOST")
            .unwrap_or_else(|| DEFAULT_HOST.into())
            .parse()
            .context("HOST must be an IP address")?;

        let port = match set("PORT") {
            Some(port) => parse_port(&port)?,
            None => DEFAULT_PORT,
        };

        let password = secret(PASSWORD)?;
        let trusted_hops = match set(TRUSTED_HOPS) {
            Some(hops) => parse_trusted_hops(&hops)?,
            None => TRUSTED_HOPS_DEFAULT,
        };
        let public_url = set(PUBLIC_URL)
            .map(|url| parse_public_url(&url).context(PUBLIC_URL))
            .transpose()?;
        let auth_header_value = set(AUTH_HEADER);
        // ! Only a header server refuses a bad MINER_PROXY_NETWORKS: a
        // password-only one started with it on `origin/main` before this
        // feature, and must still start, with a warning instead.
        let strict_networks = auth_header_value.is_some();
        let proxy_networks_value = set(PROXY_NETWORKS);
        let proxy_networks = match &proxy_networks_value {
            Some(networks) => parse_proxy_networks(networks, strict_networks)?,
            None => Vec::new(),
        };

        let mut warnings = Vec::new();

        if !strict_networks && proxy_networks_value.is_some() {
            warnings.extend(loose_proxy_network_warnings(&proxy_networks));
        }

        let auth_header_parsed = auth_header_value
            .map(|header| parse_auth_header(&header).context(AUTH_HEADER))
            .transpose()?;
        let (auth_header, auth_header_display) = match auth_header_parsed {
            Some((name, display)) => (Some(name), Some(display)),
            None => (None, None),
        };
        let auth_users = set(AUTH_USERS)
            .map(|users| parse_auth_users(&users))
            .transpose()?;

        if auth_header.is_some() {
            if public_url.is_none() {
                bail!(
                    "{AUTH_HEADER} needs {PUBLIC_URL}: the header is trusted only behind an HTTPS reverse proxy"
                );
            }

            if proxy_networks_value.is_none() {
                bail!(
                    "{AUTH_HEADER} needs an explicit {PROXY_NETWORKS}: without it, every device of the local network could forge the header"
                );
            }
        } else if auth_users.is_some() {
            bail!(
                "{AUTH_USERS} needs {AUTH_HEADER}: it names the values allowed through that header"
            );
        }

        let Some(password) = password else {
            bail!("{PASSWORD} is required (or {PASSWORD}{FILE_SUFFIX}): the dashboard never runs without a password");
        };

        if password.trim() == PASSWORD_EXAMPLE {
            bail!(
                "{PASSWORD} is still the example \"{PASSWORD_EXAMPLE}\": choose your own password"
            );
        }

        let access = match public_url {
            None => Access::Local { password },
            Some(url) => Access::Public {
                url: Box::new(url),
                password,
                trusted_hops,
                proxy_networks,
                auth_header,
                auth_header_display,
                auth_users,
            },
        };

        let mut env = Self {
            data_dir,
            log_dir,
            address: SocketAddr::new(host, port),
            access,
            vault_passphrase: secret(VAULT_PASSPHRASE)?,
            install_kind: match set(CONTAINER) {
                Some(_) => InstallKind::Docker,
                None => InstallKind::Binary,
            },
            warnings,
        };

        if env.short_password() {
            env.warnings.push(format!(
                "{PASSWORD} has fewer than {PASSWORD_MIN_CHARS} characters: choose a longer one"
            ));
        }

        if let Access::Public {
            auth_header: Some(_),
            proxy_networks,
            auth_users,
            ..
        } = &env.access
        {
            for network in proxy_networks {
                let too_broad = match network {
                    IpNet::V4(_) => network.prefix_len() < 24,
                    IpNet::V6(_) => network.prefix_len() < 120,
                };

                if too_broad {
                    env.warnings.push(format!(
                        "{network} is wider than a single proxy's address: every device it reaches can forge {AUTH_HEADER}"
                    ));
                }
            }

            if auth_users.is_none() {
                env.warnings.push(format!(
                    "{AUTH_HEADER} is set without {AUTH_USERS}: any value the proxy sends signs in, so every signed-in user of your SSO is an admin here"
                ));
            }
        }

        Ok(env)
    }

    pub fn short_password(&self) -> bool {
        self.access.password().chars().count() < PASSWORD_MIN_CHARS
    }
}

fn read_secret(
    name: &str,
    set: &impl Fn(&str) -> Option<String>,
    read: &impl Fn(&Path) -> std::io::Result<String>,
) -> anyhow::Result<Option<String>> {
    let file_variable = format!("{name}{FILE_SUFFIX}");

    match (set(name), set(&file_variable)) {
        (Some(_), Some(_)) => bail!("set {name} or {file_variable}, not both"),
        (value, None) => Ok(value),
        (None, Some(path)) => {
            let content = read(Path::new(&path))
                .with_context(|| format!("{file_variable}: {path} could not be read"))?;
            let value = content.trim_end_matches(['\r', '\n']);

            if value.is_empty() {
                bail!("{file_variable}: {path} is empty");
            }

            Ok(Some(value.to_owned()))
        }
    }
}

fn parse_port(value: &str) -> anyhow::Result<u16> {
    match value.parse::<u16>() {
        Ok(port) if port > 0 => Ok(port),
        _ => bail!("PORT must be an integer from 1 to 65535, got {value:?}"),
    }
}

fn parse_trusted_hops(value: &str) -> anyhow::Result<usize> {
    match value.trim().parse::<usize>() {
        Ok(hops) if (1..=TRUSTED_HOPS_MAX).contains(&hops) => Ok(hops),
        _ => bail!(
            "{TRUSTED_HOPS} must be a whole number from 1 to {TRUSTED_HOPS_MAX}, got {value:?}"
        ),
    }
}

// ! A bare address is a network of one. `strict` (only with MINER_AUTH_HEADER
// set) refuses a blank value, a zero prefix or host bits set; a password-only
// server keeps the looser behaviour it always had, a warning instead.
fn parse_proxy_networks(value: &str, strict: bool) -> anyhow::Result<Vec<IpNet>> {
    let networks: Vec<IpNet> = value
        .split([',', ' ', '\n', '\t'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.parse::<IpNet>()
                .or_else(|_| part.parse::<IpAddr>().map(IpNet::from))
                .map_err(|_| {
                    anyhow::anyhow!(
                        "{PROXY_NETWORKS}: {part:?} is not an address or a network like 192.0.2.0/24 or 2001:db8::/64"
                    )
                })
        })
        .collect::<anyhow::Result<_>>()?;

    if !strict {
        return Ok(networks);
    }

    if networks.is_empty() {
        bail!("{PROXY_NETWORKS} is set but names no address or network");
    }

    for network in &networks {
        if network.prefix_len() == 0 {
            bail!(
                "{PROXY_NETWORKS}: {network} covers every address, name the proxy's own address or network instead"
            );
        }

        if network.addr() != network.network() {
            bail!(
                "{PROXY_NETWORKS}: {network} has host bits set, use {} instead",
                network.trunc()
            );
        }
    }

    Ok(networks)
}

// ! Same conditions `parse_proxy_networks` refuses with a header set, said as
// a warning instead: a password-only server keeps starting with them. A
// malformed entry (not an address or a network at all) already stopped the
// start above, in every mode, so only a blank value reaches the empty case.
fn loose_proxy_network_warnings(networks: &[IpNet]) -> Vec<String> {
    if networks.is_empty() {
        return vec![format!(
            "{PROXY_NETWORKS} is set but names no address or network"
        )];
    }

    networks
        .iter()
        .filter_map(|network| {
            if network.prefix_len() == 0 {
                Some(format!(
                    "{PROXY_NETWORKS}: {network} covers every address, name the proxy's own address or network instead"
                ))
            } else if network.addr() != network.network() {
                Some(format!(
                    "{PROXY_NETWORKS}: {network} has host bits set, use {} instead",
                    network.trunc()
                ))
            } else {
                None
            }
        })
        .collect()
}

fn parse_public_url(value: &str) -> anyhow::Result<Url> {
    let url = Url::parse(value).context("not a URL")?;

    if url.scheme() != HTTPS || url.host().is_none() {
        bail!("must be an https:// address, like https://miner.example.com");
    }

    let bare = url.path() == "/"
        && url.query().is_none()
        && url.fragment().is_none()
        && url.username().is_empty()
        && url.password().is_none();

    if !bare {
        bail!("must be the address alone, without a path, like https://miner.example.com");
    }

    Ok(url)
}

// ! Reserved: the proxy, a browser or an HTTP library writes these itself,
// so a header reusing one of their names could never be told from the real
// one. An underscore is refused too: nginx drops such a header by default
// (`underscores_in_headers off;`), so it would never reach us at all.
fn parse_auth_header(value: &str) -> anyhow::Result<(HeaderName, String)> {
    let trimmed = value.trim();

    if trimmed.contains('_') {
        bail!(
            "{trimmed:?} has an underscore: nginx drops a header like this by default (underscores_in_headers off;), use hyphens instead, like Remote-User"
        );
    }

    let name = HeaderName::from_bytes(trimmed.as_bytes())
        .map_err(|_| anyhow::anyhow!("{trimmed:?} is not a valid HTTP header name"))?;

    if is_reserved_header(name.as_str()) {
        bail!("{trimmed:?} is used by the proxy itself, pick another header name");
    }

    Ok((name, trimmed.to_owned()))
}

fn is_reserved_header(name: &str) -> bool {
    const NAMES: [&str; 15] = [
        "host",
        "cookie",
        "authorization",
        "x-real-ip",
        "forwarded",
        "via",
        "user-agent",
        "connection",
        "origin",
        "referer",
        "x-request-id",
        "content-type",
        "cache-control",
        "cf-connecting-ip",
        "priority",
    ];

    name.starts_with("x-forwarded")
        || name.starts_with("accept")
        || name.starts_with("sec-fetch")
        || NAMES.contains(&name)
}

// ! Commas, spaces and newlines all separate values, like MINER_PROXY_NETWORKS:
// a comma-only split silently turned "alice bob" or "alice\nbob" into one
// value that never matched. Matching stays case-sensitive.
fn parse_auth_users(value: &str) -> anyhow::Result<Vec<String>> {
    let users: Vec<String> = value
        .split([',', ' ', '\n', '\t'])
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_owned)
        .collect();

    if users.is_empty() {
        bail!("{AUTH_USERS} is set but names no value");
    }

    Ok(users)
}
