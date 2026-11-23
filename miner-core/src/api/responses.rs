use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{AvailableUpdate, ImportTokensFlowView, LoginFlowView, Snapshot, TokenCheck};
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

// ! Each row is one day's total for one reason: `day_start` is one of the
// caller's own `day_boundaries` (DST-safe, see `HistoryStatsQuery`), the
// UI formats its label in its own zone rather than parsing a server date
// string; `reason` is Twitch's own `reason_code` on the `PointsEarned` row,
// the single source of earned points.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "PointsByDayType")]
pub struct PointsByDay {
    pub day_start: DateTime<Utc>,
    pub reason: String,
    pub points: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "DropsByCategoryType")]
pub struct DropsByCategory {
    // ! `None` for a claim with no `game` in its own data: a real category
    // name is never conflated with "no category" (the UI labels this case
    // itself, rather than matching a server-chosen placeholder string).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub category: Option<String>,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "StreamerPointsType")]
pub struct StreamerPoints {
    pub login: String,
    pub points: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "StreamerMinutesType")]
pub struct StreamerMinutes {
    pub login: String,
    pub minutes: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "StreamerPredictionsType")]
pub struct StreamerPredictions {
    pub login: String,
    pub won: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "HistoryStatsResponseType")]
pub struct HistoryStatsResponse {
    pub points_by_day: Vec<PointsByDay>,
    pub drops_by_category: Vec<DropsByCategory>,
    pub top_streamers_by_points: Vec<StreamerPoints>,
    pub points_total: i64,
    pub drops_claimed_total: u32,
    pub minutes_watched_total: i64,
    pub predictions_won_total: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub top_points_streamer: Option<StreamerPoints>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub top_category: Option<DropsByCategory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub top_watched_streamer: Option<StreamerMinutes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub top_predictions_streamer: Option<StreamerPredictions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "HistoryCurveResponseType")]
pub struct HistoryCurveResponse {
    pub events: Vec<MinerEvent>,
    // ! What the query was clamped to (`history_retention`, the hard span
    // cap): the curve's X domain follows this, not the client's request.
    pub since: DateTime<Utc>,
    pub until: DateTime<Utc>,
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
#[ts(rename = "RefreshCountType")]
pub struct RefreshCount {
    pub count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RefreshResponseType")]
pub struct RefreshResponse {
    pub refresh: RefreshCount,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "UpdateCheckType")]
pub struct UpdateCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub available: Option<AvailableUpdate>,
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
#[ts(rename = "CheckPastedTokenResponseType")]
pub struct CheckPastedTokenResponse {
    pub check: TokenCheck,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "ImportTokensResponseType")]
pub struct ImportTokensResponse {
    pub import: ImportTokensFlowView,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "ProxyTestResponseType")]
pub struct ProxyTestResponse {
    pub latency_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "CategoryCountType")]
pub struct CategoryCount {
    pub category: String,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "HistoryExportResponseType")]
pub struct HistoryExportResponse {
    // ! UTF-8 with a BOM and CRLF line endings, RFC 4180 escaping already
    // applied: the browser saves it as-is, nothing left for the UI to get wrong.
    pub csv: String,
    // ! Over every matching claim, not just the CSV body: accurate even once
    // the export hits `truncated`.
    pub counts: Vec<CategoryCount>,
    // ! The CSV body stops at `HISTORY_EXPORT_MAX` rows; `counts` does not.
    pub truncated: bool,
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
    pub code: ErrorCode,
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
