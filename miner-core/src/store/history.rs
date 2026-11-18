use chrono::{DateTime, Utc};
use rusqlite::params;

use super::{foreign_data, json_error, AccountIdsParam, Store};
use crate::error::{Error, Result};
use crate::events::{EventCategory, EventContext, EventKind, MinerEvent};

const MIGRATED_STALE_MINER_FIELD_KEY: &str = "migrated_stale_miner_field";

// ! Only the conditions a page sets, so SQLite picks an index for them: a
// condition written `(?1 IS NULL OR ...)` makes it read the table row by row.
fn page_conditions(account_ids: Option<&AccountIdsParam>, before: bool, table: &str) -> String {
    let mut conditions = Vec::new();

    if let Some(param) = account_ids {
        conditions.push(param.condition(table));
    }

    if before {
        conditions.push(format!("{table}id < :before"));
    }

    conditions
        .iter()
        .map(|condition| format!(" AND {condition}"))
        .collect()
}

// ! One read per kind through the kind index, each cut at the page size: a
// plain `kind IN (...)` sorts every matching row. Kinds are fixed names,
// never user input: they are written into the SQL.
fn kinds_condition(
    filter: &HistoryFilter,
    account_ids: Option<&AccountIdsParam>,
    before: bool,
) -> String {
    if filter.category.is_none() && filter.kinds.is_none() {
        return "1".to_owned();
    }

    let conditions = page_conditions(account_ids, before, "");
    let branches: Vec<String> = EventKind::ALL
        .into_iter()
        .filter(|kind| filter.kinds.is_none_or(|kinds| kinds.contains(kind)))
        .filter_map(|kind| {
            // ! A Drops event of a badge keeps its miner, `BADGES`, in its
            // stored context; the other Drops events store none. A generic
            // Info or Error line carries no miner of its own either: its
            // category comes only from the miner tagged in its context.
            let drops_kind = kind.category(None) == EventCategory::Drops;
            let tags_miner = kind.follows_miner() || kind.generic();
            let miner = match filter.category {
                None => "",
                Some(EventCategory::Drops) if tags_miner => {
                    " AND json_extract(context, '$.miner') = 'DROPS'"
                }
                Some(EventCategory::Badges) if tags_miner || drops_kind => {
                    " AND json_extract(context, '$.miner') = 'BADGES'"
                }
                Some(EventCategory::Points) if tags_miner => {
                    " AND json_extract(context, '$.miner') = 'POINTS'"
                }
                Some(EventCategory::Other) if tags_miner => {
                    " AND json_extract(context, '$.miner') IS NULL"
                }
                Some(EventCategory::Drops) if drops_kind => {
                    " AND IFNULL(json_extract(context, '$.miner'), '') <> 'BADGES'"
                }
                Some(category) if kind.category(None) == category => "",
                Some(_) => return None,
            };

            Some(format!(
                "SELECT id FROM (SELECT id FROM history WHERE kind = '{}'{miner}{conditions}
                     ORDER BY id DESC LIMIT :limit)",
                kind.as_str()
            ))
        })
        .collect();

    if branches.is_empty() {
        return "0".to_owned();
    }

    format!("h.id IN ({})", branches.join(" UNION ALL "))
}

pub(super) fn history_sql(
    filter: &HistoryFilter,
    account_ids: Option<&AccountIdsParam>,
    before: bool,
) -> String {
    let kinds = kinds_condition(filter, account_ids, before);
    let conditions = page_conditions(account_ids, before, "h.");

    format!(
        "SELECT h.id, h.account_id, h.at, h.kind, h.title, h.detail, h.context,
                COALESCE(a.login, '')
         FROM history h LEFT JOIN account a ON a.id = h.account_id
         WHERE {kinds}{conditions}
         ORDER BY h.id DESC LIMIT :limit"
    )
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HistoryFilter<'a> {
    pub account_ids: Option<&'a [String]>,
    pub category: Option<EventCategory>,
    pub kinds: Option<&'a [EventKind]>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HistoryExportFilter<'a> {
    pub account_ids: Option<&'a [String]>,
}

// ! Always bounded by a date range: never the whole retention window.
#[derive(Debug, Clone, Copy)]
pub struct HistoryStatsFilter<'a> {
    pub account_ids: Option<&'a [String]>,
    pub since: DateTime<Utc>,
    pub until: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub struct HistoryCurveFilter<'a> {
    pub account_id: &'a str,
    pub channel: &'a str,
    pub since: DateTime<Utc>,
    pub until: DateTime<Utc>,
}

