use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use serde_json::{json, Value};
use tokio::sync::{mpsc, watch};
use tokio::task::JoinSet;
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use super::socket::{
    keep_open, send_text, text_frame, Heartbeat, Socket, SocketPlace, SocketSession,
};
use super::{saturated, text, TwitchClient, PUBSUB_URL};
use crate::domain::{PredictionEvent, PredictionOutcome};
use crate::error::{Error, Result};
use crate::requests::RequestKind;
use crate::util::{random_hex, sleep_until_some};

// ! Twitch PubSub takes at most 50 topics per socket.
const TOPICS_PER_SOCKET: usize = 50;
const SHARDS: usize = 4;
const USER_TOPICS: usize = 4;
const TOPICS_PER_CHANNEL: usize = 4;
// ! A channel's topics stay together: a socket with 3 free places refuses the next channel.
pub const MAX_CHANNELS: usize =
    (SHARDS * (TOPICS_PER_SOCKET - TOPICS_PER_CHANNEL + 1) - USER_TOPICS) / TOPICS_PER_CHANNEL;
const PING_INTERVAL: Duration = Duration::from_secs(240);
const REPLY_TIMEOUT: Duration = Duration::from_secs(10);
const LISTEN_RETRY: Duration = Duration::from_secs(60);
const UNAUTHORIZED_PAUSE: Duration = Duration::from_secs(60);
// ! Twitch's prediction windows last 30 s to 30 min: a longer one in a message is a broken payload.
const PREDICTION_WINDOW_MAX_SECONDS: i64 = 3600;

pub const WATCH_STREAK_REASON: &str = "WATCH_STREAK";

const PLACE: SocketPlace = SocketPlace {
    kind: RequestKind::Pubsub,
    address: PUBSUB_URL,
    name: "pubsub",
    label: "PubSub",
    ping_interval: PING_INTERVAL,
    reply_timeout: REPLY_TIMEOUT,
};

#[derive(Debug, Clone, PartialEq)]
pub enum PubSubEvent {
    StreamUp {
        channel_id: String,
    },
    StreamDown {
        channel_id: String,
    },
    ViewCount {
        channel_id: String,
        viewers: u64,
    },
    PointsEarned {
        channel_id: String,
        earned: i64,
        balance: Option<i64>,
        reason: String,
    },
    PointsSpent {
        channel_id: String,
        balance: i64,
    },
    ClaimAvailable {
        channel_id: String,
        claim_id: String,
    },
    Raid {
        channel_id: String,
        raid_id: String,
        target_login: String,
    },
    Moment {
        channel_id: String,
        moment_id: String,
    },
    PredictionCreated(PredictionEvent),
    PredictionUpdated(PredictionEvent),
    PredictionMade {
        event_id: String,
        outcome_id: String,
        points: i64,
    },
    PredictionResult {
        event_id: String,
        result: String,
        points_won: i64,
    },
    DropProgress {
        drop_id: String,
        current_minutes: u32,
        required_minutes: u32,
    },
    DropClaim {
        drop_id: String,
        instance_id: String,
    },
    StreakRecovered {
        channel_id: String,
    },
    Unauthorized,
    Reconnected,
}

// ! Twitch repeats `raid_update_v2` every second of the countdown: a failed join waits this long before another.
pub const RAID_JOIN_RETRY: Duration = Duration::from_secs(10);

#[derive(Debug)]
struct RaidJoin {
    raid_id: String,
    joined: bool,
    tried_at: Instant,
}

#[derive(Debug, Default)]
pub struct RaidJoins(HashMap<String, RaidJoin>);

impl RaidJoins {
    pub fn due(&mut self, channel_id: &str, raid_id: &str, now: Instant) -> bool {
        if let Some(join) = self.0.get(channel_id) {
            let same = join.raid_id == raid_id;

            if same && (join.joined || now.duration_since(join.tried_at) < RAID_JOIN_RETRY) {
                return false;
            }
        }

        self.0.insert(
            channel_id.to_owned(),
            RaidJoin {
                raid_id: raid_id.to_owned(),
                joined: false,
                tried_at: now,
            },
        );

        true
    }

    pub fn joined(&mut self, channel_id: &str, raid_id: &str) {
        if let Some(join) = self.0.get_mut(channel_id) {
            if join.raid_id == raid_id {
                join.joined = true;
            }
        }
    }
}

