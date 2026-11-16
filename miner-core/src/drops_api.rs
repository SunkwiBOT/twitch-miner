const BASE_URL: &str = "https://drops.sunkwi.com";

const CATALOG_PATH: &str = "api/twitch/drops";
const BADGES_PATH: &str = "api/twitch/badges";

pub(crate) fn catalog_url() -> String {
    format!("{BASE_URL}/{CATALOG_PATH}")
}

pub(crate) fn badges_url() -> String {
    format!("{BASE_URL}/{BADGES_PATH}")
}

// ! The dashboard opens this address in a button with no variable in its
// label: an absolute https URL on the drops-web host, or dropped, so a
// crafted response can never point the button elsewhere.
pub(crate) fn valid_web_url(url: Option<String>) -> Option<String> {
    let host = url::Url::parse(BASE_URL).ok()?.host_str()?.to_owned();
    let url = url?;
    let parsed = url::Url::parse(&url).ok()?;

    (parsed.scheme() == "https" && parsed.host_str() == Some(host.as_str())).then_some(url)
}
