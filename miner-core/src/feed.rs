use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use tokio::sync::RwLock;

use crate::error::{Error, ErrorCode, Result};
use crate::http::read_bounded;
use crate::requests::{address_target, RequestKind, RequestLog};
use crate::store::Store;
use crate::util::locked;

// ! Each list takes one anonymous request every 5 minutes: an automatic read every 10 leaves room for a manual one.
pub const REFRESH_INTERVAL: Duration = Duration::from_secs(600);

const CACHE_MAX_AGE: chrono::Duration = chrono::Duration::days(7);

const MAX_BYTES: usize = 16 * 1024 * 1024;

pub trait FeedAnswer: Default + Serialize + DeserializeOwned + Send + Sync + 'static {
    const NAME: &'static str;
    const CACHE_KEY: &'static str;

    fn parse(payload: &Value) -> Result<Self>;
    fn count(&self) -> usize;
    fn fetched_at(&self) -> Option<DateTime<Utc>>;
    fn mark_fetched(&mut self, at: DateTime<Utc>);
    fn set_problem(&mut self, problem: Option<ErrorCode>);
}

pub struct FeedClient<T> {
    client: reqwest::Client,
    url: String,
    requests: RequestLog,
    // ! A 304 answer costs no request of the quota.
    tag: Mutex<Option<String>>,
    answer: PhantomData<fn() -> T>,
}

pub enum Fetched<T> {
    New(T),
    Unchanged,
}

pub struct Fetch<T> {
    pub result: Result<Fetched<T>>,
    pub retry_after: Duration,
}

impl<T: FeedAnswer> FeedClient<T> {
    pub(crate) fn with_url(url: &str, requests: RequestLog, agent: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(agent)
            .timeout(Duration::from_secs(30))
            .gzip(true)
            .build()
            .map_err(|e| Error::Network(e.to_string()))?;

        Ok(Self {
            client,
            url: url.to_owned(),
            requests,
            tag: Mutex::new(None),
            answer: PhantomData,
        })
    }

    pub async fn fetch(&self) -> Fetch<T> {
        let mut retry_after = Duration::ZERO;
        let result = async {
            let tag = locked(&self.tag).clone();
            let mut request = self
                .client
                .get(&self.url)
                .header("Accept", "application/json");

            if let Some(tag) = &tag {
                request = request.header("If-None-Match", tag);
            }

            let response = self
                .requests
                .start(RequestKind::Catalog, address_target(&self.url))
                .expecting(&[reqwest::StatusCode::NOT_MODIFIED])
                .send(request)
                .await?;

            for name in ["ratelimit-reset", "retry-after"] {
                if let Some(seconds) = response
                    .headers()
                    .get(name)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.trim().parse::<u64>().ok())
                {
                    retry_after = retry_after.max(Duration::from_secs(seconds.min(3600)));
                }
            }

            let status = response.status();

            if status == reqwest::StatusCode::NOT_MODIFIED && tag.is_some() {
                return Ok(Fetched::Unchanged);
            }

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(Error::CatalogRateLimited);
            }

            if !status.is_success() {
                return Err(Error::Status(status));
            }

            let tag = response
                .headers()
                .get(reqwest::header::ETAG)
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned);
            let body = read_bounded(response, MAX_BYTES).await?;
            let payload: Value = serde_json::from_slice(&body)?;
            let answer = T::parse(&payload)?;

            *locked(&self.tag) = tag;

            Ok(Fetched::New(answer))
        }
        .await;

        Fetch {
            result,
            retry_after,
        }
    }

    fn forget_tag(&self) {
        *locked(&self.tag) = None;
    }
}

pub struct Refresh {
    pub result: Result<(usize, bool)>,
    pub retry_after: Duration,
}

impl Refresh {
    pub fn wait(&self) -> Duration {
        match self.result {
            Err(Error::CatalogRateLimited) if !self.retry_after.is_zero() => self.retry_after,
            _ => REFRESH_INTERVAL.max(self.retry_after),
        }
    }
}

