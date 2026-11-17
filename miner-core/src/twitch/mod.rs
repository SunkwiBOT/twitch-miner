pub mod auth;
pub mod channels;
pub mod chat;
pub mod drops;
pub mod operations;
pub mod playback;
pub mod points;
pub mod pubsub;
pub mod quests;
pub mod rewards;
pub mod search;
mod socket;
pub mod spade;

use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};

use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, ORIGIN, REFERER,
};
use reqwest::{Method, StatusCode};
use serde_json::Value;
use tokio::sync::{RwLock, Semaphore};
use zeroize::Zeroizing;

use crate::error::{Error, NetworkErrorKind, Result};
use crate::requests::{address_target, RequestKind, RequestLog};
use operations::Operation;

// ! Twitch for Android TV ("SMARTBOX"): its device code still works.
pub const CLIENT_ID: &str = "ue6666qo983tsx6so1t0vnawi233wa";
pub const CLIENT_ORIGIN: &str = "https://android.tv.twitch.tv";
pub const TV_USER_AGENT: &str = "Mozilla/5.0 (Linux; Android 7.1; Smart Box C1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36";
pub const WEB_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36";

pub const GQL_URL: &str = "https://gql.twitch.tv/gql";
pub const OAUTH_URL: &str = "https://id.twitch.tv/oauth2";
pub const USHER_URL: &str = "https://usher.ttvnw.net/api/channel/hls/";
pub const WEB_URL: &str = "https://www.twitch.tv/";
pub const PUBSUB_URL: &str = "wss://pubsub-edge.twitch.tv/v1";
pub const CHAT_URL: &str = "wss://irc-ws.chat.twitch.tv:443";

const REQUEST_TIMEOUT: Duration = Duration::from_secs(20);
const MAX_CONCURRENT: usize = 6;
// ! Below MAX_CONCURRENT, so the slots and claims still get a request in
// while balances, directories or redemptions are on their way.
pub const POINTS_READS_AT_ONCE: usize = 4;
pub const DIRECTORY_READS_AT_ONCE: usize = 4;
const GQL_ATTEMPTS: u32 = 3;
const GQL_BACKOFF: Duration = Duration::from_millis(400);
const PROXY_TEST_TIMEOUT: Duration = Duration::from_secs(15);

pub struct TwitchHttp {
    pub client: reqwest::Client,
    socket_client: reqwest::Client,
    pub device_id: String,
    pub proxied: bool,
    pub requests: RequestLog,
    concurrent: Semaphore,
}

impl TwitchHttp {
    pub fn new(device_id: String, proxy: Option<&str>, requests: RequestLog) -> Result<Self> {
        let proxy = proxy.map(str::trim).filter(|p| !p.is_empty());
        let proxy = proxy.map(parse_proxy).transpose()?;
        let build = |http1_only: bool| {
            let mut headers = HeaderMap::new();

            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US"));

            let mut builder = reqwest::Client::builder()
                .default_headers(headers)
                .user_agent(TV_USER_AGENT)
                .timeout(REQUEST_TIMEOUT)
                .gzip(true);

            if let Some(proxy) = proxy.clone() {
                builder = builder.proxy(proxy);
            }

            // ! The websocket library fails if the server picks HTTP/2 for the upgrade.
            if http1_only {
                builder = builder.http1_only();
            }

            builder.build().map_err(|error| {
                tracing::warn!(%error, "the HTTP client could not be built");

                Error::Transport(NetworkErrorKind::Other)
            })
        };

        Ok(Self {
            client: build(false)?,
            socket_client: build(true)?,
            device_id,
            proxied: proxy.is_some(),
            requests,
            concurrent: Semaphore::new(MAX_CONCURRENT),
        })
    }

    pub async fn permit(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.concurrent
            .acquire()
            .await
            .expect("semaphore is never closed")
    }

    pub fn request(&self, method: Method, url: &str) -> reqwest::RequestBuilder {
        self.client
            .request(method, url)
            .header("Client-Id", CLIENT_ID)
            .header("X-Device-Id", &self.device_id)
            .header(ORIGIN, CLIENT_ORIGIN)
            .header(REFERER, format!("{CLIENT_ORIGIN}/"))
    }

    pub async fn post_gql(
        &self,
        account_id: Option<&str>,
        operation: &str,
        token: Option<&str>,
        body: &Value,
    ) -> Result<GqlAnswer> {
        self.post_gql_leaving(account_id, operation, token, body, || true)
            .await
    }

