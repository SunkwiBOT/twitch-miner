use chrono::{DateTime, Utc};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::domain::{Channel, Game};
use crate::twitch::note_left_out;

const BASE_URL: &str = "https://dropsalerts.com";
const CATALOG_PATH: &str = "api/twitch/drops";
const BADGES_PATH: &str = "api/twitch/badges";

// ! The anonymous limit of `/drops`: one call every 5 minutes.
pub const CATALOG_REFRESH_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5 * 60);

// ! Above each list's own refresh interval, so a list read just before the
// next fetch is still trusted for almost one more cycle.
const LIVE_CHANNELS_FRESH_MARGIN: std::time::Duration = std::time::Duration::from_secs(60);

pub const LIVE_CHANNELS_MAX_AGE: chrono::Duration = chrono::Duration::seconds(
    (CATALOG_REFRESH_INTERVAL.as_secs() + LIVE_CHANNELS_FRESH_MARGIN.as_secs()) as i64,
);

pub const BADGE_LIVE_CHANNELS_MAX_AGE: chrono::Duration = chrono::Duration::seconds(
    (crate::feed::REFRESH_INTERVAL.as_secs() + LIVE_CHANNELS_FRESH_MARGIN.as_secs()) as i64,
);

pub(crate) fn catalog_url() -> String {
    format!("{BASE_URL}/{CATALOG_PATH}")
}

pub(crate) fn badges_url() -> String {
    format!("{BASE_URL}/{BADGES_PATH}")
}

// ! The dashboard opens this address in a button with no variable in its
// label: an absolute https URL on the drops-web host, or dropped, so a
// crafted response can never point the button elsewhere.
pub(crate) fn base_host() -> Option<String> {
    url::Url::parse(BASE_URL)
        .ok()?
        .host_str()
        .map(str::to_owned)
}

pub(crate) fn valid_web_url(url: Option<String>) -> Option<String> {
    let host = base_host()?;
    let url = url?;
    let parsed = url::Url::parse(&url).ok()?;

    (parsed.scheme() == "https" && parsed.host_str() == Some(host.as_str())).then_some(url)
}

#[derive(Debug, Clone)]
pub(crate) struct CampaignChannels {
    pub(crate) read_at: DateTime<Utc>,
    pub(crate) channels: Vec<Channel>,
}

fn parse_live_channel(record: &Value) -> Option<Channel> {
    let login = record["login"].as_str()?.to_owned();
    let display_name = record["displayName"].as_str().unwrap_or(&login).to_owned();

    Some(Channel {
        id: record["id"].as_str()?.to_owned(),
        login,
        display_name,
        avatar_url: record["profileImageUrl"].as_str().map(str::to_owned),
        live: true,
        broadcast_id: record["streamId"].as_str().map(str::to_owned),
        viewers: record["viewers"].as_u64(),
        game: Some(Game {
            id: record["gameId"].as_str()?.to_owned(),
            name: record["gameName"].as_str()?.to_owned(),
            slug: None,
            box_art_url: None,
        }),
        drops_enabled: record["dropsEnabled"].as_bool().unwrap_or(false),
    })
}

// ! `liveChannelsReadAt` is `null` when the campaign was never read: a campaign
// with no entry here falls back to the miner's own lookup, same as a stale one.
pub(crate) fn parse_campaign_channels(record: &Value) -> Option<CampaignChannels> {
    let read_at = record["liveChannelsReadAt"]
        .as_str()
        .and_then(|v| v.parse::<DateTime<Utc>>().ok())?;
    let entries = record["liveChannels"].as_array();
    let total = entries.map_or(0, Vec::len);
    let channels: Vec<Channel> = entries
        .into_iter()
        .flatten()
        .filter_map(parse_live_channel)
        .collect();

    note_left_out("catalogue live channels", total, channels.len());

    Some(CampaignChannels { read_at, channels })
}

pub(crate) fn shuffle_key(install_id: &str, channel_id: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(install_id.as_bytes());
    hasher.update(b":");
    hasher.update(channel_id.as_bytes());

    hasher.finalize().into()
}
