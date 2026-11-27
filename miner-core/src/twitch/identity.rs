// ! Twitch client ids and user agents other open miners use for the same
// personas: the TV/"SMARTBOX" one we always used, plus the web, mobile-web
// and Android-app clients a bulk-imported token may have been issued for.
// Picking a user agent once per account from a persona's pool, where the
// real devices of that client differ, keeps a fleet of accounts on one
// install from all looking like the exact same device.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub const TV_CLIENT_ID: &str = "ue6666qo983tsx6so1t0vnawi233wa";
pub const WEB_CLIENT_ID: &str = "kimne78kx3ncx6brgo4mv6wki5h1ko";
pub const MOBILE_WEB_CLIENT_ID: &str = "r8s4dac0uhzifbpu9sjdiwzctle17ff";
pub const ANDROID_APP_CLIENT_ID: &str = "kd1unb4b3q4t58fwlpcbzcbnm76a8fp";

// ! Chrome 110 and later on Android sends a frozen "Android 10; K" whatever
// the device and system, without "Mobile" on a tablet or TV: a single string
// is what every such device presents.
const TV_USER_AGENTS: [&str; 1] = [
    "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36",
];

// ! Shared with `TwitchHttp::new`'s client-level default: any request made
// through `socket_client` directly, without going through `request()`'s
// per-identity header override, still needs some user agent set.
pub const DEFAULT_USER_AGENT: &str = TV_USER_AGENTS[0];

// ! Desktop Chrome freezes its platform to one of these three values.
const WEB_USER_AGENTS: [&str; 3] = [
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Safari/537.36",
];

const MOBILE_WEB_USER_AGENTS: [&str; 1] = [
    "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/158.0.0.0 Mobile Safari/537.36",
];

// ! Each Build id is one of that Android release (U: 14, AP3A: 15, BP2A: 16).
const ANDROID_APP_USER_AGENTS: [&str; 4] = [
    "Dalvik/2.1.0 (Linux; U; Android 16; SM-S938B Build/BP2A.250605.031) tv.twitch.android.app/25.3.0/2503006",
    "Dalvik/2.1.0 (Linux; U; Android 15; SM-G990B Build/AP3A.240905.015.A2) tv.twitch.android.app/25.3.0/2503006",
    "Dalvik/2.1.0 (Linux; U; Android 15; SM-A566E Build/AP3A.240905.015.A2) tv.twitch.android.app/25.3.0/2503006",
    "Dalvik/2.1.0 (Linux; U; Android 14; SM-X306B Build/UP1A.231005.007) tv.twitch.android.app/25.3.0/2503006",
];

// ! A known Twitch client persona: which app Twitch believes is talking to
// it. `Other` covers a client id a bulk-imported token carries that none
// of the known personas use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(rename = "ClientPersonaType", repr(enum = name))]
pub enum ClientPersona {
    Tv,
    Web,
    MobileWeb,
    AndroidApp,
    Other,
}

impl ClientPersona {
    pub fn from_client_id(client_id: &str) -> Self {
        match client_id {
            TV_CLIENT_ID => Self::Tv,
            WEB_CLIENT_ID => Self::Web,
            MOBILE_WEB_CLIENT_ID => Self::MobileWeb,
            ANDROID_APP_CLIENT_ID => Self::AndroidApp,
            _ => Self::Other,
        }
    }

    fn user_agents(self) -> &'static [&'static str] {
        match self {
            Self::Tv | Self::Other => &TV_USER_AGENTS,
            Self::Web => &WEB_USER_AGENTS,
            Self::MobileWeb => &MOBILE_WEB_USER_AGENTS,
            Self::AndroidApp => &ANDROID_APP_USER_AGENTS,
        }
    }

    fn pick_user_agent(self) -> String {
        let pool = self.user_agents();
        let index = (rand::random::<u32>() as usize) % pool.len();

        pool[index].to_owned()
    }

    // ! Twitch sends `Origin`/`Referer` for a browser-based client; the
    // native Android app is not one and sends neither. An unrecognised
    // client id (`Other`) is treated like the TV one, same as its user agent pool.
    pub fn origin(self) -> Option<&'static str> {
        match self {
            Self::Tv | Self::Other => Some(TV_ORIGIN),
            Self::Web => Some(WEB_ORIGIN),
            Self::MobileWeb => Some(MOBILE_WEB_ORIGIN),
            Self::AndroidApp => None,
        }
    }
}

pub const TV_ORIGIN: &str = "https://android.tv.twitch.tv";
const WEB_ORIGIN: &str = "https://www.twitch.tv";
const MOBILE_WEB_ORIGIN: &str = "https://m.twitch.tv";

// ! The client identity a stored account's Twitch requests present
// themselves as: its client id and a user agent picked once from that
// client's pool, sealed inside the account's `Credential` so the two can
// never drift apart. The account's existing `X-Device-Id` is unrelated
// and kept as is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct AccountIdentity {
    pub client_id: String,
    pub user_agent: String,
}

impl Default for AccountIdentity {
    // ! A `Credential` sealed before this field existed has none at all:
    // `#[serde(default)]` falls back to this, so every such account keeps
    // mining as the same TV client it always used, picking a single fixed
    // user agent rather than every such account landing on a different
    // pool pick at each load.
    fn default() -> Self {
        Self {
            client_id: TV_CLIENT_ID.to_owned(),
            user_agent: DEFAULT_USER_AGENT.to_owned(),
        }
    }
}

impl AccountIdentity {
    // ! A fresh identity for a new TV device-code login: same client every
    // TV account uses, a user agent picked from the TV pool so a fleet of
    // accounts does not all look like the exact same TV box.
    pub fn new_tv() -> Self {
        Self {
            client_id: TV_CLIENT_ID.to_owned(),
            user_agent: ClientPersona::Tv.pick_user_agent(),
        }
    }

    // ! Built from a bulk-imported token's own validated `client_id`: a
    // known persona gets a matching user agent picked from its pool: an
    // unrecognised client id keeps that id, paired with the TV user agent
    // since nothing is known about what device issued it.
    pub fn for_client(client_id: &str) -> Self {
        let persona = ClientPersona::from_client_id(client_id);

        Self {
            client_id: client_id.to_owned(),
            user_agent: persona.pick_user_agent(),
        }
    }

    // ! A user agent no longer in its persona's pool is an app version Twitch
    // stopped seeing: it is picked again, once, from the current pool.
    pub fn refresh_user_agent(&mut self) -> bool {
        if self
            .persona()
            .user_agents()
            .contains(&self.user_agent.as_str())
        {
            return false;
        }

        self.user_agent = self.persona().pick_user_agent();

        true
    }

    pub fn persona(&self) -> ClientPersona {
        ClientPersona::from_client_id(&self.client_id)
    }
}