    // ! `leaving` runs once the request has its slot: from there it may reach
    // Twitch, before that it never left. `false` keeps it from leaving at all.
    pub async fn post_gql_leaving(
        &self,
        account_id: Option<&str>,
        operation: &str,
        token: Option<&str>,
        body: &Value,
        leaving: impl FnOnce() -> bool,
    ) -> Result<GqlAnswer> {
        let _permit = self.permit().await;

        if !leaving() {
            return Err(Error::InvalidRequest("held back before it left".to_owned()));
        }

        let pending = self
            .requests
            .start(RequestKind::Gql, operation)
            .account(account_id);
        let mut request = self.request(Method::POST, GQL_URL).json(body);

        if let Some(token) = token {
            request = request.header(AUTHORIZATION, format!("OAuth {token}"));
        }

        let response = match request.send().await {
            Ok(response) => response,
            Err(error) => {
                let error = Error::from(error);

                pending.failed(&error);

                return Err(error);
            }
        };
        let status = response.status();

        if !status.is_success() {
            pending.answered(status);

            return Err(Error::Status(status));
        }

        match response.json::<Value>().await {
            Ok(answer) => {
                let answer = GqlAnswer::new(answer);

                pending.graphql(status, &answer.body, answer.refusal.is_some());

                Ok(answer)
            }
            Err(error) => {
                let error = Error::from(error);

                pending.failed(&error);

                Err(error)
            }
        }
    }
}

#[derive(Clone)]
pub struct TwitchClient {
    pub http: Arc<TwitchHttp>,
    pub user_id: String,
    pub login: String,
    token: Arc<RwLock<Zeroizing<String>>>,
}

#[derive(Debug, PartialEq, Eq)]
enum StatusOutcome {
    Retry(Error),
    Fail(Error),
}

impl TwitchClient {
    pub fn new(
        http: Arc<TwitchHttp>,
        user_id: String,
        login: String,
        access_token: String,
    ) -> Self {
        Self {
            http,
            user_id,
            login,
            token: Arc::new(RwLock::new(Zeroizing::new(access_token))),
        }
    }

    pub async fn token(&self) -> Zeroizing<String> {
        self.token.read().await.clone()
    }

    pub async fn set_token(&self, token: String) {
        *self.token.write().await = Zeroizing::new(token);
    }

    pub async fn gql(&self, operation: Operation, variables: Value) -> Result<Value> {
        self.gql_resent(operation, variables)
            .await
            .map(|(answer, _)| answer)
    }

    pub async fn gql_resent(
        &self,
        operation: Operation,
        variables: Value,
    ) -> Result<(Value, bool)> {
        self.send_gql(operation, &operation.request(variables))
            .await
    }

    // ! A mutation that spends points goes once: sent again after a lost
    // answer, Twitch could take it twice.
    pub async fn gql_once(
        &self,
        operation: Operation,
        variables: Value,
        leaving: impl FnOnce() -> bool,
    ) -> Result<Value> {
        let token = self.token.read().await.clone();
        let sent = self
            .http
            .post_gql_leaving(
                Some(&self.user_id),
                operation.name(),
                Some(&token),
                &operation.request(variables),
                leaving,
            )
            .await;

        match sent {
            Ok(answer) => answer.checked(operation),
            Err(Error::Status(status)) => match status_outcome(status) {
                StatusOutcome::Retry(error) | StatusOutcome::Fail(error) => Err(error),
            },
            Err(error) => Err(error),
        }
    }

    pub async fn gql_batch(
        &self,
        operation: Operation,
        variables: impl IntoIterator<Item = Value>,
    ) -> Result<Value> {
        let batch: Vec<Value> = variables
            .into_iter()
            .map(|variables| operation.request(variables))
            .collect();

        self.send_gql(operation, &Value::Array(batch))
            .await
            .map(|(answer, _)| answer)
    }

    async fn send_gql(&self, operation: Operation, body: &Value) -> Result<(Value, bool)> {
        let token = self.token.read().await.clone();
        let mut attempt = 1;

        loop {
            // ! One permit per attempt, given back before the pause: an account
            // backing off must not sit on the slots the others share.
            let sent = self
                .http
                .post_gql(Some(&self.user_id), operation.name(), Some(&token), body)
                .await;
            let retried = match sent {
                Ok(answer) => match answer.checked(operation) {
                    Err(error) if busy(&error) => error,
                    answer => return answer.map(|answer| (answer, attempt > 1)),
                },
                Err(Error::Status(status)) => match status_outcome(status) {
                    StatusOutcome::Retry(error) => error,
                    StatusOutcome::Fail(error) => return Err(error),
                },
                Err(error) => error,
            };

            if attempt == GQL_ATTEMPTS {
                return Err(retried);
            }

            tokio::time::sleep(GQL_BACKOFF * attempt).await;
            attempt += 1;
        }
    }
}

// ! Only 401 says the token is dead. Twitch, a proxy or Cloudflare also answer
// 403 for an integrity check, a proxy address or a disguised rate limit: that
// passes, so it is retried like a busy server.
fn status_outcome(status: StatusCode) -> StatusOutcome {
    if status == StatusCode::UNAUTHORIZED {
        return StatusOutcome::Fail(Error::Unauthorized);
    }

    if retried_status(status) {
        return StatusOutcome::Retry(Error::Status(status));
    }

    StatusOutcome::Fail(Error::Status(status))
}

const SERVICE_ERRORS: [&str; 2] = ["service error", "service timeout"];