pub struct Feed<T> {
    pub state: Arc<RwLock<T>>,
    pub(crate) client: FeedClient<T>,
}

impl<T: FeedAnswer> Feed<T> {
    pub fn new(kept: T, client: FeedClient<T>) -> Self {
        Self {
            state: Arc::new(RwLock::new(kept)),
            client,
        }
    }

    pub async fn first_wait(&self) -> Duration {
        self.state
            .read()
            .await
            .fetched_at()
            .map_or(Duration::ZERO, |at| {
                let due = at + chrono::Duration::from_std(REFRESH_INTERVAL).unwrap_or_default();

                (due - Utc::now()).to_std().unwrap_or(Duration::ZERO)
            })
    }

    pub async fn refresh(&self, store: &Store) -> Refresh {
        let fetch = self.client.fetch().await;
        let refresh = Refresh {
            result: self.apply(fetch.result, store).await,
            retry_after: fetch.retry_after,
        };

        match &refresh.result {
            Ok((count, true)) => tracing::info!(count, "{} refreshed", T::NAME),
            Ok(_) => {}
            Err(Error::CatalogRateLimited) => tracing::info!(
                retry_in_seconds = refresh.wait().as_secs(),
                "{} refresh postponed: the API asks to wait, the last one is kept",
                T::NAME
            ),
            Err(error) => {
                tracing::warn!(%error, "{} refresh failed, the last one is kept", T::NAME)
            }
        }

        refresh
    }

    async fn apply(&self, fetched: Result<Fetched<T>>, store: &Store) -> Result<(usize, bool)> {
        let mut state = self.state.write().await;

        match fetched {
            Ok(Fetched::New(next)) if next.count() == 0 && state.count() > 0 => {
                // ! Forgotten with the empty answer, or the next read would get a 304 for it.
                self.client.forget_tag();

                let refused = Error::InvalidResponse(format!("an empty {} came back", T::NAME));

                state.set_problem(Some(refused.code()));

                Err(refused)
            }
            Ok(Fetched::New(next)) => {
                let count = next.count();
                let json = serde_json::to_string(&next);

                *state = next;
                drop(state);
                keep(store, T::CACHE_KEY, json, T::NAME).await;

                Ok((count, true))
            }
            Ok(Fetched::Unchanged) => {
                state.mark_fetched(Utc::now());
                state.set_problem(None);

                Ok((state.count(), false))
            }
            Err(error) => {
                state.set_problem(Some(error.code()));

                Err(error)
            }
        }
    }
}

pub fn cached<T: FeedAnswer>(store: &Store) -> T {
    let kept = match store.get_setting(T::CACHE_KEY) {
        Ok(json) => json.and_then(|json| {
            serde_json::from_str::<T>(&json)
                .inspect_err(|error| {
                    tracing::warn!(%error, "kept {} unreadable, fetched again", T::NAME);
                })
                .ok()
        }),
        Err(error) => {
            tracing::warn!(%error, "kept {} unreadable", T::NAME);
            None
        }
    };
    let now = Utc::now();

    match kept {
        Some(list)
            if list
                .fetched_at()
                .is_some_and(|at| at <= now && now - at < CACHE_MAX_AGE) =>
        {
            tracing::info!(count = list.count(), "{} kept from the last start", T::NAME);
            list
        }
        _ => T::default(),
    }
}

async fn keep(
    store: &Store,
    key: &'static str,
    json: serde_json::Result<String>,
    what: &'static str,
) {
    let kept = async {
        let json = json?;
        let store = store.clone();

        tokio::task::spawn_blocking(move || store.set_setting(key, &json))
            .await
            .map_err(|error| Error::Storage(error.to_string()))?
    }
    .await;

    if let Err(error) = kept {
        tracing::warn!(%error, what, "list not kept for the next start");
    }
}
