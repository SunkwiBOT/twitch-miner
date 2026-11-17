use std::collections::HashMap;
use std::fmt;

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use zeroize::Zeroizing;

use super::vault::{Sealed, Vault};
use super::{foreign_data, json_error, Store};
use crate::config::AccountSettings;
use crate::error::{Error, Result};
use crate::twitch::auth::TokenSet;

const MASKED: &str = "[redacted]";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum AccountSource {
    #[default]
    DeviceCode,
    ImportedToken,
}

impl AccountSource {
    // ! Round-tripped through serde (the same `SCREAMING_SNAKE_CASE`
    // rename as the type derives), so the stored string can never drift
    // from the variant names declared on the enum.
    fn as_str(self) -> String {
        match serde_json::to_value(self) {
            Ok(serde_json::Value::String(value)) => value,
            _ => unreachable!("AccountSource serializes to a string"),
        }
    }

    fn parse(value: &str) -> Self {
        serde_json::from_value(serde_json::Value::String(value.to_owned())).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRecord {
    pub id: String,
    pub login: String,
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub settings: AccountSettings,
    pub position: i64,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub source: AccountSource,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Credential {
    pub tokens: TokenSet,
    pub device_id: String,
}

// ! The tokens stay out of `{:?}`: the log redaction only catches what looks
// like a key.
impl fmt::Debug for Credential {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Credential")
            .field("tokens", &MASKED)
            .field("device_id", &self.device_id)
            .finish()
    }
}

struct AccountRow {
    id: String,
    login: String,
    display_name: String,
    avatar_url: Option<String>,
    settings: String,
    position: i64,
    created_at: i64,
}

impl AccountRow {
    fn read(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            login: row.get(1)?,
            display_name: row.get(2)?,
            avatar_url: row.get(3)?,
            settings: row.get(4)?,
            position: row.get(5)?,
            created_at: row.get(6)?,
        })
    }

    fn record(self, source: AccountSource) -> Result<AccountRecord> {
        let settings = serde_json::from_str(&self.settings)
            .map_err(|error| foreign_data("the settings of an account", json_error(&error)))?;
        let created_at = DateTime::from_timestamp(self.created_at, 0)
            .ok_or_else(|| foreign_data("the creation date of an account", "out of range"))?;

        Ok(AccountRecord {
            id: self.id,
            login: self.login,
            display_name: self.display_name,
            avatar_url: self.avatar_url,
            settings,
            position: self.position,
            created_at,
            source,
        })
    }
}

fn account_sources(connection: &Connection) -> Result<HashMap<String, AccountSource>> {
    let mut statement = connection
        .prepare("SELECT account_id, source FROM account_source")
        .map_err(|error| foreign_data("the account table", error))?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|error| foreign_data("the account table", error))?;
    let mut sources = HashMap::new();

    for row in rows {
        let (account_id, value) = row.map_err(|error| foreign_data("an account", error))?;

        sources.insert(account_id, AccountSource::parse(&value));
    }

    Ok(sources)
}

impl Store {
    pub fn accounts(&self) -> Result<Vec<AccountRecord>> {
        let connection = self.lock();
        let mut statement = connection
            .prepare(
                "SELECT id, login, display_name, avatar_url, settings, position, created_at FROM account ORDER BY position, created_at",
            )
            .map_err(|error| foreign_data("the account table", error))?;
        let rows = statement
            .query_map([], AccountRow::read)
            .map_err(|error| foreign_data("the account table", error))?;
        let sources = account_sources(&connection)?;
        let mut accounts = Vec::new();

        for row in rows {
            let row = row.map_err(|error| foreign_data("an account", error))?;
            let source = sources.get(&row.id).copied().unwrap_or_default();

            accounts.push(row.record(source)?);
        }

        Ok(accounts)
    }

    pub fn account(&self, id: &str) -> Result<Option<AccountRecord>> {
        let row = self
            .lock()
            .query_row(
                "SELECT id, login, display_name, avatar_url, settings, position, created_at FROM account WHERE id = ?1",
                [id],
                AccountRow::read,
            )
            .optional()
            .map_err(|error| foreign_data("an account", error))?;
        let Some(row) = row else {
            return Ok(None);
        };
        let source = self.get_account_source(id)?.unwrap_or_default();

        row.record(source).map(Some)
    }

    fn get_account_source(&self, id: &str) -> Result<Option<AccountSource>> {
        self.lock()
            .query_row(
                "SELECT source FROM account_source WHERE account_id = ?1",
                [id],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| foreign_data("an account", error))
            .map(|value| value.map(|value| AccountSource::parse(&value)))
    }

