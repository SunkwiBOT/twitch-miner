// ! The notes of every version an update skips, from the repository's GitHub releases.
// The `latest.json` notes of the offered version stay the answer whenever GitHub has none.

use std::fmt;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use reqwest::header::{HeaderMap, ACCEPT, RETRY_AFTER};
use reqwest::StatusCode;
use semver::Version;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::api::{AvailableUpdate, InstallKind, VersionNotes};
use crate::error::{Error, Result};
use crate::requests::{address_target, RequestKind, RequestLog};

const GITHUB_SITE: &str = "https://github.com/";
const GITHUB_API: &str = "https://api.github.com";
const GITHUB_JSON: &str = "application/vnd.github+json";
const API_VERSION_HEADER: &str = "X-GitHub-Api-Version";
const API_VERSION: &str = "2022-11-28";
const RATE_LIMIT_LEFT_HEADER: &str = "x-ratelimit-remaining";
const RATE_LIMIT_RESET_HEADER: &str = "x-ratelimit-reset";
// ! The newest releases: a user further behind than this reads the latest ones only.
const PAGE_SIZE: usize = 30;
const MAX_BYTES: usize = 4 * 1024 * 1024;
const TIMEOUT: Duration = Duration::from_secs(10);
// ! GitHub's unauthenticated quota is counted per hour.
const RATE_LIMIT_WAIT: Duration = Duration::from_secs(60 * 60);
const TAG_PREFIX: char = 'v';

pub fn releases_api_url() -> String {
    let repository = crate::REPOSITORY_URL.trim_start_matches(GITHUB_SITE);

    format!("{GITHUB_API}/repos/{repository}/releases?per_page={PAGE_SIZE}")
}

#[derive(Debug, Deserialize)]
pub(crate) struct Release {
    tag_name: String,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    prerelease: bool,
    #[serde(default)]
    published_at: Option<String>,
}

impl Release {
    fn version(&self) -> Option<Version> {
        if self.draft || self.prerelease || self.published_at.is_none() {
            return None;
        }

        let version = Version::parse(self.tag_name.strip_prefix(TAG_PREFIX)?).ok()?;

        (version.pre.is_empty() && version.build.is_empty()).then_some(version)
    }
}

// ! Newest first, one entry per version, only those strictly after `installed` up to `offered`.
pub(crate) fn notes_between(
    releases: &[Release],
    installed: &Version,
    offered: &Version,
) -> Vec<VersionNotes> {
    let mut found: Vec<(Version, VersionNotes)> = releases
        .iter()
        .filter_map(|release| {
            let version = release.version()?;

            if version <= *installed || version > *offered {
                return None;
            }

            let notes = VersionNotes::new(&version.to_string(), release.body.as_deref())?;

            Some((version, notes))
        })
        .collect();

    found.sort_by(|(a, _), (b, _)| b.cmp(a));
    found.dedup_by(|(a, _), (b, _)| a == b);
    found.into_iter().map(|(_, notes)| notes).collect()
}

// ! GitHub may not list the offered release yet: its `latest.json` notes then lead.
// The flag says GitHub's answer was complete, so worth keeping.
pub(crate) fn with_latest(
    mut notes: Vec<VersionNotes>,
    offered: &Version,
    latest: Vec<VersionNotes>,
) -> (Vec<VersionNotes>, bool) {
    let complete = notes
        .first()
        .is_some_and(|first| first.version == offered.to_string());

    if !complete {
        notes.splice(0..0, latest);
    }

    (notes, complete)
}

enum Unread {
    RateLimited(Duration),
    Status(StatusCode),
    Failed(Error),
}

impl fmt::Display for Unread {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RateLimited(wait) => {
                write!(f, "GitHub's rate limit is reached for {}s", wait.as_secs())
            }
            Self::Status(status) => write!(f, "GitHub answered {status}"),
            Self::Failed(error) => write!(f, "{error}"),
        }
    }
}

impl From<Error> for Unread {
    fn from(error: Error) -> Self {
        Self::Failed(error)
    }
}

fn header_seconds(headers: &HeaderMap, name: impl reqwest::header::AsHeaderName) -> Option<u64> {
    headers.get(name)?.to_str().ok()?.trim().parse().ok()
}