pub fn topics_for(user_id: &str, channels: &[String], predictions: &[String]) -> Vec<String> {
    let mut topics = vec![
        format!("community-points-user-v1.{user_id}"),
        format!("user-drop-events.{user_id}"),
        format!("predictions-user-v1.{user_id}"),
        format!("viewer-milestones.{user_id}"),
    ];

    for id in channels.iter().take(MAX_CHANNELS).collect::<BTreeSet<_>>() {
        topics.push(format!("video-playback-by-id.{id}"));
        topics.push(format!("raid.{id}"));
        topics.push(format!("community-moments-channel-v1.{id}"));

        if predictions.contains(id) {
            topics.push(format!("predictions-channel-v1.{id}"));
        }
    }

    topics
}

pub struct PubSub {
    shards: Vec<watch::Sender<BTreeSet<String>>>,
    placed: HashMap<String, usize>,
    cancel: CancellationToken,
    tasks: JoinSet<()>,
}

impl PubSub {
    pub fn start(
        client: TwitchClient,
        events: mpsc::Sender<PubSubEvent>,
        cancel: CancellationToken,
    ) -> Self {
        let mut tasks = JoinSet::new();
        let shards = (0..SHARDS)
            .map(|_| {
                let (send, receive) = watch::channel(BTreeSet::new());
                let session = PubSubSession {
                    events: events.clone(),
                };
                tasks.spawn(keep_open(
                    &PLACE,
                    client.clone(),
                    receive,
                    cancel.clone(),
                    session,
                ));

                send
            })
            .collect();

        Self {
            shards,
            placed: HashMap::new(),
            cancel,
            tasks,
        }
    }

    // ! A channel's topics stay on their socket: one coming or going never moves the others.
    pub fn set_topics(&mut self, topics: Vec<String>) {
        let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for topic in topics {
            let id = topic.rsplit('.').next().unwrap_or_default().to_owned();

            groups.entry(id).or_default().push(topic);
        }

        let mut desired: Vec<BTreeSet<String>> = vec![BTreeSet::new(); self.shards.len()];
        let previous = std::mem::take(&mut self.placed);

        for (id, group) in &groups {
            if let Some(&shard) = previous.get(id) {
                if desired[shard].len() + group.len() <= TOPICS_PER_SOCKET {
                    desired[shard].extend(group.iter().cloned());
                    self.placed.insert(id.clone(), shard);
                }
            }
        }

        for (id, group) in &groups {
            if self.placed.contains_key(id) {
                continue;
            }

            let room = (0..desired.len())
                .filter(|shard| desired[*shard].len() + group.len() <= TOPICS_PER_SOCKET)
                .min_by_key(|shard| desired[*shard].len());

            if let Some(shard) = room {
                desired[shard].extend(group.iter().cloned());
                self.placed.insert(id.clone(), shard);
            }
        }

        for (sender, desired) in self.shards.iter().zip(desired) {
            sender.send_if_modified(|current| {
                if *current == desired {
                    false
                } else {
                    *current = desired;
                    true
                }
            });
        }
    }

    pub async fn close(&mut self) {
        self.cancel.cancel();

        while self.tasks.join_next().await.is_some() {}
    }
}

struct PubSubSession {
    events: mpsc::Sender<PubSubEvent>,
}

impl SocketSession for PubSubSession {
    async fn run(
        &mut self,
        client: &TwitchClient,
        socket: &mut Socket,
        desired: &mut watch::Receiver<BTreeSet<String>>,
        reconnected: bool,
    ) -> Result<()> {
        connection(client, socket, desired, &self.events, reconnected).await
    }

    async fn refused(&mut self, _client: &TwitchClient) -> Duration {
        let _ = self.events.send(PubSubEvent::Unauthorized).await;

        UNAUTHORIZED_PAUSE
    }
}

async fn send(socket: &mut Socket, value: Value) -> Result<()> {
    send_text(socket, &PLACE, value.to_string()).await
}