// ! Parsed outside the SQLite callback: that callback can only fail the whole
// page, and one broken entry must cost only itself.
struct HistoryRow {
    id: i64,
    account_id: String,
    account_login: String,
    at: i64,
    kind: String,
    title: String,
    detail: Option<String>,
    context: String,
}

impl HistoryRow {
    fn event(self) -> Result<MinerEvent> {
        let kind = EventKind::from_name(&self.kind)
            .ok_or_else(|| foreign_data("a history kind", "unknown name"))?;
        let context: EventContext = serde_json::from_str(&self.context)
            .map_err(|error| foreign_data("a history context", json_error(&error)))?;
        let at = DateTime::from_timestamp_millis(self.at)
            .ok_or_else(|| foreign_data("a history date", "out of range"))?;
        let id =
            u64::try_from(self.id).map_err(|_| foreign_data("a history id", "out of range"))?;
        let mut event = MinerEvent {
            id,
            account_id: self.account_id,
            account_login: self.account_login,
            at,
            kind,
            category: EventCategory::default(),
            title: self.title,
            detail: self.detail,
            context,
        };

        event.restore_miner();
        event.categorize();

        Ok(event)
    }
}

impl Store {
    pub fn push_history(&self, event: &MinerEvent) -> Result<u64> {
        self.insert_history(
            event,
            "INSERT INTO history(account_id, at, kind, title, detail, context) VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
        )?
        .ok_or_else(|| Error::Storage("history line not written".to_owned()))
    }

    // ! Checked and written in one statement: a line of an account removed
    // meanwhile is dropped instead of outliving it.
    pub fn push_account_history(&self, event: &MinerEvent) -> Result<Option<u64>> {
        self.insert_history(
            event,
            "INSERT INTO history(account_id, at, kind, title, detail, context)
             SELECT ?1, ?2, ?3, ?4, ?5, ?6 WHERE EXISTS (SELECT 1 FROM account WHERE id = ?1)",
        )
    }

    fn insert_history(&self, event: &MinerEvent, sql: &str) -> Result<Option<u64>> {
        let stored = event.stored();
        let connection = self.lock();
        let written = connection.execute(
            sql,
            params![
                stored.account_id,
                stored.at.timestamp_millis(),
                stored.kind.as_str(),
                stored.title,
                stored.detail,
                serde_json::to_string(&stored.context)?,
            ],
        )?;

        if written == 0 {
            return Ok(None);
        }

        u64::try_from(connection.last_insert_rowid())
            .map(Some)
            .map_err(|_| Error::Storage("history id out of range".to_owned()))
    }

