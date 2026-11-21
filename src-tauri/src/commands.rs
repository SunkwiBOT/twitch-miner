use super::*;
use tauri_plugin_dialog::DialogExt;

pub(crate) type Answer<T> = Result<T, ApiError>;

// ! Locked, the window may only unlock, lock again or cancel a sign-in: it holds
// no settings worth saving (see `locked_snapshot`) and must not remove an account
// without the password.
pub(super) const LOCKED_COMMANDS: [CommandName; 3] = [
    CommandName::Unlock,
    CommandName::Lock,
    CommandName::CancelLogin,
];

// ! Strict lock: while the app password is not typed, nothing the window asks
// for reads the accounts, their history or the log, changes the system or
// installs an update.
pub(crate) async fn unlocked(state: &AppState) -> Answer<()> {
    if state.manager.vault_status().await.unlocked {
        Ok(())
    } else {
        Err(ApiError::new(ErrorCode::VaultLocked))
    }
}

// ! A lock between the two reads leaves the full snapshot with a locked vault:
// it is reduced too.
pub(super) async fn visible_snapshot(manager: &Manager) -> Answer<Snapshot> {
    if manager.vault_status().await.unlocked {
        let snapshot = manager.snapshot().await?;

        if snapshot.vault.unlocked {
            return Ok(snapshot);
        }
    }

    Ok(locked_snapshot(manager).await)
}

// ! Only what the lock screen needs: no account, event, campaign, proxy or
// notification secret.
async fn locked_snapshot(manager: &Manager) -> Snapshot {
    let settings = manager.global_settings().await;
    let vault = manager.vault_status().await;

    Snapshot {
        version: miner_core::VERSION.to_owned(),
        build: None,
        update_available: None,
        update_checked_at: None,
        star_prompt: false,
        accounts: Vec::new(),
        catalog: CatalogView::default(),
        badges: BadgeCatalogView::default(),
        quiet_window_active: false,
        history_retention_days: None,
        settings: GlobalSettings {
            language: settings.language,
            theme: settings.theme,
            time_zone: settings.time_zone,
            date_format: settings.date_format,
            clock_format: settings.clock_format,
            ..GlobalSettings::default()
        },
        vault: VaultStatus {
            unlocked: false,
            ..vault
        },
        recent_events: Vec::new(),
        login: None,
        import: None,
        limits: miner_core::api::CoreLimits::CURRENT,
        import_default_patch: AccountSettingsPatch::defaults(),
    }
}

#[tauri::command]
pub(super) async fn snapshot(state: tauri::State<'_, AppState>) -> Answer<SnapshotResponse> {
    Ok(SnapshotResponse {
        snapshot: visible_snapshot(&state.manager).await?,
    })
}

#[tauri::command]
pub(super) async fn command(state: tauri::State<'_, AppState>, command: Command) -> Answer<()> {
    if !LOCKED_COMMANDS.contains(&command.name()) {
        unlocked(&state).await?;
    }

    Ok(state.manager.execute(command).await?)
}

#[tauri::command]
pub(super) async fn start_login(state: tauri::State<'_, AppState>) -> Answer<LoginFlowResponse> {
    unlocked(&state).await?;

    Ok(LoginFlowResponse {
        login: state.manager.start_login().await?,
    })
}

#[tauri::command]
pub(super) async fn check_pasted_token(
    state: tauri::State<'_, AppState>,
    token: String,
) -> Answer<CheckPastedTokenResponse> {
    unlocked(&state).await?;

    Ok(CheckPastedTokenResponse {
        check: state.manager.check_pasted_token(&token).await?,
    })
}

#[tauri::command]
pub(super) async fn import_tokens(
    state: tauri::State<'_, AppState>,
    tokens: Vec<String>,
    settings: AccountSettingsPatch,
    apply_existing_settings: bool,
) -> Answer<ImportTokensResponse> {
    unlocked(&state).await?;

    Ok(ImportTokensResponse {
        import: state
            .manager
            .start_import_tokens(tokens, settings, apply_existing_settings)
            .await?,
    })
}

#[tauri::command]
pub(super) async fn lookup_channel(
    state: tauri::State<'_, AppState>,
    login: String,
) -> Answer<ChannelResponse> {
    unlocked(&state).await?;

    Ok(ChannelResponse {
        channel: state.manager.lookup_channel(&login).await?,
    })
}

#[tauri::command]
pub(super) async fn channel_banners(
    state: tauri::State<'_, AppState>,
    logins: Vec<String>,
) -> Answer<ChannelBannersResponse> {
    unlocked(&state).await?;

    Ok(state.manager.channel_banners(&logins).await?)
}

