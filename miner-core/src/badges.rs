use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::api::Shell;
use crate::catalog::{too_large, FeedAnswer, FeedClient};
use crate::domain::{BenefitKind, Campaign};
use crate::error::{Error, ErrorCode, Result};
use crate::requests::RequestLog;
use crate::twitch::quests::BadgeRef;
use crate::twitch::{note_left_out, text, values};

const BADGES_URL: &str = "https://drops.sunkwi.com/api/twitch/badges";

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
}

pub fn badge_key(set_id: &str, version: &str) -> String {
    format!("{set_id}/{version}")
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BadgeCatalog {
    pub badges: Vec<GlobalBadge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetched_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub problem: Option<ErrorCode>,
    /// When the list is read again: later than usual after a refusal or a rate limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_refresh_at: Option<DateTime<Utc>>,
}

impl FeedAnswer for BadgeCatalog {
    fn parse(payload: &Value) -> Result<Self> {
        parse_badges(payload)
    }

    fn fetched_now(self) -> Self {
        Self {
            fetched_at: Some(Utc::now()),
            ..self
        }
    }
}

pub type BadgeCatalogClient = FeedClient<BadgeCatalog>;

impl BadgeCatalogClient {
    pub fn new(requests: RequestLog, shell: Shell) -> Result<Self> {
        Self::with_url(BADGES_URL, requests, &crate::user_agent(shell))
    }
}

fn parse_time(value: &Value) -> Option<DateTime<Utc>> {
    text(value)?.parse().ok()
}

fn count(value: &Value) -> u32 {
    value
        .as_u64()
        .map_or(0, |v| u32::try_from(v).unwrap_or(u32::MAX))
}

fn parse_requirement(record: &Value) -> Option<BadgeRequirement> {
    let source = match record["source"].as_str()? {
        "DROPS" => BadgeSource::Drops,
        "TWITCH_DESCRIPTION" => BadgeSource::TwitchDescription,
        _ => return None,
    };

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

    Ok(BadgeCatalog {
        badges,
        updated_at: parse_time(&payload["refreshedAt"]),
        fetched_at: Some(Utc::now()),
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

// ! A Drops reward names its badge only by its title: the badge is the one
// whose requirement names the campaign, then any badge of the same title.
pub fn badge_of<'a>(
    badges: &'a [GlobalBadge],
    campaign: &Campaign,
    reward_name: &str,
) -> Option<&'a GlobalBadge> {
    let wanted = normalized(reward_name);

    if wanted.is_empty() {
        return None;
    }

    let titled = |badge: &&GlobalBadge| normalized(&badge.title) == wanted;

    badges
        .iter()
        .filter(|badge| {
            badge
                .requirement
                .as_ref()
                .and_then(|r| r.campaign_id.as_deref())
                == Some(campaign.id.as_str())
        })
        .find(titled)
        .or_else(|| badges.iter().find(titled))
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

    // ! The badge list says which badge a reward names; without it (list not
    // read yet, API down), the titles Twitch gives with the owned badges do.
    pub fn owns(
        &self,
        badges: &[GlobalBadge],
        campaign: Option<&Campaign>,
        reward_name: &str,
    ) -> bool {
        let known = match campaign {
            Some(campaign) => badge_of(badges, campaign, reward_name),
            None => titled_badge(badges, reward_name),
        };

        match known {
            Some(badge) => self.keys.contains(&badge.key()),
            None => self.titles.contains(&normalized(reward_name)),
        }
    }
}

fn titled_badge<'a>(badges: &'a [GlobalBadge], name: &str) -> Option<&'a GlobalBadge> {
    let wanted = normalized(name);

    badges
        .iter()
        .find(|badge| !wanted.is_empty() && normalized(&badge.title) == wanted)
}

pub fn mark_owned_badges(campaign: &mut Campaign, badges: &[GlobalBadge], owned: &OwnedBadges) {
    let snapshot = campaign.clone();

    for drop in &mut campaign.drops {
        let badge_rewards: Vec<&str> = drop
            .benefits
            .iter()
            .filter(|b| b.kind == BenefitKind::Badge)
            .map(|b| b.name.as_str())
            .collect();

        drop.badge_owned = !badge_rewards.is_empty()
            && badge_rewards.len() == drop.benefits.len()
            && badge_rewards
                .iter()
                .all(|name| owned.owns(badges, Some(&snapshot), name));
    }
}
