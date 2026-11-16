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

    pub fn prune_history(&self, cutoff: DateTime<Utc>) -> Result<usize> {
        self.delete_in_batches(
            "DELETE FROM history WHERE id IN (SELECT id FROM history WHERE at < ?1 LIMIT ?2)",
            &cutoff.timestamp_millis(),
        )
    }

    // ! A slot fallback used to tag generic Info/Error rows with the slot's
    // miner kind; the tab filter now trusts that field, so rows written
    // before this fix stay misfiled until it is stripped once. Idempotent:
    // clearing an absent `$.miner` is a no-op, so running this again, or
    // being interrupted before the flag below is saved, changes nothing.
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
