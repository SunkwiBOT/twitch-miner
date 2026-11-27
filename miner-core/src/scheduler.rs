use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::config::{
    AccountSettings, DropsOrder, DropsScope, GameEntry, MiningLaneKind, Mode, StreamerSettings,
};
use crate::domain::{BenefitKind, Campaign, CampaignKind, Channel, PointsState, TimedDrop};

// ! Declared in rank order: the derived `Ord` ranks the reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum PointsReason {
    StreakAtRisk,
    StreakBonus,
    Subscribed,
    ListOrder,
}

impl PointsReason {
    pub fn describe(self) -> &'static str {
        match self {
            Self::StreakAtRisk => "streak at risk",
            Self::StreakBonus => "streak bonus",
            Self::Subscribed => "subscribed",
            Self::ListOrder => "list order",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(rename = "SlotPurposeType")]
pub enum SlotPurpose {
    Drop {
        campaign_id: String,
        drop_id: String,
    },
    Points {
        reason: PointsReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum SlotPurposeKind {
    Drop,
    Points,
}

impl SlotPurpose {
    pub fn kind(&self) -> SlotPurposeKind {
        match self {
            Self::Drop { .. } => SlotPurposeKind::Drop,
            Self::Points { .. } => SlotPurposeKind::Points,
        }
    }

    pub fn same_watch(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Drop {
                    campaign_id: mine, ..
                },
                Self::Drop {
                    campaign_id: theirs,
                    ..
                },
            ) => mine == theirs,
            (Self::Points { reason: mine }, Self::Points { reason: theirs }) => mine == theirs,
            _ => false,
        }
    }

