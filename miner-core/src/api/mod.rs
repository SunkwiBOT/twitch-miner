mod command;
mod push;
mod queries;
mod responses;
mod views;

pub use command::{Command, CommandName};
pub use push::{DesktopEvent, ServerMessage, ServerMessageKind};
pub use queries::{
    ChannelBannersRequest, ChannelQuery, ChannelsLookupRequest, CheckPastedTokenRequest,
    DashboardLoginRequest, FollowsQuery, HistoryCurveQuery, HistoryExportQuery, HistoryQuery,
    HistoryStatsQuery, ImportTokensRequest, LogsQuery, NotificationTestRequest, ProxyTestRequest,
    RedeemRequest, RequestsQuery, SearchQuery, SpamChannelQuery,
};
pub use responses::{
    AppInfo, AppInfoResponse, AuthResponse, AuthStatus, AutostartResponse, BadgesResponse,
    CatalogResponse, CategoryCount, ChannelBannersResponse, ChannelResponse,
    ChannelsLookupResponse, CheckPastedTokenResponse, DropsByCategory, FollowsResponse,
    HistoryCurveResponse, HistoryExportResponse, HistoryResponse, HistoryStatsResponse,
    ImportTokensResponse, LoginFlowResponse, LogsReportResponse, LogsResponse, NotificationTarget,
    NotificationTest, PointsByDay, ProxyTestResponse, RedeemResponse, RefreshCount,
    RefreshResponse, RequestsResponse, SearchResponse, SnapshotResponse, SpamBalanceResponse,
    SpamChannelResponse, StartupProblem, StartupProblemResponse, StreamerMinutes, StreamerPoints,
    StreamerPredictions, UpdateCheck, UpdateCheckResponse,
};
pub use views::{
    AccountSnapshot, AccountStats, AccountStatus, AvailableUpdate, BadgeCatalogView, BinaryAsset,
    CampaignProgress, CatalogView, CoreLimits, DropProgressView, DropState, ImportTokenOutcome,
    ImportTokensFlowStatus, ImportTokensFlowView, LoginFlowStatus, LoginFlowView, SlotView,
    Snapshot, StreamerView, TokenCheck, VersionNotes,
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

// ! How this copy runs, which also says how it updates: the desktop app installs its own
// updates, the Docker image is pulled again, and the bare server binary is replaced by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum InstallKind {
    Desktop,
    Docker,
    Binary,
}

impl InstallKind {
    pub fn is_server(self) -> bool {
        self != Self::Desktop
    }
}

// ! Matches the release workflow's Linux server asset names
// (`twitch-miner-server_linux_x64`/`_linux_arm64`, `.github/workflows/release.yml`):
// the only other place this name is built.
pub const BINARY_ASSET_PREFIX: &str = "twitch-miner-server_";

// ! `std::env::consts` names the processor the way Rust does (`x86_64`/`aarch64`); the
// release asset names it the way the Docker build's platform folders do instead. Only
// Linux has a release asset: a server built elsewhere gets no update steps.
pub fn binary_asset_arch() -> Option<&'static str> {
    asset_arch(std::env::consts::OS, std::env::consts::ARCH)
}

fn asset_arch(os: &str, arch: &str) -> Option<&'static str> {
    match (os, arch) {
        ("linux", "x86_64") => Some("linux_x64"),
        ("linux", "aarch64") => Some("linux_arm64"),
        _ => None,
    }
}

pub const SNAPSHOT_PUSH_DEBOUNCE_MS: u64 = 700;

pub const CHANNELS_LOOKUP_MAX: usize = 500;

pub const FOLLOWS_MAX: usize = 1_000;

// ! Each line is checked one at a time against Twitch's own rate limit: a
// short list stays quick, a long one would make the sequential check feel stuck.
pub const TOKENS_IMPORT_MAX: usize = 50;

// ! Comfortably past any real token (the `oauth:`/`OAuth ` prefix included):
// a longer line is pasted garbage, refused before it ever reaches Twitch.
pub const TOKEN_LINE_MAX_CHARS: usize = 256;

pub const HISTORY_LIMIT_MAX: u32 = 500;
pub const HISTORY_LIMIT_DEFAULT: u32 = 100;
pub const HISTORY_EXPORT_MAX: usize = 10_000;
// ! How many rows the stats page's "top streamers" lists keep, past the one
// shown in a tile's empty space.
pub const HISTORY_STATS_TOP_MAX: usize = 10;
// ! The stats page and the per-streamer curve are asked for one bounded span
// at a time, never the whole retention window; this caps it regardless of
// `history_retention`, well above any period the UI offers.
pub const HISTORY_STATS_MAX_SPAN_DAYS: i64 = 400;

pub const LOGS_LIMIT_DEFAULT: u32 = 100;
pub const LOGS_LIMIT_MAX: u32 = 100;

