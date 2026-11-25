use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::config::{AccountSettings, AccountSettingsPatch, GlobalSettings};
use crate::domain::{BadgeRef, Campaign, Channel, PointsState};
use crate::error::ErrorCode;
use crate::events::{MinerEvent, MinerKind};
use crate::scheduler::{LeftOutCampaign, QueueItem, SlotPurpose};
use crate::spam::SpamChannelEntry;
use crate::store::{AccountSource, VaultStatus};
use crate::twitch::identity::ClientPersona;
use crate::twitch::quests::Quest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum AccountStatus {
    #[default]
    Starting,
    Mining,
    Idle,
    Paused,
    LoginRequired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "DropProgressViewType")]
pub struct DropProgressView {
    pub campaign_id: String,
    pub campaign_name: String,
    pub game: String,
    pub drop_id: String,
    pub drop_name: String,
    pub current_minutes: u32,
    pub required_minutes: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "SlotViewType")]
pub struct SlotView {
    pub channel: Channel,
    pub purpose: SlotPurpose,
    // ! A Drops slot watching for a badge says Badges.
    pub miner: MinerKind,
    pub since: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub last_beacon_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub segments_requested: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub drop: Option<DropProgressView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub points: Option<PointsState>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "StreamerViewType")]
pub struct StreamerView {
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub channel: Option<Channel>,
    pub points: PointsState,
    #[serde(default)]
    pub watching: bool,
    #[serde(default)]
    pub streak_at_risk: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "AccountStatsType")]
pub struct AccountStats {
    pub drops_claimed: u64,
    pub points_earned: i64,
    pub minutes_watched: u64,
    pub bonuses_claimed: u64,
    pub predictions_won: u64,
    pub predictions_lost: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "AccountSnapshotType")]
pub struct AccountSnapshot {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub source: AccountSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub avatar_url: Option<String>,
    pub settings: AccountSettings,
    pub status: AccountStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<ErrorCode>,
    pub slots: Vec<SlotView>,
    pub queue: Vec<QueueItem>,
    // ! From the same plan as `queue`: every running campaign out of it, with why.
    #[serde(default)]
    pub left_out: Vec<LeftOutCampaign>,
    pub streamers: Vec<StreamerView>,
    // ! A catalogue campaign comes as this account's state only: the interface adds the catalogue.
    pub progress: Vec<CampaignProgress>,
    pub other_campaigns: Vec<Campaign>,
    #[serde(default)]
    pub quests: Vec<Quest>,
    #[serde(default)]
    pub owned_badges: Vec<BadgeRef>,
    // ! By badge key: when Twitch last awarded the reward that gives it, when known.
    #[serde(default)]
    pub owned_badge_dates: HashMap<String, DateTime<Utc>>,
    // ! By drop id: when Twitch awarded a claimed drop's rewards, inside its window.
    #[serde(default)]
    pub drop_dates: HashMap<String, DateTime<Utc>>,
    // ! The drops whose claim Twitch refused, until their link state changes.
    #[serde(default)]
    pub refused_claims: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub badges_read_at: Option<DateTime<Utc>>,
    pub stats: AccountStats,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub spam_channels: Vec<SpamChannelEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "CampaignProgressType")]
pub struct CampaignProgress {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub linked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub link_url: Option<String>,
    pub in_inventory: bool,
    // ! Only the rewards whose state differs from the catalogue's.
    pub drops: Vec<DropState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "DropStateType")]
pub struct DropState {
    pub id: String,
    pub current_minutes: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub instance_id: Option<String>,
    pub claimed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub preconditions_met: Option<bool>,
    pub badge_owned: bool,
    pub current_subs: u32,
    pub periods_done: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_period_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "CatalogViewType")]
pub struct CatalogView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub read_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub fetched_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<ErrorCode>,
    pub interval_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "BadgeCatalogViewType")]
pub struct BadgeCatalogView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub read_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub fetched_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<ErrorCode>,
    pub interval_seconds: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_refresh_at: Option<DateTime<Utc>>,
    // ! The manual refresh button's own cooldown, always set: unlike `next_refresh_at`,
    // which only carries the auto-retry schedule shown while a problem is active.
    pub next_manual_refresh_at: DateTime<Utc>,
}

// ! Only present for `InstallKind::Binary`, built from the one shared prefix constant
// and the running server's own processor (see `binary_asset_arch`): the exact file to install.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "BinaryAssetType")]
pub struct BinaryAsset {
    pub prefix: String,
    pub arch: String,
}

// ! Markdown written for one release.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "VersionNotesType")]
pub struct VersionNotes {
    pub version: String,
    pub notes: String,
}

