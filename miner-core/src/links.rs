use crate::drops_api;

// ! Every host the app's own links may target, besides a campaign or badge's
// own publisher site (resolved from the catalogue, never from this list).
pub fn allowed_hosts() -> Vec<String> {
    let mut hosts: Vec<String> = [
        "github.com",
        "discord.gg",
        "twitch.tv",
        "www.twitch.tv",
        "twitch.sunkwi.com",
        "kick.sunkwi.com",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();

    if let Some(host) = drops_api::base_host() {
        hosts.push(host);
    }

    hosts
}
