const BASE_URL: &str = "https://drops.sunkwi.com";

const CATALOG_PATH: &str = "api/twitch/drops";
const BADGES_PATH: &str = "api/twitch/badges";

pub(crate) fn catalog_url() -> String {
    format!("{BASE_URL}/{CATALOG_PATH}")
}

pub(crate) fn badges_url() -> String {
    format!("{BASE_URL}/{BADGES_PATH}")
}