async fn listen(
    socket: &mut Socket,
    verb: &'static str,
    topics: Vec<String>,
    per_message: usize,
    token: &str,
    pending: &mut HashMap<String, (&'static str, Vec<String>)>,
) -> Result<()> {
    for chunk in topics.chunks(per_message) {
        let nonce = random_hex(16);

        send(
            socket,
            json!({"type": verb, "nonce": nonce, "data": {"topics": chunk, "auth_token": token}}),
        )
        .await?;
        pending.insert(nonce, (verb, chunk.to_vec()));
    }

    Ok(())
}

async fn connection(
    client: &TwitchClient,
    socket: &mut Socket,
    desired: &mut watch::Receiver<BTreeSet<String>>,
    events: &mpsc::Sender<PubSubEvent>,
    reconnected: bool,
) -> Result<()> {
    let token = client.token().await;
    let mut subscribed: BTreeSet<String> = BTreeSet::new();
    let mut refused: BTreeSet<String> = BTreeSet::new();
    let mut retrying: BTreeSet<String> = BTreeSet::new();
    let mut retry_at: Option<Instant> = None;
    let mut pending: HashMap<String, (&'static str, Vec<String>)> = HashMap::new();
    let mut heartbeat = Heartbeat::new(&PLACE, Instant::now());
    let mut announce = reconnected;

    loop {
        if retry_at.is_some_and(|at| at <= Instant::now()) {
            retrying = std::mem::take(&mut refused);
            retry_at = None;
        }

        let wanted = desired.borrow().clone();
        let to_add: Vec<String> = wanted
            .difference(&subscribed)
            .filter(|topic| !refused.contains(*topic))
            .cloned()
            .collect();
        let to_remove: Vec<String> = subscribed.difference(&wanted).cloned().collect();

        refused.retain(|topic| wanted.contains(topic));

        // ! Out before in: the socket never holds more than its 50 topics.
        if !to_remove.is_empty() {
            listen(
                socket,
                "UNLISTEN",
                to_remove.clone(),
                10,
                &token,
                &mut pending,
            )
            .await?;

            for topic in to_remove {
                subscribed.remove(&topic);
            }
        }

        if !to_add.is_empty() {
            // ! Twitch refuses a whole message for one bad topic: a refused topic is asked alone, or it would hold back the others for good.
            let (alone, grouped): (Vec<String>, Vec<String>) = to_add
                .iter()
                .cloned()
                .partition(|topic| retrying.contains(topic));

            listen(socket, "LISTEN", grouped, 10, &token, &mut pending).await?;
            listen(socket, "LISTEN", alone, 1, &token, &mut pending).await?;
            subscribed.extend(to_add);
        }

        retrying.clear();

        if announce {
            announce = false;
            if events.send(PubSubEvent::Reconnected).await.is_err() {
                return Ok(());
            }
        }

        if subscribed.is_empty() && refused.is_empty() {
            return Ok(());
        }

        tokio::select! { biased;
            changed = desired.changed() => {
                if changed.is_err() { return Ok(()); }
            }
            _ = sleep_until_some(retry_at) => {}
            beat = heartbeat.due() => {
                beat?;
                send(socket, json!({"type": "PING"})).await?;
                heartbeat.pinged();
            }
            message = socket.next() => {
                let Some(payload) = text_frame(socket, &PLACE, message).await? else { continue };
                let value: Value = match serde_json::from_str(&payload) { Ok(v) => v, Err(_) => continue };

                match value["type"].as_str().unwrap_or("") {
                    "PONG" => heartbeat.ponged(),
                    "RECONNECT" => return Err(Error::Network("pubsub asked to reconnect".into())),
                    "RESPONSE" => {
                        let (verb, asked) = value["nonce"].as_str().and_then(|nonce| pending.remove(nonce)).unwrap_or_default();

                        match value["error"].as_str().unwrap_or("") {
                            "" => {}
                            "ERR_BADAUTH" => return Err(Error::Unauthorized),
                            other => {
                                tracing::warn!(
                                    account = %client.login,
                                    error = other,
                                    topics = %topic_kinds(&asked),
                                    "PubSub {verb} refused"
                                );

                                if verb == "LISTEN" {
                                    for topic in asked {
                                        subscribed.remove(&topic);
                                        refused.insert(topic);
                                    }

                                    retry_at.get_or_insert(Instant::now() + LISTEN_RETRY);
                                }
                            }
                        }
                    }
                    "MESSAGE" => {
                        if let Some(event) = parse_message(&value["data"]) {
                            if events.send(event).await.is_err() { return Ok(()); }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn topic_kinds(topics: &[String]) -> String {
    topics
        .iter()
        .filter_map(|topic| topic.split('.').next())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join(",")
}

fn topic_channel(topic: &str) -> Option<String> {
    topic
        .rsplit('.')
        .next()
        .filter(|v| !v.is_empty())
        .map(str::to_owned)
}

pub fn parse_message(data: &Value) -> Option<PubSubEvent> {
    let topic = data["topic"].as_str()?;
    let message: Value = serde_json::from_str(data["message"].as_str()?).ok()?;
    let kind = message["type"].as_str().unwrap_or("");
    let body = &message["data"];
    let prefix = topic.split('.').next().unwrap_or("");

    match prefix {
        "video-playback-by-id" => {
            let channel_id = topic_channel(topic)?;

            match kind {
                "stream-up" => Some(PubSubEvent::StreamUp { channel_id }),
                "stream-down" => Some(PubSubEvent::StreamDown { channel_id }),
                "viewcount" => Some(PubSubEvent::ViewCount {
                    channel_id,
                    viewers: message["viewers"].as_u64()?,
                }),
                _ => None,
            }
        }
        "community-points-user-v1" => match kind {
            "points-earned" => Some(PubSubEvent::PointsEarned {
                channel_id: text(&body["channel_id"])?,
                earned: body["point_gain"]["total_points"].as_i64()?,
                balance: body["balance"]["balance"].as_i64(),
                reason: text(&body["point_gain"]["reason_code"]).unwrap_or_default(),
            }),
            "points-spent" => Some(PubSubEvent::PointsSpent {
                channel_id: text(&body["balance"]["channel_id"])?,
                balance: body["balance"]["balance"].as_i64()?,
            }),
            "claim-available" => Some(PubSubEvent::ClaimAvailable {
                channel_id: text(&body["claim"]["channel_id"])?,
                claim_id: text(&body["claim"]["id"])?,
            }),
            _ => None,
        },
        "raid" => {
            if kind != "raid_update_v2" && kind != "raid_update" {
                return None;
            }

            let raid = &message["raid"];

            Some(PubSubEvent::Raid {
                channel_id: topic_channel(topic)?,
                raid_id: text(&raid["id"])?,
                target_login: text(&raid["target_login"]).unwrap_or_default(),
            })
        }
        "community-moments-channel-v1" => {
            if kind != "active" {
                return None;
            }

            Some(PubSubEvent::Moment {
                channel_id: topic_channel(topic)?,
                moment_id: text(&body["moment_id"])?,
            })
        }
        "predictions-channel-v1" => {
            let event = parse_prediction(&body["event"], &topic_channel(topic)?)?;

            match kind {
                "event-created" => Some(PubSubEvent::PredictionCreated(event)),
                "event-updated" => Some(PubSubEvent::PredictionUpdated(event)),
                _ => None,
            }
        }
        "predictions-user-v1" => {
            let prediction = &body["prediction"];

            match kind {
                "prediction-made" => Some(PubSubEvent::PredictionMade {
                    event_id: text(&prediction["event_id"])?,
                    outcome_id: text(&prediction["outcome_id"])?,
                    points: prediction["points"].as_i64().unwrap_or(0),
                }),
                "prediction-result" => Some(PubSubEvent::PredictionResult {
                    event_id: text(&prediction["event_id"])?,
                    result: text(&prediction["result"]["type"]).unwrap_or_default(),
                    points_won: prediction["result"]["points_won"].as_i64().unwrap_or(0),
                }),
                _ => None,
            }
        }
        "user-drop-events" => match kind {
            "drop-progress" => Some(PubSubEvent::DropProgress {
                drop_id: text(&body["drop_id"])?,
                current_minutes: saturated(body["current_progress_min"].as_u64().unwrap_or(0)),
                required_minutes: saturated(body["required_progress_min"].as_u64().unwrap_or(0)),
            }),
            "drop-claim" => Some(PubSubEvent::DropClaim {
                drop_id: text(&body["drop_id"])?,
                instance_id: text(&body["drop_instance_id"])?,
            }),
            _ => None,
        },
        "viewer-milestones" => {
            if kind != "streak-recovered" {
                return None;
            }

            Some(PubSubEvent::StreakRecovered {
                channel_id: text(&body["channel_id"]).unwrap_or_default(),
            })
        }
        _ => None,
    }
}

fn parse_prediction(event: &Value, channel_id: &str) -> Option<PredictionEvent> {
    let outcomes = event["outcomes"]
        .as_array()?
        .iter()
        .filter_map(|o| {
            Some(PredictionOutcome {
                id: text(&o["id"])?,
                title: text(&o["title"]).unwrap_or_default(),
                color: text(&o["color"]).unwrap_or_default(),
                total_users: o["total_users"].as_i64().unwrap_or(0),
                total_points: o["total_points"].as_i64().unwrap_or(0),
                top_points: o["top_predictors"]
                    .as_array()
                    .and_then(|p| p.iter().filter_map(|p| p["points"].as_i64()).max())
                    .unwrap_or(0),
                percentage_users: 0.0,
                odds: 0.0,
                odds_percentage: 0.0,
            })
        })
        .collect();
    let mut prediction = PredictionEvent {
        id: text(&event["id"])?,
        channel_id: text(&event["channel_id"]).unwrap_or_else(|| channel_id.to_owned()),
        title: text(&event["title"]).unwrap_or_default(),
        status: text(&event["status"]).unwrap_or_default(),
        created_at: text(&event["created_at"])
            .and_then(|v| v.parse::<DateTime<Utc>>().ok())
            .unwrap_or_else(Utc::now),
        window_seconds: event["prediction_window_seconds"]
            .as_i64()
            .unwrap_or(0)
            .clamp(0, PREDICTION_WINDOW_MAX_SECONDS),
        outcomes,
        winning_outcome_id: text(&event["winning_outcome_id"]),
    };

    prediction.update_outcomes();

    Some(prediction)
}