impl VersionNotes {
    pub fn new(version: &str, notes: Option<&str>) -> Option<Self> {
        notes
            .map(str::trim)
            .filter(|notes| !notes.is_empty())
            .map(|notes| Self {
                version: version.to_owned(),
                notes: notes.to_owned(),
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "AvailableUpdateType")]
pub struct AvailableUpdate {
    pub version: String,
    // ! Every version from the installed one (excluded) up to this one, newest first.
    pub notes: Vec<VersionNotes>,
}

impl AvailableUpdate {
    // ! Only this version's notes, from its release's `latest.json`.
    pub fn new(version: String, notes: Option<&str>) -> Self {
        let notes = VersionNotes::new(&version, notes).into_iter().collect();

        Self { version, notes }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(rename = "SnapshotType")]
pub struct Snapshot {
    pub version: String,
    pub install_kind: super::InstallKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub binary_asset: Option<BinaryAsset>,
    // ! The dashboard's interface build: a tab that loaded another one reloads, even when the version is the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub build: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub update_available: Option<AvailableUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub update_checked_at: Option<DateTime<Utc>>,
    // ! Due 48 h after the first start, until the window is closed once.
    pub star_prompt: bool,
    pub accounts: Vec<AccountSnapshot>,
    pub catalog: CatalogView,
    pub badges: BadgeCatalogView,
    pub settings: GlobalSettings,
    // ! Derived from `settings.history_retention`, so the UI never computes
    // its own day count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub history_retention_days: Option<u32>,
    // ! Computed server-side from `settings.quiet_window` and `settings.time_zone`,
    // so a clock that drifted in the browser never disagrees with it.
    #[serde(default)]
    pub quiet_window_active: bool,
    pub vault: VaultStatus,
    pub recent_events: Vec<MinerEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub login: Option<LoginFlowView>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub import: Option<ImportTokensFlowView>,
    // ! The core's own limits, so the UI shows and checks them without a copy.
    pub limits: CoreLimits,
    // ! Built from `AccountSettings::default()`, see
    // `AccountSettingsPatch::defaults()`.
    pub import_default_patch: AccountSettingsPatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "CoreLimitsType")]
pub struct CoreLimits {
    pub token_import_max: u32,
    pub token_line_max_chars: u32,
    pub channels_lookup_max: u32,
    pub search_min_chars: u32,
    pub passphrase_min_chars: u32,
    pub spam_burst_max: u32,
    pub spam_delay_min_ms: u32,
    pub spam_delay_max_ms: u32,
    pub spam_redeem_text_max_chars: u32,
    pub requests_bucket_minutes: u32,
    pub max_watched_channels: u32,
    pub bet_percentage_min: u32,
    pub bet_percentage_max: u32,
    pub bet_points_min: u32,
    pub bet_points_max: u32,
    pub bet_delay_seconds_min: u32,
    pub bet_delay_seconds_max: u32,
    pub bet_timing_percent_min: u32,
    pub bet_timing_percent_max: u32,
    pub bet_gap_percent_min: u32,
    pub bet_gap_percent_max: u32,
    #[ts(as = "Vec<crate::config::ProxyScheme>")]
    pub proxy_schemes: [crate::config::ProxyScheme; 6],
}

impl CoreLimits {
    pub const CURRENT: Self = Self {
        token_import_max: crate::api::TOKENS_IMPORT_MAX as u32,
        token_line_max_chars: crate::api::TOKEN_LINE_MAX_CHARS as u32,
        channels_lookup_max: crate::api::CHANNELS_LOOKUP_MAX as u32,
        search_min_chars: crate::miner::manager::SEARCH_MIN_CHARS as u32,
        passphrase_min_chars: crate::miner::manager::PASSPHRASE_MIN_CHARS as u32,
        spam_burst_max: crate::spam::BURST_MAX,
        spam_delay_min_ms: crate::config::SPAM_DELAY_MIN_MS,
        spam_delay_max_ms: crate::config::SPAM_DELAY_MAX_MS,
        spam_redeem_text_max_chars: crate::spam::REDEEM_TEXT_MAX_CHARS as u32,
        requests_bucket_minutes: crate::requests::BUCKET_MINUTES as u32,
        max_watched_channels: crate::config::MAX_WATCHED_CHANNELS as u32,
        bet_percentage_min: *crate::config::BET_PERCENTAGE.start() as u32,
        bet_percentage_max: *crate::config::BET_PERCENTAGE.end() as u32,
        bet_points_min: crate::config::BET_POINTS_MIN,
        bet_points_max: crate::predictions::MAX_PREDICTION_POINTS as u32,
        bet_delay_seconds_min: *crate::config::BET_DELAY_SECONDS.start(),
        bet_delay_seconds_max: *crate::config::BET_DELAY_SECONDS.end(),
        bet_timing_percent_min: *crate::config::BET_TIMING_PERCENT.start() as u32,
        bet_timing_percent_max: *crate::config::BET_TIMING_PERCENT.end() as u32,
        bet_gap_percent_min: *crate::config::BET_GAP_PERCENT.start() as u32,
        bet_gap_percent_max: *crate::config::BET_GAP_PERCENT.end() as u32,
        proxy_schemes: crate::config::ProxyScheme::ALL,
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "LoginFlowViewType")]
pub struct LoginFlowView {
    pub flow_id: String,
    pub user_code: String,
    pub verification_uri: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub status: LoginFlowStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub problem: Option<ErrorCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum LoginFlowStatus {
    Pending,
    Completed,
    Expired,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "TokenCheckType")]
pub struct TokenCheck {
    pub login: String,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub avatar_url: Option<String>,
    pub already_added: bool,
    pub persona: ClientPersona,
    pub scopes_ok: bool,
}

// ! Followed through the snapshot like `LoginFlowView`: the commit step runs
// as a cancellable background job, not a single blocking request, so a long
// list or a lock mid-import never leaves the UI with no way to know where it stopped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "ImportTokensFlowViewType")]
pub struct ImportTokensFlowView {
    pub flow_id: String,
    pub lines: Vec<ImportTokenOutcome>,
    pub status: ImportTokensFlowStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum ImportTokensFlowStatus {
    Running,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "status", rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(rename = "ImportTokenOutcomeType")]
pub enum ImportTokenOutcome {
    Pending,
    Checking,
    Imported { account_id: String },
    Failed { problem: ErrorCode },
}
