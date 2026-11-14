use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::config::{
    AccountSettings, DropsOrder, DropsScope, MiningLaneKind, Mode, StreamerSettings,
};
use crate::domain::{Campaign, CampaignKind, Channel, PointsState};

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
    NoDropAvailable,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Plan {
    pub slots: Vec<Slot>,
    pub queue: Vec<QueueItem>,
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

fn filtered(games: &[String], names: &[String], campaign: &Campaign) -> bool {
    games.iter().any(|g| game_matches(campaign, g))
        || names.iter().any(|needle| {
            !needle.trim().is_empty()
                && (contains_ci(&campaign.name, needle)
                    || campaign.drops.iter().any(|d| contains_ci(&d.name, needle)))
        })
}

fn drops_filtered(settings: &AccountSettings, campaign: &Campaign) -> bool {
    filtered(
        &settings.drops.exclude_games,
        &settings.drops.exclude_names,
        campaign,
    )
}

fn badges_filtered(settings: &AccountSettings, campaign: &Campaign) -> bool {
    filtered(
        &settings.badges.exclude_games,
        &settings.badges.exclude_names,
        campaign,
    )
}

// ! The Drops filters and the badge filters each keep a campaign out of their
// own kinds only: a badge campaign is out once both filter it.
pub fn excluded(settings: &AccountSettings, campaign: &Campaign, now: DateTime<Utc>) -> bool {
    let drops = &settings.drops;

    if drops
        .exclude_campaign_ids
        .iter()
        .any(|id| id == &campaign.id)
    {
        return true;
    }

    let by_drops = campaign.kind == CampaignKind::Quest || drops_filtered(settings, campaign);

    if campaign.gives_badge(now, &drops.exclude_drop_ids) {
        by_drops && badges_filtered(settings, campaign)
    } else {
        by_drops
    }
}

fn priority_index(settings: &AccountSettings, campaign: &Campaign) -> Option<usize> {
    settings
        .drops
        .priority_games
        .iter()
        .position(|g| game_matches(campaign, g))
}

type RankKey = (u8, i64, u8, i64);

// ! A quest has no game account to link: "linked only" never keeps it out.
fn in_scope(scope: DropsScope, campaign: &Campaign) -> bool {
    match scope {
        DropsScope::LinkedOnly => {
            campaign.kind == CampaignKind::Quest || campaign.linked == Some(true)
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

pub fn lane_of(
    settings: &AccountSettings,
    campaign: &Campaign,
    now: DateTime<Utc>,
) -> Option<(MiningLaneKind, usize)> {
    let drops = &settings.drops;
    let priority = priority_index(settings, campaign);
    let chosen = selected(settings, campaign);

    drops
        .mining_order
        .iter()
        .enumerate()
        .filter(|(_, lane)| lane.enabled)
        .find(|(_, lane)| match lane.kind {
            MiningLaneKind::PriorityGames => {
                campaign.kind == CampaignKind::Drops
                    && priority.is_some()
                    && in_scope(drops.scope, campaign)
                    && !drops_filtered(settings, campaign)
            }
            MiningLaneKind::Badges => {
                campaign.gives_badge(now, &drops.exclude_drop_ids)
                    && in_scope(settings.badges.scope, campaign)
                    && !badges_filtered(settings, campaign)
                    && (chosen || !settings.badges.selection_only)
            }
            MiningLaneKind::OtherCampaigns => {
                campaign.kind == CampaignKind::Drops
                    && in_scope(drops.scope, campaign)
                    && !drops_filtered(settings, campaign)
                    && (chosen || !drops.selection_only)
            }
        })
        .map(|(position, lane)| (lane.kind, position))
}

fn automatic_key(
    linked_first: bool,
    order: DropsOrder,
    campaign: &Campaign,
    excluded: &[String],
) -> (QueueReason, u8, i64) {
    let linked_rank = u8::from(linked_first && campaign.linked != Some(true));

    match order {
        DropsOrder::EndingSoonest => (
            QueueReason::EndingSoon,
            linked_rank,
            campaign.ends_at.timestamp(),
        ),
        DropsOrder::LowestRemaining => (
            QueueReason::LowestRemaining,
            linked_rank,
            i64::from(campaign.remaining_minutes(excluded)),
        ),
    }
}

// ! A tier per campaign: 0 for the campaigns placed by hand, then one per
// lane of the mining order, in its order.
fn rank_tiers<'a>(
    settings: &AccountSettings,
    campaigns: &'a [Campaign],
    now: DateTime<Utc>,
) -> Vec<(&'a Campaign, QueueReason, u8)> {
    let drops = &settings.drops;
    let mut ranked: Vec<(&Campaign, QueueReason, RankKey)> = campaigns
        .iter()
        .filter(|c| {
            c.active_at(now) && !c.finished() && c.next_drop(now, &drops.exclude_drop_ids).is_some()
        })
        .filter(|c| !excluded(settings, c, now))
        .filter(|c| !drops.skip_unfinishable || c.can_finish(now, &drops.exclude_drop_ids))
        .filter_map(|c| {
            let pinned = drops.campaign_order.iter().position(|id| id == &c.id);
            let lane = lane_of(settings, c, now);

            if lane.is_none() && (pinned.is_none() || held_back(settings, c, now)) {
                return None;
            }

            let excluded = &drops.exclude_drop_ids;

            let key = match (pinned, lane) {
                (Some(index), _) => (QueueReason::Ordered, (0u8, index as i64, 0, 0)),
                (None, Some((kind, position))) => {
                    let tier = 1 + position as u8;

                    match kind {
                        MiningLaneKind::PriorityGames => (
                            QueueReason::Priority,
                            (tier, priority_index(settings, c).unwrap_or(0) as i64, 0, 0),
                        ),
                        MiningLaneKind::Badges => {
                            let badges = &settings.badges;
                            let (_, linked, value) =
                                automatic_key(badges.linked_first, badges.order, c, excluded);

                            (QueueReason::Badge, (tier, 0, linked, value))
                        }
                        MiningLaneKind::OtherCampaigns => {
                            let (reason, linked, value) =
                                automatic_key(drops.linked_first, drops.order, c, excluded);

                            (reason, (tier, 0, linked, value))
                        }
                    }
                }
                (None, None) => return None,
            };

            Some((c, key.0, key.1))
        })
        .collect();

    ranked.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.id.cmp(&b.0.id)));

    ranked
        .into_iter()
        .map(|(c, reason, key)| (c, reason, key.0))
        .collect()
}

pub fn rank_campaigns<'a>(
    settings: &AccountSettings,
    campaigns: &'a [Campaign],
    now: DateTime<Utc>,
) -> Vec<(&'a Campaign, QueueReason)> {
    rank_tiers(settings, campaigns, now)
        .into_iter()
        .map(|(c, reason, _)| (c, reason))
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub struct CurrentDrop<'a> {
    pub campaign_id: &'a str,
    pub channel_id: &'a str,
}

