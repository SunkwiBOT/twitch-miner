use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{LoginFlowView, Snapshot};
use crate::badges::GlobalBadge;
use crate::domain::{Campaign, Channel};
use crate::error::ErrorCode;
use crate::events::MinerEvent;
use crate::logs::LogLine;
use crate::requests::RequestEntry;
use crate::spam::{SpamBalance, SpamChannelView};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "SnapshotResponseType")]
pub struct SnapshotResponse {
    pub snapshot: Snapshot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "HistoryResponseType")]
pub struct HistoryResponse {
    pub events: Vec<MinerEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_before_id: Option<u64>,
}

// ! Not in the snapshot pushed at every change: the interface asks again when `read_at` moves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "BadgesResponseType")]
pub struct BadgesResponse {
    pub badges: Vec<GlobalBadge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "CatalogResponseType")]
pub struct CatalogResponse {
    pub campaigns: Vec<Campaign>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "LogsResponseType")]
pub struct LogsResponse {
    pub lines: Vec<LogLine>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_before_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RequestsResponseType")]
pub struct RequestsResponse {
    pub requests: Vec<RequestEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_live_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_stored_id: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "CatalogRefreshType")]
pub struct CatalogRefresh {
    pub campaign_count: u32,
    pub next_refresh_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "CatalogRefreshResponseType")]
pub struct CatalogRefreshResponse {
    pub refresh: CatalogRefresh,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "UpdateCheckType")]
pub struct UpdateCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub available: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "UpdateCheckResponseType")]
pub struct UpdateCheckResponse {
    pub check: UpdateCheck,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "LogsReportResponseType")]
pub struct LogsReportResponse {
    pub report: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "LoginFlowResponseType")]
pub struct LoginFlowResponse {
    pub login: LoginFlowView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "ChannelResponseType")]
pub struct ChannelResponse {
    pub channel: Channel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "SearchResponseType")]
pub struct SearchResponse {
    pub games: Vec<crate::domain::SearchGame>,
    pub channels: Vec<crate::domain::SearchChannel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "ChannelBannersResponseType")]
pub struct ChannelBannersResponse {
    pub banners: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "ChannelsLookupResponseType")]
pub struct ChannelsLookupResponse {
    pub channels: Vec<Channel>,
    pub missing: Vec<String>,
    pub unchecked: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "FollowsResponseType")]
pub struct FollowsResponse {
    pub channels: Vec<Channel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum NotificationTarget {
    Discord,
    Telegram,
    Webhook,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "NotificationTestType")]
pub struct NotificationTest {
    pub target: NotificationTarget,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<ErrorCode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "NotificationTestResponseType")]
pub struct NotificationTestResponse {
    pub targets: Vec<NotificationTest>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "ProxyTestResponseType")]
pub struct ProxyTestResponse {
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AuthStatusType")]
pub struct AuthStatus {
    pub authorized: bool,
    pub public: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AuthResponseType")]
pub struct AuthResponse {
    pub auth: AuthStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AppInfoType")]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub platform: String,
    pub data_dir: String,
    // ! The updater would install the app beside a portable executable and leave it old.
    pub portable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AppInfoResponseType")]
pub struct AppInfoResponse {
    pub app: AppInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AutostartResponseType")]
pub struct AutostartResponse {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "StartupProblemType")]
pub struct StartupProblem {
    pub detail: String,
    pub data_dir: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "StartupProblemResponseType")]
pub struct StartupProblemResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<StartupProblem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SpamChannelResponseType")]
pub struct SpamChannelResponse {
    pub channel: SpamChannelView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SpamBalanceResponseType")]
pub struct SpamBalanceResponse {
    pub balance: SpamBalance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RedeemResponseType")]
pub struct RedeemResponse {
    pub burst_id: String,
}
