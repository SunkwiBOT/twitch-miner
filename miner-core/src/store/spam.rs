use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rusqlite::{params, OptionalExtension};

use super::{foreign_data, Store};
use crate::error::Result;
use crate::spam::{SpamBalance, SpamRow};

fn moment(millis: Option<i64>) -> Result<Option<DateTime<Utc>>> {
    millis
        .map(|millis| {
            DateTime::from_timestamp_millis(millis)
                .ok_or_else(|| foreign_data("a Spam Points date", "out of range"))
        })
        .transpose()
}

impl Store {
    pub fn spam_rows(&self) -> Result<HashMap<String, Vec<SpamRow>>> {
        let connection = self.lock();
        let mut statement = connection
            .prepare(
                "SELECT account_id, login, added_at, used_at, balance, balance_at FROM spam_channel",
            )
            .map_err(|error| foreign_data("the Spam Points table", error))?;
        let rows = statement.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                [row.get(2)?, row.get(3)?, row.get(5)?],
                row.get(4)?,
            ))
        })?;
        let mut by_account: HashMap<String, Vec<SpamRow>> = HashMap::new();

        for row in rows {
            let (account_id, login, [added_at, used_at, balance_at], balance) = row?;

            by_account.entry(account_id).or_default().push(SpamRow {
                login,
                added_at: moment(added_at)?,
                used_at: moment(used_at)?,
                balance,
                balance_at: moment(balance_at)?,
            });
        }

        Ok(by_account)
    }

    pub fn add_spam_channel(&self, account_id: &str, login: &str, at: DateTime<Utc>) -> Result<()> {
        self.lock().execute(
            "INSERT INTO spam_channel(account_id, login, added_at) VALUES(?1, ?2, ?3)
             ON CONFLICT(account_id, login) DO UPDATE
                 SET added_at = coalesce(spam_channel.added_at, excluded.added_at)",
            params![account_id, login, at.timestamp_millis()],
        )?;

        Ok(())
    }

    pub fn mark_spam_used(&self, account_id: &str, login: &str, at: DateTime<Utc>) -> Result<()> {
        self.lock().execute(
            "INSERT INTO spam_channel(account_id, login, used_at) VALUES(?1, ?2, ?3)
             ON CONFLICT(account_id, login) DO UPDATE SET used_at = excluded.used_at",
            params![account_id, login, at.timestamp_millis()],
        )?;

        Ok(())
    }

    pub fn record_spam_balance(
        &self,
        account_id: &str,
        login: &str,
        balance: i64,
        at: DateTime<Utc>,
        in_points: bool,
    ) -> Result<(SpamBalance, bool)> {
        let connection = self.lock();
        let kept: Option<(Option<i64>, Option<i64>)> = connection
            .query_row(
                "SELECT balance, balance_at FROM spam_channel WHERE account_id = ?1 AND login = ?2",
                params![account_id, login],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;

        if let Some((Some(kept), Some(changed_at))) = kept {
            if let Some(changed_at) = moment(Some(changed_at))?.filter(|_| kept == balance) {
                return Ok((
                    SpamBalance {
                        balance,
                        changed_at,
                    },
                    false,
                ));
            }
        }

        if kept.is_none() && !in_points {
            return Ok((
                SpamBalance {
                    balance,
                    changed_at: at,
                },
                false,
            ));
        }

        connection.execute(
            "INSERT INTO spam_channel(account_id, login, balance, balance_at) VALUES(?1, ?2, ?3, ?4)
             ON CONFLICT(account_id, login) DO UPDATE SET balance = excluded.balance,
                 balance_at = excluded.balance_at",
            params![account_id, login, balance, at.timestamp_millis()],
        )?;

        Ok((
            SpamBalance {
                balance,
                changed_at: at,
            },
            true,
        ))
    }

    pub fn forget_spam_channel(&self, account_id: &str, login: &str) -> Result<()> {
        self.lock().execute(
            "DELETE FROM spam_channel WHERE account_id = ?1 AND login = ?2",
            params![account_id, login],
        )?;

        Ok(())
    }
}
