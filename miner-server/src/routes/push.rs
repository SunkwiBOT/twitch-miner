use super::*;

const SESSION_ENDED_REASON: &str = "session ended";
// ! A client that stops reading would otherwise hold its push channel, and
// the end of its session, open.
const SEND_TIMEOUT: Duration = Duration::from_secs(10);
// ! A proxy closes a silent websocket (nginx after 60 s), and a connection
// that died halfway would otherwise stay open: a ping left unanswered ends it.
const PING_INTERVAL: Duration = Duration::from_secs(30);
const INCOMING_MESSAGE_MAX_BYTES: usize = 4 * 1024;

// ! Only for a page of this same dashboard: a websocket skips the browser's
// CORS checks, so another site could otherwise read the snapshot (webhook
// URLs, proxies) through it.
pub(super) async fn events(
    State(state): State<Shared>,
    lease: Option<Extension<SessionLease>>,
    headers: HeaderMap,
    upgrade: WebSocketUpgrade,
) -> Response {
    if !state.access.allows_origin(&headers) {
        return Failure::from(ErrorCode::Unauthorized).into_response();
    }

    let lease = lease.map(|Extension(lease)| lease);

    upgrade
        .max_message_size(INCOMING_MESSAGE_MAX_BYTES)
        .max_frame_size(INCOMING_MESSAGE_MAX_BYTES)
        .on_upgrade(move |socket| stream_events(state, socket, lease))
}

async fn send_message(socket: &mut WebSocket, message: Message) -> bool {
    matches!(
        tokio::time::timeout(SEND_TIMEOUT, socket.send(message)).await,
        Ok(Ok(()))
    )
}

async fn send(socket: &mut WebSocket, message: &ServerMessage) -> bool {
    let Ok(text) = serde_json::to_string(message) else {
        return false;
    };

    send_message(socket, Message::Text(text.into())).await
}

async fn send_snapshot(socket: &mut WebSocket, text: Option<Utf8Bytes>) -> bool {
    match text {
        Some(text) => send_message(socket, Message::Text(text)).await,
        None => true,
    }
}

async fn stream_events(state: Shared, mut socket: WebSocket, lease: Option<SessionLease>) {
    let mut receiver = state.manager.subscribe();
    let mut bursts = state.manager.subscribe_bursts();
    let mut snapshots = state.snapshots.subscribe();

    state.snapshots.publish().await;

    let first = snapshots.borrow_and_update().clone();

    if !send_snapshot(&mut socket, first).await {
        return;
    }

    let session_ended = async {
        match &lease {
            Some(lease) => lease.ended().await,
            None => std::future::pending().await,
        }
    };
    tokio::pin!(session_ended);

    let mut ping =
        tokio::time::interval_at(tokio::time::Instant::now() + PING_INTERVAL, PING_INTERVAL);
    let mut answered = true;

    ping.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    // ! Biased: once the session ends, not one more message goes out.
    loop {
        tokio::select! {
            biased;

            _ = &mut session_ended => {
                let close = CloseFrame {
                    code: close_code::POLICY,
                    reason: SESSION_ENDED_REASON.into(),
                };
                send_message(&mut socket, Message::Close(Some(close))).await;

                return;
            }
            published = snapshots.changed() => {
                if published.is_err() {
                    return;
                }

                let text = snapshots.borrow_and_update().clone();

                if !send_snapshot(&mut socket, text).await {
                    return;
                }
            }
            event = receiver.recv() => match event {
                Ok(event) => {
                    if !send(&mut socket, &ServerMessage::Event { event: Box::new(event) }).await {
                        return;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => state.snapshots.mark_stale(),
                Err(_) => return,
            },
            progress = bursts.recv() => match progress {
                Ok(progress) => {
                    if !send(&mut socket, &ServerMessage::Burst { progress }).await {
                        return;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {}
                Err(_) => return,
            },
            _ = ping.tick() => {
                if !answered || !send_message(&mut socket, Message::Ping(Default::default())).await {
                    return;
                }

                answered = false;
            }
            incoming = socket.recv() => match incoming {
                Some(Ok(Message::Close(_))) | None | Some(Err(_)) => return,
                Some(Ok(_)) => answered = true,
            }
        }
    }
}
