use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ts_rs::TS;

use super::operations::Operation;
use super::{count, gql_error, note_left_out, parse_time, text, values, TwitchClient};
use crate::domain::{
    BadgeRef, Benefit, BenefitKind, Campaign, CampaignKind, CampaignStatus, Evolution, Game,
    Repeat, TimedDrop, TrackLane,
};
use crate::error::{Error, Result};

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
    #[serde(default)]
    pub badges: Vec<BadgeRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub chest_size: Option<u32>,
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
    pub current_subs: u32,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub repeat: Option<Repeat>,
    #[serde(default)]
    pub after_all: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub evolution: Option<Evolution>,
    // ! Filled from the progress for a group on a track, then kept by `attach_tracks` only when
    // its track offers several lanes to choose from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub lane: Option<TrackLane>,
}

impl QuestGroup {
    pub fn subs_met(&self) -> bool {
        self.current_subs >= self.required_subs
    }

    // ! Twitch's next window, or the end of the current one once its minutes are all in: a group
    // watched every day waits there until it may count again.
    pub fn waiting_until(&self, now: DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.next_eligible_at.filter(|at| *at > now).or_else(|| {
            self.window_ends_at.filter(|at| {
                *at > now
                    && self.status != QuestStatus::Claimable
                    && self.required_minutes > 0
                    && self.current_minutes >= self.required_minutes
                    && !self.earned()
            })
        })
    }

    // ! A repeated group keeps the status of the period it last earned: once
    // the next window Twitch named has opened, that status is the old period's.
    fn new_period(&self, now: DateTime<Utc>) -> bool {
        self.status == QuestStatus::Claimed
            && self
                .repeat
                .is_some_and(|repeat| self.grant_count < repeat.times)
            && self.next_eligible_at.is_some_and(|at| at <= now)
    }

    // ! Minutes count only inside their window: once it closed, the next watch starts a new one.
    fn current_minutes(&self, now: DateTime<Utc>) -> u32 {
        match self.status {
            QuestStatus::Claimable => self.required_minutes,
            _ if self.new_period(now) => 0,
            _ if self.window_ends_at.is_some_and(|at| at <= now) => 0,
            _ => self.current_minutes,
        }
    }

    pub fn earned(&self) -> bool {
        let grants = self
            .grant_count
            .saturating_add(u32::from(self.status == QuestStatus::Claimable));

        match self.repeat {
            Some(repeat) => grants >= repeat.times,
            None => matches!(self.status, QuestStatus::Claimable | QuestStatus::Claimed),
        }
    }

    // ! `earned` alone calls a claimable grant done, though the miner still
    // has to send the claim for it; a repeated group also stays open once a
    // new period began, even while its status still names the last one.
    pub fn open(&self) -> bool {
        self.status != QuestStatus::Unknown
            && (!self.earned() || self.status == QuestStatus::Claimable)
    }