    pub fn keeps_stream(&self, next: &Self) -> bool {
        match (self, next) {
            (
                Self::Drop {
                    campaign_id: mine, ..
                },
                Self::Drop {
                    campaign_id: theirs,
                    ..
                },
            ) => mine == theirs,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slot {
    pub channel: Channel,
    pub purpose: SlotPurpose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum QueueReason {
    Ordered,
    Priority,
    Badge,
    EndingSoon,
    LowestRemaining,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum QueueBlock {
    NoLiveChannel,
    OneCampaignAtATime,
    SetAside,
}

// ! Why a campaign of the account stays out of the queue. `Finished` and
// `Owned` say its rewards are earned; the others what keeps it from being mined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum LeftOutReason {
    Finished,
    Owned,
    NotActive,
    Ignored,
    EvolutionsOff,
    Filtered,
    PointsOnly,
    SubRequired,
    EvolutionLocked,
    Waiting,
    Unfinishable,
    NotSelected,
    NotLinked,
    LaneOff,
}

impl LeftOutReason {
    pub fn earned(self) -> bool {
        matches!(self, Self::Finished | Self::Owned)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "LeftOutCampaignType")]
pub struct LeftOutCampaign {
    pub campaign_id: String,
    pub reason: LeftOutReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "QueueItemType")]
pub struct QueueItem {
    pub campaign_id: String,
    pub reason: QueueReason,
    pub live_channels: usize,
    pub remaining_minutes: u32,
    pub ends_at: DateTime<Utc>,
    pub linked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub blocked: Option<QueueBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub set_aside_until: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub set_aside_stall_minutes: Option<u32>,
    // ! An estimate: the minutes the campaigns ahead still hold the drop slot, as planned now.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub expected_start: Option<DateTime<Utc>>,
}

// ! A campaign ranked above the one given the Drops slot, and what keeps it
// from being mined. A campaign whose channels were never read is not listed:
// nothing is known of it yet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassedOver {
    pub campaign_id: String,
    pub block: QueueBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Plan {
    pub slots: Vec<Slot>,
    pub queue: Vec<QueueItem>,
    pub left_out: Vec<LeftOutCampaign>,
    #[serde(default)]
    pub passed_over: Vec<PassedOver>,
}

// ! Twitch pays the watch streak bonus a few minutes into the watch of a new
// broadcast.
pub const STREAK_BONUS_WINDOW: chrono::Duration = chrono::Duration::minutes(10);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreakBonus {
    pub broadcast_id: String,
    pub watched_since: Option<DateTime<Utc>>,
    pub settled: bool,
}

impl StreakBonus {
    pub fn new(broadcast_id: impl Into<String>) -> Self {
        Self {
            broadcast_id: broadcast_id.into(),
            watched_since: None,
            settled: false,
        }
    }

    pub fn window_passed(&self, now: DateTime<Utc>) -> bool {
        self.watched_since
            .is_some_and(|since| now - since >= STREAK_BONUS_WINDOW)
    }

    pub fn pending(&self, now: DateTime<Utc>) -> bool {
        !self.settled && !self.window_passed(now)
    }
}

#[derive(Debug, Clone)]
pub struct StreamerState<'a> {
    pub settings: &'a StreamerSettings,
    pub channel: Option<&'a Channel>,
    pub points: &'a PointsState,
    pub streak_bonus: Option<&'a StreakBonus>,
}

fn contains_ci(haystack: &str, needle: &str) -> bool {
    haystack.to_lowercase().contains(&needle.to_lowercase())
}

fn game_matches(campaign: &Campaign, name: &str) -> bool {
    let name = name.trim();

    if name.is_empty() {
        return false;
    }

    // ! A name without Latin letters or digits ("原神") has no slug: two empty
    // slugs would make every such game match.
    let slug = crate::util::game_slug(name);

    campaign.game.name.eq_ignore_ascii_case(name)
        || campaign.game.id == name
        || (!slug.is_empty() && campaign.game.slug().eq_ignore_ascii_case(&slug))
}

// ! An id, carried from search, matches exactly and never falls back to the
// name match below: a category renamed on Twitch keeps matching by id.
fn game_entry_matches(campaign: &Campaign, entry: &GameEntry) -> bool {
    match entry.id.as_deref().filter(|id| !id.is_empty()) {
        Some(id) => campaign.game.id == id,
        None => game_matches(campaign, &entry.name),
    }
}

fn names_match(names: &[String], campaign: &Campaign) -> bool {
    names.iter().any(|needle| {
        !needle.trim().is_empty()
            && (contains_ci(&campaign.name, needle)
                || campaign.drops.iter().any(|d| contains_ci(&d.name, needle)))
    })
}

fn drops_filtered(settings: &AccountSettings, campaign: &Campaign) -> bool {
    settings
        .drops
        .exclude_games
        .iter()
        .any(|g| game_entry_matches(campaign, g))
        || names_match(&settings.drops.exclude_names, campaign)
}

fn badges_filtered(settings: &AccountSettings, campaign: &Campaign) -> bool {
    settings
        .badges
        .exclude_games
        .iter()
        .any(|g| game_entry_matches(campaign, g))
        || names_match(&settings.badges.exclude_names, campaign)
}

// ! The Drops filters and the badge filters each keep a campaign out of their
// own kinds only: a badge campaign is out once both filter it.
pub fn excluded(settings: &AccountSettings, campaign: &Campaign, now: DateTime<Utc>) -> bool {
    exclusion(
        settings,
        campaign,
        campaign.gives_badge(now, &settings.drops.exclude_drop_ids),
    )
    .is_some()
}

// ! A badge mined to the end no longer "gives" one, but its claim still follows the badge filters.
pub fn claim_excluded(settings: &AccountSettings, campaign: &Campaign) -> bool {
    exclusion(settings, campaign, campaign.has_badge()).is_some()
}

fn exclusion(
    settings: &AccountSettings,
    campaign: &Campaign,
    badge: bool,
) -> Option<LeftOutReason> {
    if settings
        .drops
        .exclude_campaign_ids
        .iter()
        .any(|id| id == &campaign.id)
    {
        return Some(LeftOutReason::Ignored);
    }

    if !settings.badges.evolutions && campaign.evolves() {
        return Some(LeftOutReason::EvolutionsOff);
    }

    let by_drops = campaign.kind == CampaignKind::Quest || drops_filtered(settings, campaign);
    let filtered = if badge {
        by_drops && badges_filtered(settings, campaign)
    } else {
        by_drops
    };

    filtered.then_some(LeftOutReason::Filtered)
}

// ! Why no reward of the campaign can be watched now. A reward still earnable
// later waits; otherwise the most telling block of its open rewards wins.
fn idle_reason(campaign: &Campaign, now: DateTime<Utc>, excluded: &[String]) -> LeftOutReason {
    if campaign.finished(now, excluded) {
        return LeftOutReason::Finished;
    }

    let open: Vec<&TimedDrop> = campaign
        .drops
        .iter()
        .filter(|d| !d.complete() && d.ends_at > now)
        .collect();

    if open.iter().any(|d| d.earnable(now, excluded)) {
        return LeftOutReason::Waiting;
    }

    let blocked = |test: fn(&TimedDrop) -> bool| open.iter().copied().any(test);

    if blocked(|d| !d.badge_owned && !d.subs_met()) {
        LeftOutReason::SubRequired
    } else if blocked(|d| !d.badge_owned && d.evolution.as_ref().is_some_and(|e| !e.ready())) {
        LeftOutReason::EvolutionLocked
    } else if blocked(|d| d.badge_owned) {
        LeftOutReason::Owned
    } else if open.is_empty() {
        LeftOutReason::NotActive
    } else {
        LeftOutReason::Ignored
    }
}

fn priority_index(settings: &AccountSettings, campaign: &Campaign) -> Option<usize> {
    settings
        .drops
        .priority_games
        .iter()
        .position(|g| game_entry_matches(campaign, g))
}

type RankKey = (u8, i64, u8, i64, usize);

// ! "Linked only" never keeps out a quest, and keeps out a campaign only once Twitch says "not linked".
fn in_scope(scope: DropsScope, campaign: &Campaign) -> bool {
    match scope {
        DropsScope::LinkedOnly => {
            campaign.kind == CampaignKind::Quest || campaign.claims_delivered()
        }
        DropsScope::All => true,
    }
}

fn selected(settings: &AccountSettings, campaign: &Campaign) -> bool {
    settings
        .drops
        .selected_campaign_ids
        .iter()
        .any(|id| id == &campaign.id)
}

// ! "Only my selection": a kind with it on mines only the campaigns starred by
// hand (the priority games are the Drops selection already).
fn held_back(settings: &AccountSettings, campaign: &Campaign, now: DateTime<Utc>) -> bool {
    !selected(settings, campaign)
        && ((settings.drops.selection_only && campaign.kind == CampaignKind::Drops)
            || (settings.badges.selection_only
                && campaign.gives_badge(now, &settings.drops.exclude_drop_ids)))
}

// ! Whether a lane takes the campaign: `None` when the lane is not meant for
// its kind, the reason when it is meant for it but refuses it.
fn lane_fit(
    settings: &AccountSettings,
    campaign: &Campaign,
    now: DateTime<Utc>,
    kind: MiningLaneKind,
) -> Option<Result<(), LeftOutReason>> {
    let drops = &settings.drops;
    let chosen = selected(settings, campaign);
    let check = |scope: DropsScope, filtered: bool, selection_only: bool| {
        if !in_scope(scope, campaign) {
            Err(LeftOutReason::NotLinked)
        } else if filtered {
            Err(LeftOutReason::Filtered)
        } else if selection_only && !chosen {
            Err(LeftOutReason::NotSelected)
        } else {
            Ok(())
        }
    };

    match kind {
        MiningLaneKind::PriorityGames => (campaign.kind == CampaignKind::Drops
            && priority_index(settings, campaign).is_some())
        .then(|| check(drops.scope, drops_filtered(settings, campaign), false)),
        MiningLaneKind::Badges => campaign.gives_badge(now, &drops.exclude_drop_ids).then(|| {
            check(
                settings.badges.scope,
                badges_filtered(settings, campaign),
                settings.badges.selection_only,
            )
        }),
        MiningLaneKind::OtherCampaigns => (campaign.kind == CampaignKind::Drops).then(|| {
            check(
                drops.scope,
                drops_filtered(settings, campaign),
                drops.selection_only,
            )
        }),
    }
}

// ! The first enabled lane taking the campaign; else the refusal of the first
// enabled lane meant for it, or `LaneOff` when none of those is enabled.
pub fn lane_of(
    settings: &AccountSettings,
    campaign: &Campaign,
    now: DateTime<Utc>,
) -> Result<(MiningLaneKind, usize), LeftOutReason> {
    let mut refusal = None;

    for (position, lane) in settings.drops.mining_order.iter().enumerate() {
        if !lane.enabled {
            continue;
        }

        match lane_fit(settings, campaign, now, lane.kind) {
            Some(Ok(())) => return Ok((lane.kind, position)),
            Some(Err(reason)) => {
                refusal.get_or_insert(reason);
            }
            None => {}
        }
    }

    Err(refusal.unwrap_or(LeftOutReason::LaneOff))
}

fn automatic_key(
    linked_first: bool,
    order: DropsOrder,
    campaign: &Campaign,
    now: DateTime<Utc>,
    excluded: &[String],
) -> (QueueReason, u8, i64) {
    let linked_rank = if linked_first {
        campaign.link_standing() as u8
    } else {
        0
    };

    match order {
        DropsOrder::EndingSoonest => (
            QueueReason::EndingSoon,
            linked_rank,
            campaign.ends_at.timestamp(),
        ),
        DropsOrder::LowestRemaining => (
            QueueReason::LowestRemaining,
            linked_rank,
            i64::from(campaign.remaining_minutes(now, excluded)),
        ),
    }
}

// ! A tie-breaker between campaigns otherwise ranked equal, after the user's
// explicit mining order and lane placement. The default priority order
// (`BenefitKind::ALL`) is neutral: every campaign gets the same rank, so this
// never changes today's behaviour unless the user reorders reward kinds.
fn reward_tie_break(
    campaign: &Campaign,
    now: DateTime<Utc>,
    excluded: &[String],
    priority: &[BenefitKind],
) -> usize {
    if priority == BenefitKind::ALL.as_slice() {
        return 0;
    }

    campaign
        .next_drop(now, excluded, priority)
        .map_or(0, |drop| {
            BenefitKind::rank_of(drop.benefits.iter().map(|b| b.kind), priority)
        })
}

// ! A campaign is ranked (`Ok`) or left out for one reason: its rewards earned
// first, then a setting that excludes it, then what keeps it from being watched.
fn rank_key(
    settings: &AccountSettings,
    campaign: &Campaign,
    now: DateTime<Utc>,
) -> Result<(QueueReason, RankKey), LeftOutReason> {
    let drops = &settings.drops;
    let excluded = &drops.exclude_drop_ids;
    let idle = campaign
        .next_drop(now, excluded, &drops.reward_priority)
        .is_none()
        .then(|| idle_reason(campaign, now, excluded));

    if let Some(reason) = idle.filter(|reason| reason.earned()) {
        return Err(reason);
    }

    if let Some(reason) = exclusion(settings, campaign, campaign.gives_badge(now, excluded)) {
        return Err(reason);
    }

    if let Some(reason) = idle {
        return Err(reason);
    }

    if !campaign.active_at(now) {
        return Err(LeftOutReason::NotActive);
    }

    if drops.skip_unfinishable && !campaign.can_finish(now, excluded) {
        return Err(LeftOutReason::Unfinishable);
    }

    let pinned = drops
        .campaign_order
        .iter()
        .position(|id| id == &campaign.id);
    let lane = lane_of(settings, campaign, now);
    let reward_rank = reward_tie_break(campaign, now, excluded, &drops.reward_priority);

    match (pinned, lane) {
        (Some(_), Err(_)) if held_back(settings, campaign, now) => Err(LeftOutReason::NotSelected),
        (Some(index), _) => Ok((QueueReason::Ordered, (0u8, index as i64, 0, 0, 0))),
        (None, Err(reason)) => Err(reason),
        (None, Ok((kind, position))) => {
            let tier = 1 + position as u8;

            Ok(match kind {
                MiningLaneKind::PriorityGames => (
                    QueueReason::Priority,
                    (
                        tier,
                        priority_index(settings, campaign).unwrap_or(0) as i64,
                        0,
                        0,
                        reward_rank,
                    ),
                ),
                MiningLaneKind::Badges => {
                    let badges = &settings.badges;
                    let (_, linked, value) =
                        automatic_key(badges.linked_first, badges.order, campaign, now, excluded);

                    (QueueReason::Badge, (tier, 0, linked, value, reward_rank))
                }
                MiningLaneKind::OtherCampaigns => {
                    let (reason, linked, value) =
                        automatic_key(drops.linked_first, drops.order, campaign, now, excluded);

                    (reason, (tier, 0, linked, value, reward_rank))
                }
            })
        }
    }
}

// ! Where the user's order puts a campaign: its tier (0 for the campaigns
// placed by hand, then one per lane of the mining order, in its order), then
// its place among those placed by hand or among the priority categories.
// Campaigns of equal standing are ordered automatically.
type Standing = (u8, i64);

type Ranked<'a> = (
    Vec<(&'a Campaign, QueueReason, Standing)>,
    Vec<LeftOutCampaign>,
);

// ! A campaign already over is neither ranked nor left out.
fn rank_standings<'a>(
    settings: &AccountSettings,
    campaigns: &'a [Campaign],
    now: DateTime<Utc>,
) -> Ranked<'a> {
    let mut ranked: Vec<(&Campaign, QueueReason, RankKey)> = Vec::new();
    let mut left_out = Vec::new();

