use std::collections::BTreeSet;
use std::future::Future;
use std::time::Duration;

use futures_util::SinkExt;
use reqwest::StatusCode;
use reqwest_websocket::{Message, Upgrade as _};
use tokio::sync::watch;
use tokio::time::{Instant, Interval, MissedTickBehavior};
use tokio_util::sync::CancellationToken;

use super::TwitchClient;
use crate::error::{Error, NetworkErrorKind, Result};
use crate::requests::{address_target, RequestKind, SocketEvent};
use crate::util::sleep_until_some;

const WARN_AFTER_TRIES: u32 = 3;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(20);
const STEADY_AFTER: Duration = Duration::from_secs(60);

pub type Socket = reqwest_websocket::WebSocket;

type Frame = std::result::Result<Message, reqwest_websocket::Error>;

pub struct SocketPlace {
    pub kind: RequestKind,
    pub address: &'static str,
    pub name: &'static str,
    pub label: &'static str,
    pub ping_interval: Duration,
    pub reply_timeout: Duration,
}

pub trait SocketSession {
    fn run(
        &mut self,
        client: &TwitchClient,
        socket: &mut Socket,
        desired: &mut watch::Receiver<BTreeSet<String>>,
        reconnected: bool,
    ) -> impl Future<Output = Result<()>> + Send;

    fn refused(&mut self, client: &TwitchClient) -> impl Future<Output = Duration> + Send;
}

pub async fn keep_open(
    place: &'static SocketPlace,
    client: TwitchClient,
    mut desired: watch::Receiver<BTreeSet<String>>,
    cancel: CancellationToken,
    mut session: impl SocketSession,
) {
    let mut failures = 0u32;
    let mut dropped = false;

    loop {
        if desired.borrow().is_empty() {
            tokio::select! { biased;
                _ = cancel.cancelled() => return,
                changed = desired.changed() => { if changed.is_err() { return; } }
            }

            continue;
        }

        let started = Instant::now();
        let event = if dropped {
            SocketEvent::Reconnect
        } else {
            SocketEvent::Connect
        };
        let opened = tokio::select! { biased;
            _ = cancel.cancelled() => return,
            opened = open(&client, place, event) => opened,
        };
        let result = match opened {
            Ok(mut socket) => {
                let run = session.run(&client, &mut socket, &mut desired, dropped);
                let Some(result) = hold(&client, place, &cancel, run).await else {
                    return;
                };

                result
            }
            Err(error) => Err(error),
        };

        dropped = result.is_err();

        let delay = match result {
            Ok(()) => {
                failures = 0;

                continue;
            }
            // ! The account may be waiting for this worker to stop, its queue full: the stop wins.
            Err(Error::Unauthorized) => tokio::select! { biased;
                _ = cancel.cancelled() => return,
                pause = session.refused(&client) => pause,
            },
            Err(error) => {
                if started.elapsed() > STEADY_AFTER {
                    failures = 0;
                }

                if failures == 0 {
                    tracing::info!(account = %client.login, %error, "{} connection dropped; reconnecting", place.label);
                } else if failures == WARN_AFTER_TRIES {
                    tracing::warn!(
                        account = %client.login,
                        tries = failures,
                        %error,
                        "{} still cannot reconnect; trying again",
                        place.label
                    );
                }

                let delay = Duration::from_secs(1 << failures.min(6));

                failures = failures.saturating_add(1);

                delay
            }
        };

        tokio::select! { biased;
            _ = cancel.cancelled() => return,
            _ = tokio::time::sleep(delay) => {}
        }
    }
}

