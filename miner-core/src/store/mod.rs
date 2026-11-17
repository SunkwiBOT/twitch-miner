mod accounts;
mod history;
mod requests;
mod spam;
pub mod vault;

pub use accounts::{AccountRecord, AccountSource, Credential};
pub use history::{HistoryExportFilter, HistoryFilter};

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::config::GlobalSettings;
use crate::error::{Error, Result};
use crate::requests::RequestFilter;
use vault::{Sealed, Vault, VaultMode};

const DATABASE_FILE: &str = "miner.db";
const GLOBAL_SETTINGS_KEY: &str = "global";
const VAULT_MODE_KEY: &str = "vault_mode";
const VAULT_SALT_KEY: &str = "vault_salt";
const VAULT_CHECK_KEY: &str = "vault_check";
const PRUNE_BATCH: usize = 5000;
const PRUNE_PAUSE: Duration = Duration::from_millis(5);

// ! There is no migration: data this build cannot read is never replaced by
// defaults, the user is told to start from an empty folder.
const FOREIGN_DATA: &str =
    "the data folder was written by another version of the app: delete it, then sign in again";

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS setting (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS account (
    id           TEXT PRIMARY KEY,
    login        TEXT NOT NULL,
    display_name TEXT NOT NULL,
    avatar_url   TEXT,
    settings     TEXT NOT NULL,
    position     INTEGER NOT NULL,
    created_at   INTEGER NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS credential (
    account_id TEXT PRIMARY KEY REFERENCES account(id) ON DELETE CASCADE,
    nonce      TEXT NOT NULL,
    ciphertext TEXT NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS history (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id TEXT NOT NULL,
    at         INTEGER NOT NULL,
    kind       TEXT NOT NULL,
    title      TEXT NOT NULL,
    detail     TEXT,
    context    TEXT NOT NULL
) STRICT;
CREATE INDEX IF NOT EXISTS history_account ON history(account_id, id DESC);
CREATE INDEX IF NOT EXISTS history_at ON history(at);
CREATE INDEX IF NOT EXISTS history_kind ON history(kind);

CREATE TABLE IF NOT EXISTS request (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id TEXT,
    at         INTEGER NOT NULL,
    kind       TEXT NOT NULL,
    target     TEXT NOT NULL,
    socket     TEXT,
    outcome    TEXT NOT NULL,
    status     INTEGER,
    detail     TEXT,
    duration   INTEGER NOT NULL,
    count      INTEGER,
    longest    INTEGER
) STRICT;

CREATE TABLE IF NOT EXISTS spam_channel (
    account_id TEXT NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    login      TEXT NOT NULL,
    added_at   INTEGER,
    used_at    INTEGER,
    balance    INTEGER,
    balance_at INTEGER,
    PRIMARY KEY (account_id, login)
) STRICT;

CREATE TABLE IF NOT EXISTS account_source (
    account_id TEXT PRIMARY KEY REFERENCES account(id) ON DELETE CASCADE,
    source     TEXT NOT NULL
) STRICT;
";

// ! Says where and how the data broke, never the stored value: settings hold
// proxy passwords and webhook URLs.
fn foreign_data(what: &str, detail: impl fmt::Display) -> Error {
    Error::Storage(format!("{what} unreadable ({detail}): {FOREIGN_DATA}"))
}

// ! Account ids are client-supplied: never inlined into the SQL text, each
// gets its own numbered placeholder bound by `bind()`.
pub(super) struct AccountIdsParam<'a> {
    ids: &'a [String],
    keys: Vec<String>,
}

impl<'a> AccountIdsParam<'a> {
    pub(super) fn new(ids: &'a [String]) -> Self {
        let keys = (0..ids.len()).map(|i| format!(":account{i}")).collect();

        Self { ids, keys }
    }

    pub(super) fn condition(&self, table: &str) -> String {
        if self.ids.is_empty() {
            return "0".to_owned();
        }

        format!("{table}account_id IN ({})", self.keys.join(", "))
    }

    pub(super) fn bind<'b>(&'b self, named: &mut Vec<(&'b str, &'b dyn rusqlite::ToSql)>) {
        for (key, id) in self.keys.iter().zip(self.ids) {
            named.push((key.as_str(), id));
        }
    }
}

fn json_error(error: &serde_json::Error) -> String {
    format!(
        "{:?} error at line {}, column {}",
        error.classify(),
        error.line(),
        error.column()
    )
}

fn put_setting(connection: &Connection, key: &str, value: &str) -> Result<()> {
    connection.execute(
        "INSERT INTO setting(key, value) VALUES(?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(rename = "VaultStatusType")]
pub struct VaultStatus {
    pub mode: VaultMode,
    pub unlocked: bool,
}

#[derive(Clone)]
pub struct Store {
    connection: Arc<Mutex<Connection>>,
    pub dir: PathBuf,
}

impl Store {
    pub fn open(dir: &Path) -> Result<Self> {
        crate::util::private_dir(dir).map_err(|e| Error::Storage(e.to_string()))?;

        let path = dir.join(DATABASE_FILE);

        create_private(&path)?;

        let connection = Connection::open(&path)?;

        // ! `secure_delete`: a deleted row (an old sealed credential) is
        // overwritten, not left in a free page.
        connection.execute_batch(
            "PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON; PRAGMA busy_timeout = 5000;
             PRAGMA secure_delete = ON;",
        )?;
        connection.execute_batch(SCHEMA)?;

        let store = Self {
            connection: Arc::new(Mutex::new(connection)),
            dir: dir.to_owned(),
        };

        store.verify()?;
        store.clear_stale_miner_field()?;

        Ok(store)
    }

    fn verify(&self) -> Result<()> {
        self.global_settings()?;
        self.vault_mode()?;
        self.accounts()?;
        self.lock()
            .prepare("SELECT account_id, nonce, ciphertext FROM credential")
            .map_err(|error| foreign_data("the credential table", error))?;
        self.spam_rows()?;
        self.history(&HistoryFilter::default(), None, 1)?;
        self.requests(&RequestFilter::default(), Utc::now(), 0, None, 1)?;

        Ok(())
    }

    // ! Batches with pauses between: a long journal never holds back the accounts' writes.
    fn delete_in_batches(&self, sql: &str, key: &dyn rusqlite::ToSql) -> Result<usize> {
        let mut deleted = 0;

        loop {
            let removed = self.lock().execute(sql, params![key, PRUNE_BATCH as i64])?;

            deleted += removed;

            if removed < PRUNE_BATCH {
                return Ok(deleted);
            }

            std::thread::sleep(PRUNE_PAUSE);
        }
    }

    fn lock(&self) -> MutexGuard<'_, Connection> {
        crate::util::locked(&self.connection)
    }

    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        Ok(self
            .lock()
            .query_row("SELECT value FROM setting WHERE key = ?1", [key], |r| {
                r.get(0)
            })
            .optional()?)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        put_setting(&self.lock(), key, value)
    }

    pub fn delete_setting(&self, key: &str) -> Result<()> {
        self.lock()
            .execute("DELETE FROM setting WHERE key = ?1", [key])?;

        Ok(())
    }

    pub fn global_settings(&self) -> Result<GlobalSettings> {
        match self.get_setting(GLOBAL_SETTINGS_KEY)? {
            Some(json) => serde_json::from_str(&json)
                .map_err(|error| foreign_data("the app settings", json_error(&error))),
            None => Ok(GlobalSettings::default()),
        }
    }

    pub fn save_global_settings(&self, settings: &GlobalSettings) -> Result<()> {
        self.set_setting(GLOBAL_SETTINGS_KEY, &serde_json::to_string(settings)?)
    }

    pub fn vault_mode(&self) -> Result<VaultMode> {
        match self.get_setting(VAULT_MODE_KEY)? {
            Some(name) => VaultMode::from_name(&name)
                .ok_or_else(|| foreign_data("the vault mode", "unknown name")),
            None => Ok(VaultMode::KeyFile),
        }
    }

    pub fn open_default_vault(&self) -> Result<Option<Vault>> {
        match self.vault_mode()? {
            VaultMode::KeyFile => Ok(Some(Vault::from_key_file(
                &self.dir,
                !self.has_credentials()?,
            )?)),
            VaultMode::Passphrase => Ok(None),
        }
    }

    pub fn has_credentials(&self) -> Result<bool> {
        Ok(self
            .lock()
            .query_row("SELECT EXISTS(SELECT 1 FROM credential)", [], |row| {
                row.get(0)
            })?)
    }

    pub fn unlock_with_passphrase(&self, passphrase: &str) -> Result<Vault> {
        let salt = self
            .get_setting(VAULT_SALT_KEY)?
            .ok_or(Error::InvalidVault)?;
        let check: Sealed = serde_json::from_str(
            &self
                .get_setting(VAULT_CHECK_KEY)?
                .ok_or(Error::InvalidVault)?,
        )
        .map_err(|_| Error::InvalidVault)?;
        let vault = Vault::from_passphrase(passphrase, &salt)?;

        vault.verify(&check)?;

        Ok(vault)
    }

    pub fn rekey(
        &self,
        current: &Vault,
        next: &Vault,
        mode: VaultMode,
        salt: Option<&str>,
    ) -> Result<()> {
        let passphrase = match mode {
            VaultMode::Passphrase => Some((
                salt.ok_or(Error::InvalidVault)?,
                serde_json::to_string(&next.check_blob()?)?,
            )),
            VaultMode::KeyFile => None,
        };
        let mut connection = self.lock();
        let transaction = connection.transaction()?;
        let sealed: Vec<(String, Sealed)> = transaction
            .prepare("SELECT account_id, nonce, ciphertext FROM credential ORDER BY account_id")?
            .query_map([], |row| {
                Ok((
                    row.get(0)?,
                    Sealed {
                        nonce: row.get(1)?,
                        ciphertext: row.get(2)?,
                    },
                ))
            })?
            .collect::<rusqlite::Result<_>>()?;

        for (account_id, credential) in sealed {
            let resealed = next.seal(&current.open(&credential)?)?;

            transaction.execute(
                "UPDATE credential SET nonce = ?2, ciphertext = ?3 WHERE account_id = ?1",
                params![account_id, resealed.nonce, resealed.ciphertext],
            )?;
        }

        put_setting(&transaction, VAULT_MODE_KEY, mode.as_str())?;

        match &passphrase {
            Some((salt, check)) => {
                put_setting(&transaction, VAULT_SALT_KEY, salt)?;
                put_setting(&transaction, VAULT_CHECK_KEY, check)?;
            }
            None => {
                transaction.execute(
                    "DELETE FROM setting WHERE key IN (?1, ?2)",
                    params![VAULT_SALT_KEY, VAULT_CHECK_KEY],
                )?;
            }
        }

        transaction.commit()?;

        // ! The credentials sealed with the old key leave the write-ahead log for
        // good; best effort, the new ones are committed already.
        if let Err(error) = connection.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);") {
            tracing::debug!(%error, "the write-ahead log was not emptied after the rekey");
        }

        // ! Only once the passphrase seals everything: the old key opens
        // nothing any more.
        if passphrase.is_some() {
            vault::remove_key_file(&self.dir);
        }

        Ok(())
    }
}

// ! Created private before SQLite opens it, so the file is never readable by
// others, not even for an instant; SQLite gives its -wal and -shm files the
// same mode.
fn create_private(path: &Path) -> Result<()> {
    let mut options = std::fs::OpenOptions::new();

    options.write(true).create(true).truncate(false);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        options.mode(0o600);
    }

    options
        .open(path)
        .map_err(|error| Error::Storage(error.to_string()))?;

    Ok(())
}