    pub fn upsert_account(&self, record: &AccountRecord) -> Result<()> {
        let connection = self.lock();

        connection.execute(
            "INSERT INTO account(id, login, display_name, avatar_url, settings, position, created_at)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET login = excluded.login, display_name = excluded.display_name,
                 avatar_url = excluded.avatar_url, settings = excluded.settings, position = excluded.position",
            params![
                record.id,
                record.login,
                record.display_name,
                record.avatar_url,
                serde_json::to_string(&record.settings)?,
                record.position,
                record.created_at.timestamp(),
            ],
        )?;

        // ! Updated on every login (device code or a pasted token): the
        // most recent credential decides how `remove_account` treats a
        // removal, not however the account was first added.
        connection.execute(
            "INSERT INTO account_source(account_id, source) VALUES(?1, ?2)
             ON CONFLICT(account_id) DO UPDATE SET source = excluded.source",
            params![record.id, record.source.as_str()],
        )?;

        Ok(())
    }

    pub fn save_account_settings(&self, id: &str, settings: &AccountSettings) -> Result<()> {
        let mut connection = self.lock();
        let transaction = connection.transaction()?;
        let changed = transaction.execute(
            "UPDATE account SET settings = ?2 WHERE id = ?1",
            params![id, serde_json::to_string(settings)?],
        )?;

        if changed == 0 {
            return Err(Error::UnknownAccount(id.to_owned()));
        }

        let balances_only: Vec<String> = transaction
            .prepare(
                "SELECT login FROM spam_channel
                 WHERE account_id = ?1 AND added_at IS NULL AND used_at IS NULL",
            )?
            .query_map(params![id], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;

        for login in balances_only {
            if !settings.points.streamers.iter().any(|s| s.login == login) {
                transaction.execute(
                    "DELETE FROM spam_channel WHERE account_id = ?1 AND login = ?2",
                    params![id, login],
                )?;
            }
        }

        transaction.commit()?;

        Ok(())
    }

    pub fn reorder_accounts(&self, ids: &[String]) -> Result<()> {
        let mut connection = self.lock();
        let transaction = connection.transaction()?;

        for (position, id) in ids.iter().enumerate() {
            transaction.execute(
                "UPDATE account SET position = ?2 WHERE id = ?1",
                params![id, position as i64],
            )?;
        }

        transaction.commit()?;

        Ok(())
    }

    pub fn delete_account(&self, id: &str) -> Result<()> {
        self.lock()
            .execute("DELETE FROM account WHERE id = ?1", [id])?;
        self.delete_in_batches(
            "DELETE FROM history WHERE id IN (SELECT id FROM history WHERE account_id = ?1 LIMIT ?2)",
            &id,
        )?;
        self.delete_in_batches(
            "DELETE FROM request WHERE id IN (SELECT id FROM request WHERE account_id = ?1 LIMIT ?2)",
            &id,
        )?;
        // ! `account_source` and `credential` are declared `ON DELETE
        // CASCADE` on `account.id`, so this delete removes them too.
        Ok(())
    }

    pub fn save_credential(
        &self,
        account_id: &str,
        credential: &Credential,
        vault: &Vault,
    ) -> Result<()> {
        let plain = Zeroizing::new(serde_json::to_vec(credential)?);
        let sealed = vault.seal(&plain)?;

        self.lock().execute(
            "INSERT INTO credential(account_id, nonce, ciphertext) VALUES(?1, ?2, ?3)
             ON CONFLICT(account_id) DO UPDATE SET nonce = excluded.nonce, ciphertext = excluded.ciphertext",
            params![account_id, sealed.nonce, sealed.ciphertext],
        )?;

        Ok(())
    }

    pub fn load_credential(&self, account_id: &str, vault: &Vault) -> Result<Option<Credential>> {
        let sealed: Option<Sealed> = self
            .lock()
            .query_row(
                "SELECT nonce, ciphertext FROM credential WHERE account_id = ?1",
                [account_id],
                |row| {
                    Ok(Sealed {
                        nonce: row.get(0)?,
                        ciphertext: row.get(1)?,
                    })
                },
            )
            .optional()?;
        let Some(sealed) = sealed else {
            return Ok(None);
        };
        let plain = vault.open(&sealed)?;

        Ok(Some(
            serde_json::from_slice(&plain).map_err(|_| Error::InvalidVault)?,
        ))
    }
}