// ! GraphQL errors arrive with status 200. Persisted-query misses mean a
// hash is outdated; integrity failures mean the operation needs the web client.
fn gql_refusal(body: &Value) -> Option<&'static str> {
    let refusals: Vec<Option<&'static str>> = match body {
        Value::Array(items) => items.iter().map(item_refusal).collect(),
        _ => vec![item_refusal(body)],
    };

    refusals
        .iter()
        .flatten()
        .find(|refusal| !SERVICE_ERRORS.contains(refusal))
        .or_else(|| {
            refusals
                .iter()
                .all(Option::is_some)
                .then(|| refusals.first().and_then(Option::as_ref))
                .flatten()
        })
        .copied()
}

fn item_refusal(item: &Value) -> Option<&'static str> {
    values(&item["errors"]).find_map(|error| {
        let message = error["message"].as_str().unwrap_or("");

        if message.contains("PersistedQueryNotFound") {
            return Some("persisted query outdated");
        }

        if message.contains("integrity") {
            return Some("integrity check failed");
        }

        SERVICE_ERRORS
            .iter()
            .find(|service| message.contains(*service))
            .copied()
    })
}

pub(crate) fn gql_error(operation: Operation, response: &Value) -> Option<Error> {
    values(&response["errors"]).next().map(|error| Error::Gql {
        operation: operation.name(),
        code: text(&error["message"]).unwrap_or_default(),
    })
}

pub struct GqlAnswer {
    pub body: Value,
    refusal: Option<&'static str>,
}

impl GqlAnswer {
    pub(crate) fn new(body: Value) -> Self {
        let refusal = gql_refusal(&body);

        Self { body, refusal }
    }

    pub(crate) fn checked(self, operation: Operation) -> Result<Value> {
        match self.refusal {
            Some(code) => Err(Error::Gql {
                operation: operation.name(),
                code: code.to_owned(),
            }),
            None => Ok(self.body),
        }
    }
}

fn retried_status(status: StatusCode) -> bool {
    status == StatusCode::FORBIDDEN
        || status == StatusCode::TOO_MANY_REQUESTS
        || status.is_server_error()
}

fn busy(error: &Error) -> bool {
    matches!(error, Error::Gql { code, .. } if SERVICE_ERRORS.contains(&code.as_str()))
}

pub fn transient(error: &Error) -> bool {
    match error {
        Error::Status(status) => retried_status(*status),
        Error::Transport(kind) => *kind == NetworkErrorKind::Timeout,
        other => busy(other),
    }
}

// ! No message quotes the URL, nor its scheme: without one, `user:password@host` reads as scheme `user`.
pub fn parse_proxy(url: &str) -> Result<reqwest::Proxy> {
    let parsed =
        url::Url::parse(url).map_err(|_| Error::InvalidProxy("the proxy is not a URL".into()))?;

    if !matches!(
        parsed.scheme(),
        "http" | "https" | "socks5" | "socks5h" | "socks4" | "socks4a"
    ) {
        return Err(Error::InvalidProxy(
            "the proxy must start with http://, https://, socks4:// or socks5://".into(),
        ));
    }

    if parsed.host_str().is_none() {
        return Err(Error::InvalidProxy("proxy URL has no host".into()));
    }

    reqwest::Proxy::all(parsed.as_str())
        .map_err(|_| Error::InvalidProxy("the HTTP client refused the proxy".into()))
}

// ! Shared by every settings path that stores a proxy (account settings,
// global settings, a token import's batch proxy): trims it and validates
// it once, as the one place that decides what a stored proxy string looks like.
pub fn trim_and_validate_proxy(proxy: &str) -> Result<String> {
    let proxy = proxy.trim().to_owned();

    if !proxy.is_empty() {
        parse_proxy(&proxy)?;
    }

    Ok(proxy)
}

pub async fn test_proxy(proxy: &str, requests: RequestLog) -> Result<Duration> {
    let http = TwitchHttp::new(crate::util::random_device_id(), Some(proxy), requests)?;
    let address = format!("{OAUTH_URL}/validate");
    let started = std::time::Instant::now();
    let response = http
        .requests
        .start(RequestKind::ProxyTest, address_target(&address))
        .expecting(&[StatusCode::UNAUTHORIZED])
        .send(http.client.get(&address).timeout(PROXY_TEST_TIMEOUT))
        .await?;
    let status = response.status();

    // ! 401 is the expected answer without a token: the proxy reached Twitch.
    if status != StatusCode::UNAUTHORIZED && !status.is_success() {
        return Err(Error::Status(status));
    }

    Ok(started.elapsed())
}

pub fn values(value: &Value) -> impl Iterator<Item = &Value> {
    value.as_array().into_iter().flatten()
}

pub(crate) fn note_left_out(list: &'static str, total: usize, kept: usize) {
    if kept < total {
        tracing::debug!(list, left_out = total - kept, "items of an answer left out");
    }
}

pub fn text(value: &Value) -> Option<String> {
    value.as_str().filter(|v| !v.is_empty()).map(str::to_owned)
}

pub fn parse_time(value: &Value) -> Option<DateTime<Utc>> {
    text(value)?.parse().ok()
}

pub fn saturated(value: u64) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

pub fn count(value: &Value) -> u32 {
    value.as_u64().map_or(0, saturated)
}