    pub fn label(&self) -> &str {
        if self.name.is_empty() {
            self.rewards
                .first()
                .map(|r| r.name.as_str())
                .unwrap_or_default()
        } else {
            self.name.as_str()
        }
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

    // ! The watch groups of a quest are lanes (a track the viewer picks on
    // Twitch): once one is in progress, only that one counts. A subscription
    // group and the badge of a whole collection stand apart. On a Twitch track,
    // the lane equipped in the Drops panel keeps the watch while it has a group
    // left to earn: the others wait instead of taking its place.
    pub fn lanes(&self) -> Vec<&QuestGroup> {
        let lane = |group: &QuestGroup| group.required_subs == 0 && !group.after_all;
        let started = self
            .groups
            .iter()
            .any(|g| lane(g) && g.status != QuestStatus::NotStarted);
        let held = |track: &str| {
            self.groups.iter().any(|g| {
                g.open()
                    && g.lane
                        .as_ref()
                        .is_some_and(|l| l.equipped && l.track == track)
            })
        };

        self.groups
            .iter()
            .filter(|g| !started || !lane(g) || g.status != QuestStatus::NotStarted)
            .filter(|g| {
                g.lane
                    .as_ref()
                    .is_none_or(|l| l.equipped || !held(&l.track))
            })
            .collect()
    }

    // ! Only a sitewide quest names the categories its watch counts on.
    // Twitch's `completedRewardCampaigns` marks the whole campaign once one
    // of its tracks (subs, say) is done: a sibling group (a watch lane) can
    // still be open, so a group's own state decides.
    pub fn mined_at(&self, now: DateTime<Utc>) -> bool {
        self.sitewide && self.active_at(now) && self.groups.iter().any(QuestGroup::open)
    }

    pub fn claim_instance(&self, user_id: &str, group: &QuestGroup) -> String {
        let campaign = group.progress_campaign_id.as_deref().unwrap_or(&self.id);

        format!("{user_id}#{campaign}#{}", group.id)
    }

    pub fn to_campaign(&self, user_id: &str, now: DateTime<Utc>) -> Option<Campaign> {
        let lanes: Vec<&QuestGroup> = self
            .lanes()
            .into_iter()
            .filter(|g| !g.categories.is_empty() && g.status != QuestStatus::Unknown)
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
                name: group.label().to_owned(),
                starts_at: self.starts_at,
                ends_at: self.ends_at,
                // ! Unlike a Drops reward, a quest's subscription group (and the badge of a whole
                // collection, gated the same way) needs its count first: its watch time does not
                // count until then, so the drop carries none and waits for the subscriptions.
                required_minutes: if group.subs_met() {
                    group.required_minutes
                } else {
                    0
                },
                required_subs: group.required_subs,
                current_subs: group.current_subs,
                current_minutes: group.current_minutes(now),
                instance_id: (group.status == QuestStatus::Claimable)
                    .then(|| self.claim_instance(user_id, group)),
                claimed: group.status == QuestStatus::Claimed && !group.new_period(now),
                preconditions_met: Some(group.waiting_until(now).is_none()),
                precondition_drop_ids: Vec::new(),
                benefits: group
                    .rewards
                    .iter()
                    .map(|reward| Benefit {
                        id: reward.id.clone(),
                        name: reward.name.clone(),
                        kind: if reward.badges.is_empty() {
                            BenefitKind::Other
                        } else {
                            BenefitKind::Badge
                        },
                        image_url: reward
                            .badges
                            .first()
                            .filter(|_| reward.badges.len() == 1)
                            .and_then(|b| b.image_url.clone())
                            .or_else(|| reward.image_url.clone()),
                        badges: reward.badges.clone(),
                        chest_size: reward.chest_size,
                    })
                    .collect(),
                badge_owned: false,
                repeat: group.repeat,
                periods_done: group.grant_count,
                next_period_at: group.waiting_until(now),
                evolution: group.evolution.clone(),
                lane: group.lane.clone(),
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
            web_url: None,
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

    pub fn on_track(&self) -> bool {
        self.groups.iter().any(|g| g.lane.is_some())
    }

    // ! Twitch's own repeat wins over the badge list's; a group keeps its lane
    // only on a track that offers a choice.
    pub fn attach_tracks(&mut self, tracks: &Tracks) {
        for group in &mut self.groups {
            let known = tracks.groups.get(&group.id);

            group.repeat = known.and_then(|k| k.repeat).or(group.repeat);
            group.lane = known
                .and_then(|k| k.lane.as_ref())
                .filter(|(track, _)| tracks.choice(track))
                .map(|(track, lane)| TrackLane {
                    track: track.clone(),
                    lane: lane.clone(),
                    equipped: tracks.selected.get(track) == Some(lane),
                    refused: false,
                });
        }
    }
}

struct GroupTrack {
    lane: Option<(String, String)>,
    repeat: Option<Repeat>,
}

// ! The tracks of the campaigns read, with every lane each offers and the one
// the account has equipped on it.
#[derive(Default)]
pub struct Tracks {
    groups: HashMap<String, GroupTrack>,
    lanes: HashMap<String, HashSet<String>>,
    selected: HashMap<String, String>,
}

impl Tracks {
    // ! One answer per campaign asked; a campaign Twitch does not describe adds nothing.
    pub fn parse(answers: &Value) -> Self {
        let mut tracks = Self::default();

        for answer in values(answers) {
            for group in values(&answer["data"]["dropsCampaign"]["rewardGroups"]) {
                let Some(id) = text(&group["id"]) else {
                    continue;
                };
                let lane = parse_lane(group);

                if let Some((track, lane)) = &lane {
                    tracks
                        .lanes
                        .entry(track.clone())
                        .or_default()
                        .insert(lane.clone());
                }

                tracks.groups.insert(
                    id,
                    GroupTrack {
                        lane,
                        repeat: parse_repeatable(&group["progressCriteria"]["repeatableConfig"]),
                    },
                );
            }
        }

        tracks
    }

    // ! A track with a single lane offers nothing to choose: only one with
    // several asks for the lane to be equipped.
    fn choice(&self, track: &str) -> bool {
        self.lanes.get(track).is_some_and(|lanes| lanes.len() > 1)
    }

    pub fn choices(&self) -> Vec<String> {
        let mut tracks: Vec<String> = self
            .lanes
            .keys()
            .filter(|track| self.choice(track))
            .cloned()
            .collect();

        tracks.sort();
        tracks
    }

    pub fn select(&mut self, body: &Value) -> Result<()> {
        let list = &body["data"]["trackSelections"];

        if !list.is_array() {
            return Err(Error::InvalidResponse("trackSelections missing".into()));
        }

        self.selected = values(list)
            .filter_map(|selection| Some((text(&selection["track"])?, text(&selection["lane"])?)))
            .collect();

        Ok(())
    }
}

fn parse_lane(group: &Value) -> Option<(String, String)> {
    Some((text(&group["track"])?, text(&group["lane"])?))
}

// ! A group earned once per window, `repeatableTimes` windows in all.
fn parse_repeatable(config: &Value) -> Option<Repeat> {
    let times = count(&config["repeatableTimes"]);

    (times > 1).then(|| Repeat {
        times,
        every_hours: Some(count(&config["resetIntervalSeconds"]) / 3600).filter(|hours| *hours > 0),
    })
}

pub struct QuestsRead {
    pub quests: Vec<Quest>,
    pub owned: Vec<BadgeRef>,
    pub selected: Option<BadgeRef>,
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
    current_subs: u32,
    grant_count: u32,
    next_eligible_at: Option<DateTime<Utc>>,
    window_ends_at: Option<DateTime<Utc>>,
    badges: HashMap<String, BadgeRef>,
    lane: Option<(String, String)>,
    repeat: Option<Repeat>,
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
                    current_subs: count(&me["currentSubs"]),
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
                    lane: parse_lane(group),
                    repeat: parse_repeatable(&group["progressCriteria"]["repeatableConfig"]),
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
                            .saturating_add(count(&needs["turboSubsGoal"]))
                            .saturating_add(count(&needs["eventTriggerGoal"])),
                        categories: values(&group["categories"])
                            .filter_map(parse_game)
                            .collect(),
                        rewards: values(&group["rewards"])
                            .filter_map(|reward| {
                                let reward_id = text(&reward["id"])?;

                                Some(QuestReward {
                                    name: text(&reward["name"]).unwrap_or_default(),
                                    image_url: text(&reward["thumbnailImage"]["image1xURL"]),
                                    badges: me
                                        .and_then(|p| p.badges.get(&reward_id).cloned())
                                        .into_iter()
                                        .collect(),
                                    chest_size: None,
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
                        current_subs: me.map_or(0, |p| p.current_subs),
                        repeat: me.and_then(|p| p.repeat),
                        after_all: false,
                        evolution: None,
                        lane: me
                            .and_then(|p| p.lane.clone())
                            .map(|(track, lane)| TrackLane {
                                track,
                                lane,
                                equipped: false,
                                refused: false,
                            }),
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

        let user = progress
            .pointer("/data/currentUser")
            .filter(|v| v.is_object())
            .ok_or_else(|| Error::InvalidResponse("currentUser missing".into()))?;
        let inventory = user
            .get("inventory")
            .filter(|v| v.is_object())
            .ok_or_else(|| Error::InvalidResponse("currentUser.inventory missing".into()))?;
        let owned = self
            .gql(Operation::OwnedBadges, json!({ "login": self.login }))
            .await?;

        let mut quests = parse_quests(list, inventory);
        let tracks = self.tracks(&quests).await?;

        for quest in &mut quests {
            quest.attach_tracks(&tracks);
        }

        Ok(QuestsRead {
            quests,
            owned: parse_owned_badges(&owned)?,
            selected: parse_badge(&user["selectedBadge"]),
        })
    }

    // ! Asked only for the campaigns whose progress names a track: the others cost nothing more.
    async fn tracks(&self, quests: &[Quest]) -> Result<Tracks> {
        let asked: Vec<Value> = quests
            .iter()
            .filter(|quest| quest.on_track())
            .map(|quest| json!({"id": quest.id}))
            .collect();

        if asked.is_empty() {
            return Ok(Tracks::default());
        }

        let answers = self.gql_batch(Operation::QuestTracks, asked).await?;
        let mut tracks = Tracks::parse(&answers);
        let choices = tracks.choices();

        if !choices.is_empty() {
            let selections = self
                .gql(Operation::TrackSelections, json!({"tracks": choices}))
                .await?;

            tracks.select(&selections)?;
        }

        Ok(tracks)
    }

    // ! Sent once: the miner asks again only on its own next try. `Some` is Twitch's refusal.
    pub async fn select_lane(&self, lane: &TrackLane) -> Result<Option<String>> {
        let response = self
            .gql_once(
                Operation::SetTrackSelection,
                json!({"input": {"track": lane.track, "lane": lane.lane}}),
                || true,
            )
            .await?;

        if let Some(error) = gql_error(Operation::SetTrackSelection, &response) {
            return Err(error);
        }

        let answer = &response["data"]["setTrackSelection"];

        if !answer.is_object() {
            return Err(Error::InvalidResponse("setTrackSelection missing".into()));
        }

        Ok(text(&answer["error"]))
    }

    pub async fn select_badge(&self, badge: &BadgeRef) -> Result<Option<BadgeRef>> {
        let response = self
            .gql(
                Operation::SelectBadge,
                json!({"input": {"badgeSetID": badge.set_id, "badgeSetVersion": badge.version}}),
            )
            .await?;

        if let Some(error) = gql_error(Operation::SelectBadge, &response) {
            return Err(error);
        }

        Ok(parse_badge(
            &response["data"]["selectGlobalBadge"]["user"]["selectedBadge"],
        ))
    }
}
