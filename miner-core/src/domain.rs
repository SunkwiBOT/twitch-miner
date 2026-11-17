use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "GameType")]
pub struct Game {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub box_art_url: Option<String>,
}

impl Game {
    pub fn slug(&self) -> String {
        self.slug
            .clone()
            .unwrap_or_else(|| crate::util::game_slug(&self.name))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "ChannelRefType")]
pub struct ChannelRef {
    pub id: String,
    pub login: String,
    #[serde(default)]
    pub display_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum BenefitKind {
    Badge,
    Emote,
    InGameItem,
    #[default]
    Other,
}

impl BenefitKind {
    pub const ALL: [Self; 4] = [Self::Badge, Self::Emote, Self::InGameItem, Self::Other];

    pub fn from_twitch(distribution_type: Option<&str>) -> Self {
        match distribution_type {
            Some("BADGE") => Self::Badge,
            Some("EMOTE") => Self::Emote,
            Some("DIRECT_ENTITLEMENT") => Self::InGameItem,
            _ => Self::Other,
        }
    }

    // ! The best (lowest) rank among a drop's benefits: a chest offering
    // several kinds is as wanted as its most wanted one.
    pub fn rank_of(kinds: impl Iterator<Item = Self>, priority: &[Self]) -> usize {
        kinds
            .map(|kind| {
                priority
                    .iter()
                    .position(|p| *p == kind)
                    .unwrap_or(priority.len())
            })
            .min()
            .unwrap_or(priority.len())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "BadgeRefType")]
pub struct BadgeRef {
    pub set_id: String,
    pub version: String,
    #[serde(default)]
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub image_url: Option<String>,
}

impl BadgeRef {
    pub fn key(&self) -> String {
        crate::badges::badge_key(&self.set_id, &self.version)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "BenefitType")]
pub struct Benefit {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub kind: BenefitKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub image_url: Option<String>,
    // ! A chest gives one of its badges at random: `badges` lists them all.
    #[serde(default)]
    pub badges: Vec<BadgeRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub chest_size: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "RepeatType")]
pub struct Repeat {
    pub times: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub every_hours: Option<u32>,
}

impl Repeat {
    pub fn every(&self) -> Option<chrono::Duration> {
        self.every_hours
            .filter(|hours| *hours > 0)
            .map(|hours| chrono::Duration::hours(i64::from(hours)))
    }
}

// ! An evolution counts only while the badge it evolves from is equipped; `from` is unknown until the badge list names it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "EvolutionType")]
pub struct Evolution {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub from: Option<BadgeRef>,
    pub owned: bool,
}

impl Evolution {
    pub fn ready(&self) -> bool {
        self.from.is_some() && self.owned
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "TimedDropType")]
pub struct TimedDrop {
    pub id: String,
    pub name: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub required_minutes: u32,
    #[serde(default)]
    pub required_subs: u32,
    #[serde(default)]
    pub current_subs: u32,
    #[serde(default)]
    pub current_minutes: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub instance_id: Option<String>,
    #[serde(default)]
    pub claimed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub preconditions_met: Option<bool>,
    #[serde(default)]
    pub precondition_drop_ids: Vec<String>,
    #[serde(default)]
    pub benefits: Vec<Benefit>,
    #[serde(default)]
    pub badge_owned: bool,
    // ! A repeated drop is earned once per period, `times` periods in all:
    // `claimed` and the minutes are those of the current period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub repeat: Option<Repeat>,
    #[serde(default)]
    pub periods_done: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_period_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub evolution: Option<Evolution>,
}

impl TimedDrop {
    pub fn complete(&self) -> bool {
        match self.repeat {
            Some(repeat) => self.periods_done >= repeat.times,
            None => self.claimed || self.current_minutes >= self.required_minutes,
        }
    }

    pub fn periods(&self) -> u32 {
        self.repeat.map_or(1, |repeat| repeat.times.max(1))
    }

    pub fn waiting(&self, now: DateTime<Utc>) -> bool {
        self.next_period_at.is_some_and(|at| at > now)
    }

    pub fn earned_for_now(&self, now: DateTime<Utc>) -> bool {
        self.complete()
            || self.claimed
            || self.current_minutes >= self.required_minutes
            || self.waiting(now)
    }

    pub fn claimable(&self) -> bool {
        !self.claimed && self.instance_id.is_some()
    }

    pub fn remaining_minutes(&self) -> u32 {
        self.required_minutes.saturating_sub(self.current_minutes)
    }

    pub fn minutes_left(&self, now: DateTime<Utc>) -> u32 {
        let periods = self.periods().saturating_sub(self.periods_done);

        if self.repeat.is_none() || periods == 0 {
            return if self.complete() {
                0
            } else {
                self.remaining_minutes()
            };
        }

        if self.claimed || self.waiting(now) {
            periods.saturating_mul(self.required_minutes)
        } else {
            (periods - 1)
                .saturating_mul(self.required_minutes)
                .saturating_add(self.remaining_minutes())
        }
    }