#[tauri::command]
pub(super) async fn search(
    state: tauri::State<'_, AppState>,
    query: String,
) -> Answer<SearchResponse> {
    unlocked(&state).await?;

    Ok(state.manager.search(&query).await?)
}

#[tauri::command]
pub(super) async fn lookup_channels(
    state: tauri::State<'_, AppState>,
    logins: Vec<String>,
) -> Answer<ChannelsLookupResponse> {
    unlocked(&state).await?;

    Ok(state.manager.lookup_channels(&logins).await?)
}

#[tauri::command]
pub(super) async fn follows(
    state: tauri::State<'_, AppState>,
    account_id: String,
) -> Answer<FollowsResponse> {
    unlocked(&state).await?;

    Ok(FollowsResponse {
        channels: state.manager.followed_channels(&account_id).await?,
    })
}

#[tauri::command]
pub(super) async fn spam_channel(
    state: tauri::State<'_, AppState>,
    account_id: String,
    login: String,
) -> Answer<SpamChannelResponse> {
    unlocked(&state).await?;

    Ok(SpamChannelResponse {
        channel: state.manager.spam_channel(&account_id, &login).await?,
    })
}

#[tauri::command]
pub(super) async fn spam_balance(
    state: tauri::State<'_, AppState>,
    account_id: String,
    login: String,
) -> Answer<SpamBalanceResponse> {
    unlocked(&state).await?;

    Ok(SpamBalanceResponse {
        balance: state.manager.spam_balance(&account_id, &login).await?,
    })
}

#[tauri::command]
pub(super) async fn redeem(
    state: tauri::State<'_, AppState>,
    request: RedeemRequest,
) -> Answer<RedeemResponse> {
    unlocked(&state).await?;

    Ok(RedeemResponse {
        burst_id: state.manager.redeem(request).await?,
    })
}

#[tauri::command]
pub(super) async fn test_notification(
    state: tauri::State<'_, AppState>,
    request: NotificationTestRequest,
) -> Answer<NotificationTest> {
    unlocked(&state).await?;

    Ok(state.manager.test_notification_target(request).await?)
}

#[tauri::command]
pub(super) async fn test_proxy(
    state: tauri::State<'_, AppState>,
    url: String,
) -> Answer<ProxyTestResponse> {
    unlocked(&state).await?;

    Ok(state.manager.test_proxy(&url).await?)
}

#[tauri::command]
pub(super) async fn history_export(
    state: tauri::State<'_, AppState>,
    query: HistoryExportQuery,
) -> Answer<HistoryExportResponse> {
    unlocked(&state).await?;

    Ok(state.manager.history_export(&query)?)
}

#[tauri::command]
pub(super) async fn history(
    state: tauri::State<'_, AppState>,
    query: HistoryQuery,
) -> Answer<HistoryResponse> {
    unlocked(&state).await?;

    Ok(state.manager.history_page(&query)?)
}

#[tauri::command]
pub(super) async fn history_stats(
    state: tauri::State<'_, AppState>,
    query: HistoryStatsQuery,
) -> Answer<HistoryStatsResponse> {
    unlocked(&state).await?;

    Ok(state.manager.history_stats(&query).await?)
}

#[tauri::command]
pub(super) async fn history_curve(
    state: tauri::State<'_, AppState>,
    query: HistoryCurveQuery,
) -> Answer<HistoryCurveResponse> {
    unlocked(&state).await?;

    Ok(state.manager.history_curve(&query).await?)
}

#[tauri::command]
pub(super) async fn refresh_badges(state: tauri::State<'_, AppState>) -> Answer<RefreshResponse> {
    unlocked(&state).await?;

    Ok(RefreshResponse {
        refresh: state.manager.refresh_badges().await?,
    })
}

#[tauri::command]
pub(super) async fn badges(state: tauri::State<'_, AppState>) -> Answer<BadgesResponse> {
    unlocked(&state).await?;

    Ok(state.manager.badge_list().await)
}

#[tauri::command]
pub(super) async fn catalog(state: tauri::State<'_, AppState>) -> Answer<CatalogResponse> {
    unlocked(&state).await?;

    Ok(state.manager.catalog_list().await)
}

#[tauri::command]
pub(super) async fn logs(
    state: tauri::State<'_, AppState>,
    query: LogsQuery,
) -> Answer<LogsResponse> {
    unlocked(&state).await?;

    Ok(state.manager.logs_page(&query))
}

#[tauri::command]
pub(super) async fn logs_report(
    state: tauri::State<'_, AppState>,
    query: LogsQuery,
) -> Answer<LogsReportResponse> {
    unlocked(&state).await?;

    Ok(state.manager.logs_report(&query).await)
}