    for campaign in campaigns.iter().filter(|c| c.ends_at > now) {
        match rank_key(settings, campaign, now) {
            Ok((reason, key)) => ranked.push((campaign, reason, key)),
            Err(reason) => left_out.push(LeftOutCampaign {
                campaign_id: campaign.id.clone(),
                reason,
            }),
        }
    }

    ranked.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.id.cmp(&b.0.id)));

    let ranked = ranked
        .into_iter()
        .map(|(c, reason, key)| (c, reason, (key.0, key.1)))
        .collect();

    (ranked, left_out)
}

pub fn rank_campaigns<'a>(
    settings: &AccountSettings,
    campaigns: &'a [Campaign],
    now: DateTime<Utc>,
) -> Vec<(&'a Campaign, QueueReason)> {
    rank_standings(settings, campaigns, now)
        .0
        .into_iter()
        .map(|(c, reason, _)| (c, reason))
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub struct CurrentDrop<'a> {
    pub campaign_id: &'a str,
    pub channel_id: &'a str,
}

// ! Shared with the account's stall tracking (`slots.rs`): a channel only
// credits a campaign if Twitch marked it drops-enabled, or the campaign
// names its channels directly.
pub fn credits_drops(channel: &Channel, campaign: &Campaign) -> bool {
    channel.drops_enabled || !campaign.allowed_channels.is_empty()
}

