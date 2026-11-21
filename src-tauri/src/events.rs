use super::*;

pub(super) async fn forward_events(
    app: AppHandle,
    manager: Arc<Manager>,
    mut receiver: tokio::sync::broadcast::Receiver<miner_core::events::MinerEvent>,
) {
    let data_dir = app.state::<DataFolder>().path.clone();
    let desktop = DesktopNotifier::open(
        &data_dir,
        &app.config().identifier,
        tauri::is_dev(),
        include_bytes!("../icons/128x128.png"),
        manager.requests.clone(),
    )
    .await;

    let window = app.state::<WindowShown>().0.subscribe();
    let mut bursts = manager.subscribe_bursts();
    let mut dirty = false;
    let mut held = false;
    let mut sent_unlocked: Option<bool> = None;
    let mut tray_view: Option<TrayView> = None;
    let mut debounce = tokio::time::interval(std::time::Duration::from_millis(
        miner_core::api::SNAPSHOT_PUSH_DEBOUNCE_MS,
    ));

    // ! One waiter kept armed between iterations, so a change signalled while
    // the loop is busy is not lost.
    let changed = manager.changed.notified();
    tokio::pin!(changed);
    changed.as_mut().enable();

    loop {
        tokio::select! {
            _ = &mut changed => {
                dirty = true;
                changed.set(manager.changed.notified());
                changed.as_mut().enable();
            }
            _ = debounce.tick() => {
                if !dirty && !held {
                    continue;
                }

                let shown = *window.borrow();

                if dirty || shown {
                    dirty = false;

                    let Ok(snapshot) = visible_snapshot(&manager).await else {
                        continue;
                    };
                    let view = TrayView {
                        language: i18n::resolve(&snapshot.settings.language),
                        sign_in_required: snapshot
                            .accounts
                            .iter()
                            .any(|account| account.status == AccountStatus::LoginRequired),
                    };

                    if tray_view != Some(view) {
                        tray::show(&app, tray_view, view);
                        tray_view = Some(view);
                    }

                    // ! A window hidden in the tray only gets a lock or an unlock: the rest waits until it shows.
                    if shown || sent_unlocked != Some(snapshot.vault.unlocked) {
                        sent_unlocked = Some(snapshot.vault.unlocked);
                        held = false;
                        let _ = app.emit(DesktopEvent::Snapshot.name(), snapshot);
                    } else {
                        held = true;
                    }
                }
            }
            event = receiver.recv() => match event {
                Ok(event) => {
                    let settings = manager.global_settings().await;

                    if miner_core::notifications::desktop_wanted(&settings.notifications, event.kind) {
                        desktop.notify(event.clone(), settings.notification_language());
                    }

                    if manager.vault_status().await.unlocked {
                        let _ = app.emit(DesktopEvent::MinerEvent.name(), event);
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => { dirty = true; }
                Err(_) => break,
            },
            progress = bursts.recv() => match progress {
                Ok(progress) => {
                    if manager.vault_status().await.unlocked {
                        let _ = app.emit(DesktopEvent::BurstProgress.name(), progress);
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {}
                Err(_) => break,
            }
        }
    }
}
