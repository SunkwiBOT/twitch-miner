// ! The update a check found is kept: it is the one downloaded and installed, so
// only the version the user was shown and confirmed is ever installed.

use base64::Engine;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use miner_core::error::{ApiError, ErrorCode};
use miner_core::requests::{address_target, url_target, Pending, RequestKind};
use miner_core::Manager;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager as _, State};
use tauri_plugin_updater::{Update, Updater, UpdaterExt};

pub const UPDATE_PROGRESS_EVENT: &str = "UPDATE_PROGRESS";
pub const UPDATE_AVAILABLE_EVENT: &str = "UPDATE_AVAILABLE";

const PROGRESS_STEP_BYTES: u64 = 1024 * 1024;
const CHECK_TIMEOUT: Duration = Duration::from_secs(30);
const STALL_TIMEOUT: Duration = Duration::from_secs(30);

// ! Matches the extra platform key the release workflow adds to `latest.json` for the
// portable Windows copy, signed the same way as every other published asset.
const PORTABLE_TARGET: &str = "windows-x86_64-portable";

// ! The NSIS installer leaves its uninstaller beside the executable; a portable copy has none.
const PORTABLE_MARKER: &str = "uninstall.exe";

// ! The flag a freshly swapped-in exe is launched with to prove it can start at all, without
// opening a window, taking the single-instance lock, or touching the vault: it exits
// immediately, before any of that runs, so the check never collides with the instance still
// running under the old (now `.bak`) executable.
pub const SELFTEST_ARG: &str = "--update-selftest";
// ! A fresh exe may still be scanned by antivirus software on its first launch; the self-test
// deadline allows for that instead of only the time a plain process start takes.
const SELFTEST_TIMEOUT: Duration = Duration::from_secs(15);

type Answer<T> = Result<T, ApiError>;

#[derive(Clone, Serialize)]
pub struct UpdateInfoDto {
    pub version: String,
    pub current_version: String,
    pub notes: Option<String>,
}

impl UpdateInfoDto {
    fn of(update: &Update) -> Self {
        Self {
            version: update.version.clone(),
            current_version: update.current_version.clone(),
            notes: update.body.clone(),
        }
    }
}

#[derive(Default)]
pub struct FoundUpdate(Mutex<Option<Found>>);

struct Found {
    update: Update,
    first: bool,
}

#[derive(Clone, Serialize)]
pub struct FailedUpdateDto {
    pub version: String,
}

#[derive(Clone, Serialize)]
pub struct UpdateProgressDto {
    pub downloaded: u64,
    pub total: Option<u64>,
}

#[derive(Default)]
pub struct PendingUpdate(Mutex<Option<DownloadedUpdate>>);

struct DownloadedUpdate {
    update: Update,
    bytes: Vec<u8>,
}

// ! Installing stops the accounts and restarts the app: only someone who typed
// the app password may ask, and only once the core runs.
pub(crate) async fn may_update(state: Option<&crate::AppState>) -> Answer<()> {
    let state = state.ok_or_else(|| ApiError::new(ErrorCode::NotSupported))?;

    crate::unlocked(state).await
}

async fn unlocked(app: &AppHandle) -> Answer<()> {
    may_update(app.try_state::<crate::AppState>().as_deref()).await
}

fn installed_beside(dir: &Path) -> bool {
    dir.join(PORTABLE_MARKER).exists()
}

pub fn portable() -> bool {
    cfg!(windows)
        && std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|dir| !installed_beside(dir)))
            .unwrap_or(false)
}

// ! A good portable update leaves the previous exe at `<name>.bak`: Windows would not let the
// install that just ran delete it (still mapped as the running process's own image), so the
// removal waits for the next start, once the new executable itself is the one running.
pub fn cleanup_stale_backup() {
    let Ok(exe_path) = std::env::current_exe() else {
        tracing::warn!("could not resolve the current executable to clean up its update backup");
        return;
    };
    let backup_path = sibling(&exe_path, "bak");
    if backup_path.exists() {
        if let Err(error) = std::fs::remove_file(&backup_path) {
            tracing::warn!("the previous executable backup could not be removed: {error}");
        }
    }
}

