use std::collections::HashSet;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::api::Shell;
use crate::domain::{
    BadgeRef, Benefit, BenefitKind, Campaign, CampaignKind, Evolution, Game, Repeat,
};
use crate::error::{Error, ErrorCode, Result};
use crate::feed::{FeedAnswer, FeedClient, MANUAL_REFRESH_GAP, REFRESH_INTERVAL};
use crate::http::too_large;
use crate::requests::{RequestKind, RequestLog};
use crate::twitch::drops::{parse_category, parse_repeat};
use crate::twitch::quests::{Quest, QuestGroup};
use crate::twitch::{count, note_left_out, parse_time, text, values};

const MAX_BADGES: usize = 5000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum BadgeSource {
    #[default]
    Drops,
    TwitchDescription,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "BadgeRequirementType")]
pub struct BadgeRequirement {
    pub source: BadgeSource,
    pub watch_time_only: bool,
    pub required_minutes: u32,
    pub required_subs: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub game_id: Option<String>,
    pub game_name: String,
    #[serde(default)]
    pub channels: Vec<String>,
    #[serde(default)]
    pub channel_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub starts_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub ends_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub campaign_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub campaign_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub repeat: Option<Repeat>,
    #[serde(default)]
    pub categories: Vec<Game>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub collection: Option<BadgeCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub evolves_from: Option<String>,
}

// ! A badge of an event collection: `chest_size` badges come out of one chest
// at random, and `complete_all` is the badge of the whole collection,
// subscriptions included.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "BadgeCollectionType")]
pub struct BadgeCollection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub chest_size: Option<u32>,
    #[serde(default)]
    pub complete_all: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "GlobalBadgeType")]
pub struct GlobalBadge {
    pub set_id: String,
    pub version_id: String,
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub click_url: Option<String>,
    // ! From the public catalogue only: its own page on drops.sunkwi.com.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub web_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub first_seen_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub requirement: Option<BadgeRequirement>,
}

impl GlobalBadge {
    pub fn key(&self) -> String {
        badge_key(&self.set_id, &self.version_id)
    }

    pub fn to_ref(&self) -> BadgeRef {
        BadgeRef {
            set_id: self.set_id.clone(),
            version: self.version_id.clone(),
            title: self.title.clone(),
            image_url: self.image_url.clone(),
        }
    }

    pub fn campaign_id(&self) -> Option<&str> {
        self.requirement.as_ref()?.campaign_id.as_deref()
    }

    // ! Twitch describes an evolution as "earned by evolving…" before the list names the badge to equip.
    pub fn evolution(&self, badges: &[GlobalBadge]) -> Option<Evolution> {
        let from = self
            .requirement
            .as_ref()
            .and_then(|r| r.evolves_from.as_deref());

        if from.is_none() && !self.description.to_lowercase().contains("evolving") {
            return None;
        }

        Some(Evolution {
            from: from.and_then(|key| {
                badges
                    .iter()
                    .find(|badge| badge.key() == key)
                    .map(GlobalBadge::to_ref)
            }),
            owned: false,
        })
    }
}

