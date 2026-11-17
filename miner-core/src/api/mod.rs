mod command;
mod push;
mod queries;
mod responses;
mod views;

pub use command::{Command, CommandName};
pub use push::{DesktopEvent, ServerMessage, ServerMessageKind};
pub use queries::{
    ChannelBannersRequest, ChannelQuery, ChannelsLookupRequest, CheckPastedTokenRequest,
    DashboardLoginRequest, FollowsQuery, HistoryExportQuery, HistoryQuery, ImportTokensRequest,
    LogsQuery, NotificationTestRequest, ProxyTestRequest, RedeemRequest, RequestsQuery,
    SearchQuery, SpamChannelQuery,
};
pub use responses::{
    AppInfo, AppInfoResponse, AuthResponse, AuthStatus, AutostartResponse, BadgesResponse,
    CatalogResponse, CategoryCount, ChannelBannersResponse, ChannelResponse,
    ChannelsLookupResponse, CheckPastedTokenResponse, FollowsResponse, HistoryExportResponse,
    HistoryResponse, ImportTokensResponse, LoginFlowResponse, LogsReportResponse, LogsResponse,
    NotificationTarget, NotificationTest, ProxyTestResponse, RedeemResponse, RefreshCount,
    RefreshResponse, RequestsResponse, SearchResponse, SnapshotResponse, SpamBalanceResponse,
    SpamChannelResponse, StartupProblem, StartupProblemResponse, UpdateCheck, UpdateCheckResponse,
};
pub use views::{
    AccountSnapshot, AccountStats, AccountStatus, BadgeCatalogView, CampaignProgress, CatalogView,
    DropProgressView, DropState, ImportTokenOutcome, ImportTokensFlowStatus, ImportTokensFlowView,
    LoginFlowStatus, LoginFlowView, SlotView, Snapshot, StreamerView, TokenCheck,
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Desktop,
    Server,
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
    #[serde(rename = "/api/catalog/refresh")]
    CatalogRefresh,
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
            Self::CatalogRefresh => "/api/catalog/refresh",
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