fn updater(app: &AppHandle) -> Answer<Updater> {
    let handle = app.clone();

    let mut builder = app
        .updater_builder()
        .timeout(CHECK_TIMEOUT)
        .configure_client(|client| client.read_timeout(STALL_TIMEOUT))
        // ! Windows only: the plugin starts the installer, then ends the process
        // itself, past `RunEvent::Exit`. The accounts stop here first, like on every other way out.
        .on_before_exit(move || {
            tauri::async_runtime::block_on(crate::lifecycle::stop_miner(&handle));
            handle.cleanup_before_exit();
            miner_core::logs::close_file_writer();
        });
    if portable() {
        builder = builder.target(PORTABLE_TARGET);
    }
    builder.build().map_err(|error| {
        tracing::warn!("the updater is unavailable: {error}");
        ApiError::new(ErrorCode::NotSupported)
    })
}

fn recorded(app: &AppHandle, kind: RequestKind, target: String) -> Option<Pending> {
    app.try_state::<crate::AppState>()
        .map(|state| state.manager.requests.start(kind, target))
}

fn finished<T>(request: Option<Pending>, outcome: &Result<T, tauri_plugin_updater::Error>) {
    match (request, outcome) {
        (Some(pending), Ok(_)) => pending.succeeded(None),
        (Some(pending), Err(error)) => {
            pending.failed(&miner_core::Error::Network(error.to_string()));
        }
        (None, _) => {}
    }
}

// ! A missing `windows-x86_64-portable` key in `latest.json` means this release predates the
// portable self-update path (or its asset upload failed): surfaced as unsupported instead of a
// generic network failure.
fn portable_update_error(error: &tauri_plugin_updater::Error) -> ErrorCode {
    if portable() && matches!(error, tauri_plugin_updater::Error::TargetNotFound(_)) {
        return ErrorCode::NotSupported;
    }
    ErrorCode::NetworkError
}

async fn find_update(app: &AppHandle) -> Answer<Option<Update>> {
    let updater = updater(app)?;
    let request = recorded(
        app,
        RequestKind::UpdateCheck,
        address_target(&miner_core::updates::latest_release_url()),
    );
    let found = updater.check().await;

    finished(request, &found);

    found.map_err(|error| {
        tracing::info!("the update check failed: {error}");
        ApiError::new(portable_update_error(&error))
    })
}

fn record(app: &AppHandle, update: Option<Update>) -> Option<UpdateInfoDto> {
    let manager = app
        .try_state::<crate::AppState>()
        .map(|state| state.manager.clone());

    if let Some(manager) = &manager {
        manager.update_checked();
    }

    let found = app.state::<FoundUpdate>();
    let mut found = miner_core::util::locked(&found.0);

    let Some(update) = update else {
        *found = None;

        if let Some(manager) = &manager {
            manager.clear_update();
        }

        return None;
    };

    let announced = manager.is_some_and(|manager| {
        manager
            .announce_update(&update.version)
            .inspect_err(|error| tracing::warn!(%error, "the new version was not announced"))
            .unwrap_or(false)
    });
    let known_first = found
        .as_ref()
        .is_some_and(|known| known.first && known.update.version == update.version);
    let info = UpdateInfoDto::of(&update);

    *found = Some(Found {
        update,
        first: announced || known_first,
    });

    if !announced {
        return None;
    }

    tracing::info!(
        "version {} is available (installed: {})",
        info.version,
        info.current_version
    );

    Some(info)
}

#[tauri::command]
pub async fn check_for_update(app: AppHandle) -> Answer<Option<UpdateInfoDto>> {
    unlocked(&app).await?;

    let update = find_update(&app).await?;
    let info = update.as_ref().map(UpdateInfoDto::of);

    record(&app, update);

    Ok(info)
}