async fn open(client: &TwitchClient, place: &SocketPlace, event: SocketEvent) -> Result<Socket> {
    let name = place.name;
    let pending = client
        .http
        .requests
        .start(place.kind, address_target(place.address))
        .account(Some(&client.user_id))
        .socket(event);
    let sent = tokio::time::timeout(
        CONNECT_TIMEOUT,
        client
            .http
            .socket_client
            .get(place.address)
            .header("Client-Id", &client.identity.client_id)
            .header(reqwest::header::USER_AGENT, &client.identity.user_agent)
            .upgrade()
            .send(),
    )
    .await;
    let response = match sent {
        Ok(Ok(response)) => response,
        Ok(Err(error)) => {
            let error = socket_error(name, error);

            pending.failed(&error);

            return Err(error);
        }
        Err(_) => {
            pending.failed(&Error::Transport(NetworkErrorKind::Timeout));

            return Err(Error::Network(format!("{name} connect timeout")));
        }
    };
    let status = response.status();

    match response.into_websocket().await {
        Ok(socket) => {
            pending.succeeded(Some(status));

            Ok(socket)
        }
        Err(error) => {
            let error = socket_error(name, error);

            if status == StatusCode::SWITCHING_PROTOCOLS {
                pending.failed(&error);
            } else {
                pending.answered(status);
            }

            Err(error)
        }
    }
}

async fn hold(
    client: &TwitchClient,
    place: &SocketPlace,
    cancel: &CancellationToken,
    session: impl Future<Output = Result<()>>,
) -> Option<Result<()>> {
    let close = client
        .http
        .requests
        .start(place.kind, address_target(place.address))
        .account(Some(&client.user_id))
        .socket(SocketEvent::Close);
    let result = tokio::select! { biased;
        _ = cancel.cancelled() => None,
        result = session => Some(result),
    };

    match &result {
        Some(Err(Error::Network(reason))) => close.closed(reason),
        Some(Err(error)) => close.closed(&error.to_string()),
        _ => close.succeeded(None),
    }

    result
}

pub async fn send_text(socket: &mut Socket, place: &SocketPlace, text: String) -> Result<()> {
    tokio::time::timeout(place.reply_timeout, socket.send(Message::Text(text)))
        .await
        .map_err(|_| Error::Network(format!("{} send timeout", place.name)))?
        .map_err(|e| socket_error(&format!("{} send", place.name), e))
}

pub async fn text_frame(
    socket: &mut Socket,
    place: &SocketPlace,
    frame: Option<Frame>,
) -> Result<Option<String>> {
    let Some(frame) = frame else {
        return Err(Error::Network(format!("{} closed", place.name)));
    };

    match frame.map_err(|e| socket_error(&format!("{} read", place.name), e))? {
        Message::Text(text) => Ok(Some(text)),
        Message::Ping(data) => {
            let _ = socket.send(Message::Pong(data)).await;

            Ok(None)
        }
        Message::Close { .. } => Err(Error::Network(format!("{} close frame", place.name))),
        _ => Ok(None),
    }
}

pub struct Heartbeat {
    place: &'static SocketPlace,
    ping: Interval,
    awaiting_pong: Option<Instant>,
}

impl Heartbeat {
    pub fn new(place: &'static SocketPlace, first_ping: Instant) -> Self {
        let mut ping = tokio::time::interval_at(first_ping, place.ping_interval);

        ping.set_missed_tick_behavior(MissedTickBehavior::Delay);

        Self {
            place,
            ping,
            awaiting_pong: None,
        }
    }

    pub async fn due(&mut self) -> Result<()> {
        let late = self
            .awaiting_pong
            .map(|sent| sent + self.place.reply_timeout);

        tokio::select! { biased;
            _ = sleep_until_some(late) => Err(Error::Network(format!("{} pong timeout", self.place.name))),
            _ = self.ping.tick() => Ok(()),
        }
    }

    pub fn pinged(&mut self) {
        self.awaiting_pong = Some(Instant::now());
    }

    pub fn ponged(&mut self) {
        self.awaiting_pong = None;
    }
}

// ! The library's own text only names the layer ("reqwest error"): the cause is one level down.
pub fn socket_error(context: &str, error: reqwest_websocket::Error) -> Error {
    match error {
        reqwest_websocket::Error::Reqwest(error) => Error::from(error),
        other => match std::error::Error::source(&other) {
            Some(cause) => Error::Network(format!("{context}: {cause}")),
            None => Error::Network(format!("{context}: {other}")),
        },
    }
}