fn pick_channel<'a>(
    settings: &AccountSettings,
    campaign: &Campaign,
    live: &'a [Channel],
    watching: Option<&str>,
    subscribed: &[&str],
) -> Option<&'a Channel> {
    let preferred = |channel: &Channel| {
        settings.drops.streamers_first
            && credits_drops(channel, campaign)
            && settings
                .points
                .streamers
                .iter()
                .any(|s| s.enabled && s.login == channel.login)
    };
    // ! A reward behind subscriptions is watched on a channel the account is
    // subscribed to, as far as the channel points already read tell.
    let behind_subs = campaign
        .drops
        .iter()
        .any(|d| d.required_subs > 0 && d.subs_counted() && !d.complete());
    let subscriber =
        |channel: &Channel| behind_subs && subscribed.contains(&channel.login.as_str());

    let eligible: Vec<&Channel> = live
        .iter()
        .filter(|c| c.live && campaign.accepts_channel(&c.id))
        .filter(|c| {
            // ! Twitch credits drop minutes only on a stream of the campaign's
            // game, also for a channel of its allow list.
            c.game.as_ref().is_some_and(|g| campaign.plays(g))
        })
        .collect();

    let kind = |c: &Channel| (subscriber(c), preferred(c), c.drops_enabled);
    let best_kind = eligible.iter().map(|c| kind(c)).max()?;
    let best = eligible.iter().copied().find(|c| kind(c) == best_kind)?;

    watching
        .and_then(|id| eligible.iter().copied().find(|c| c.id == id))
        .filter(|current| kind(current) == kind(best))
        .or(Some(best))
}