// ! The window asks when it opens: the check at start may end before it listens
// to `UPDATE_AVAILABLE_EVENT`. Allowed while locked: the lock screen only says a
// version exists, with nothing to install.
#[tauri::command]
pub fn found_update(found: State<'_, FoundUpdate>) -> Option<UpdateInfoDto> {
    miner_core::util::locked(&found.0)
        .as_ref()
        .filter(|found| found.first)
        .map(|found| UpdateInfoDto::of(&found.update))
}

#[tauri::command]
pub fn take_failed_update(app: AppHandle) -> Option<FailedUpdateDto> {
    let state = app.try_state::<crate::AppState>()?;

    state
        .manager
        .take_failed_update()
        .inspect_err(|error| tracing::warn!(%error, "the failed update was not read"))
        .ok()
        .flatten()
        .map(|version| FailedUpdateDto { version })
}

async fn check_in_background(app: &AppHandle) -> bool {
    let Ok(update) = find_update(app).await else {
        return false;
    };

    if let Some(info) = record(app, update) {
        let _ = app.emit(UPDATE_AVAILABLE_EVENT, info);
    }

    true
}

pub async fn check_regularly(app: AppHandle, manager: Arc<Manager>) {
    miner_core::updates::on_schedule(&manager, || check_in_background(&app)).await;
}

fn signature_refused(error: &tauri_plugin_updater::Error) -> bool {
    use tauri_plugin_updater::Error;

    matches!(
        error,
        Error::Minisign(_)
            | Error::Base64(_)
            | Error::SignatureUtf8(_)
            | Error::SignedVersionMismatch { .. }
            | Error::MissingSignedVersion
    )
}

#[tauri::command]
pub async fn download_update(
    app: AppHandle,
    found: State<'_, FoundUpdate>,
    pending: State<'_, PendingUpdate>,
) -> Answer<String> {
    unlocked(&app).await?;

    let update = miner_core::util::locked(&found.0)
        .as_ref()
        .map(|found| found.update.clone())
        .ok_or_else(|| ApiError::new(ErrorCode::NotFound))?;
    let version = update.version.clone();

    tracing::info!("downloading version {version}");

    let emitter = app.clone();
    let mut downloaded: u64 = 0;
    let mut pushed: Option<u64> = None;
    let request = recorded(
        &app,
        RequestKind::UpdateDownload,
        url_target(&update.download_url),
    );
    let bytes = update
        .download(
            move |chunk, total| {
                downloaded += chunk as u64;

                let step = match total {
                    Some(total) if total > 0 => downloaded * 100 / total,
                    _ => downloaded / PROGRESS_STEP_BYTES,
                };

                if pushed != Some(step) {
                    pushed = Some(step);
                    let _ = emitter.emit(
                        UPDATE_PROGRESS_EVENT,
                        UpdateProgressDto { downloaded, total },
                    );
                }
            },
            || {},
        )
        .await;

    finished(request, &bytes);

    let bytes = bytes.map_err(|error| {
        if signature_refused(&error) {
            tracing::error!(
                "the download of version {version} does not match its signature: {error}"
            );

            return ApiError::new(ErrorCode::UpdateSignatureRefused);
        }

        tracing::warn!("the update download failed: {error}");
        ApiError::new(ErrorCode::NetworkError)
    })?;

    *miner_core::util::locked(&pending.0) = Some(DownloadedUpdate { update, bytes });

    Ok(version)
}

