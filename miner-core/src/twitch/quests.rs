use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ts_rs::TS;

use super::operations::Operation;
use super::{note_left_out, text, values, TwitchClient};
use crate::badges::badge_key;
use crate::domain::{
    Benefit, BenefitKind, Campaign, CampaignKind, CampaignStatus, Game, TimedDrop,
};
use crate::error::{Error, Result};

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
        badge_key(&self.set_id, &self.version)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum QuestStatus {
    #[default]
    NotStarted,
    InProgress,
    Claimable,
    Claimed,
    // ! A status this build does not know (expired, ineligible…): never mined.
    Unknown,
}

impl QuestStatus {
    fn from_twitch(status: Option<&str>) -> Self {
        match status {
            Some("IN_PROGRESS") => Self::InProgress,
            Some("CLAIMABLE") => Self::Claimable,
            Some("CLAIMED" | "FULFILLED") => Self::Claimed,
            Some(_) => Self::Unknown,
            None => Self::NotStarted,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "QuestRewardType")]
pub struct QuestReward {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub badge: Option<BadgeRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "QuestGroupType")]
pub struct QuestGroup {
    pub id: String,
    #[serde(default)]
    pub name: String,
    pub required_minutes: u32,
    #[serde(default)]
    pub required_subs: u32,
    #[serde(default)]
    pub categories: Vec<Game>,
    #[serde(default)]
    pub rewards: Vec<QuestReward>,
    #[serde(default)]
    pub status: QuestStatus,
    #[serde(default)]
    pub current_minutes: u32,
    #[serde(default)]
    pub grant_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub next_eligible_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub window_ends_at: Option<DateTime<Utc>>,
    // ! The campaign id of the account's progress, the one its claim names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub progress_campaign_id: Option<String>,
}

impl QuestGroup {
    pub fn watch_only(&self) -> bool {
        self.required_subs == 0 && self.required_minutes > 0
    }

    pub fn waiting_until(&self, now: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.next_eligible_at.filter(|at| *at > now)
    }

    pub fn minable(&self, now: DateTime<Utc>) -> bool {
        self.watch_only()
            && !self.categories.is_empty()
            && matches!(
                self.status,
                QuestStatus::NotStarted | QuestStatus::InProgress
            )
            && self.current_minutes < self.required_minutes
            && self.waiting_until(now).is_none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(rename = "QuestType")]
pub struct Quest {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub brand: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub about_url: Option<String>,
    pub sitewide: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub game: Option<Game>,
    #[serde(default)]
    pub groups: Vec<QuestGroup>,
    #[serde(default)]
    pub completed: bool,
}

impl Quest {
    pub fn active_at(&self, now: DateTime<Utc>) -> bool {
        self.starts_at <= now && now < self.ends_at
    }

    // ! A quest of several groups is a set of lanes (a track the viewer
    // picks on Twitch): once one is in progress, only that one counts.
    pub fn lanes(&self) -> Vec<&QuestGroup> {
        let started: Vec<&QuestGroup> = self
            .groups
            .iter()
            .filter(|g| g.status != QuestStatus::NotStarted)
            .collect();

        if started.is_empty() {
            self.groups.iter().collect()
        } else {
            started
        }
    }

    pub fn minable(&self, now: DateTime<Utc>) -> bool {
        self.mined_at(now) && self.lanes().iter().any(|g| g.minable(now))
    }

    // ! Only a sitewide quest names the categories its watch counts on.
    pub fn mined_at(&self, now: DateTime<Utc>) -> bool {
        self.sitewide && !self.completed && self.active_at(now)
    }

    pub fn claim_instance(&self, user_id: &str, group: &QuestGroup) -> String {
        let campaign = group.progress_campaign_id.as_deref().unwrap_or(&self.id);

        format!("{user_id}#{campaign}#{}", group.id)
    }