pub const REQUESTS_LIMIT_DEFAULT: u32 = 100;
pub const REQUESTS_LIMIT_MAX: u32 = 500;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RoutesApi", repr(enum = name))]
pub enum ApiRoute {
    #[serde(rename = "/api/health")]
    Health,
    #[serde(rename = "/api/auth")]
    Auth,
    #[serde(rename = "/api/activity")]
    Activity,
    #[serde(rename = "/api/snapshot")]
    Snapshot,
    #[serde(rename = "/api/command")]
    Command,
    #[serde(rename = "/api/catalog")]
    Catalog,
    #[serde(rename = "/api/updates/check")]
    UpdatesCheck,
    #[serde(rename = "/api/login-flow")]
    LoginFlow,
    #[serde(rename = "/api/login-flow/check-token")]
    CheckPastedToken,
    #[serde(rename = "/api/login-flow/import-tokens")]
    ImportTokens,
    #[serde(rename = "/api/channel")]
    Channel,
    #[serde(rename = "/api/channels/lookup")]
    ChannelsLookup,
    #[serde(rename = "/api/search")]
    Search,
    #[serde(rename = "/api/channels/banners")]
    ChannelBanners,
    #[serde(rename = "/api/follows")]
    Follows,
    #[serde(rename = "/api/notifications/test")]
    NotificationsTest,
    #[serde(rename = "/api/proxy/test")]
    ProxyTest,
    #[serde(rename = "/api/history")]
    History,
    #[serde(rename = "/api/history/export")]
    HistoryExport,
    #[serde(rename = "/api/history/stats")]
    HistoryStats,
    #[serde(rename = "/api/history/curve")]
    HistoryCurve,
    #[serde(rename = "/api/badges")]
    Badges,
    #[serde(rename = "/api/badges/refresh")]
    BadgesRefresh,
    #[serde(rename = "/api/logs")]
    Logs,
    #[serde(rename = "/api/logs/report")]
    LogsReport,
    #[serde(rename = "/api/requests")]
    Requests,
    #[serde(rename = "/api/requests/report")]
    RequestsReport,
    #[serde(rename = "/api/events")]
    Events,
    #[serde(rename = "/api/spam/channel")]
    SpamChannel,
    #[serde(rename = "/api/spam/balance")]
    SpamBalance,
    #[serde(rename = "/api/spam/redeem")]
    SpamRedeem,
}

impl ApiRoute {
    pub fn path(self) -> &'static str {
        match self {
            Self::Health => "/api/health",
            Self::Auth => "/api/auth",
            Self::Activity => "/api/activity",
            Self::Snapshot => "/api/snapshot",
            Self::Command => "/api/command",
            Self::Catalog => "/api/catalog",
            Self::UpdatesCheck => "/api/updates/check",
            Self::LoginFlow => "/api/login-flow",
            Self::CheckPastedToken => "/api/login-flow/check-token",
            Self::ImportTokens => "/api/login-flow/import-tokens",
            Self::Channel => "/api/channel",
            Self::ChannelsLookup => "/api/channels/lookup",
            Self::Search => "/api/search",
            Self::ChannelBanners => "/api/channels/banners",
            Self::Follows => "/api/follows",
            Self::NotificationsTest => "/api/notifications/test",
            Self::ProxyTest => "/api/proxy/test",
            Self::History => "/api/history",
            Self::HistoryExport => "/api/history/export",
            Self::HistoryStats => "/api/history/stats",
            Self::HistoryCurve => "/api/history/curve",
            Self::Badges => "/api/badges",
            Self::BadgesRefresh => "/api/badges/refresh",
            Self::Logs => "/api/logs",
            Self::LogsReport => "/api/logs/report",
            Self::Requests => "/api/requests",
            Self::RequestsReport => "/api/requests/report",
            Self::Events => "/api/events",
            Self::SpamChannel => "/api/spam/channel",
            Self::SpamBalance => "/api/spam/balance",
            Self::SpamRedeem => "/api/spam/redeem",
        }
    }
}

// ! The hosts Twitch's channel-points beacon needs reachable: the first one, then the names it points to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(repr(enum = name))]
pub enum BeaconHost {
    #[serde(rename = "spade.twitch.tv")]
    Spade,
    #[serde(rename = "spade.sci.twitch.tv")]
    SpadeScience,
    #[serde(rename = "science-edge-external-prod-*.elb.amazonaws.com")]
    ScienceEdge,
}

impl BeaconHost {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Spade => "spade.twitch.tv",
            Self::SpadeScience => "spade.sci.twitch.tv",
            Self::ScienceEdge => "science-edge-external-prod-*.elb.amazonaws.com",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(repr(enum = name))]
pub enum TwitchPage {
    // ! Where a device code is typed, when Twitch's device flow names none.
    #[serde(rename = "https://www.twitch.tv/activate")]
    Activate,
}

impl TwitchPage {
    pub const fn url(self) -> &'static str {
        match self {
            Self::Activate => "https://www.twitch.tv/activate",
        }
    }
}

// ! The headers a reverse proxy in front of the dashboard sends, as the server reads them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(repr(enum = name))]
pub enum ForwardedHeader {
    #[serde(rename = "x-forwarded-for")]
    For,
    #[serde(rename = "x-forwarded-proto")]
    Proto,
}

impl ForwardedHeader {
    pub const fn name(self) -> &'static str {
        match self {
            Self::For => "x-forwarded-for",
            Self::Proto => "x-forwarded-proto",
        }
    }
}