    pub fn active_at(&self, now: DateTime<Utc>) -> bool {
        self.starts_at <= now && now < self.ends_at
    }

    // ! Only Twitch's own count says a subscription was made: one running
    // already, Prime or a renewal never counts for a drop.
    pub fn subs_met(&self) -> bool {
        self.current_subs >= self.required_subs
    }

    pub fn minable(&self, excluded: &[String]) -> bool {
        self.subs_met()
            && !self.badge_owned
            && self.evolution.as_ref().is_none_or(Evolution::ready)
            && !excluded.iter().any(|id| id == &self.id)
    }

    pub fn earnable(&self, now: DateTime<Utc>, excluded: &[String]) -> bool {
        !self.complete() && self.minable(excluded) && self.ends_at > now
    }

    pub fn gives_badge(&self) -> bool {
        self.benefits.iter().any(|b| b.kind == BenefitKind::Badge)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(repr(enum = name))]
pub enum CampaignStatus {
    #[default]
    Active,
    Upcoming,
    Expired,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum CampaignKind {
    #[default]
    Drops,
    Quest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "CampaignType")]
pub struct Campaign {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub kind: CampaignKind,
    pub game: Game,
    // ! The watch counts on any of the categories; `game` is the first.
    #[serde(default)]
    pub categories: Vec<Game>,
    pub status: CampaignStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub linked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub link_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub image_url: Option<String>,
    // ! From the public catalogue only: its game's page on drops.sunkwi.com, anchored to the campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub web_url: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    #[serde(default)]
    pub allowed_channels: Vec<ChannelRef>,
    #[serde(default)]
    pub drops: Vec<TimedDrop>,
    #[serde(default)]
    pub in_inventory: bool,
}

impl Campaign {
    // ! Twitch answers a claim on a game the account has not linked as if it
    // took it, and gives nothing: the drop waits for the link.
    pub fn claims_delivered(&self) -> bool {
        self.linked != Some(false) || !self.needs_link()
    }

    // ! Twitch hands chat badges out itself, though it says "not connected" for
    // their campaign too: a campaign of badges only has no game account to link.
    pub fn needs_link(&self) -> bool {
        let mut benefits = self.drops.iter().flat_map(|d| &d.benefits).peekable();

        benefits.peek().is_none() || benefits.any(|b| b.kind != BenefitKind::Badge)
    }

    pub fn active_at(&self, now: DateTime<Utc>) -> bool {
        self.status == CampaignStatus::Active && self.starts_at <= now && now < self.ends_at
    }

    pub fn finished(&self, now: DateTime<Utc>, excluded: &[String]) -> bool {
        self.drops.iter().any(TimedDrop::complete)
            && !self.drops.iter().any(|d| d.earnable(now, excluded))
    }

    pub fn remaining_minutes(&self, now: DateTime<Utc>, excluded: &[String]) -> u32 {
        self.drops
            .iter()
            .filter(|d| d.earnable(now, excluded))
            .map(|d| d.minutes_left(now))
            .max()
            .unwrap_or(0)
    }

    // ! Kept while one reward still fits in its window: the shorter ones are not lost.
    // A repeated one needs its last period to open and be watched before the end.
    pub fn can_finish(&self, now: DateTime<Utc>, excluded: &[String]) -> bool {
        let mut left = self
            .drops
            .iter()
            .filter(|d| d.earnable(now, excluded))
            .peekable();

        left.peek().is_none()
            || left.any(|d| {
                let open = !d.claimed && !d.waiting(now);
                let start = d.starts_at.max(now).max(d.next_period_at.unwrap_or(now));
                let periods = d.periods().saturating_sub(d.periods_done).max(1);
                let last_start = match d.repeat.and_then(|r| r.every()) {
                    Some(every) => i32::try_from(periods - 1)
                        .ok()
                        .and_then(|n| every.checked_mul(n))
                        .and_then(|span| start.checked_add_signed(span)),
                    None => Some(start),
                };
                let Some(last_start) = last_start else {
                    return false;
                };
                let last = if periods == 1 && open {
                    d.remaining_minutes()
                } else {
                    d.required_minutes
                };
                let window = d.ends_at.min(self.ends_at) - last_start;

                i64::from(last) <= window.num_minutes()
            })
    }