#[tauri::command]
pub(super) async fn requests(
    state: tauri::State<'_, AppState>,
    query: RequestsQuery,
) -> Answer<RequestsResponse> {
    unlocked(&state).await?;

    Ok(state.manager.requests_page(query).await?)
}

#[tauri::command]
pub(super) async fn requests_report(
    state: tauri::State<'_, AppState>,
    query: RequestsQuery,
) -> Answer<LogsReportResponse> {
    unlocked(&state).await?;

    Ok(state.manager.requests_report(query).await?)
}

#[tauri::command]
pub(super) async fn close_window(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    action: CloseAction,
) -> Answer<()> {
    let action = if action == CloseAction::Tray {
        CloseAction::Tray
    } else {
        CloseAction::Quit
    };

    let mut settings = state.manager.global_settings().await;
    settings.close_action = action;
    state.manager.set_global_settings(settings).await?;

    if action == CloseAction::Tray {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.hide();
            window_shown(&app, false);
        }
    } else {
        lifecycle::quit(app).await;
    }

    Ok(())
}

#[tauri::command]
pub(super) fn app_info(folder: tauri::State<'_, DataFolder>) -> AppInfoResponse {
    AppInfoResponse {
        app: AppInfo {
            name: miner_core::APP_NAME.to_owned(),
            version: miner_core::VERSION.to_owned(),
            platform: std::env::consts::OS.to_owned(),
            data_dir: folder.path.to_string_lossy().into_owned(),
        },
    }
}

#[tauri::command]
pub(super) fn startup_problem(folder: tauri::State<'_, DataFolder>) -> StartupProblemResponse {
    StartupProblemResponse {
        problem: folder.problem.as_ref().map(|detail| StartupProblem {
            detail: detail.clone(),
            data_dir: folder.path.to_string_lossy().into_owned(),
        }),
    }
}

#[tauri::command]
pub(super) async fn quit(app: AppHandle) {
    lifecycle::quit(app).await;
}

fn open_directory(app: &AppHandle, dir: &Path) -> Answer<()> {
    tauri_plugin_opener::OpenerExt::opener(app)
        .open_path(dir.to_string_lossy().to_string(), None::<&str>)
        .map_err(|error| {
            tracing::warn!("opening a folder failed: {error}");
            ApiError::new(ErrorCode::StorageError)
        })
}

#[tauri::command]
pub(super) async fn open_logs_directory(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    folder: tauri::State<'_, DataFolder>,
) -> Answer<()> {
    unlocked(&state).await?;

    open_directory(&app, &folder.path.join("logs"))
}

#[tauri::command]
pub(super) fn open_data_directory(
    app: AppHandle,
    folder: tauri::State<'_, DataFolder>,
) -> Answer<()> {
    open_directory(&app, &folder.path)
}

// ! No `ts-rs` binding: hand-mirrored as `ExportCsvResultType` in `ui/src/lib/api/tauri.ts`.
#[derive(serde::Serialize)]
pub(super) struct ExportCsvResult {
    path: String,
    counts: Vec<miner_core::api::CategoryCount>,
    truncated: bool,
}

#[tauri::command]
pub(super) async fn export_csv_file(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    query: HistoryExportQuery,
) -> Answer<Option<ExportCsvResult>> {
    unlocked(&state).await?;

    let response = state.manager.history_export(&query)?;
    let filename = format!(
        "claimed-drops-{}.csv",
        chrono::Utc::now().format("%Y-%m-%d")
    );

    // ! Opens the dialog and writes the file itself, from the query: the
    // webview never names a path or the CSV content, so it needs no
    // `dialog:allow-save` capability. `None` means the user closed the dialog.
    let Some(path) = app
        .dialog()
        .file()
        .set_file_name(&filename)
        .add_filter("CSV", &["csv"])
        .blocking_save_file()
    else {
        return Ok(None);
    };

    let path = path.into_path().map_err(|error| {
        tracing::warn!("resolving the export path failed: {error}");
        ApiError::new(ErrorCode::StorageError)
    })?;

    // ! `csv` already carries the UTF-8 BOM from `history_export`: written as-is here.
    std::fs::write(&path, &response.csv).map_err(|error| {
        tracing::warn!("writing the CSV export failed: {error}");
        ApiError::new(ErrorCode::StorageError)
    })?;

    Ok(Some(ExportCsvResult {
        path: path.to_string_lossy().into_owned(),
        counts: response.counts,
        truncated: response.truncated,
    }))
}
