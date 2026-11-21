// ! Twitch does not show the campaign list to the TV client, so campaigns
// come from this public catalogue.

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::InstallKind;
use crate::config::GameEntry;
use crate::domain::Campaign;
use crate::drops_api::{parse_campaign_channels, CampaignChannels, CATALOG_REFRESH_INTERVAL};
use crate::error::{Error, ErrorCode, Result};
use crate::feed::{FeedAnswer, FeedClient};
use crate::http::too_large;
use crate::requests::{RequestKind, RequestLog};
use crate::twitch::drops::parse_campaign;
use crate::twitch::note_left_out;

const MAX_CAMPAIGNS: usize = 3000;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Catalog {
    pub campaigns: Vec<Campaign>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetched_at: Option<DateTime<Utc>>,
    #[serde(skip)]
    pub problem: Option<ErrorCode>,
    #[serde(skip)]
    pub(crate) live_channels: HashMap<String, CampaignChannels>,
}

impl FeedAnswer for Catalog {
    const NAME: &'static str = "catalogue";
    const CACHE_KEY: &'static str = "catalog_cache";
    const KIND: RequestKind = RequestKind::Catalog;
    const REFRESH_INTERVAL: Duration = CATALOG_REFRESH_INTERVAL;
    const MANUAL_REFRESH_GAP: Duration = CATALOG_REFRESH_INTERVAL;

    fn parse(payload: &Value) -> Result<Self> {
        parse_catalog(payload)
    }

    fn count(&self) -> usize {
        self.campaigns.len()
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

    // ! `liveChannels` refreshes every poll regardless, so only a change to
    // the campaigns themselves counts as a meaningful update.
    fn meaningfully_changed(&self, previous: &Self) -> bool {
        self.campaigns != previous.campaigns
    }
}

pub type CatalogClient = FeedClient<Catalog>;

impl CatalogClient {
    pub fn new(requests: RequestLog, install_kind: InstallKind, install_id: &str) -> Result<Self> {
        Self::with_headers(
            &crate::drops_api::catalog_url(),
            requests,
            &crate::user_agent(install_kind),
            &[("x-install-id", install_id.to_owned())],
        )
    }
}

pub fn parse_catalog(payload: &Value) -> Result<Catalog> {
    let records = payload["games"]
        .as_array()
        .ok_or_else(|| Error::InvalidResponse("catalogue games must be a list".into()))?;

    let mut campaigns = Vec::new();
    let mut live_channels = HashMap::new();

    for game in records {
        let box_art = game["gameBoxArtURL"].as_str().map(str::to_owned);
        let rewards = game["rewards"].as_array();
        let items: Vec<&Value> = match rewards {
            Some(list) => list.iter().collect(),
            None => vec![game],
        };

        let before = campaigns.len();
        let total = items.len();

        for record in items {
            if let Some(mut campaign) = parse_campaign(record, false) {
                if campaign.game.box_art_url.is_none() {
                    campaign.game.box_art_url = box_art.clone();
                }

                if let Some(channels) = parse_campaign_channels(record) {
                    live_channels.entry(campaign.id.clone()).or_insert(channels);
                }

                campaigns.push(campaign);
            }
        }

        if campaigns.len() > MAX_CAMPAIGNS {
            return Err(too_large());
        }

        note_left_out("catalogue campaigns", total, campaigns.len() - before);
    }

    // ! The catalogue can list a campaign more than once: the interface keys its
    // lists by id.
    let mut seen: HashSet<String> = HashSet::new();

    campaigns.retain(|campaign| seen.insert(campaign.id.clone()));
    campaigns.sort_by(|a, b| a.ends_at.cmp(&b.ends_at).then_with(|| a.id.cmp(&b.id)));

    let now = Utc::now();

    Ok(Catalog {
        campaigns,
        read_at: Some(now),
        updated_at: payload["refreshedAt"]
            .as_str()
            .and_then(|v| v.parse::<DateTime<Utc>>().ok()),
        fetched_at: Some(now),
        problem: None,
        live_channels,
    })
}

// ! Fills in the id and image a legacy name-only entry (or one added before
// its category ever ran a campaign) is still missing, and refreshes a
// stored image once the catalogue's own differs: never a new request, only
// what the catalogue already holds from its regular refresh. A name-only
// entry that turns out to be a category already listed by id is dropped,
// the first one kept, so a list never holds the same category twice.
pub fn complete_game_entries(entries: &mut Vec<GameEntry>, campaigns: &[Campaign]) -> bool {
    let mut changed = false;

    for entry in entries.iter_mut() {
        let found = match entry.id.as_deref().filter(|id| !id.is_empty()) {
            Some(id) => campaigns.iter().find(|c| c.game.id == id),
            None => campaigns
                .iter()
                .find(|c| c.game.name.eq_ignore_ascii_case(entry.name.trim())),
        };

        let Some(campaign) = found else {
            continue;
        };

        if entry.id.is_none() {
            entry.id = Some(campaign.game.id.clone());
            changed = true;
        }

        if campaign.game.box_art_url.is_some() && entry.box_art_url != campaign.game.box_art_url {
            entry.box_art_url = campaign.game.box_art_url.clone();
            changed = true;
        }
    }

    let before = entries.len();
    let mut seen = HashSet::new();

    entries.retain(|entry| match entry.id.as_deref().filter(|id| !id.is_empty()) {
        Some(id) => seen.insert(id.to_owned()),
        None => true,
    });

    changed || entries.len() != before
}