pub fn rank_streamers<'a>(
    settings: &AccountSettings,
    streamers: &[StreamerState<'a>],
    now: DateTime<Utc>,
) -> Vec<(&'a Channel, PointsReason)> {
    let points = &settings.points;
    let mut live: Vec<(PointsReason, bool, usize, &Channel)> = streamers
        .iter()
        .enumerate()
        .filter(|(_, s)| s.settings.enabled)
        .filter_map(|(index, s)| {
            let channel = s.channel.filter(|c| c.live)?;
            let streaks = points.streak_priority && s.settings.watch_streak;
            let bonus = s.streak_bonus.filter(|bonus| {
                channel.broadcast_id.as_deref() == Some(bonus.broadcast_id.as_str())
                    && bonus.pending(now)
            });
            let at_risk = s
                .points
                .streak
                .as_ref()
                .is_some_and(|streak| streak.at_risk(now));

            let reason = match bonus {
                _ if streaks && at_risk => PointsReason::StreakAtRisk,
                Some(_) if streaks => PointsReason::StreakBonus,
                _ if points.subscriber_priority && s.points.subscribed => PointsReason::Subscribed,
                _ => PointsReason::ListOrder,
            };

            let waiting = reason == PointsReason::StreakBonus
                && bonus.is_some_and(|bonus| bonus.watched_since.is_none());

            Some((reason, waiting, index, channel))
        })
        .collect();

    live.sort_by_key(|(reason, waiting, index, _)| (*reason, *waiting, *index));

    live.into_iter()
        .map(|(reason, _, _, channel)| (channel, reason))
        .collect()
}

