use std::time::Instant;

use reqwest::StatusCode;
use serde_json::Value;

use super::log::Done;
use super::{RequestKind, RequestLog, RequestOutcome, SocketEvent};
use crate::error::{Error, NetworkErrorKind, Result};

fn millis(started: Instant) -> u64 {
    u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)
}

fn graphql_error(body: &Value) -> Option<String> {
    let first = |item: &Value| {
        item.get("errors")
            .and_then(Value::as_array)
            .and_then(|errors| errors.first())
            .map(|error| {
                error
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned()
            })
    };

    match body {
        Value::Array(items) => items.iter().find_map(first),
        _ => first(body),
    }
}

#[must_use = "a request is recorded only once it is finished"]
pub struct Pending {
    log: RequestLog,
    account_id: Option<String>,
    kind: RequestKind,
    target: String,
    socket: Option<SocketEvent>,
    expected: &'static [StatusCode],
    started: Instant,
}

impl Pending {
    pub(super) fn new(log: RequestLog, kind: RequestKind, target: String) -> Self {
        Self {
            log,
            account_id: None,
            kind,
            target,
            socket: None,
            expected: &[],
            started: Instant::now(),
        }
    }

    pub fn account(mut self, account_id: Option<&str>) -> Self {
        self.account_id = account_id.map(str::to_owned);

        self
    }

    pub fn socket(mut self, event: SocketEvent) -> Self {
        self.socket = Some(event);

        self
    }

    pub fn expecting(mut self, statuses: &'static [StatusCode]) -> Self {
        self.expected = statuses;

        self
    }

    fn finish(self, outcome: RequestOutcome, status: Option<u16>, detail: Option<String>) {
        let duration_ms = millis(self.started);

        self.log.record(Done {
            account_id: self.account_id,
            kind: self.kind,
            target: self.target,
            socket: self.socket,
            outcome,
            status,
            detail,
            duration_ms,
        });
    }

    pub fn succeeded(self, status: Option<StatusCode>) {
        self.finish(
            RequestOutcome::Ok,
            status.map(|status| status.as_u16()),
            None,
        );
    }

    pub fn answered(self, status: StatusCode) {
        let outcome = if status.is_success() || self.expected.contains(&status) {
            RequestOutcome::Ok
        } else {
            RequestOutcome::Status
        };

        self.finish(outcome, Some(status.as_u16()), None);
    }

    // ! Twitch adds an error next to the data when it could not read one field: only one failing the call counts.
    pub fn graphql(self, status: StatusCode, body: &Value, failed: bool) {
        let outcome = if failed {
            RequestOutcome::Graphql
        } else {
            RequestOutcome::Ok
        };

        self.finish(outcome, Some(status.as_u16()), graphql_error(body));
    }

    pub fn closed(self, reason: &str) {
        self.finish(RequestOutcome::Closed, None, Some(reason.to_owned()));
    }

    pub fn failed(self, error: &Error) {
        match error {
            Error::Status(status) => self.answered(*status),
            Error::Unauthorized => self.answered(StatusCode::UNAUTHORIZED),
            Error::Transport(NetworkErrorKind::Timeout) => {
                self.finish(RequestOutcome::Timeout, None, None)
            }
            Error::Transport(NetworkErrorKind::Connect) => {
                self.finish(RequestOutcome::Connect, None, None)
            }
            Error::Transport(NetworkErrorKind::Other) => {
                self.finish(RequestOutcome::Network, None, None)
            }
            Error::Gql { code, .. } => {
                self.finish(RequestOutcome::Graphql, None, Some(code.clone()))
            }
            other => self.finish(RequestOutcome::Network, None, Some(other.to_string())),
        }
    }

    pub fn sent(self, sent: &reqwest::Result<reqwest::Response>) {
        match sent {
            Ok(response) => self.answered(response.status()),
            Err(error) => match error.status() {
                Some(status) => self.answered(status),
                None if error.is_timeout() => self.finish(RequestOutcome::Timeout, None, None),
                None if error.is_connect() => self.finish(RequestOutcome::Connect, None, None),
                None => self.finish(RequestOutcome::Network, None, None),
            },
        }
    }

    pub async fn send(self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let sent = request.send().await;

        self.sent(&sent);

        Ok(sent?)
    }
}
