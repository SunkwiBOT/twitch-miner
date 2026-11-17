use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::config::{AccountSettings, GlobalSettings};

#[derive(Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(
    tag = "command",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
#[ts(rename = "CommandType")]
// ! Variants without fields are `{}`: serde ignores unknown keys on unit variants of a tagged enum.
pub enum Command {
    CancelLogin {},
    CancelImportTokens {},
    DismissImportTokens {},
    RemoveAccount {
        account_id: String,
        // ! See `Manager::remove_account`.
        revoke_token: bool,
    },
    SetAccountSettings {
        account_id: String,
        #[serde(deserialize_with = "crate::config::strict_account_settings")]
        settings: AccountSettings,
    },
    ReorderAccounts {
        account_ids: Vec<String>,
    },
    SetGlobalSettings {
        #[serde(deserialize_with = "crate::config::strict_global_settings")]
        settings: GlobalSettings,
    },
    RefreshAccount {
        account_id: String,
    },
    ClaimDrop {
        account_id: String,
        drop_instance_id: String,
    },
    SetPassphrase {
        current: Option<String>,
        next: Option<String>,
    },
    Unlock {
        passphrase: String,
    },
    Lock {},
    OrderCampaign {
        account_id: String,
        campaign_id: String,
        position: Option<u32>,
    },
    AddSpamChannel {
        account_id: String,
        login: String,
    },
    ForgetSpamChannel {
        account_id: String,
        login: String,
    },
    CancelBurst {
        burst_id: String,
    },
    DismissStarPrompt {},
    PauseAccount {
        account_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[ts(optional)]
        until: Option<DateTime<Utc>>,
    },
    PauseAllAccounts {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[ts(optional)]
        until: Option<DateTime<Utc>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[ts(repr(enum = name))]
pub enum CommandName {
    CancelLogin,
    CancelImportTokens,
    DismissImportTokens,
    RemoveAccount,
    SetAccountSettings,
    ReorderAccounts,
    SetGlobalSettings,
    RefreshAccount,
    ClaimDrop,
    SetPassphrase,
    Unlock,
    Lock,
    OrderCampaign,
    AddSpamChannel,
    ForgetSpamChannel,
    CancelBurst,
    DismissStarPrompt,
    PauseAccount,
    PauseAllAccounts,
}

// ! By hand: the log redaction only knows some secret shapes, a passphrase or proxy password must never reach `{:?}`.
impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CancelLogin {} => f.write_str("CancelLogin"),
            Self::CancelImportTokens {} => f.write_str("CancelImportTokens"),
            Self::DismissImportTokens {} => f.write_str("DismissImportTokens"),
            Self::RemoveAccount {
                account_id,
                revoke_token,
            } => f
                .debug_struct("RemoveAccount")
                .field("account_id", account_id)
                .field("revoke_token", revoke_token)
                .finish(),
            Self::SetAccountSettings { account_id, .. } => f
                .debug_struct("SetAccountSettings")
                .field("account_id", account_id)
                .finish_non_exhaustive(),
            Self::ReorderAccounts { account_ids } => f
                .debug_struct("ReorderAccounts")
                .field("account_ids", account_ids)
                .finish(),
            Self::SetGlobalSettings { .. } => {
                f.debug_struct("SetGlobalSettings").finish_non_exhaustive()
            }
            Self::RefreshAccount { account_id } => f
                .debug_struct("RefreshAccount")
                .field("account_id", account_id)
                .finish(),
            Self::ClaimDrop {
                account_id,
                drop_instance_id,
            } => f
                .debug_struct("ClaimDrop")
                .field("account_id", account_id)
                .field("drop_instance_id", drop_instance_id)
                .finish(),
            Self::SetPassphrase { .. } => f.debug_struct("SetPassphrase").finish_non_exhaustive(),
            Self::Unlock { .. } => f.debug_struct("Unlock").finish_non_exhaustive(),
            Self::Lock {} => f.write_str("Lock"),
            Self::OrderCampaign {
                account_id,
                campaign_id,
                position,
            } => f
                .debug_struct("OrderCampaign")
                .field("account_id", account_id)
                .field("campaign_id", campaign_id)
                .field("position", position)
                .finish(),
            Self::AddSpamChannel { account_id, login } => f
                .debug_struct("AddSpamChannel")
                .field("account_id", account_id)
                .field("login", login)
                .finish(),
            Self::ForgetSpamChannel { account_id, login } => f
                .debug_struct("ForgetSpamChannel")
                .field("account_id", account_id)
                .field("login", login)
                .finish(),
            Self::CancelBurst { burst_id } => f
                .debug_struct("CancelBurst")
                .field("burst_id", burst_id)
                .finish(),
            Self::DismissStarPrompt {} => f.write_str("DismissStarPrompt"),
            Self::PauseAccount { account_id, until } => f
                .debug_struct("PauseAccount")
                .field("account_id", account_id)
                .field("until", until)
                .finish(),
            Self::PauseAllAccounts { until } => f
                .debug_struct("PauseAllAccounts")
                .field("until", until)
                .finish(),
        }
    }
}

impl Command {
    pub fn name(&self) -> CommandName {
        match self {
            Self::CancelLogin {} => CommandName::CancelLogin,
            Self::CancelImportTokens {} => CommandName::CancelImportTokens,
            Self::DismissImportTokens {} => CommandName::DismissImportTokens,
            Self::RemoveAccount { .. } => CommandName::RemoveAccount,
            Self::SetAccountSettings { .. } => CommandName::SetAccountSettings,
            Self::ReorderAccounts { .. } => CommandName::ReorderAccounts,
            Self::SetGlobalSettings { .. } => CommandName::SetGlobalSettings,
            Self::RefreshAccount { .. } => CommandName::RefreshAccount,
            Self::ClaimDrop { .. } => CommandName::ClaimDrop,
            Self::SetPassphrase { .. } => CommandName::SetPassphrase,
            Self::Unlock { .. } => CommandName::Unlock,
            Self::Lock {} => CommandName::Lock,
            Self::OrderCampaign { .. } => CommandName::OrderCampaign,
            Self::AddSpamChannel { .. } => CommandName::AddSpamChannel,
            Self::ForgetSpamChannel { .. } => CommandName::ForgetSpamChannel,
            Self::CancelBurst { .. } => CommandName::CancelBurst,
            Self::DismissStarPrompt {} => CommandName::DismissStarPrompt,
            Self::PauseAccount { .. } => CommandName::PauseAccount,
            Self::PauseAllAccounts { .. } => CommandName::PauseAllAccounts,
        }
    }
}
