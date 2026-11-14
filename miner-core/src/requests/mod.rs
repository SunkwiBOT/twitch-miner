mod log;
mod pending;
mod targets;

pub use log::RequestLog;
pub use pending::Pending;
pub use targets::{address_target, host_target, url_target};

use chrono::{DateTime, DurationRound, TimeDelta, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const LIVE_CAPACITY: usize = 1000;
pub const BUCKET_MINUTES: i64 = 10;
pub const REPORT_MAX: usize = 4000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum RequestKind {
    Gql,
    Auth,
    Playlist,
    Segment,
    ChannelPage,
    Beacon,
    Pubsub,
    Chat,
    Catalog,
    Discord,
    Telegram,
    Webhook,
    Picture,
    UpdateCheck,
    UpdateDownload,
    ProxyTest,
}

impl RequestKind {
    pub const ALL: [Self; 16] = [
        Self::Gql,
        Self::Auth,
        Self::Playlist,
        Self::Segment,
        Self::ChannelPage,
        Self::Beacon,
        Self::Pubsub,
        Self::Chat,
        Self::Catalog,
        Self::Discord,
        Self::Telegram,
        Self::Webhook,
        Self::Picture,
        Self::UpdateCheck,
        Self::UpdateDownload,
        Self::ProxyTest,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Gql => "GQL",
            Self::Auth => "AUTH",
            Self::Playlist => "PLAYLIST",
            Self::Segment => "SEGMENT",
            Self::ChannelPage => "CHANNEL_PAGE",
            Self::Beacon => "BEACON",
            Self::Pubsub => "PUBSUB",
            Self::Chat => "CHAT",
            Self::Catalog => "CATALOG",
            Self::Discord => "DISCORD",
            Self::Telegram => "TELEGRAM",
            Self::Webhook => "WEBHOOK",
            Self::Picture => "PICTURE",
            Self::UpdateCheck => "UPDATE_CHECK",
            Self::UpdateDownload => "UPDATE_DOWNLOAD",
            Self::ProxyTest => "PROXY_TEST",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum RequestOutcome {
    Ok,
    Status,
    Timeout,
    Connect,
    Network,
    Graphql,
    Closed,
}

impl RequestOutcome {
    pub const ALL: [Self; 7] = [
        Self::Ok,
        Self::Status,
        Self::Timeout,
        Self::Connect,
        Self::Network,
        Self::Graphql,
        Self::Closed,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Status => "STATUS",
            Self::Timeout => "TIMEOUT",
            Self::Connect => "CONNECT",
            Self::Network => "NETWORK",
            Self::Graphql => "GRAPHQL",
            Self::Closed => "CLOSED",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|outcome| outcome.as_str() == name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum SocketEvent {
    Connect,
    Reconnect,
    Close,
}

impl SocketEvent {
    pub const ALL: [Self; 3] = [Self::Connect, Self::Reconnect, Self::Close];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Connect => "CONNECT",
            Self::Reconnect => "RECONNECT",
            Self::Close => "CLOSE",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|event| event.as_str() == name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum RequestSource {
    Live,
    Stored,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RequestGroupType")]
pub struct RequestGroup {
    pub count: u32,
    pub max_ms: u64,
    pub until: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RequestEntryType")]
pub struct RequestEntry {
    pub id: u64,
    pub source: RequestSource,
    pub at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub account_id: Option<String>,
    pub kind: RequestKind,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub socket: Option<SocketEvent>,
    pub outcome: RequestOutcome,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub status: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub detail: Option<String>,
    pub duration_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub group: Option<RequestGroup>,
}

impl RequestEntry {
    pub fn failed(&self) -> bool {
        self.outcome != RequestOutcome::Ok
    }

    pub fn report_line(&self, login: Option<&str>) -> String {
        let mut line = format!("{} ", self.at.format("%Y-%m-%d %H:%M:%S"));

        if let Some(login) = login {
            line.push_str(&format!("[{login}] "));
        }

        line.push_str(self.kind.as_str());

        if !self.target.is_empty() {
            line.push_str(&format!(" {}", self.target));
        }

        if let Some(socket) = self.socket {
            line.push_str(&format!(" {}", socket.as_str()));
        }

        if let Some(status) = self.status {
            line.push_str(&format!(" {status}"));
        }

        if self.failed() {
            line.push_str(&format!(" {}", self.outcome.as_str()));
        }

        if let Some(detail) = &self.detail {
            line.push_str(&format!(" ({detail})"));
        }

        match &self.group {
            Some(group) => line.push_str(&format!(
                " ×{} until {}, mean {} ms, max {} ms",
                group.count,
                group.until.format("%H:%M"),
                self.duration_ms,
                group.max_ms
            )),
            None => line.push_str(&format!(" {} ms", self.duration_ms)),
        }

        line
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RequestFilter<'a> {
    pub account_id: Option<&'a str>,
    pub kinds: Option<&'a [RequestKind]>,
    pub errors_only: bool,
}

impl RequestFilter<'_> {
    pub fn keeps(&self, entry: &RequestEntry) -> bool {
        self.account_id
            .is_none_or(|account| entry.account_id.as_deref() == Some(account))
            && self.kinds.is_none_or(|kinds| kinds.contains(&entry.kind))
            && (!self.errors_only || entry.failed())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RequestCursor {
    pub live_id: Option<u64>,
    pub stored_id: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RequestPage {
    pub requests: Vec<RequestEntry>,
    pub next: RequestCursor,
}

pub fn bucket_of(at: DateTime<Utc>) -> DateTime<Utc> {
    at.duration_trunc(TimeDelta::minutes(BUCKET_MINUTES))
        .unwrap_or(at)
}

pub fn bucket_end(start: DateTime<Utc>) -> DateTime<Utc> {
    start + TimeDelta::minutes(BUCKET_MINUTES)
}