// ! An account mining points only mines no campaign, but still tells the earned ones apart.
fn points_only(
    ranked: &[(&Campaign, QueueReason, Standing)],
    left_out: Vec<LeftOutCampaign>,
) -> Vec<LeftOutCampaign> {
    let points_only = |campaign_id: String| LeftOutCampaign {
        campaign_id,
        reason: LeftOutReason::PointsOnly,
    };

    ranked
        .iter()
        .map(|(campaign, _, _)| points_only(campaign.id.clone()))
        .chain(left_out.into_iter().map(|item| {
            if item.reason.earned() {
                item
            } else {
                points_only(item.campaign_id)
            }
        }))
        .collect()
}

pub fn plan(
    settings: &AccountSettings,
    campaigns: &[Campaign],
    aside: &HashMap<String, DateTime<Utc>>,
    live_by_campaign: &HashMap<String, Vec<Channel>>,
    streamers: &[StreamerState<'_>],
    current: Option<CurrentDrop<'_>>,
    now: DateTime<Utc>,
) -> Plan {
    let slots = settings.slots();
    let mut plan = Plan::default();
    let mut used: Vec<String> = Vec::new();

    let (mut ranked, left_out) = rank_standings(settings, campaigns, now);

    if settings.mode == Mode::Points {
        plan.left_out = points_only(&ranked, left_out);
    } else {
        plan.left_out = left_out;

        let subscribed: Vec<&str> = streamers
            .iter()
            .filter(|s| s.points.subscribed)
            .map(|s| s.settings.login.as_str())
            .collect();

        let watching = |campaign: &Campaign| {
            current
                .filter(|current| current.campaign_id == campaign.id)
                .map(|current| current.channel_id)
        };
        let minable = |campaign: &Campaign| {
            let live = live_by_campaign
                .get(&campaign.id)
                .map(Vec::as_slice)
                .unwrap_or(&[]);

            !aside.contains_key(&campaign.id)
                && campaign
                    .next_drop(
                        now,
                        &settings.drops.exclude_drop_ids,
                        &settings.drops.reward_priority,
                    )
                    .is_some()
                && pick_channel(settings, campaign, live, None, &subscribed).is_some()
        };

        let natural: Vec<(&Campaign, Standing)> = ranked.iter().map(|(c, _, s)| (*c, *s)).collect();

        // ! The campaign being mined keeps the slot over the ones of equal
        // standing ranked above it: an automatic order changing, or another
        // campaign of its category, never interrupts it.
        if let Some(current) = current {
            let viable = ranked
                .iter()
                .position(|(c, _, _)| c.id == current.campaign_id)
                .filter(|&index| {
                    let (campaign, _, standing) = &ranked[index];
                    let live = live_by_campaign
                        .get(&campaign.id)
                        .map(Vec::as_slice)
                        .unwrap_or(&[]);

                    !aside.contains_key(current.campaign_id)
                        && pick_channel(settings, campaign, live, watching(campaign), &subscribed)
                            .is_some()
                        && !ranked[..index]
                            .iter()
                            .any(|(c, _, above)| above < standing && minable(c))
                });

            if let Some(index) = viable {
                let item = ranked.remove(index);
                ranked.insert(0, item);
            }
        }

        let mut mining = false;
        let mut held: u32 = 0;

        for (campaign, reason, _) in ranked {
            let live = live_by_campaign
                .get(&campaign.id)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let mut item = QueueItem {
                campaign_id: campaign.id.clone(),
                reason,
                live_channels: live.iter().filter(|c| c.live).count(),
                remaining_minutes: campaign
                    .remaining_minutes(now, &settings.drops.exclude_drop_ids),
                ends_at: campaign.ends_at,
                linked: campaign.linked,
                blocked: None,
                set_aside_until: None,
                set_aside_stall_minutes: None,
                expected_start: None,
            };

            if let Some(retry_at) = aside.get(&campaign.id).copied() {
                item.blocked = Some(QueueBlock::SetAside);
                item.set_aside_until = Some(retry_at);
                plan.queue.push(item);

                continue;
            }

            let chosen = pick_channel(settings, campaign, live, watching(campaign), &subscribed);

            match (
                mining,
                chosen,
                campaign.next_drop(
                    now,
                    &settings.drops.exclude_drop_ids,
                    &settings.drops.reward_priority,
                ),
            ) {
                (false, Some(channel), Some(drop)) => {
                    mining = true;
                    held = campaign.hold_minutes(now, &settings.drops.exclude_drop_ids);
                    used.push(channel.id.clone());
                    plan.slots.push(Slot {
                        channel: channel.clone(),
                        purpose: SlotPurpose::Drop {
                            campaign_id: campaign.id.clone(),
                            drop_id: drop.id.clone(),
                        },
                    });
                }
                (true, Some(_), _) => {
                    item.blocked = Some(QueueBlock::OneCampaignAtATime);
                    item.expected_start = Some(now + chrono::Duration::minutes(i64::from(held)));
                    held = held.saturating_add(
                        campaign.hold_minutes(now, &settings.drops.exclude_drop_ids),
                    );
                }
                (true, None, _) => item.blocked = Some(QueueBlock::OneCampaignAtATime),
                (false, _, _) => item.blocked = Some(QueueBlock::NoLiveChannel),
            }

            plan.queue.push(item);
        }

        let mined = plan.slots.iter().find_map(|slot| match &slot.purpose {
            SlotPurpose::Drop { campaign_id, .. } => Some(campaign_id.as_str()),
            SlotPurpose::Points { .. } => None,
        });

        // ! Only a campaign of a higher standing is told: those of equal standing wait their turn by design.
        if let Some(mined) = mined.and_then(|id| natural.iter().find(|(c, _)| c.id == id)) {
            plan.passed_over = natural
                .iter()
                .filter(|(c, standing)| standing < &mined.1 && live_by_campaign.contains_key(&c.id))
                .map(|(c, _)| PassedOver {
                    campaign_id: c.id.clone(),
                    block: if aside.contains_key(&c.id) {
                        QueueBlock::SetAside
                    } else if minable(c) {
                        QueueBlock::OneCampaignAtATime
                    } else {
                        QueueBlock::NoLiveChannel
                    },
                })
                .collect();
        }
    }

    if settings.mode != Mode::Drops {
        for (channel, reason) in rank_streamers(settings, streamers, now) {
            if plan.slots.len() >= slots {
                break;
            }

            if used.contains(&channel.id) {
                continue;
            }

            used.push(channel.id.clone());
            plan.slots.push(Slot {
                channel: channel.clone(),
                purpose: SlotPurpose::Points { reason },
            });
        }
    }

    plan
}
