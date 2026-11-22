use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use super::operations::Operation;
use super::{gql_error, note_left_out, parse_time, saturated, text, values, TwitchClient};
use crate::domain::{
    Benefit, BenefitKind, Campaign, CampaignKind, CampaignStatus, ChannelRef, Game, Repeat,
    TimedDrop,
};
use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Award {
    pub at: DateTime<Utc>,
    pub count: u32,
}

pub struct Inventory {
    pub campaigns: Vec<Campaign>,
    pub awards: HashMap<String, Award>,
    // ! The reward's own name, by id: what dates a badge once its campaign is gone.
    pub names: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimOutcome {
    Claimed,
    AlreadyClaimed,
    Refused(String),
}

impl TwitchClient {
    pub async fn inventory(&self) -> Result<Inventory> {
        let response = self.gql(Operation::Inventory, Value::Null).await?;
        let inventory = response
            .pointer("/data/currentUser/inventory")
            .filter(|v| v.is_object())
            .ok_or_else(|| Error::InvalidResponse("currentUser.inventory missing".into()))?;

        Ok(parse_inventory(inventory))
    }

    pub async fn claim_drop(&self, instance_id: &str) -> Result<ClaimOutcome> {
        let (response, resent) = self
            .gql_resent(
                Operation::ClaimDrop,
                json!({"input": {"dropInstanceID": instance_id}}),
            )
            .await?;

        if let Some(error) = gql_error(Operation::ClaimDrop, &response) {
            return Err(error);
        }

        Ok(claim_outcome(&response, resent))
    }
}

// ! Sent again after a lost answer, "already claimed" is this very claim taken the first time.
pub(crate) fn claim_outcome(response: &Value, resent: bool) -> ClaimOutcome {
    match response["data"]["claimDropRewards"]["status"].as_str() {
        Some("ELIGIBLE_FOR_ALL") => ClaimOutcome::Claimed,
        Some("DROP_INSTANCE_ALREADY_CLAIMED") if resent => ClaimOutcome::Claimed,
        Some("DROP_INSTANCE_ALREADY_CLAIMED") => ClaimOutcome::AlreadyClaimed,
        Some(other) => ClaimOutcome::Refused(other.to_owned()),
        None => ClaimOutcome::Refused("unknown".to_owned()),
    }
}

// ! `totalCount` and `lastAwardedAt` cover every award of the reward id, in any
// campaign: Twitch reuses an id across the campaigns of a game.
pub fn parse_inventory(inventory: &Value) -> Inventory {
    let awards = values(&inventory["gameEventDrops"])
        .filter_map(|v| {
            Some((
                text(&v["id"])?,
                Award {
                    at: text(&v["lastAwardedAt"])?.parse::<DateTime<Utc>>().ok()?,
                    count: v["totalCount"].as_u64().map_or(1, saturated),
                },
            ))
        })
        .collect();
    let names = values(&inventory["gameEventDrops"])
        .filter_map(|v| Some((text(&v["id"])?, text(&v["name"])?)))
        .collect();
    let records: Vec<&Value> = values(&inventory["dropCampaignsInProgress"]).collect();
    let campaigns: Vec<Campaign> = records
        .iter()
        .filter_map(|record| parse_campaign(record, true))
        .collect();

    note_left_out("inventory campaigns", records.len(), campaigns.len());

    Inventory {
        campaigns,
        awards,
        names,
    }
}

fn parse_status(value: &Value) -> CampaignStatus {
    match value.as_str().unwrap_or("ACTIVE") {
        "UPCOMING" => CampaignStatus::Upcoming,
        "EXPIRED" => CampaignStatus::Expired,
        "DISABLED" => CampaignStatus::Disabled,
        _ => CampaignStatus::Active,
    }
}

const REPEAT_TIMES_MAX: u32 = 366;

const REPEAT_HOURS_MAX: f64 = 8784.0;

pub fn parse_repeat(value: &Value) -> Option<Repeat> {
    let times = saturated(value["times"].as_u64()?);
    let every = value["everyHours"].as_f64().filter(|hours| *hours > 0.0);

    if !(2..=REPEAT_TIMES_MAX).contains(&times) || every.is_some_and(|h| h > REPEAT_HOURS_MAX) {
        return None;
    }

    Some(Repeat {
        times,
        every_hours: every.map(|hours| hours.round() as u32),
    })
}

pub fn parse_category(node: &Value) -> Option<Game> {
    Some(Game {
        id: text(&node["id"])?,
        name: text(&node["displayName"]).or_else(|| text(&node["name"]))?,
        slug: text(&node["slug"]),
        box_art_url: text(&node["boxArtURL"]),
    })
}

// ! A campaign with no game of its own counts on its categories: the first
// stands for its game.
pub fn parse_campaign(record: &Value, from_inventory: bool) -> Option<Campaign> {
    let id = text(&record["id"])?;
    let categories: Vec<Game> = values(&record["categories"])
        .filter_map(parse_category)
        .collect();
    let game_node = &record["game"];
    let game = match text(&game_node["displayName"]).or_else(|| text(&game_node["name"])) {
        Some(name) => Game {
            id: text(&game_node["id"]).unwrap_or_default(),
            name,
            slug: text(&game_node["slug"]),
            box_art_url: text(&game_node["boxArtURL"]),
        },
        None => categories.first()?.clone(),
    };

    // ! An allow list Twitch switched off ("isEnabled": false) limits nothing.
    let allow_enabled = record["allow"]["isEnabled"].as_bool().unwrap_or(true);
    let allowed_channels = values(&record["allow"]["channels"])
        .filter(|_| allow_enabled)
        .filter_map(|c| {
            Some(ChannelRef {
                id: text(&c["id"])?,
                login: text(&c["name"])
                    .or_else(|| text(&c["login"]))
                    .unwrap_or_default(),
                display_name: text(&c["displayName"]).unwrap_or_default(),
            })
        })
        .collect();

    // ! The public catalogue holds no account: a `self` block there would mark every user's drops.
    let account = if from_inventory {
        &record["self"]
    } else {
        &Value::Null
    };
    let drops = values(&record["timeBasedDrops"])
        .filter_map(|d| {
            let me = if from_inventory {
                &d["self"]
            } else {
                &Value::Null
            };

            Some(TimedDrop {
                id: text(&d["id"])?,
                name: text(&d["name"]).unwrap_or_default(),
                starts_at: parse_time(&d["startAt"])?,
                ends_at: parse_time(&d["endAt"])?,
                required_minutes: d["requiredMinutesWatched"].as_u64().map_or(0, saturated),
                required_subs: d["requiredSubs"].as_u64().map_or(0, saturated),
                current_subs: me["currentSubs"].as_u64().map_or(0, saturated),
                current_minutes: me["currentMinutesWatched"].as_u64().map_or(0, saturated),
                instance_id: text(&me["dropInstanceID"]),
                claimed: me["isClaimed"].as_bool().unwrap_or(false),
                preconditions_met: me["hasPreconditionsMet"].as_bool(),
                precondition_drop_ids: values(&d["preconditionDrops"])
                    .filter_map(|p| text(&p["id"]))
                    .collect(),
                benefits: values(&d["benefitEdges"])
                    .filter_map(|edge| {
                        let benefit = &edge["benefit"];

                        Some(Benefit {
                            id: text(&benefit["id"])?,
                            name: text(&benefit["name"]).unwrap_or_default(),
                            kind: BenefitKind::from_twitch(benefit["distributionType"].as_str()),
                            image_url: text(&benefit["imageAssetURL"]),
                            badges: Vec::new(),
                            chest_size: None,
                        })
                    })
                    .collect(),
                badge_owned: false,
                repeat: parse_repeat(&d["repeat"]),
                periods_done: 0,
                next_period_at: None,
                evolution: None,
            })
        })
        .collect();

    let mut campaign = Campaign {
        id,
        name: text(&record["name"]).unwrap_or_default(),
        kind: CampaignKind::Drops,
        game,
        categories,
        status: parse_status(&record["status"]),
        linked: account["isAccountConnected"].as_bool(),
        link_url: text(&record["accountLinkURL"]),
        image_url: text(&record["imageURL"]),
        web_url: crate::drops_api::valid_web_url(text(&record["webUrl"])),
        starts_at: parse_time(&record["startAt"])?,
        ends_at: parse_time(&record["endAt"])?,
        allowed_channels,
        drops,
        in_inventory: from_inventory,
    };

    if !campaign.needs_link() {
        campaign.linked = None;
        campaign.link_url = None;
    }

    Some(campaign)
}
