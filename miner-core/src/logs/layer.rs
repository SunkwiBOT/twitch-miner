use chrono::Utc;
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

use super::{redact, LogBuffer, LogLevel, LogLine};

const OWN_TARGETS: [&str; 3] = ["miner_core", "twitch_miner", "twitchminer"];

#[derive(Default)]
struct Collector {
    message: String,
    account: Option<String>,
}

impl Visit for Collector {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        match field.name() {
            "message" => self.message = format!("{value:?}"),
            "account" => self.account = Some(format!("{value:?}").trim_matches('"').to_owned()),
            name => {
                if !self.message.is_empty() {
                    self.message.push(' ');
                }

                self.message.push_str(&format!("{name}={value:?}"));
            }
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "message" => self.message = value.to_owned(),
            "account" => self.account = Some(value.to_owned()),
            name => {
                if !self.message.is_empty() {
                    self.message.push(' ');
                }

                self.message.push_str(&format!("{name}={value}"));
            }
        }
    }
}

pub struct LogLayer {
    pub buffer: LogBuffer,
}

impl<S: Subscriber> Layer<S> for LogLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let metadata = event.metadata();

        if *metadata.level() > Level::INFO {
            return;
        }

        let target = metadata.target();
        let own = OWN_TARGETS.iter().any(|crate_name| {
            target
                .strip_prefix(crate_name)
                .is_some_and(|rest| rest.is_empty() || rest.starts_with("::"))
        });

        if !own {
            return;
        }

        let mut collector = Collector::default();
        event.record(&mut collector);

        self.buffer.push(LogLine {
            id: 0,
            at: Utc::now(),
            level: LogLevel::from(metadata.level()),
            target: target.rsplit("::").next().unwrap_or(target).to_owned(),
            account: collector.account,
            text: redact(&collector.message).into_owned(),
        });
    }
}