// ! A refused answer with no request left waits for the reset GitHub gives, at most an hour.
fn rate_limit_wait(status: StatusCode, headers: &HeaderMap) -> Option<Duration> {
    if status != StatusCode::FORBIDDEN && status != StatusCode::TOO_MANY_REQUESTS {
        return None;
    }

    if let Some(seconds) = header_seconds(headers, RETRY_AFTER) {
        return Some(Duration::from_secs(seconds).min(RATE_LIMIT_WAIT));
    }

    if header_seconds(headers, RATE_LIMIT_LEFT_HEADER) != Some(0) {
        return None;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let wait = header_seconds(headers, RATE_LIMIT_RESET_HEADER)
        .map(|reset| Duration::from_secs(reset.saturating_sub(now)))
        .unwrap_or(RATE_LIMIT_WAIT);

    Some(wait.min(RATE_LIMIT_WAIT))
}

struct Known {
    offered: Version,
    notes: Vec<VersionNotes>,
}

#[derive(Default)]
struct State {
    known: Option<Known>,
    paused_until: Option<Instant>,
}

pub struct ReleaseNotes {
    client: reqwest::Client,
    url: String,
    state: Mutex<State>,
}

impl ReleaseNotes {
    pub fn new(install_kind: InstallKind) -> Result<Self> {
        Self::with_url(&releases_api_url(), install_kind)
    }

    pub fn with_url(url: &str, install_kind: InstallKind) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(crate::user_agent(install_kind))
            .timeout(TIMEOUT)
            .redirect(reqwest::redirect::Policy::none())
            .gzip(true)
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;

        Ok(Self {
            client,
            url: url.to_owned(),
            state: Mutex::new(State::default()),
        })
    }

    // ! Read once per offered version: later checks of the same version reuse the answer.
    pub async fn describe(
        &self,
        requests: &RequestLog,
        version: &str,
        latest_notes: Option<&str>,
    ) -> AvailableUpdate {
        let fallback = AvailableUpdate::new(version.to_owned(), latest_notes);
        let (Ok(installed), Ok(offered)) = (
            Version::parse(crate::VERSION),
            Version::parse(version.strip_prefix(TAG_PREFIX).unwrap_or(version)),
        ) else {
            return fallback;
        };
        let mut state = self.state.lock().await;

        if let Some(known) = state
            .known
            .as_ref()
            .filter(|known| known.offered == offered)
        {
            return AvailableUpdate {
                version: version.to_owned(),
                notes: known.notes.clone(),
            };
        }

        if state
            .paused_until
            .is_some_and(|until| Instant::now() < until)
        {
            return fallback;
        }

        let releases = match self.releases(requests).await {
            Ok(releases) => releases,
            Err(unread) => {
                if let Unread::RateLimited(wait) = unread {
                    state.paused_until = Some(Instant::now() + wait);
                }

                tracing::info!("the release notes were not read: {unread}");

                return fallback;
            }
        };
        let (notes, complete) = with_latest(
            notes_between(&releases, &installed, &offered),
            &offered,
            fallback.notes,
        );

        if complete {
            state.known = Some(Known {
                offered,
                notes: notes.clone(),
            });
        }

        AvailableUpdate {
            version: version.to_owned(),
            notes,
        }
    }

    async fn releases(&self, requests: &RequestLog) -> std::result::Result<Vec<Release>, Unread> {
        let request = self
            .client
            .get(&self.url)
            .header(ACCEPT, GITHUB_JSON)
            .header(API_VERSION_HEADER, API_VERSION);
        let response = requests
            .start(RequestKind::UpdateCheck, address_target(&self.url))
            .send(request)
            .await?;
        let status = response.status();

        if let Some(wait) = rate_limit_wait(status, response.headers()) {
            return Err(Unread::RateLimited(wait));
        }

        if !status.is_success() {
            return Err(Unread::Status(status));
        }

        let body = crate::http::read_bounded(response, MAX_BYTES).await?;

        serde_json::from_slice(&body)
            .map_err(|e| Unread::Failed(Error::InvalidResponse(e.to_string())))
    }
}
