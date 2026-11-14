mod buffer;
mod files;
mod layer;
mod redact;

pub use buffer::{LogBuffer, LogFilter, LogPage, CAPACITY};
pub use files::{
    close_file_writer, daily_files, keep_file_writer, log_panics, FileWriter, LOG_FILES_KEPT,
};
pub use layer::LogLayer;
pub use redact::{redact, Redacted, RedactedLine};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::Level;
use ts_rs::TS;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(repr(enum = name))]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<&Level> for LogLevel {
    fn from(level: &Level) -> Self {
        match *level {
            Level::ERROR => Self::Error,
            Level::WARN => Self::Warn,
            Level::INFO => Self::Info,
            Level::DEBUG => Self::Debug,
            Level::TRACE => Self::Trace,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ! `pad`, not `write_str`: "{:<5}" lines the levels up in the report.
        f.pad(match self {
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(rename = "LogLineType")]
pub struct LogLine {
    pub id: u64,
    pub at: DateTime<Utc>,
    pub level: LogLevel,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub account: Option<String>,
    pub text: String,
}
