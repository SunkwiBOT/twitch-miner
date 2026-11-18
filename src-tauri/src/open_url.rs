// ! `tauri-plugin-opener` runs `xdg-open` without an XDG activation token, so on
// Wayland an already-running browser opens the tab but never comes to the front:
// a `gdk::AppLaunchContext` built from the clicked window requests a real one,
// with the plugin as the fallback and the only path on Windows and macOS.
//
// ! `gtk::current_event_time()` runs inside the queued main-thread task below, not inside
// a real GTK event handler, so it returns 0 ("now"), never the click's actual event time.

use miner_core::api::{AccountSnapshot, CampaignProgress};
use miner_core::badges::GlobalBadge;
use miner_core::domain::Campaign;
use miner_core::error::{ApiError, ErrorCode};
use miner_core::links;
use tauri::Url;

use crate::commands::unlocked;
use crate::AppState;

type Answer<T> = Result<T, ApiError>;

fn refused<T>() -> Answer<T> {
    Err(ApiError::new(ErrorCode::OpenUrlRefused))
}

// ! `tauri::Url` follows WHATWG, not a hand-rolled split: `https://evil.example\@github.com/`
// parses to host `evil.example`, matching a real browser, where a naive
// scheme/host split would see `github.com` past the `@`.
fn https_url(url: &str) -> Answer<Url> {
    let parsed = Url::parse(url).map_err(|_| ApiError::new(ErrorCode::OpenUrlRefused))?;

    if parsed.scheme() == "https" {
        Ok(parsed)
    } else {
        refused()
    }
}

// ! The app's own links: the host must be on the one list `miner-core`
// exports (`links::allowed_hosts`). A campaign or badge's own link goes
// through `open_campaign_link`/`open_badge_link` instead, which resolve the
// URL from the account's own campaigns first, then the catalogue, and never
// take one from the caller.
#[tauri::command(async)]
pub(super) fn open_url(window: tauri::WebviewWindow, url: String) -> Answer<()> {
    validate_scope(&url)?;
    dispatch(&window, &url)
}

fn validate_scope(url: &str) -> Answer<()> {
    let parsed = https_url(url)?;
    let host = parsed
        .host_str()
        .ok_or_else(|| ApiError::new(ErrorCode::OpenUrlRefused))?;

    if links::allowed_hosts().iter().any(|allowed| allowed == host) {
        Ok(())
    } else {
        refused()
    }
}

// ! `CampaignProgress.link_url` overrides the catalogue's for a campaign shown on the
// account (it is `None` there when the two agree); a campaign the catalogue does not
// list at all is shipped whole in `other_campaigns`.
fn campaign_link(
    progress: &[CampaignProgress],
    others: &[Campaign],
    catalog: &[Campaign],
    campaign_id: &str,
) -> Answer<String> {
    let catalogued = || catalog_link_url(catalog, campaign_id);
    let from_progress = progress
        .iter()
        .find(|entry| entry.id == campaign_id)
        .map(|entry| entry.link_url.clone().or_else(catalogued));
    let from_others = others
        .iter()
        .find(|campaign| campaign.id == campaign_id)
        .map(|campaign| campaign.link_url.clone());
    let url = from_progress
        .or(from_others)
        .unwrap_or_else(catalogued)
        .ok_or_else(|| ApiError::new(ErrorCode::OpenUrlRefused))?;

    https_url(&url)?;
    Ok(url)
}

fn catalog_link_url(catalog: &[Campaign], campaign_id: &str) -> Option<String> {
    catalog
        .iter()
        .find(|campaign| campaign.id == campaign_id)
        .and_then(|campaign| campaign.link_url.clone())
}

fn host_of(url: &str) -> Answer<String> {
    https_url(url)?
        .host_str()
        .map(str::to_owned)
        .ok_or_else(|| ApiError::new(ErrorCode::OpenUrlRefused))
}

fn badge_link(badges: &[GlobalBadge], badge_key: &str) -> Answer<String> {
    let url = badges
        .iter()
        .find(|badge| badge.key() == badge_key)
        .and_then(|badge| badge.click_url.clone())
        .ok_or_else(|| ApiError::new(ErrorCode::OpenUrlRefused))?;

    https_url(&url)?;
    Ok(url)
}

// ! "Link your game account" opens the publisher site named in the campaign's own
// `link_url`: the account id and the campaign id pick it out of that account's own
// campaigns first (override, then a campaign the catalogue does not list), then the
// catalogue, so the caller can never point it anywhere else. The opened URL's host is
// returned for the "opened on {site}" toast, since it can differ from the one on screen.
#[tauri::command]
pub(super) async fn open_campaign_link(
    window: tauri::WebviewWindow,
    state: tauri::State<'_, AppState>,
    account_id: String,
    campaign_id: String,
) -> Answer<String> {
    unlocked(&state).await?;

    let account: AccountSnapshot = state
        .manager
        .account_snapshot(&account_id)
        .await
        .ok_or_else(|| ApiError::new(ErrorCode::OpenUrlRefused))?;
    let catalog = state.manager.catalog_list().await.campaigns;
    let url = campaign_link(
        &account.progress,
        &account.other_campaigns,
        &catalog,
        &campaign_id,
    )?;

    dispatch(&window, &url)?;
    host_of(&url)
}

#[tauri::command]
pub(super) async fn open_badge_link(
    window: tauri::WebviewWindow,
    state: tauri::State<'_, AppState>,
    badge_key: String,
) -> Answer<()> {
    unlocked(&state).await?;

    let badges = state.manager.badge_list().await.badges;
    let url = badge_link(&badges, &badge_key)?;
    dispatch(&window, &url)
}

// ! GTK only works on the main thread (it panics anywhere else), and an async
// command runs on a worker: the launch is handed to the main thread and its
// result awaited here.
#[cfg(target_os = "linux")]
fn dispatch(window: &tauri::WebviewWindow, url: &str) -> Answer<()> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let main_window = window.clone();
    let target = url.to_owned();
    let queued = window.run_on_main_thread(move || {
        let _ = sender.send(open_via_gtk(&main_window, &target));
    });
    let launched = match queued {
        Ok(()) => receiver
            .recv()
            .unwrap_or_else(|_| Err("the main thread dropped the launch".to_owned())),
        Err(error) => Err(error.to_string()),
    };

    match launched {
        Ok(()) => Ok(()),
        Err(reason) => {
            tracing::warn!("opening a link through GTK failed, falling back: {reason}");
            open_via_plugin(window, url)
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn dispatch(window: &tauri::WebviewWindow, url: &str) -> Answer<()> {
    open_via_plugin(window, url)
}

fn open_via_plugin(window: &tauri::WebviewWindow, url: &str) -> Answer<()> {
    use tauri_plugin_opener::OpenerExt;

    window
        .opener()
        .open_url(url, None::<&str>)
        .map_err(|error| {
            tracing::warn!("opening a link failed: {error}");
            ApiError::new(ErrorCode::OpenUrlFailed)
        })
}

#[cfg(target_os = "linux")]
fn open_via_gtk(window: &tauri::WebviewWindow, url: &str) -> Result<(), String> {
    use gtk::prelude::WidgetExt;

    let gtk_window = window.gtk_window().map_err(|error| error.to_string())?;
    let context = gtk_window
        .display()
        .app_launch_context()
        .ok_or_else(|| "the display has no launch context".to_owned())?;
    context.set_timestamp(gtk::current_event_time());
    gio::AppInfo::launch_default_for_uri(url, Some(&context)).map_err(|error| error.to_string())
}