pub fn badge_key(set_id: &str, version: &str) -> String {
    format!("{set_id}/{version}")
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BadgeCatalog {
    pub badges: Vec<GlobalBadge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetched_at: Option<DateTime<Utc>>,
    #[serde(skip)]
    pub problem: Option<ErrorCode>,
    #[serde(skip)]
    pub next_refresh_at: Option<DateTime<Utc>>,
}

impl FeedAnswer for BadgeCatalog {
    const NAME: &'static str = "badge list";
    const CACHE_KEY: &'static str = "badges_cache";
    const KIND: RequestKind = RequestKind::Badges;
    const REFRESH_INTERVAL: Duration = REFRESH_INTERVAL;
    const MANUAL_REFRESH_GAP: Duration = MANUAL_REFRESH_GAP;

    fn parse(payload: &Value) -> Result<Self> {
        parse_badges(payload)
    }

    fn count(&self) -> usize {
        self.badges.len()
    }

    fn fetched_at(&self) -> Option<DateTime<Utc>> {
        self.fetched_at
    }

    fn mark_fetched(&mut self, at: DateTime<Utc>) {
        self.fetched_at = Some(at);
    }

    fn set_problem(&mut self, problem: Option<ErrorCode>) {
        self.problem = problem;
    }

    fn mark_next_refresh(&mut self, at: Option<DateTime<Utc>>) {
        self.next_refresh_at = at;
    }
}

pub type BadgeCatalogClient = FeedClient<BadgeCatalog>;

impl BadgeCatalogClient {
    pub fn new(requests: RequestLog, shell: Shell) -> Result<Self> {
        Self::with_url(
            &crate::drops_api::badges_url(),
            requests,
            &crate::user_agent(shell),
        )
    }
}

fn parse_requirement(record: &Value) -> Option<BadgeRequirement> {
    let source = match record["source"].as_str()? {
        "DROPS" => BadgeSource::Drops,
        "TWITCH_DESCRIPTION" => BadgeSource::TwitchDescription,
        _ => return None,
    };
    let collection = &record["collection"];

    Some(BadgeRequirement {
        source,
        watch_time_only: record["watchTimeOnly"].as_bool().unwrap_or(false),
        required_minutes: count(&record["requiredMinutes"]),
        required_subs: count(&record["requiredSubs"]),
        game_id: text(&record["gameId"]),
        game_name: text(&record["gameName"]).unwrap_or_default(),
        channels: values(&record["channels"])
            .filter_map(|c| c.as_str().map(str::to_owned))
            .collect(),
        channel_count: count(&record["channelCount"]),
        starts_at: parse_time(&record["startAt"]),
        ends_at: parse_time(&record["endAt"]),
        campaign_id: text(&record["campaignId"]),
        campaign_name: text(&record["campaignName"]),
        repeat: parse_repeat(&record["repeat"]),
        categories: values(&record["categories"])
            .filter_map(parse_category)
            .collect(),
        collection: collection.is_object().then(|| BadgeCollection {
            chest_size: collection["chestSize"]
                .as_u64()
                .and_then(|size| u32::try_from(size).ok()),
            complete_all: collection["completeAll"].as_bool().unwrap_or(false),
        }),
        evolves_from: text(&record["evolvesFrom"]["setId"]).map(|set_id| {
            badge_key(
                &set_id,
                &text(&record["evolvesFrom"]["versionId"]).unwrap_or_else(|| "1".to_owned()),
            )
        }),
    })
}

pub fn parse_badges(payload: &Value) -> Result<BadgeCatalog> {
    let records = payload["badges"]
        .as_array()
        .ok_or_else(|| Error::InvalidResponse("badges must be a list".into()))?;

    if records.len() > MAX_BADGES {
        return Err(too_large());
    }

    let mut seen: HashSet<String> = HashSet::new();
    let badges: Vec<GlobalBadge> = records
        .iter()
        .filter_map(|record| {
            Some(GlobalBadge {
                set_id: text(&record["setId"])?,
                version_id: text(&record["versionId"])?,
                title: text(&record["title"])?,
                description: text(&record["description"]).unwrap_or_default(),
                image_url: text(&record["imageUrl4x"])
                    .or_else(|| text(&record["imageUrl2x"]))
                    .or_else(|| text(&record["imageUrl1x"])),
                click_url: text(&record["clickUrl"]),
                web_url: crate::drops_api::valid_web_url(text(&record["webUrl"])),
                first_seen_at: parse_time(&record["firstSeenAt"]),
                requirement: record
                    .get("requirement")
                    .filter(|r| r.is_object())
                    .and_then(parse_requirement),
            })
        })
        .filter(|badge| seen.insert(badge.key()))
        .collect();

    note_left_out("badges", records.len(), badges.len());

    let now = Utc::now();

    Ok(BadgeCatalog {
        badges,
        read_at: Some(now),
        updated_at: parse_time(&payload["refreshedAt"]),
        fetched_at: Some(now),
        problem: None,
        next_refresh_at: None,
    })
}

fn normalized(title: &str) -> String {
    title
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn titled<'a>(
    mut badges: impl Iterator<Item = &'a GlobalBadge>,
    name: &str,
) -> Option<&'a GlobalBadge> {
    let wanted = normalized(name);

    badges.find(|badge| !wanted.is_empty() && normalized(&badge.title) == wanted)
}

