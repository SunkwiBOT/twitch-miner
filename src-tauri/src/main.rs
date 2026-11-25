#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod autostart;
mod commands;
mod env;
mod events;
mod lifecycle;
mod logging;
mod open_url;
mod tray;
mod updates;

use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use miner_core::api::{
    AccountStatus, AppInfo, AppInfoResponse, AutostartResponse, BadgeCatalogView, BadgesResponse,
    CatalogResponse, CatalogView, ChannelBannersResponse, ChannelResponse, ChannelsLookupResponse,
    CheckPastedTokenResponse, Command, CommandName, DesktopEvent, FollowsResponse,
    HistoryCurveQuery, HistoryCurveResponse, HistoryExportQuery, HistoryExportResponse,
    HistoryQuery, HistoryResponse, HistoryStatsQuery, HistoryStatsResponse, ImportTokensResponse,
    InstallKind, LoginFlowResponse, LogsQuery, LogsReportResponse, LogsResponse, NotificationTest,
    NotificationTestRequest, ProxyTestResponse, RedeemRequest, RedeemResponse, RefreshResponse,
    RequestsQuery, RequestsResponse, SearchResponse, Snapshot, SnapshotResponse,
    SpamBalanceResponse, SpamChannelResponse, StartupProblem, StartupProblemResponse,
};
use miner_core::config::{AccountSettingsPatch, CloseAction, GlobalSettings};
use miner_core::error::{ApiError, ErrorCode};
use miner_core::i18n;
use miner_core::notifications::desktop::DesktopNotifier;
use miner_core::release_notes::ReleaseNotes;
use miner_core::store::VaultStatus;
use miner_core::Manager;
use tauri::{AppHandle, Emitter, Manager as _, RunEvent, WindowEvent};

use crate::env::DesktopEnv;
use crate::tray::TrayView;

use autostart::*;
use commands::*;
use events::*;
use logging::*;

pub struct AppState {
    pub manager: Arc<Manager>,
    pub stopped: AtomicBool,
}

pub struct DataFolder {
    pub path: PathBuf,
    pub problem: Option<miner_core::Error>,
}

pub struct WindowShown(pub tokio::sync::watch::Sender<bool>);

pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
        window_shown(app, true);
    }
}

pub fn window_shown(app: &AppHandle, shown: bool) {
    if let Some(state) = app.try_state::<WindowShown>() {
        state.0.send_replace(shown);
    }
}

fn configure_window(window: &tauri::WebviewWindow) {
    if let Ok(icon) = tauri::image::Image::from_bytes(include_bytes!("../icons/128x128.png")) {
        let _ = window.set_icon(icon);
    }

    #[cfg(target_os = "linux")]
    gtk::Window::set_default_icon_name("twitchminer");
}

#[cfg(target_os = "linux")]
fn prepare_linux_display() {
    // ! The GTK program name becomes the Wayland app id; KWin looks up
    // "<app id>.desktop" for the icon, so it must match the product name.
    glib::set_prgname(Some("Twitch Miner"));
    glib::set_application_name("Twitch Miner");

    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
        // ! Avoids blank windows on NVIDIA drivers.
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
}

#[cfg(not(target_os = "linux"))]
fn prepare_linux_display() {}