    pub fn history(
        &self,
        filter: &HistoryFilter,
        before_id: Option<u64>,
        limit: usize,
    ) -> Result<(Vec<MinerEvent>, Option<u64>)> {
        let connection = self.lock();
        let before = before_id.map(|id| i64::try_from(id).unwrap_or(i64::MAX));
        let account_param = filter.account_ids.map(AccountIdsParam::new);
        let mut statement = connection
            .prepare(&history_sql(
                filter,
                account_param.as_ref(),
                before.is_some(),
            ))
            .map_err(|error| foreign_data("the history table", error))?;
        let fetched = (limit + 1) as i64;
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> = vec![(":limit", &fetched)];

        if let Some(param) = &account_param {
            param.bind(&mut named);
        }

        if let Some(before) = &before {
            named.push((":before", before));
        }

        let mut rows: Vec<HistoryRow> = statement
            .query_map(named.as_slice(), |row| {
                Ok(HistoryRow {
                    id: row.get(0)?,
                    account_id: row.get(1)?,
                    at: row.get(2)?,
                    kind: row.get(3)?,
                    title: row.get(4)?,
                    detail: row.get(5)?,
                    context: row.get(6)?,
                    account_login: row.get(7)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;

        // ! The page and its cursor follow the stored rows, so a skipped row
        // never ends the journal early.
        let next = if rows.len() > limit {
            rows.truncate(limit);
            rows.last().and_then(|row| u64::try_from(row.id).ok())
        } else {
            None
        };

        let events = rows
            .into_iter()
            .filter_map(|row| {
                let id = row.id;

                row.event()
                    .inspect_err(|error| tracing::warn!(id, "history entry skipped: {error}"))
                    .ok()
            })
            .collect();

        Ok((events, next))
    }

    // ! For the claimed-drops CSV export: one claim per row, most recent
    // first, over the whole account (or account filter) and period asked.
    pub fn claimed_drops(
        &self,
        filter: &HistoryExportFilter,
        limit: usize,
    ) -> Result<Vec<MinerEvent>> {
        let connection = self.lock();
        let account_param = filter.account_ids.map(AccountIdsParam::new);
        let mut conditions = vec!["h.kind = 'DROP_CLAIMED'".to_owned()];

        if let Some(param) = &account_param {
            conditions.push(param.condition("h."));
        }

        let sql = format!(
            "SELECT h.id, h.account_id, h.at, h.kind, h.title, h.detail, h.context,
                    COALESCE(a.login, '')
             FROM history h LEFT JOIN account a ON a.id = h.account_id
             WHERE {}
             ORDER BY h.at DESC, h.id DESC LIMIT :limit",
            conditions.join(" AND ")
        );
        let mut statement = connection
            .prepare(&sql)
            .map_err(|error| foreign_data("the history table", error))?;
        let limit = limit as i64;
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> = vec![(":limit", &limit)];

        if let Some(param) = &account_param {
            param.bind(&mut named);
        }

        let rows: Vec<HistoryRow> = statement
            .query_map(named.as_slice(), |row| {
                Ok(HistoryRow {
                    id: row.get(0)?,
                    account_id: row.get(1)?,
                    at: row.get(2)?,
                    kind: row.get(3)?,
                    title: row.get(4)?,
                    detail: row.get(5)?,
                    context: row.get(6)?,
                    account_login: row.get(7)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                let id = row.id;

                row.event()
                    .inspect_err(|error| tracing::warn!(id, "history entry skipped: {error}"))
                    .ok()
            })
            .collect())
    }
    // ! Unlimited, unlike `claimed_drops`: the category breakdown and the
    // truncation flag must reflect every matching claim, not just the page
    // capped for the CSV body.
    pub fn claimed_drops_counts(
        &self,
        filter: &HistoryExportFilter,
    ) -> Result<(Vec<(String, u32)>, u64)> {
        let connection = self.lock();
        let account_param = filter.account_ids.map(AccountIdsParam::new);
        let mut conditions = vec!["h.kind = 'DROP_CLAIMED'".to_owned()];

        if let Some(param) = &account_param {
            conditions.push(param.condition("h."));
        }

        let sql = format!(
            "SELECT COALESCE(json_extract(h.context, '$.game'), ''), COUNT(*)
             FROM history h
             WHERE {}
             GROUP BY 1",
            conditions.join(" AND ")
        );
        let mut statement = connection
            .prepare(&sql)
            .map_err(|error| foreign_data("the history table", error))?;
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> = vec![];

        if let Some(param) = &account_param {
            param.bind(&mut named);
        }

        let counts: Vec<(String, u32)> = statement
            .query_map(named.as_slice(), |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;
        let total = counts.iter().map(|(_, count)| u64::from(*count)).sum();

        Ok((counts, total))
    }

    // ! One GROUP BY per metric over the caller's own range, not the
    // account's full retention; `POINTS_EARNED` is the single kind summed
    // (it fires for every gain, chests and bet payouts included, so none
    // is counted twice). Bucketed in Rust against the caller's sorted
    // `day_boundaries` (`partition_point`) rather than a SQL `GROUP BY`,
    // since those cut points are a caller-supplied, non-uniform list.
    pub fn points_by_day(
        &self,
        filter: &HistoryStatsFilter,
        day_boundaries: &[DateTime<Utc>],
    ) -> Result<Vec<(DateTime<Utc>, String, i64)>> {
        if day_boundaries.is_empty() {
            return Ok(Vec::new());
        }

        let connection = self.lock();
        let account_param = filter.account_ids.map(AccountIdsParam::new);
        let mut conditions = vec![
            "h.kind = 'POINTS_EARNED'".to_owned(),
            "h.at >= :since".to_owned(),
            "h.at < :until".to_owned(),
        ];

        if let Some(param) = &account_param {
            conditions.push(param.condition("h."));
        }

        let sql = format!(
            "SELECT h.at,
                    COALESCE(json_extract(h.context, '$.reason'), '') AS reason,
                    COALESCE(json_extract(h.context, '$.earned'), 0) AS points
             FROM history h
             WHERE {}",
            conditions.join(" AND ")
        );
        let mut statement = connection
            .prepare(&sql)
            .map_err(|error| foreign_data("the history table", error))?;
        let since = filter.since.timestamp_millis();
        let until = filter.until.timestamp_millis();
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> =
            vec![(":since", &since), (":until", &until)];

        if let Some(param) = &account_param {
            param.bind(&mut named);
        }

        let rows: Vec<(i64, String, i64)> = statement
            .query_map(named.as_slice(), |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })?
            .collect::<rusqlite::Result<_>>()?;

        let mut boundary_millis: Vec<i64> = day_boundaries
            .iter()
            .map(DateTime::timestamp_millis)
            .collect();

        // ! Backstop: the caller (`manager::history_stats`) already floors
        // every boundary to `since`, but a boundary below it would otherwise
        // drop every row up to the real first boundary.
        if let Some(first) = boundary_millis.first_mut() {
            *first = (*first).max(since);
        }
        let mut totals: std::collections::BTreeMap<(i64, String), i64> =
            std::collections::BTreeMap::new();

        for (at, reason, points) in rows {
            let index = boundary_millis.partition_point(|boundary| *boundary <= at);

            // ! Before the first boundary: outside every day the caller asked for.
            if index == 0 {
                continue;
            }

            *totals
                .entry((boundary_millis[index - 1], reason))
                .or_insert(0) += points;
        }

        Ok(totals
            .into_iter()
            .map(|((at, reason), points)| {
                (
                    DateTime::from_timestamp_millis(at).unwrap_or_default(),
                    reason,
                    points,
                )
            })
            .collect())
    }

    // ! `None` for a claim stored before the campaign's category was known,
    // or without one in the drop's own data: the UI labels it, not a
    // server-chosen placeholder string merged into a real one.
    pub fn drops_by_category(
        &self,
        filter: &HistoryStatsFilter,
    ) -> Result<Vec<(Option<String>, u32)>> {
        let connection = self.lock();
        let account_param = filter.account_ids.map(AccountIdsParam::new);
        let mut conditions = vec![
            "h.kind = 'DROP_CLAIMED'".to_owned(),
            "h.at >= :since".to_owned(),
            "h.at < :until".to_owned(),
        ];

        if let Some(param) = &account_param {
            conditions.push(param.condition("h."));
        }

        let sql = format!(
            "SELECT NULLIF(json_extract(h.context, '$.game'), ''), COUNT(*)
             FROM history h
             WHERE {}
             GROUP BY 1
             ORDER BY 2 DESC",
            conditions.join(" AND ")
        );
        let mut statement = connection
            .prepare(&sql)
            .map_err(|error| foreign_data("the history table", error))?;
        let since = filter.since.timestamp_millis();
        let until = filter.until.timestamp_millis();
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> =
            vec![(":since", &since), (":until", &until)];

        if let Some(param) = &account_param {
            param.bind(&mut named);
        }

        let rows: Vec<(Option<String>, u32)> = statement
            .query_map(named.as_slice(), |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;

        Ok(rows)
    }

    // ! `POINTS_EARNED` is the single source for earned points (see
    // `points_by_day`); a chest or a resolved bet already carries its
    // channel there, so no other kind is needed to attribute a streamer's
    // total.
    pub fn streamer_points(
        &self,
        filter: &HistoryStatsFilter,
        limit: usize,
    ) -> Result<Vec<(String, i64)>> {
        let connection = self.lock();
        let account_param = filter.account_ids.map(AccountIdsParam::new);
        let mut conditions = vec![
            "h.kind = 'POINTS_EARNED'".to_owned(),
            "h.at >= :since".to_owned(),
            "h.at < :until".to_owned(),
            "json_extract(h.context, '$.channel') IS NOT NULL".to_owned(),
        ];

        if let Some(param) = &account_param {
            conditions.push(param.condition("h."));
        }

        let sql = format!(
            "SELECT json_extract(h.context, '$.channel'),
                    SUM(COALESCE(json_extract(h.context, '$.earned'), 0))
             FROM history h
             WHERE {}
             GROUP BY 1
             ORDER BY 2 DESC
             LIMIT :limit",
            conditions.join(" AND ")
        );
        let mut statement = connection
            .prepare(&sql)
            .map_err(|error| foreign_data("the history table", error))?;
        let since = filter.since.timestamp_millis();
        let until = filter.until.timestamp_millis();
        let limit = limit as i64;
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> =
            vec![(":since", &since), (":until", &until), (":limit", &limit)];

        if let Some(param) = &account_param {
            param.bind(&mut named);
        }

        let rows: Vec<(String, i64)> = statement
            .query_map(named.as_slice(), |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;

        Ok(rows)
    }

    pub fn streamer_predictions_won(
        &self,
        filter: &HistoryStatsFilter,
    ) -> Result<Vec<(String, u32)>> {
        let connection = self.lock();
        let account_param = filter.account_ids.map(AccountIdsParam::new);
        let mut conditions = vec![
            "h.kind = 'PREDICTION_RESULT'".to_owned(),
            "json_extract(h.context, '$.bet_result') = 'WIN'".to_owned(),
            "json_extract(h.context, '$.channel') IS NOT NULL".to_owned(),
            "h.at >= :since".to_owned(),
            "h.at < :until".to_owned(),
        ];

        if let Some(param) = &account_param {
            conditions.push(param.condition("h."));
        }

        let sql = format!(
            "SELECT json_extract(h.context, '$.channel'), COUNT(*)
             FROM history h
             WHERE {}
             GROUP BY 1
             ORDER BY 2 DESC",
            conditions.join(" AND ")
        );
        let mut statement = connection
            .prepare(&sql)
            .map_err(|error| foreign_data("the history table", error))?;
        let since = filter.since.timestamp_millis();
        let until = filter.until.timestamp_millis();
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> =
            vec![(":since", &since), (":until", &until)];

        if let Some(param) = &account_param {
            param.bind(&mut named);
        }

        let rows: Vec<(String, u32)> = statement
            .query_map(named.as_slice(), |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?;

        Ok(rows)
    }

    pub fn predictions_won_total(&self, filter: &HistoryStatsFilter) -> Result<u32> {
        let connection = self.lock();
        let account_param = filter.account_ids.map(AccountIdsParam::new);
        let mut conditions = vec![
            "h.kind = 'PREDICTION_RESULT'".to_owned(),
            "json_extract(h.context, '$.bet_result') = 'WIN'".to_owned(),
            "h.at >= :since".to_owned(),
            "h.at < :until".to_owned(),
        ];

        if let Some(param) = &account_param {
            conditions.push(param.condition("h."));
        }

        let sql = format!(
            "SELECT COUNT(*) FROM history h WHERE {}",
            conditions.join(" AND ")
        );
        let mut statement = connection
            .prepare(&sql)
            .map_err(|error| foreign_data("the history table", error))?;
        let since = filter.since.timestamp_millis();
        let until = filter.until.timestamp_millis();
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> =
            vec![(":since", &since), (":until", &until)];

        if let Some(param) = &account_param {
            param.bind(&mut named);
        }

        Ok(statement.query_row(named.as_slice(), |row| row.get(0))?)
    }

    // ! Paired in order, not guessed: the account's own `WatchStarted`/
    // `WatchStopped` timestamps give a real duration, something no single
    // event carries. A session still running (no stop at all) contributes
    // nothing, rather than estimating one; a session that straddles `since`
    // (started earlier, stopped inside the window) counts only its known
    // part inside `[since, until)` — the start row is fetched regardless of
    // `since` so the pairing survives the clamp.
    pub fn watch_minutes_by_streamer(
        &self,
        filter: &HistoryStatsFilter,
    ) -> Result<Vec<(String, i64)>> {
        let connection = self.lock();
        let account_param = filter.account_ids.map(AccountIdsParam::new);
        let mut conditions = vec![
            "h.kind IN ('WATCH_STARTED', 'WATCH_STOPPED')".to_owned(),
            "h.at < :until".to_owned(),
            "json_extract(h.context, '$.channel') IS NOT NULL".to_owned(),
        ];

        if let Some(param) = &account_param {
            conditions.push(param.condition("h."));
        }

        // ! Never the whole retention window: only rows inside `[since, until)`,
        // plus, per account and channel, the single most recent `WATCH_STARTED`
        // before `since` (the one a straddling session would pair against).
        let sql = format!(
            "SELECT account_id, channel, kind, at FROM (
                 SELECT h.account_id AS account_id,
                        json_extract(h.context, '$.channel') AS channel,
                        h.kind AS kind,
                        h.at AS at,
                        h.id AS id,
                        CASE
                            WHEN h.kind = 'WATCH_STARTED' AND h.at < :since
                            THEN ROW_NUMBER() OVER (
                                PARTITION BY h.account_id, json_extract(h.context, '$.channel'),
                                             h.kind = 'WATCH_STARTED' AND h.at < :since
                                ORDER BY h.id DESC
                            )
                            ELSE NULL
                        END AS rn
                 FROM history h
                 WHERE {}
             )
             WHERE at >= :since OR rn = 1
             ORDER BY account_id, channel, id ASC",
            conditions.join(" AND ")
        );
        let mut statement = connection
            .prepare(&sql)
            .map_err(|error| foreign_data("the history table", error))?;
        let since = filter.since.timestamp_millis();
        let until = filter.until.timestamp_millis();
        let mut named: Vec<(&str, &dyn rusqlite::ToSql)> =
            vec![(":since", &since), (":until", &until)];

        if let Some(param) = &account_param {
            param.bind(&mut named);
        }

        let rows: Vec<(String, String, String, i64)> = statement
            .query_map(named.as_slice(), |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })?
            .collect::<rusqlite::Result<_>>()?;

        let mut minutes: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        let mut open: Option<(String, String, i64)> = None;

        for (account_id, login, kind, at) in rows {
            match kind.as_str() {
                "WATCH_STARTED" => open = Some((account_id, login, at)),
                "WATCH_STOPPED" => {
                    if let Some((started_account, started_login, started_at)) = open.take() {
                        if started_account == account_id && started_login == login && at > since {
                            let clipped_start = started_at.max(since);

                            if at > clipped_start {
                                *minutes.entry(login).or_insert(0) += (at - clipped_start) / 60_000;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let mut rows: Vec<(String, i64)> = minutes.into_iter().collect();
        rows.sort_by_key(|row| std::cmp::Reverse(row.1));

        Ok(rows)
    }

    // ! One account, one streamer: every balance-affecting or annotated kind
    // the curve plots, in order, reusing `MinerEvent` parsing as-is.
    pub fn balance_curve(&self, filter: &HistoryCurveFilter) -> Result<Vec<MinerEvent>> {
        let connection = self.lock();
        let sql = "SELECT h.id, h.account_id, h.at, h.kind, h.title, h.detail, h.context,
                          COALESCE(a.login, '')
                   FROM history h LEFT JOIN account a ON a.id = h.account_id
                   WHERE h.account_id = :account_id
                     AND json_extract(h.context, '$.channel') = :channel
                     AND h.kind IN (
                       'POINTS_EARNED', 'POINTS_SPENT', 'BONUS_CLAIMED', 'RAID_JOINED',
                       'MOMENT_CLAIMED', 'STREAK_RECOVERED', 'PREDICTION_PLACED',
                       'PREDICTION_RESULT'
                     )
                     AND h.at >= :since AND h.at < :until
                   ORDER BY h.at ASC, h.id ASC";
        let mut statement = connection
            .prepare(sql)
            .map_err(|error| foreign_data("the history table", error))?;
        let since = filter.since.timestamp_millis();
        let until = filter.until.timestamp_millis();
        let named: Vec<(&str, &dyn rusqlite::ToSql)> = vec![
            (":account_id", &filter.account_id),
            (":channel", &filter.channel),
            (":since", &since),
            (":until", &until),
        ];

        let rows: Vec<HistoryRow> = statement
            .query_map(named.as_slice(), |row| {
                Ok(HistoryRow {
                    id: row.get(0)?,
                    account_id: row.get(1)?,
                    at: row.get(2)?,
                    kind: row.get(3)?,
                    title: row.get(4)?,
                    detail: row.get(5)?,
                    context: row.get(6)?,
                    account_login: row.get(7)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                let id = row.id;

                row.event()
                    .inspect_err(|error| tracing::warn!(id, "history entry skipped: {error}"))
                    .ok()
            })
            .collect())
    }

    pub fn prune_history(&self, cutoff: DateTime<Utc>) -> Result<usize> {
        self.delete_in_batches(
            "DELETE FROM history WHERE id IN (SELECT id FROM history WHERE at < ?1 LIMIT ?2)",
            &cutoff.timestamp_millis(),
        )
    }

    // ! A stale `$.miner` tag can misfile a generic Info/Error row under the
    // wrong tab filter until it is stripped. Idempotent: clearing an absent
    // field is a no-op, so running this again, or being interrupted before
    // the flag below is saved, changes nothing.
    pub(super) fn clear_stale_miner_field(&self) -> Result<()> {
        if self.get_setting(MIGRATED_STALE_MINER_FIELD_KEY)?.is_some() {
            return Ok(());
        }

        self.lock().execute(
            "UPDATE history SET context = json_remove(context, '$.miner')
             WHERE kind IN ('INFO', 'ERROR') AND json_extract(context, '$.miner') IS NOT NULL",
            [],
        )?;

        self.set_setting(MIGRATED_STALE_MINER_FIELD_KEY, "1")?;

        Ok(())
    }
}