    pub fn to_campaign(&self, user_id: &str, now: DateTime<Utc>) -> Option<Campaign> {
        let lanes: Vec<&QuestGroup> = self
            .lanes()
            .into_iter()
            .filter(|g| g.watch_only() && !g.categories.is_empty())
            .collect();
        let mut categories: Vec<Game> = Vec::new();

        for group in &lanes {
            for category in &group.categories {
                if !categories.iter().any(|c| c.id == category.id) {
                    categories.push(category.clone());
                }
            }
        }

        let game = categories.first().cloned()?;
        let drops = lanes
            .iter()
            .map(|group| TimedDrop {
                id: group.id.clone(),
                name: if group.name.is_empty() {
                    group
                        .rewards
                        .first()
                        .map(|r| r.name.clone())
                        .unwrap_or_default()
                } else {
                    group.name.clone()
                },
                starts_at: self.starts_at,
                ends_at: self.ends_at,
                required_minutes: group.required_minutes,
                required_subs: group.required_subs,
                current_minutes: if group.status == QuestStatus::Claimable {
                    group.required_minutes
                } else {
                    group.current_minutes
                },
                instance_id: (group.status == QuestStatus::Claimable)
                    .then(|| self.claim_instance(user_id, group)),
                claimed: group.status == QuestStatus::Claimed,
                preconditions_met: Some(group.waiting_until(now).is_none()),
                precondition_drop_ids: Vec::new(),
                benefits: group
                    .rewards
                    .iter()
                    .map(|reward| Benefit {
                        id: reward.id.clone(),
                        name: reward.name.clone(),
                        kind: if reward.badge.is_some() {
                            BenefitKind::Badge
                        } else {
                            BenefitKind::Other
                        },
                        image_url: reward
                            .badge
                            .as_ref()
                            .and_then(|b| b.image_url.clone())
                            .or_else(|| reward.image_url.clone()),
                    })
                    .collect(),
                badge_owned: false,
            })
            .collect();

        Some(Campaign {
            id: self.id.clone(),
            name: self.name.clone(),
            kind: CampaignKind::Quest,
            game,
            categories,
            status: CampaignStatus::Active,
            linked: None,
            link_url: None,
            image_url: self.image_url.clone(),
            starts_at: self.starts_at,
            ends_at: self.ends_at,
            allowed_channels: Vec::new(),
            drops,
            in_inventory: self
                .groups
                .iter()
                .any(|g| g.status != QuestStatus::NotStarted),
        })
    }
}

pub struct QuestsRead {
    pub quests: Vec<Quest>,
    pub owned: Vec<BadgeRef>,
}

fn parse_time(value: &Value) -> Option<DateTime<Utc>> {
    text(value)?.parse().ok()
}

fn count(value: &Value) -> u32 {
    value
        .as_u64()
        .map_or(0, |v| u32::try_from(v).unwrap_or(u32::MAX))
}

fn parse_game(node: &Value) -> Option<Game> {
    Some(Game {
        id: text(&node["id"])?,
        name: text(&node["displayName"]).or_else(|| text(&node["name"]))?,
        slug: text(&node["slug"]),
        // ! Quests send the box art as a template, "-{width}x{height}.jpg".
        box_art_url: text(&node["boxArtURL"]).map(|url| url.replace("{width}x{height}", "285x380")),
    })
}

fn parse_badge(node: &Value) -> Option<BadgeRef> {
    Some(BadgeRef {
        set_id: text(&node["setID"])?,
        version: text(&node["version"]).unwrap_or_else(|| "1".to_owned()),
        title: text(&node["title"]).unwrap_or_default(),
        image_url: text(&node["imageURL"]),
    })
}

struct GroupProgress {
    campaign_id: Option<String>,
    name: String,
    status: QuestStatus,
    current_minutes: u32,
    grant_count: u32,
    next_eligible_at: Option<DateTime<Utc>>,
    window_ends_at: Option<DateTime<Utc>>,
    badges: HashMap<String, BadgeRef>,
}

fn parse_progress(inventory: &Value) -> HashMap<String, GroupProgress> {
    let mut progress = HashMap::new();

    for campaign in values(&inventory["viewerRewardDropCampaignsInProgress"]) {
        for group in values(&campaign["rewardGroups"]) {
            let Some(id) = text(&group["id"]) else {
                continue;
            };
            let me = &group["self"];

            progress.insert(
                id,
                GroupProgress {
                    campaign_id: text(&campaign["id"]),
                    name: text(&group["name"]).unwrap_or_default(),
                    status: QuestStatus::from_twitch(me["status"].as_str()),
                    current_minutes: count(&me["currentMinutesWatched"]),
                    grant_count: count(&me["grantCount"]),
                    next_eligible_at: parse_time(&me["nextEligibleAt"]),
                    window_ends_at: parse_time(&me["currentWindow"]["expiresAt"]),
                    badges: values(&group["rewards"])
                        .filter_map(|reward| {
                            Some((
                                text(&reward["id"])?,
                                parse_badge(&reward["dropDetail"]["badge"])?,
                            ))
                        })
                        .collect(),
                },
            );
        }
    }

    progress
}

pub fn parse_quests(available: &Value, inventory: &Value) -> Vec<Quest> {
    let progress = parse_progress(inventory);
    let completed: HashSet<String> = values(&inventory["completedRewardCampaigns"])
        .filter_map(|c| text(&c["id"]))
        .collect();
    let records: Vec<&Value> = values(available).collect();
    let quests: Vec<Quest> = records
        .iter()
        .filter_map(|record| {
            let id = text(&record["id"])?;
            let groups: Vec<QuestGroup> = values(&record["rewardGroups"])
                .filter_map(|group| {
                    let id = text(&group["id"])?;
                    let me = progress.get(&id);
                    let needs = &group["unlockRequirements"];

                    Some(QuestGroup {
                        name: me.map(|p| p.name.clone()).unwrap_or_default(),
                        required_minutes: count(&needs["minuteWatchedGoal"]),
                        required_subs: count(&needs["subsGoal"])
                            + count(&needs["turboSubsGoal"])
                            + count(&needs["eventTriggerGoal"]),
                        categories: values(&group["categories"])
                            .filter_map(parse_game)
                            .collect(),
                        rewards: values(&group["rewards"])
                            .filter_map(|reward| {
                                let reward_id = text(&reward["id"])?;

                                Some(QuestReward {
                                    name: text(&reward["name"]).unwrap_or_default(),
                                    image_url: text(&reward["thumbnailImage"]["image1xURL"]),
                                    badge: me.and_then(|p| p.badges.get(&reward_id).cloned()),
                                    id: reward_id,
                                })
                            })
                            .collect(),
                        status: me.map(|p| p.status).unwrap_or_default(),
                        current_minutes: me.map_or(0, |p| p.current_minutes),
                        grant_count: me.map_or(0, |p| p.grant_count),
                        next_eligible_at: me.and_then(|p| p.next_eligible_at),
                        window_ends_at: me.and_then(|p| p.window_ends_at),
                        progress_campaign_id: me.and_then(|p| p.campaign_id.clone()),
                        id,
                    })
                })
                .collect();

            let finished = completed.contains(&id)
                || groups.iter().any(|g: &QuestGroup| {
                    g.progress_campaign_id
                        .as_ref()
                        .is_some_and(|campaign| completed.contains(campaign))
                });

            Some(Quest {
                name: text(&record["name"]).unwrap_or_default(),
                brand: text(&record["brand"]).unwrap_or_default(),
                starts_at: parse_time(&record["startsAt"])?,
                ends_at: parse_time(&record["endsAt"])?,
                image_url: text(&record["image"]["image1xURL"]),
                summary: text(&record["summary"]).unwrap_or_default(),
                about_url: text(&record["aboutURL"]).or_else(|| text(&record["externalURL"])),
                sitewide: record["isSitewide"].as_bool().unwrap_or(false),
                game: parse_game(&record["game"]),
                groups,
                completed: finished,
                id,
            })
        })
        .collect();

    note_left_out("quests", records.len(), quests.len());

    quests
}

pub fn parse_owned_badges(body: &Value) -> Result<Vec<BadgeRef>> {
    let viewer = &body["data"]["channelViewer"];

    if !viewer.is_object() {
        return Err(Error::InvalidResponse("channelViewer missing".into()));
    }

    Ok(values(&viewer["earnedBadges"])
        .filter_map(parse_badge)
        .collect())
}

impl TwitchClient {
    pub async fn quests(&self) -> Result<QuestsRead> {
        let available = self.gql(Operation::Quests, Value::Null).await?;
        let progress = self.gql(Operation::QuestProgress, Value::Null).await?;
        let list = &available["data"]["rewardCampaignsAvailableToUser"];

        if !list.is_array() {
            return Err(Error::InvalidResponse(
                "rewardCampaignsAvailableToUser missing".into(),
            ));
        }

        let inventory = progress
            .pointer("/data/currentUser/inventory")
            .filter(|v| v.is_object())
            .ok_or_else(|| Error::InvalidResponse("currentUser.inventory missing".into()))?;
        let owned = self
            .gql(Operation::OwnedBadges, json!({ "login": self.login }))
            .await?;

        Ok(QuestsRead {
            quests: parse_quests(list, inventory),
            owned: parse_owned_badges(&owned)?,
        })
    }
}
