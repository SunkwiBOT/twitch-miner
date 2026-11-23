pub mod api;
pub mod badges;
pub mod catalog;
pub mod config;
pub mod domain;
pub(crate) mod drops_api;
pub mod error;
pub mod events;
pub mod feed;
pub mod http;
pub mod i18n;
pub mod links;
pub mod logs;
pub mod miner;
pub mod notifications;
pub mod predictions;
pub mod requests;
pub mod scheduler;
pub mod spam;
pub mod store;
pub mod twitch;
pub mod updates;
pub mod util;

pub use error::{Error, Result};
pub use miner::manager::Manager;

pub const APP_NAME: &str = "Twitch Miner";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const REPOSITORY_URL: &str = env!("CARGO_PKG_REPOSITORY");

// ! The version `main` always declares: `.github/bump-version.py` writes the
// real one into the release tag only.
pub const UNRELEASED_VERSION: &str = "0.1.0";

// ! What the app tells the services it calls besides Twitch (to Twitch it
// stays the TV client): the kind of install, the system and the processor.
pub fn user_agent(install_kind: api::InstallKind) -> String {
    let kind = match install_kind {
        api::InstallKind::Desktop => "desktop",
        api::InstallKind::Docker => "docker",
        api::InstallKind::Binary => "server",
    };

    format!(
        "TwitchMiner/{VERSION} ({kind}; {}; {}; +{REPOSITORY_URL})",
        std::env::consts::OS,
        std::env::consts::ARCH
    )
}