#[derive(Debug, Default)]
pub struct RewardBadges<'a> {
    pub badges: Vec<&'a GlobalBadge>,
    pub chest_size: Option<u32>,
}

impl RewardBadges<'_> {
    pub fn refs(&self) -> Vec<BadgeRef> {
        self.badges.iter().map(|badge| badge.to_ref()).collect()
    }

    pub fn requirement(&self) -> Option<&BadgeRequirement> {
        self.badges.first()?.requirement.as_ref()
    }
}

pub struct RewardOf<'r> {
    pub campaign_id: &'r str,
    pub name: &'r str,
    pub badge: bool,
    pub required_minutes: u32,
    pub required_subs: u32,
}

// ! The list names the campaign of each badge: a reward is the badge of its
// campaign titled like it; a chest (a Poké Ball, a GTA VI chest) is titled like
// none and holds the campaign's watch badges no reward is named after. Only a
// campaign the list does not know falls back on the title of any badge.
pub fn reward_badges<'a>(
    badges: &'a [GlobalBadge],
    reward: &RewardOf<'_>,
    names: &HashSet<String>,
) -> RewardBadges<'a> {
    let mut own = badges
        .iter()
        .filter(|badge| badge.campaign_id() == Some(reward.campaign_id))
        .peekable();

    if own.peek().is_none() {
        return RewardBadges {
            badges: reward
                .badge
                .then(|| titled(badges.iter(), reward.name))
                .flatten()
                .into_iter()
                .collect(),
            chest_size: None,
        };
    }

    let own: Vec<&GlobalBadge> = own.collect();

    if let Some(badge) = titled(own.iter().copied(), reward.name) {
        return RewardBadges {
            badges: vec![badge],
            chest_size: None,
        };
    }

    if reward.required_subs > 0 {
        return RewardBadges::default();
    }

    let chest: Vec<&GlobalBadge> = own
        .into_iter()
        .filter(|badge| {
            badge.requirement.as_ref().is_some_and(|r| {
                r.collection.is_some_and(|c| !c.complete_all)
                    && r.required_subs == 0
                    && (reward.required_minutes == 0
                        || r.required_minutes == reward.required_minutes)
            }) && !names.contains(&normalized(&badge.title))
        })
        .collect();
    let chest_size = chest
        .iter()
        .filter_map(|badge| badge.requirement.as_ref()?.collection?.chest_size)
        .max();

    RewardBadges {
        badges: chest,
        chest_size,
    }
}

pub fn reward_names<'a>(names: impl Iterator<Item = &'a str>) -> HashSet<String> {
    names
        .map(normalized)
        .filter(|name| !name.is_empty())
        .collect()
}

pub fn known_badges<'a>(badges: &'a [GlobalBadge], refs: &[BadgeRef]) -> RewardBadges<'a> {
    RewardBadges {
        badges: refs
            .iter()
            .filter_map(|known| badges.iter().find(|badge| badge.key() == known.key()))
            .collect(),
        chest_size: None,
    }
}

// ! A reward the list knows gets its badges, and its drop the period and the
// categories of that badge.
pub fn attach_badges(campaign: &mut Campaign, badges: &[GlobalBadge]) {
    let names = reward_names(
        campaign
            .drops
            .iter()
            .flat_map(|drop| drop.benefits.iter().map(|benefit| benefit.name.as_str())),
    );
    let mut categories: Vec<Game> = Vec::new();

    for drop in &mut campaign.drops {
        for benefit in &mut drop.benefits {
            if !benefit.badges.is_empty() {
                continue;
            }

            let found = reward_badges(
                badges,
                &RewardOf {
                    campaign_id: &campaign.id,
                    name: &benefit.name,
                    badge: benefit.kind == BenefitKind::Badge,
                    required_minutes: drop.required_minutes,
                    required_subs: drop.required_subs,
                },
                &names,
            );

            if found.badges.is_empty() {
                continue;
            }

            benefit.kind = BenefitKind::Badge;
            benefit.badges = found.refs();
            benefit.chest_size = found.chest_size;

            if let Some(requirement) = found.requirement() {
                drop.repeat = drop.repeat.or(requirement.repeat);

                if categories.is_empty() && requirement.categories.len() > 1 {
                    categories.clone_from(&requirement.categories);
                }
            }
        }
    }

    if campaign.categories.is_empty() {
        campaign.categories = categories;
    }
}