#[tauri::command]
pub async fn apply_update(
    app: AppHandle,
    pending: State<'_, PendingUpdate>,
    version: String,
) -> Answer<()> {
    unlocked(&app).await?;

    let downloaded = {
        let mut slot = miner_core::util::locked(&pending.0);

        match slot.as_ref() {
            Some(candidate) if candidate.update.version == version => slot.take(),
            _ => None,
        }
    }
    .ok_or_else(|| ApiError::new(ErrorCode::NotFound))?;

    tracing::info!("installing version {version}");

    let is_portable = portable();
    let app_for_install = app.clone();

    // ! Off the async workers: the Windows hook above blocks on stopping the
    // accounts, which an async worker thread is not allowed to do; the portable
    // swap and its self-test launch a child process and wait on it the same way.
    let installed = if is_portable {
        match tauri::async_runtime::spawn_blocking(move || {
            let exe_path = std::env::current_exe().map_err(|error| {
                tracing::warn!("the current executable could not be resolved: {error}");
                ErrorCode::UpdateInstallFailed
            })?;
            install_portable_update(
                &app_for_install,
                &exe_path,
                &downloaded.bytes,
                &downloaded.update.signature,
            )
        })
        .await
        {
            Ok(result) => result,
            Err(error) => {
                tracing::warn!("the update could not be installed: {error}");
                Err(ErrorCode::UpdateInstallFailed)
            }
        }
    } else {
        match tauri::async_runtime::spawn_blocking(move || {
            downloaded.update.install(&downloaded.bytes)
        })
        .await
        {
            Ok(result) => result.map_err(|error| {
                if signature_refused(&error) {
                    tracing::error!("version {version} does not match its signature: {error}");

                    return ErrorCode::UpdateSignatureRefused;
                }

                tracing::warn!("the update could not be installed: {error}");
                ErrorCode::UpdateInstallFailed
            }),
            Err(error) => {
                tracing::warn!("the update could not be installed: {error}");
                Err(ErrorCode::UpdateInstallFailed)
            }
        }
    };

    if let Err(code) = installed {
        // ! The Windows hook may have stopped the accounts before the installer
        // failed to start; restarting on the current version brings them back.
        if crate::lifecycle::miner_stopped(&app) {
            if let Some(state) = app.try_state::<crate::AppState>() {
                if let Err(error) = state.manager.note_failed_update(&version) {
                    tracing::warn!(%error, "the failed update was not noted");
                }
            }

            app.restart();
        }

        return Err(ApiError::new(code));
    }

    // ! Elsewhere the new files are in place and this process still runs: the
    // accounts stop now, then the new version starts.
    crate::lifecycle::stop_miner(&app).await;
    tracing::info!("restarting on version {version}");
    app.restart();
}

fn updater_pubkey(app: &AppHandle) -> Option<String> {
    app.config()
        .plugins
        .0
        .get("updater")?
        .get("pubkey")?
        .as_str()
        .map(str::to_owned)
}

fn decode_minisign_text(value: &str) -> Option<String> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(value)
        .ok()?;
    String::from_utf8(decoded).ok()
}

// ! Re-checks the already-downloaded bytes against the app's own updater public key right
// before they replace the running executable. `Update::download` already verified the same
// signature once; this is the last gate before an action this process cannot undo on itself.
fn verify_minisign(data: &[u8], signature_b64: &str, pubkey_b64: &str) -> bool {
    let Some(pubkey_text) = decode_minisign_text(pubkey_b64) else {
        return false;
    };
    let Some(signature_text) = decode_minisign_text(signature_b64) else {
        return false;
    };
    let Ok(public_key) = minisign_verify::PublicKey::decode(&pubkey_text) else {
        return false;
    };
    let Ok(signature) = minisign_verify::Signature::decode(&signature_text) else {
        return false;
    };
    public_key.verify(data, &signature, true).is_ok()
}

fn sibling(exe_path: &Path, suffix: &str) -> PathBuf {
    let mut name = exe_path.file_name().unwrap_or_default().to_os_string();
    name.push(".");
    name.push(suffix);
    exe_path.with_file_name(name)
}

