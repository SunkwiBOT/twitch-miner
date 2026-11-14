use chrono::{DateTime, Utc};
use rusqlite::params;

use super::{foreign_data, Store};
use crate::error::Result;
use crate::requests::{
    bucket_end, RequestEntry, RequestFilter, RequestGroup, RequestKind, RequestOutcome,
    RequestSource, SocketEvent,
};

fn requests_sql(filter: &RequestFilter, before: bool) -> String {
    let mut conditions = vec!["(at < :until OR id <= :earlier_run)".to_owned()];

    if before {
        conditions.push("id < :before".to_owned());
    }

    if filter.account_id.is_some() {
        conditions.push("account_id = :account".to_owned());
    }

    if let Some(kinds) = filter.kinds {
        let names: Vec<String> = kinds
            .iter()
            .map(|kind| format!("'{}'", kind.as_str()))
            .collect();

        conditions.push(if names.is_empty() {
            "0".to_owned()
        } else {
            format!("kind IN ({})", names.join(", "))
        });
    }

    if filter.errors_only {
        conditions.push(format!("outcome <> '{}'", RequestOutcome::Ok.as_str()));
    }

    format!(
        "SELECT id, account_id, at, kind, target, socket, outcome, status, detail, duration,
                count, longest
         FROM request WHERE {}
         ORDER BY id DESC LIMIT :limit",
        conditions.join(" AND ")
    )
}

struct RequestRow {
    id: i64,
    account_id: Option<String>,
    at: i64,
    kind: String,
    target: String,
    socket: Option<String>,
    outcome: String,
    status: Option<i64>,
    detail: Option<String>,
    duration: i64,
    count: Option<i64>,
    longest: Option<i64>,
}

impl RequestRow {
    fn entry(self) -> Result<RequestEntry> {
        let unsigned = |value: i64, what: &str| {
            u64::try_from(value).map_err(|_| foreign_data(what, "out of range"))
        };
        let kind = RequestKind::from_name(&self.kind)
            .ok_or_else(|| foreign_data("a request kind", "unknown name"))?;
        let outcome = RequestOutcome::from_name(&self.outcome)
            .ok_or_else(|| foreign_data("a request outcome", "unknown name"))?;
        let socket = match self.socket.as_deref() {
            Some(name) => Some(
                SocketEvent::from_name(name)
                    .ok_or_else(|| foreign_data("a request socket event", "unknown name"))?,
            ),
            None => None,
        };
        let at = DateTime::from_timestamp_millis(self.at)
            .ok_or_else(|| foreign_data("a request date", "out of range"))?;
        let status = match self.status {
            Some(status) => Some(
                u16::try_from(status)
                    .map_err(|_| foreign_data("a request status", "out of range"))?,
            ),
            None => None,
        };
        let group = match (self.count, self.longest) {
            (Some(count), Some(longest)) => Some(RequestGroup {
                count: u32::try_from(count)
                    .map_err(|_| foreign_data("a request count", "out of range"))?,
                max_ms: unsigned(longest, "a request duration")?,
                until: bucket_end(at),
            }),
            _ => None,
        };

        Ok(RequestEntry {
            id: unsigned(self.id, "a request id")?,
            source: RequestSource::Stored,
            at,
            account_id: self.account_id,
            kind,
            target: self.target,
            socket,
            outcome,
            status,
            detail: self.detail,
            duration_ms: unsigned(self.duration, "a request duration")?,
            group,
        })
    }
}

impl Store {
    pub fn push_requests(&self, rows: &[RequestEntry]) -> Result<()> {
        let mut connection = self.lock();
        let transaction = connection.transaction()?;

        {
            let mut insert = transaction.prepare_cached(
                "INSERT INTO request(account_id, at, kind, target, socket, outcome, status, detail,
                     duration, count, longest)
                 VALUES((SELECT id FROM account WHERE id = ?1), ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                     ?11)",
            )?;

            for row in rows {
                insert.execute(params![
                    row.account_id,
                    row.at.timestamp_millis(),
                    row.kind.as_str(),
                    row.target,
                    row.socket.map(SocketEvent::as_str),
                    row.outcome.as_str(),
                    row.status,
                    row.detail,
                    i64::try_from(row.duration_ms).unwrap_or(i64::MAX),
                    row.group.as_ref().map(|group| group.count),
                    row.group
                        .as_ref()
                        .map(|group| i64::try_from(group.max_ms).unwrap_or(i64::MAX)),
                ])?;
            }
        }

        transaction.commit()?;

        Ok(())
    }

    pub fn last_request_id(&self) -> Result<u64> {
        let last: i64 =
            self.lock()
                .query_row("SELECT COALESCE(MAX(id), 0) FROM request", [], |row| {
                    row.get(0)
                })?;

        Ok(u64::try_from(last).unwrap_or(0))
    }

    pub fn requests(
        &self,
        filter: &RequestFilter,
        until: DateTime<Utc>,
        earlier_run: u64,
        before_id: Option<u64>,
        limit: usize,
    ) -> Result<(Vec<RequestEntry>, Option<u64>)> {
        let connection = self.lock();
        let before = before_id.map(|id| i64::try_from(id).unwrap_or(i64::MAX));
        let mut statement = connection
            .prepare(&requests_sql(filter, before.is_some()))
            .map_err(|error| foreign_data("the request table", error))?;
        let fetched = (limit + 1) as i64;
        let until = until.timestamp_millis();
        let earlier_run = i64::try_from(earlier_run).unwrap_or(i64::MAX);
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> = vec![
            (":limit", &fetched),
            (":until", &until),
            (":earlier_run", &earlier_run),
        ];

        if let Some(account_id) = &filter.account_id {
            named.push((":account", account_id));
        }

        if let Some(before) = &before {
            named.push((":before", before));
        }

        let mut rows: Vec<RequestRow> = statement
            .query_map(named.as_slice(), |row| {
                Ok(RequestRow {
                    id: row.get(0)?,
                    account_id: row.get(1)?,
                    at: row.get(2)?,
                    kind: row.get(3)?,
                    target: row.get(4)?,
                    socket: row.get(5)?,
                    outcome: row.get(6)?,
                    status: row.get(7)?,
                    detail: row.get(8)?,
                    duration: row.get(9)?,
                    count: row.get(10)?,
                    longest: row.get(11)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;

        let next = if rows.len() > limit {
            rows.truncate(limit);
            rows.last().and_then(|row| u64::try_from(row.id).ok())
        } else {
            None
        };

        let entries = rows
            .into_iter()
            .filter_map(|row| {
                let id = row.id;

                row.entry()
                    .inspect_err(|error| tracing::warn!(id, "request entry skipped: {error}"))
                    .ok()
            })
            .collect();

        Ok((entries, next))
    }

    pub fn prune_requests(&self, cutoff: DateTime<Utc>) -> Result<usize> {
        self.delete_in_batches(
            "DELETE FROM request WHERE id IN (SELECT id FROM request WHERE at < ?1 LIMIT ?2)",
            &cutoff.timestamp_millis(),
        )
    }
}
