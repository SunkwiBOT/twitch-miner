// ! Twitch does not show the campaign list to the TV client, so campaigns
// come from this public catalogue.

use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::Shell;
use crate::domain::Campaign;
use crate::error::{Error, ErrorCode, Result};
use crate::feed::{FeedAnswer, FeedClient};
use crate::http::too_large;
use crate::requests::RequestLog;
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
}

impl FeedAnswer for Catalog {
    const NAME: &'static str = "catalogue";
    const CACHE_KEY: &'static str = "catalog_cache";

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
}

pub type CatalogClient = FeedClient<Catalog>;

impl CatalogClient {
    pub fn new(requests: RequestLog, shell: Shell) -> Result<Self> {
        Self::with_url(
            &crate::drops_api::catalog_url(),
            requests,
            &crate::user_agent(shell),
        )
    }
}

pub fn parse_catalog(payload: &Value) -> Result<Catalog> {
    let records = payload["games"]
        .as_array()
        .ok_or_else(|| Error::InvalidResponse("catalogue games must be a list".into()))?;

    let mut campaigns = Vec::new();

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
    })
}