pub fn best_channel<'a>(
    settings: &AccountSettings,
    campaign: &Campaign,
    live: &'a [Channel],
) -> Option<&'a Channel> {
    pick_channel(settings, campaign, live, None)
}

fn pick_channel<'a>(
    settings: &AccountSettings,
    campaign: &Campaign,
    live: &'a [Channel],
    watching: Option<&str>,
) -> Option<&'a Channel> {
    let preferred = |channel: &Channel| {
        settings.drops.streamers_first
            && (channel.drops_enabled || !campaign.allowed_channels.is_empty())
            && settings
                .points
                .streamers
                .iter()
                .any(|s| s.enabled && s.login == channel.login)
    };

    let eligible: Vec<&Channel> = live
        .iter()
        .filter(|c| c.live && campaign.accepts_channel(&c.id))
        .filter(|c| {
            campaign.allowed_channels.is_empty()
                || campaign.allowed_channels.iter().any(|a| a.id == c.id)
        })
        .filter(|c| {
            // ! Twitch credits drop minutes only on a stream of the campaign's
            // game, also for a channel of its allow list.
            c.game.as_ref().is_some_and(|g| campaign.plays(g))
        })
        .collect();

    let kind = |c: &Channel| (preferred(c), c.drops_enabled);
    let best = eligible
        .iter()
        .copied()
        .max_by_key(|c| (kind(c), c.viewers.unwrap_or(0)))?;

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

pub fn plan(
    settings: &AccountSettings,
    campaigns: &[Campaign],
    live_by_campaign: &HashMap<String, Vec<Channel>>,
    streamers: &[StreamerState<'_>],
    current: Option<CurrentDrop<'_>>,
    now: DateTime<Utc>,
) -> Plan {
    let slots = settings.slots();
    let mut plan = Plan::default();
    let mut used: Vec<String> = Vec::new();

    if settings.mode != Mode::Points {
        let mut ranked = rank_tiers(settings, campaigns, now);

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

            campaign
                .next_drop(now, &settings.drops.exclude_drop_ids)
                .is_some()
                && pick_channel(settings, campaign, live, None).is_some()
        };

        if let Some(current) = current {
            let viable = ranked
                .iter()
                .position(|(c, _, _)| c.id == current.campaign_id)
                .filter(|&index| {
                    let (campaign, _, tier) = &ranked[index];
                    let live = live_by_campaign
                        .get(&campaign.id)
                        .map(Vec::as_slice)
                        .unwrap_or(&[]);
                    let above: Vec<u8> = ranked[..index]
                        .iter()
                        .filter(|(c, _, _)| minable(c))
                        .map(|(_, _, t)| *t)
                        .collect();

                    pick_channel(settings, campaign, live, watching(campaign)).is_some()
                        && !above.iter().any(|t| *t == 0 || t < tier)
                });

            if let Some(index) = viable {
                let item = ranked.remove(index);
                ranked.insert(0, item);
            }
        }

        let mut mining = false;

        for (campaign, reason, _) in ranked {
            let live = live_by_campaign
                .get(&campaign.id)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let chosen = pick_channel(settings, campaign, live, watching(campaign));
            let mut item = QueueItem {
                campaign_id: campaign.id.clone(),
                reason,
                live_channels: live.iter().filter(|c| c.live).count(),
                remaining_minutes: campaign.remaining_minutes(&settings.drops.exclude_drop_ids),
                ends_at: campaign.ends_at,
                linked: campaign.linked,
                blocked: None,
            };

            match (
                mining,
                chosen,
                campaign.next_drop(now, &settings.drops.exclude_drop_ids),
            ) {
                (false, Some(channel), Some(drop)) => {
                    mining = true;
                    used.push(channel.id.clone());
                    plan.slots.push(Slot {
                        channel: channel.clone(),
                        purpose: SlotPurpose::Drop {
                            campaign_id: campaign.id.clone(),
                            drop_id: drop.id.clone(),
                        },
                    });
                }
                (false, None, _) => item.blocked = Some(QueueBlock::NoLiveChannel),
                (true, _, _) => item.blocked = Some(QueueBlock::OneCampaignAtATime),
                (false, Some(_), None) => item.blocked = Some(QueueBlock::NoDropAvailable),
            }

            plan.queue.push(item);
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