pub struct OwnedBadges {
    keys: HashSet<String>,
    titles: HashSet<String>,
}

impl OwnedBadges {
    pub fn new(owned: &[BadgeRef]) -> Self {
        Self {
            keys: owned.iter().map(BadgeRef::key).collect(),
            titles: owned
                .iter()
                .map(|badge| normalized(&badge.title))
                .filter(|title| !title.is_empty())
                .collect(),
        }
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.keys.contains(key)
    }

    // ! Without the list (not read yet, API down), the titles Twitch gives
    // with the owned badges say it; a chest is owned once each of its badges is.
    pub fn owns(&self, benefit: &Benefit) -> bool {
        if benefit.badges.is_empty() {
            self.titles.contains(&normalized(&benefit.name))
        } else {
            benefit
                .badges
                .iter()
                .all(|badge| self.keys.contains(&badge.key()))
        }
    }
}

// ! A quest's reward is a badge even when Twitch does not say so: only a
// Drops reward has its kind.
pub fn mark_owned(campaign: &mut Campaign, owned: &OwnedBadges) {
    let quest = campaign.kind == CampaignKind::Quest;

    for drop in &mut campaign.drops {
        drop.badge_owned = !drop.benefits.is_empty()
            && drop.benefits.iter().all(|benefit| {
                (quest || benefit.kind == BenefitKind::Badge) && owned.owns(benefit)
            });
    }
}

impl Quest {
    fn listed_id(&self, badges: &[GlobalBadge]) -> String {
        std::iter::once(&self.id)
            .chain(
                self.groups
                    .iter()
                    .filter_map(|g| g.progress_campaign_id.as_ref()),
            )
            .find(|id| badges.iter().any(|b| b.campaign_id() == Some(id.as_str())))
            .unwrap_or(&self.id)
            .clone()
    }

    // ! The badge list says what each reward gives, how often its group
    // repeats and which one is the badge of the whole collection: that one
    // waits until Twitch has every other group earned, subscriptions included.
    pub fn attach_badges(&mut self, badges: &[GlobalBadge], owned: &OwnedBadges) {
        let campaign_id = self.listed_id(badges);
        let names = reward_names(
            self.groups
                .iter()
                .flat_map(|g| g.rewards.iter().map(|r| r.name.as_str())),
        );

        for group in &mut self.groups {
            for reward in &mut group.rewards {
                let found = if reward.badges.is_empty() {
                    reward_badges(
                        badges,
                        &RewardOf {
                            campaign_id: &campaign_id,
                            name: &reward.name,
                            badge: true,
                            required_minutes: group.required_minutes,
                            required_subs: group.required_subs,
                        },
                        &names,
                    )
                } else {
                    known_badges(badges, &reward.badges)
                };

                if reward.badges.is_empty() {
                    reward.badges = found.refs();
                    reward.chest_size = found.chest_size;
                }

                if let Some(requirement) = found.requirement() {
                    if requirement.collection.is_some_and(|c| c.complete_all) {
                        group.after_all = true;
                        group.required_subs = group.required_subs.max(requirement.required_subs);
                    } else {
                        group.repeat = group.repeat.or(requirement.repeat);
                    }
                }

                if let [badge] = found.badges.as_slice() {
                    group.evolution = badge.evolution(badges).map(|evolution| Evolution {
                        owned: evolution
                            .from
                            .as_ref()
                            .is_some_and(|from| owned.has_key(&from.key())),
                        ..evolution
                    });
                }
            }
        }

        let earned: Vec<bool> = self.groups.iter().map(QuestGroup::earned).collect();

        for (index, group) in self.groups.iter_mut().enumerate() {
            let made = if group.after_all {
                earned
                    .iter()
                    .enumerate()
                    .all(|(other, done)| other == index || *done)
            } else {
                earned[index]
            };

            group.current_subs = if made {
                group.required_subs
            } else if group.after_all {
                0
            } else {
                group.current_subs
            };
        }
    }
}
