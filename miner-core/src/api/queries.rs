use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, IntoDeserializer, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use ts_rs::TS;

use super::NotificationTarget;
use crate::error::Error;
use crate::events::{EventCategory, EventKind};
use crate::logs::LogLevel;
use crate::requests::RequestKind;
use crate::spam::{valid_burst_id, BURST_MAX, REDEEM_TEXT_MAX_CHARS, REWARD_ID_MAX_CHARS};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "ChannelBannersRequestType")]
pub struct ChannelBannersRequest {
    pub logins: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "ChannelsLookupRequestType")]
pub struct ChannelsLookupRequest {
    pub logins: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "FollowsQueryType")]
pub struct FollowsQuery {
    pub account_id: String,
}

fn listed<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct Listed<T>(PhantomData<T>);

    impl<'de, T: Deserialize<'de>> Visitor<'de> for Listed<T> {
        type Value = Option<Vec<T>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a list, or its items joined by commas")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
            value
                .split(',')
                .filter(|item| !item.is_empty())
                .map(|item| T::deserialize(IntoDeserializer::<E>::into_deserializer(item)))
                .collect::<Result<_, _>>()
                .map(Some)
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut items: A) -> Result<Self::Value, A::Error> {
            let mut list = Vec::new();

            while let Some(item) = items.next_element()? {
                list.push(item);
            }

            Ok(Some(list))
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    deserializer.deserialize_any(Listed(PhantomData))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "HistoryQueryType")]
pub struct HistoryQuery {
    #[serde(
        default,
        deserialize_with = "listed",
        skip_serializing_if = "Option::is_none"
    )]
    #[ts(optional)]
    pub account_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub category: Option<EventCategory>,
    #[serde(
        default,
        deserialize_with = "listed",
        skip_serializing_if = "Option::is_none"
    )]
    #[ts(optional)]
    pub kinds: Option<Vec<EventKind>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub before_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "LogsQueryType")]
pub struct LogsQuery {
    #[serde(
        default,
        deserialize_with = "listed",
        skip_serializing_if = "Option::is_none"
    )]
    #[ts(optional)]
    pub account_ids: Option<Vec<String>>,
    #[serde(
        default,
        deserialize_with = "listed",
        skip_serializing_if = "Option::is_none"
    )]
    #[ts(optional)]
    pub levels: Option<Vec<LogLevel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub before_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub limit: Option<u32>,
}

// ! Live requests come first, then stored ones: a page carries on with the cursor of the part it ended in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "RequestsQueryType")]
pub struct RequestsQuery {
    #[serde(
        default,
        deserialize_with = "listed",
        skip_serializing_if = "Option::is_none"
    )]
    #[ts(optional)]
    pub account_ids: Option<Vec<String>>,
    #[serde(
        default,
        deserialize_with = "listed",
        skip_serializing_if = "Option::is_none"
    )]
    #[ts(optional)]
    pub kinds: Option<Vec<RequestKind>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub errors_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub before_live_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub before_stored_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "ChannelQueryType")]
pub struct ChannelQuery {
    pub login: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "SearchQueryType")]
pub struct SearchQuery {
    pub query: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "SpamChannelQueryType")]
pub struct SpamChannelQuery {
    pub account_id: String,
    pub login: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "RedeemRequestType")]
pub struct RedeemRequest {
    pub account_id: String,
    pub login: String,
    pub burst_id: String,
    pub reward_id: String,
    pub count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub text: Option<String>,
}

// ! The text is the viewer's own message to the streamer: it stays out of the logs.
impl std::fmt::Debug for RedeemRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedeemRequest")
            .field("account_id", &self.account_id)
            .field("login", &self.login)
            .field("burst_id", &self.burst_id)
            .field("reward_id", &self.reward_id)
            .field("count", &self.count)
            .finish_non_exhaustive()
    }
}

impl RedeemRequest {
    pub fn message(&self) -> Option<&str> {
        self.text
            .as_deref()
            .map(str::trim)
            .filter(|text| !text.is_empty())
    }

    pub fn validate(&self) -> crate::error::Result<()> {
        if !(1..=BURST_MAX).contains(&self.count) {
            return Err(Error::InvalidRequest(format!(
                "{} redemptions, 1 to {BURST_MAX}",
                self.count
            )));
        }

        if !valid_burst_id(&self.burst_id) {
            return Err(Error::InvalidRequest("malformed burst id".to_owned()));
        }

        if self.reward_id.is_empty() || self.reward_id.chars().count() > REWARD_ID_MAX_CHARS {
            return Err(Error::InvalidRequest("malformed reward id".to_owned()));
        }

        if self
            .message()
            .is_some_and(|text| text.chars().count() > REDEEM_TEXT_MAX_CHARS)
        {
            return Err(Error::InvalidRequest("message too long".to_owned()));
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "ProxyTestRequestType")]
pub struct ProxyTestRequest {
    pub url: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "DashboardLoginRequestType")]
pub struct DashboardLoginRequest {
    pub password: String,
}

impl std::fmt::Debug for DashboardLoginRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("DashboardLoginRequest { .. }")
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(
    tag = "target",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
#[ts(rename = "NotificationTestRequestType")]
pub enum NotificationTestRequest {
    Discord { webhook_url: String },
    Telegram { bot_token: String, chat_id: String },
    Webhook { url: String, authorization: String },
}

impl NotificationTestRequest {
    pub fn target(&self) -> NotificationTarget {
        match self {
            Self::Discord { .. } => NotificationTarget::Discord,
            Self::Telegram { .. } => NotificationTarget::Telegram,
            Self::Webhook { .. } => NotificationTarget::Webhook,
        }
    }
}

// ! A webhook URL, a bot token or a chat id stays out of `{:?}`: the same
// redaction rule as every other secret in the interface.
impl std::fmt::Debug for NotificationTestRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotificationTestRequest")
            .field("target", &self.target())
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(rename = "HistoryExportQueryType")]
pub struct HistoryExportQuery {
    #[serde(
        default,
        deserialize_with = "listed",
        skip_serializing_if = "Option::is_none"
    )]
    #[ts(optional)]
    pub account_ids: Option<Vec<String>>,
}
