use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::Snapshot;
use crate::events::MinerEvent;
use crate::spam::BurstProgress;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(rename = "ServerMessageType")]
pub enum ServerMessage {
    Snapshot { snapshot: Box<Snapshot> },
    Event { event: Box<MinerEvent> },
    Burst { progress: BurstProgress },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum ServerMessageKind {
    Snapshot,
    Event,
    Burst,
}

impl ServerMessage {
    pub fn kind(&self) -> ServerMessageKind {
        match self {
            Self::Snapshot { .. } => ServerMessageKind::Snapshot,
            Self::Event { .. } => ServerMessageKind::Event,
            Self::Burst { .. } => ServerMessageKind::Burst,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum DesktopEvent {
    Snapshot,
    MinerEvent,
    CloseRequested,
    BurstProgress,
}

impl DesktopEvent {
    pub fn name(self) -> &'static str {
        match self {
            Self::Snapshot => "SNAPSHOT",
            Self::MinerEvent => "MINER_EVENT",
            Self::CloseRequested => "CLOSE_REQUESTED",
            Self::BurstProgress => "BURST_PROGRESS",
        }
    }
}