fn main() {
    // ! See `updates::SELFTEST_ARG`: exits before any window, data folder, or log file is touched.
    if std::env::args().any(|arg| arg == updates::SELFTEST_ARG) {
        std::process::exit(0);
    }

    updates::cleanup_stale_backup();

    prepare_linux_display();

    let env = match DesktopEnv::from_env() {
        Ok(env) => env,
        Err(error) => {
            eprintln!("invalid environment: {error:#}");
            std::process::exit(2);
        }
    };

    let dir = env.data_dir.clone();
    let _ = miner_core::util::private_dir(&dir);
    let log_buffer = miner_core::logs::LogBuffer::new();
    let mut file_writer = Some(miner_core::logs::keep_file_writer(init_logging(
        &dir,
        log_buffer.clone(),
    )));

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            show_main_window(app)
        }))
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(updates::PendingUpdate::default())
        .manage(updates::FoundUpdate::default())
        .invoke_handler(tauri::generate_handler![
            snapshot,
            command,
            catalog,
            start_login,
            check_pasted_token,
            import_tokens,
            lookup_channel,
            lookup_channels,
            search,
            channel_banners,
            follows,
            spam_channel,
            spam_balance,
            redeem,
            test_notification,
            test_proxy,
            history,
            history_export,
            history_stats,
            history_curve,
            badges,
            refresh_badges,
            logs,
            logs_report,
            requests,
            requests_report,
            close_window,
            app_info,
            startup_problem,
            quit,
            open_logs_directory,
            open_data_directory,
            export_csv_file,
            open_url::open_url,
            open_url::open_campaign_link,
            open_url::open_badge_link,
            autostart_status,
            set_autostart,
            updates::check_for_update,
            updates::found_update,
            updates::take_failed_update,
            updates::download_update,
            updates::apply_update
        ])
        .setup(move |app| {
            let autostart = refresh_autostart(app.handle());

            tracing::info!(
                version = miner_core::VERSION,
                exe = %std::env::current_exe().map(|path| path.display().to_string()).unwrap_or_default(),
                autostart,
                portable = updates::portable(),
                "the app started"
            );

            let opened = Manager::open(&dir, log_buffer.clone(), InstallKind::Desktop);

            app.manage(DataFolder {
                path: dir.clone(),
                problem: opened.as_ref().err().cloned(),
            });

            // ! An error out of `setup` would end the app before any window
            // opens (nothing at all on Windows): the window says why instead.
            let manager = match opened {
                Ok(manager) => manager,
                Err(error) => {
                    tracing::error!("the core could not start: {error}");

                    if let Some(window) = app.get_webview_window("main") {
                        configure_window(&window);
                        let _ = window.show();
                    }

                    return Ok(());
                }
            };

            app.manage(AppState {
                manager: manager.clone(),
                stopped: AtomicBool::new(false),
            });
            // ! `Manager::open` just built its HTTP clients the same way: this one cannot fail where those did not.
            app.manage(ReleaseNotes::new(InstallKind::Desktop)?);
            app.manage(WindowShown(tokio::sync::watch::channel(false).0));

            // ! The window is created hidden (tauri.conf.json) so it never flashes on screen
            // first. A locked app password still shows it: nothing mines before the
            // password is typed, so hiding would only leave the app idle.
            let (settings, unlocked) = tauri::async_runtime::block_on(async {
                (
                    manager.global_settings().await,
                    manager.vault_status().await.unlocked,
                )
            });
            let hidden = unlocked && settings.start_minimized;
            let handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let event_receiver = manager.subscribe();

                manager.start().await;

                let desktop = open_desktop_notifier(&handle, &manager).await;

                tauri::async_runtime::spawn(updates::check_regularly(
                    handle.clone(),
                    manager.clone(),
                    desktop.clone(),
                ));
                forward_events(handle, manager, event_receiver, desktop).await;
            });

            tray::install(app.handle(), i18n::resolve(&settings.language))?;

            if let Some(window) = app.get_webview_window("main") {
                configure_window(&window);

                if !hidden {
                    let _ = window.show();
                    window_shown(app.handle(), true);
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let app = window.app_handle();

                let Some(state) = app.try_state::<AppState>() else {
                    return;
                };

                let manager = state.manager.clone();
                let action = tauri::async_runtime::block_on(async {
                    manager.global_settings().await.close_action
                });

                match action {
                    CloseAction::Tray => {
                        api.prevent_close();
                        let _ = window.hide();
                        window_shown(app, false);
                    }
                    CloseAction::Quit => {
                        api.prevent_close();
                        tauri::async_runtime::spawn(lifecycle::quit(app.clone()));
                    }
                    CloseAction::Ask => {
                        api.prevent_close();
                        let _ = window.emit(DesktopEvent::CloseRequested.name(), ());
                    }
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building the application")
        .run(move |app, event| match event {
            // ! Only an exit the app did not start itself: its own exits carry a code and
            // have already been through `quit`.
            RunEvent::ExitRequested {
                code: None, api, ..
            } => {
                api.prevent_exit();
                tauri::async_runtime::spawn(lifecycle::quit(app.clone()));
            }
            // ! `run` never returns: the log file's last lines are written here.
            RunEvent::Exit => drop(file_writer.take()),
            #[cfg(target_os = "macos")]
            RunEvent::Reopen {
                has_visible_windows: false,
                ..
            } => show_main_window(app),
            _ => {}
        });
}