// ! Windows allows renaming a file still mapped by the running process, just not overwriting it
// in place, so the live exe is moved aside before the verified replacement takes its spot; a
// failed swap restores it immediately so the app is never left without an executable.
fn replace_exe(exe_path: &Path, backup_path: &Path, temp_path: &Path) -> io::Result<()> {
    std::fs::rename(exe_path, backup_path)?;
    if let Err(error) = std::fs::rename(temp_path, exe_path) {
        if let Err(restore_error) = restore_backup(exe_path, backup_path) {
            tracing::error!("the previous executable could not be restored: {restore_error}");
        }
        return Err(error);
    }
    Ok(())
}

fn restore_backup(exe_path: &Path, backup_path: &Path) -> io::Result<()> {
    std::fs::rename(backup_path, exe_path)
}

// ! Undoes a swap that already put the new exe in place: the file that failed its self-test is
// discarded (nothing holds it open yet, since it never ran far enough to start) and the previous
// build backed up by `replace_exe` takes its spot back.
fn rollback_failed_start(exe_path: &Path, backup_path: &Path) -> io::Result<()> {
    std::fs::remove_file(exe_path)?;
    restore_backup(exe_path, backup_path)
}

// ! A signature check only proves the bytes were signed, not that the resulting executable can
// actually run on this machine (corrupt download, missing runtime dependency, antivirus
// quarantine): this is the last chance to catch that before the user is asked to restart into it.
fn verify_exe_starts(exe_path: &Path) -> bool {
    let Ok(mut child) = Command::new(exe_path).arg(SELFTEST_ARG).spawn() else {
        return false;
    };
    let deadline = Instant::now() + SELFTEST_TIMEOUT;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return status.success(),
            Ok(None) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(20));
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                return false;
            }
            Err(_) => return false,
        }
    }
}

// ! No Tauri bundle format applies to a bare portable exe, so it is written beside itself and
// swapped in directly instead of going through `Update::install`. The previous exe is kept at
// `<name>.bak` as the rollback copy: there is nothing to restore it from if the signature check
// below fails, since that happens before the exe on disk is touched at all.
fn install_portable_update(
    app: &AppHandle,
    exe_path: &Path,
    bytes: &[u8],
    signature: &str,
) -> Result<(), ErrorCode> {
    let pubkey = updater_pubkey(app).ok_or(ErrorCode::UpdateInstallFailed)?;
    if !verify_minisign(bytes, signature, &pubkey) {
        tracing::error!("the portable update does not match its signature");
        return Err(ErrorCode::UpdateSignatureRefused);
    }

    let temp_path = sibling(exe_path, "new");
    let backup_path = sibling(exe_path, "bak");
    if let Err(error) = std::fs::write(&temp_path, bytes) {
        tracing::warn!("the portable update could not be written: {error}");
        return Err(ErrorCode::UpdateInstallFailed);
    }
    if let Err(error) = replace_exe(exe_path, &backup_path, &temp_path) {
        tracing::warn!("the portable update could not be installed: {error}");
        if let Err(error) = std::fs::remove_file(&temp_path) {
            tracing::warn!("the downloaded update file could not be removed: {error}");
        }
        return Err(ErrorCode::UpdateInstallFailed);
    }

    // ! The swap only replaces bytes on disk: launching the result here, before the user is
    // ever asked to restart into it, is what the `.bak` copy exists to recover from.
    if !verify_exe_starts(exe_path) {
        match rollback_failed_start(exe_path, &backup_path) {
            Err(error) => tracing::error!("the portable update rollback failed: {error}"),
            Ok(()) => tracing::warn!("the portable update failed its self-test; rolled back"),
        }
        return Err(ErrorCode::UpdateInstallFailed);
    }

    // ! The self-test passed, but this process still runs mapped from `backup_path` (Windows
    // lets it be renamed, not deleted, while mapped): `cleanup_stale_backup` removes it the
    // next time the new executable starts, once nothing holds it open anymore.
    Ok(())
}
