use std::collections::BTreeSet;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use reqwest_websocket::Message;
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use super::socket::{
    keep_open, send_text, socket_error, text_frame, Heartbeat, Socket, SocketPlace, SocketSession,
};
use super::{TwitchClient, CHAT_URL};
use crate::error::{Error, Result};
use crate::requests::RequestKind;
use crate::util::sleep_until_some;

pub const MAX_CHANNELS: usize = 100;
// ! Twitch limits joins to 20 every 10 seconds for a normal account.
const JOINS_PER_BURST: usize = 20;
const JOIN_BURST_PAUSE: Duration = Duration::from_secs(11);
const PING_INTERVAL: Duration = Duration::from_secs(240);
const REPLY_TIMEOUT: Duration = Duration::from_secs(15);
const REFUSED_PAUSE: Duration = Duration::from_secs(30 * 60);
const MENTIONS_QUEUE: usize = 32;
const MENTION_TEXT_MAX: usize = 300;

const PLACE: SocketPlace = SocketPlace {
    kind: RequestKind::Chat,
    address: CHAT_URL,
    name: "chat",
    label: "chat",
    ping_interval: PING_INTERVAL,
    reply_timeout: REPLY_TIMEOUT,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMention {
    pub channel: String,
    pub chatter: String,
    pub text: String,
}

pub struct Chat {
    desired: watch::Sender<BTreeSet<String>>,
    mentions: mpsc::Receiver<ChatMention>,
    cancel: CancellationToken,
    task: Option<JoinHandle<()>>,
}

impl Chat {
    pub fn start(client: TwitchClient, cancel: CancellationToken) -> Self {
        let (desired, receiver) = watch::channel(BTreeSet::new());
        let (mentioned, mentions) = mpsc::channel(MENTIONS_QUEUE);
        let session = ChatSession { mentioned };
        let task = tokio::spawn(keep_open(&PLACE, client, receiver, cancel.clone(), session));

        Self {
            desired,
            mentions,
            cancel,
            task: Some(task),
        }
    }

    // ! Waits forever once the chat is closed, so it can sit in a `select!`.
    pub async fn next_mention(&mut self) -> ChatMention {
        match self.mentions.recv().await {
            Some(mention) => mention,
            None => std::future::pending().await,
        }
    }

    pub fn set_channels(&self, channels: BTreeSet<String>) {
        self.desired.send_if_modified(|current| {
            if *current == channels {
                false
            } else {
                *current = channels;
                true
            }
        });
    }

    pub async fn close(&mut self) {
        self.cancel.cancel();

        if let Some(task) = self.task.take() {
            let _ = task.await;
        }
    }
}

struct ChatSession {
    mentioned: mpsc::Sender<ChatMention>,
}

impl SocketSession for ChatSession {
    async fn run(
        &mut self,
        client: &TwitchClient,
        socket: &mut Socket,
        desired: &mut watch::Receiver<BTreeSet<String>>,
        _reconnected: bool,
    ) -> Result<()> {
        connection(client, socket, desired, &self.mentioned).await
    }

    async fn refused(&mut self, client: &TwitchClient) -> Duration {
        tracing::warn!(account = %client.login, "Twitch refused the chat sign-in; chat presence paused");

        REFUSED_PAUSE
    }
}

async fn send(socket: &mut Socket, line: &str) -> Result<()> {
    send_text(socket, &PLACE, format!("{line}\r\n")).await
}

async fn command(socket: &mut Socket, verb: &str, channels: &[String]) -> Result<()> {
    for chunk in channels.chunks(JOINS_PER_BURST) {
        let list: Vec<String> = chunk.iter().map(|login| format!("#{login}")).collect();

        send(socket, &format!("{verb} {}", list.join(","))).await?;
    }

    Ok(())
}

async fn signed_in(socket: &mut Socket) -> Result<()> {
    loop {
        let message = tokio::time::timeout(REPLY_TIMEOUT, socket.next())
            .await
            .map_err(|_| Error::Network("chat sign-in timeout".into()))?
            .ok_or_else(|| Error::Network("chat closed".into()))?
            .map_err(|e| socket_error("chat read", e))?;
        let Message::Text(text) = message else {
            continue;
        };

        for line in text.lines() {
            if line.contains(" 001 ") {
                return Ok(());
            }

            if line.contains("NOTICE") && line.to_ascii_lowercase().contains("auth") {
                return Err(Error::Unauthorized);
            }
        }
    }
}

async fn connection(
    client: &TwitchClient,
    socket: &mut Socket,
    desired: &mut watch::Receiver<BTreeSet<String>>,
    mentioned: &mpsc::Sender<ChatMention>,
) -> Result<()> {
    let token = client.token().await;

    send(
        socket,
        &zeroize::Zeroizing::new(format!("PASS oauth:{}", token.as_str())),
    )
    .await?;
    send(socket, &format!("NICK {}", client.login)).await?;
    signed_in(socket).await?;

    let mut joined: BTreeSet<String> = BTreeSet::new();
    let mut wanted = desired.borrow_and_update().clone();
    let mut changed = true;
    let mut next_burst = tokio::time::Instant::now();
    let mut heartbeat = Heartbeat::new(&PLACE, tokio::time::Instant::now() + PING_INTERVAL);

    loop {
        if changed {
            changed = false;

            if wanted.is_empty() {
                let _ = socket.send(Message::Text("QUIT\r\n".into())).await;

                return Ok(());
            }

            let to_part: Vec<String> = joined.difference(&wanted).cloned().collect();

            if !to_part.is_empty() {
                command(socket, "PART", &to_part).await?;

                for login in &to_part {
                    joined.remove(login);
                }
            }
        }

        // ! Bursts go out between reads: a wait inside a JOIN would leave Twitch's PINGs unanswered for up to 44 s.
        if !joined.is_superset(&wanted) && tokio::time::Instant::now() >= next_burst {
            let burst: Vec<String> = wanted
                .difference(&joined)
                .take(JOINS_PER_BURST)
                .cloned()
                .collect();

            command(socket, "JOIN", &burst).await?;
            joined.extend(burst);
            next_burst = tokio::time::Instant::now() + JOIN_BURST_PAUSE;
        }

        let burst_due = (!joined.is_superset(&wanted)).then_some(next_burst);

        tokio::select! { biased;
            update = desired.changed() => {
                if update.is_err() { return Ok(()); }

                wanted = desired.borrow_and_update().clone();
                changed = true;
            }
            _ = sleep_until_some(burst_due) => {}
            beat = heartbeat.due() => {
                beat?;
                send(socket, "PING :tmi.twitch.tv").await?;
                heartbeat.pinged();
            }
            message = socket.next() => {
                let Some(text) = text_frame(socket, &PLACE, message).await? else { continue };

                for line in text.lines() {
                    match reply(line) {
                        Reply::Ping(server) => send(socket, &format!("PONG {server}")).await?,
                        Reply::Pong => heartbeat.ponged(),
                        Reply::Reconnect => return Err(Error::Network("chat asked to reconnect".into())),
                        Reply::Message { channel, chatter, text } => {
                            if let Some(mention) = mention(&client.login, channel, chatter, text) {
                                let _ = mentioned.try_send(mention);
                            }
                        }
                        Reply::Other => {}
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Reply<'a> {
    Ping(&'a str),
    Pong,
    Reconnect,
    Message {
        channel: &'a str,
        chatter: &'a str,
        text: &'a str,
    },
    Other,
}

fn mention(login: &str, channel: &str, chatter: &str, text: &str) -> Option<ChatMention> {
    if chatter.eq_ignore_ascii_case(login) {
        return None;
    }

    let bytes = text.as_bytes();
    let login = login.as_bytes();
    let named = text.match_indices('@').any(|(at, _)| {
        let rest = &bytes[at + 1..];

        rest.len() >= login.len()
            && rest[..login.len()].eq_ignore_ascii_case(login)
            && rest
                .get(login.len())
                .is_none_or(|next| !(next.is_ascii_alphanumeric() || *next == b'_'))
    });

    if !named {
        return None;
    }

    let text: String = text.chars().take(MENTION_TEXT_MAX).collect();

    Some(ChatMention {
        channel: channel.to_ascii_lowercase(),
        chatter: chatter.to_ascii_lowercase(),
        text,
    })
}

fn reply(line: &str) -> Reply<'_> {
    let line = line.trim_end_matches('\r');

    if let Some(server) = line.strip_prefix("PING ") {
        return Reply::Ping(server);
    }

    let rest = line.strip_prefix('@').map_or(line, |tagged| {
        tagged.split_once(' ').map_or("", |(_, rest)| rest)
    });
    let (prefix, rest) = match rest.strip_prefix(':') {
        Some(prefixed) => prefixed.split_once(' ').unwrap_or(("", "")),
        None => ("", rest),
    };
    let (command, params) = rest.split_once(' ').unwrap_or((rest, ""));

    match command {
        "PONG" => Reply::Pong,
        "RECONNECT" => Reply::Reconnect,
        "PRIVMSG" => {
            let chatter = prefix.split_once('!').map_or(prefix, |(nick, _)| nick);

            match params.split_once(" :") {
                Some((target, text)) if !chatter.is_empty() => Reply::Message {
                    channel: target.trim_start_matches('#'),
                    chatter,
                    text,
                },
                _ => Reply::Other,
            }
        }
        _ => Reply::Other,
    }
}
