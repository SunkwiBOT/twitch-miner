use super::*;

// ! The versions named "Twitch Drops & Points Miner" registered the start with
// the system under that name: a start asked for there moves to the new name once.
fn move_old_autostart(app: &AppHandle) {
    use tauri_plugin_autostart::ManagerExt;

    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let mut builder = auto_launch::AutoLaunchBuilder::new();

    builder
        .set_app_name("Twitch Drops & Points Miner")
        .set_app_path(&exe.display().to_string());

    #[cfg(target_os = "macos")]
    builder.set_macos_launch_mode(auto_launch::MacOSLaunchMode::LaunchAgent);

    let Ok(old) = builder.build() else {
        return;
    };

    if !old.is_enabled().unwrap_or(false) {
        return;
    }

    if let Err(error) = old.disable() {
        tracing::warn!("old start with the system not removed: {error}");
    }

    if let Err(error) = app.autolaunch().enable() {
        tracing::warn!("start with the system not moved to the new name: {error}");
    }
}

// ! The entry holds the executable's path from the day it was turned on, and the
// enabled check only looks for the entry: a portable copy moved or renamed since
// would never start. Windows keeps one entry per user and one for the machine
// (written when the app runs elevated): both are removed, then one is written
// on the current file, so a stale copy is never started beside it.
pub(super) fn refresh_autostart(app: &AppHandle) -> bool {
    use tauri_plugin_autostart::ManagerExt;

    move_old_autostart(app);

    let launcher = app.autolaunch();

    if !launcher.is_enabled().unwrap_or(false) {
        return false;
    }

    if let Err(error) = launcher.disable().and_then(|()| launcher.enable()) {
        tracing::warn!("start with the system not refreshed: {error}");
    }

    true
}

#[tauri::command]
pub(super) fn autostart_status(app: AppHandle) -> AutostartResponse {
    use tauri_plugin_autostart::ManagerExt;

    AutostartResponse {
        enabled: app.autolaunch().is_enabled().unwrap_or(false),
    }
}

#[tauri::command]
pub(super) async fn set_autostart(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    enabled: bool,
) -> Answer<()> {
    use tauri_plugin_autostart::ManagerExt;

    unlocked(&state).await?;

    let launcher = app.autolaunch();
    let result = if enabled {
        launcher.enable()
    } else {
        launcher.disable()
    };

    result.map_err(|error| {
        tracing::warn!("autostart change failed: {error}");
        ApiError::new(ErrorCode::StorageError)
    })
}