    pub fn next_drop(
        &self,
        now: DateTime<Utc>,
        excluded: &[String],
        priority: &[BenefitKind],
    ) -> Option<&TimedDrop> {
        let completed: Vec<&str> = self
            .drops
            .iter()
            .filter(|d| d.complete())
            .map(|d| d.id.as_str())
            .collect();

        self.drops
            .iter()
            .filter(|d| !d.earned_for_now(now) && d.active_at(now) && d.minable(excluded))
            .filter(|d| {
                d.preconditions_met.unwrap_or(true)
                    && d.precondition_drop_ids
                        .iter()
                        .all(|id| completed.contains(&id.as_str()))
            })
            .min_by_key(|d| {
                (
                    BenefitKind::rank_of(d.benefits.iter().map(|b| b.kind), priority),
                    d.required_minutes,
                    d.id.clone(),
                )
            })
    }

    pub fn gives_badge(&self, now: DateTime<Utc>, excluded: &[String]) -> bool {
        self.kind == CampaignKind::Quest
            || self
                .drops
                .iter()
                .any(|d| d.gives_badge() && d.earnable(now, excluded))
    }

    pub fn has_badge(&self) -> bool {
        self.kind == CampaignKind::Quest || self.drops.iter().any(TimedDrop::gives_badge)
    }

    pub fn evolves(&self) -> bool {
        self.drops.iter().any(|d| d.evolution.is_some())
    }

    // ! What a watch or an event of the campaign is for: a quest, or the drop
    // when it gives a badge; without a drop, every drop of the campaign.
    pub fn mined_for_badge(&self, drop_id: Option<&str>) -> bool {
        self.kind == CampaignKind::Quest
            || match drop_id {
                Some(id) => self
                    .drops
                    .iter()
                    .find(|d| d.id == id)
                    .is_some_and(TimedDrop::gives_badge),
                None => !self.drops.is_empty() && self.drops.iter().all(TimedDrop::gives_badge),
            }
    }

    pub fn played(&self) -> impl Iterator<Item = &Game> {
        std::iter::once(&self.game).chain(
            self.categories
                .iter()
                .filter(move |category| category.id != self.game.id),
        )
    }

    pub fn plays(&self, game: &Game) -> bool {
        std::iter::once(&self.game)
            .chain(&self.categories)
            .any(|g| {
                g.id == game.id || (!g.name.is_empty() && g.name.eq_ignore_ascii_case(&game.name))
            })
    }

    pub fn accepts_channel(&self, channel_id: &str) -> bool {
        self.allowed_channels.is_empty() || self.allowed_channels.iter().any(|c| c.id == channel_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SearchGameType")]
pub struct SearchGame {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub box_art_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "SearchChannelType")]
pub struct SearchChannel {
    pub login: String,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub avatar_url: Option<String>,
    pub live: bool,
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "ChannelType")]
pub struct Channel {
    pub id: String,
    pub login: String,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub live: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub broadcast_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub viewers: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub game: Option<Game>,
    #[serde(default)]
    pub drops_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "PointsStateType")]
pub struct PointsState {
    pub balance: i64,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub earned_session: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub streak: Option<WatchStreak>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub last_bonus_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub subscribed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[ts(rename = "WatchStreakType")]
pub struct WatchStreak {
    pub count: u32,
    #[serde(default)]
    pub missed_streams: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub expires_at: Option<DateTime<Utc>>,
}

const STREAK_RISK_WINDOW: chrono::Duration = chrono::Duration::hours(6);

impl WatchStreak {
    pub fn at_risk(&self, now: DateTime<Utc>) -> bool {
        self.missed_streams > 0
            || self
                .expires_at
                .is_some_and(|expires| expires > now && expires - now < STREAK_RISK_WINDOW)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionOutcome {
    pub id: String,
    pub title: String,
    pub color: String,
    pub total_users: i64,
    pub total_points: i64,
    pub top_points: i64,
    #[serde(default)]
    pub percentage_users: f64,
    #[serde(default)]
    pub odds: f64,
    #[serde(default)]
    pub odds_percentage: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionEvent {
    pub id: String,
    pub channel_id: String,
    pub title: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub window_seconds: i64,
    pub outcomes: Vec<PredictionOutcome>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_outcome_id: Option<String>,
}

impl PredictionEvent {
    pub fn update_outcomes(&mut self) {
        let total_users: i64 = self.outcomes.iter().map(|o| o.total_users).sum();
        let total_points: i64 = self.outcomes.iter().map(|o| o.total_points).sum();

        for outcome in &mut self.outcomes {
            outcome.percentage_users = if total_users > 0 {
                outcome.total_users as f64 * 100.0 / total_users as f64
            } else {
                0.0
            };

            outcome.odds = if outcome.total_points > 0 {
                total_points as f64 / outcome.total_points as f64
            } else {
                0.0
            };

            outcome.odds_percentage = if outcome.odds > 0.0 {
                100.0 / outcome.odds
            } else {
                0.0
            };
        }
    }

    pub fn closes_at(&self) -> DateTime<Utc> {
        chrono::Duration::try_seconds(self.window_seconds)
            .and_then(|window| self.created_at.checked_add_signed(window))
            .unwrap_or(self.created_at)
    }
}
