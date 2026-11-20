use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures_util::future::join_all;
use tokio::sync::{broadcast, mpsc, oneshot, watch, Mutex, Notify, RwLock};
use tokio_util::sync::CancellationToken;

use super::account::{self, AccountCommand, AccountContext, AccountHandle};
use super::banners::Banners;
use super::record_event;
use super::spam_points::{spam_channels, SpamAccess, SpamPoints};
use crate::api::{
    AccountSnapshot, AccountStats, AccountStatus, BadgeCatalogView, BadgesResponse,
    CatalogResponse, CatalogView, CategoryCount, ChannelsLookupResponse, Command, DropsByCategory,
    HistoryCurveQuery, HistoryCurveResponse, HistoryExportQuery, HistoryExportResponse,
    HistoryQuery, HistoryResponse, HistoryStatsQuery, HistoryStatsResponse, ImportTokenOutcome,
    ImportTokensFlowStatus, ImportTokensFlowView, LoginFlowStatus, LoginFlowView, LogsQuery,
    LogsReportResponse, LogsResponse, NotificationTarget, NotificationTest,
    NotificationTestRequest, PointsByDay, ProxyTestResponse, RedeemRequest, RefreshCount,
    RequestsQuery, RequestsResponse, Shell, Snapshot, StreamerMinutes, StreamerPoints,
    StreamerPredictions, TokenCheck, CHANNELS_LOOKUP_MAX, FOLLOWS_MAX, HISTORY_EXPORT_MAX,
    HISTORY_LIMIT_DEFAULT, HISTORY_LIMIT_MAX, HISTORY_STATS_MAX_SPAN_DAYS, HISTORY_STATS_TOP_MAX,
    LOGS_LIMIT_DEFAULT, LOGS_LIMIT_MAX, REQUESTS_LIMIT_DEFAULT, REQUESTS_LIMIT_MAX,
    TOKENS_IMPORT_MAX, TOKEN_LINE_MAX_CHARS,
};
use crate::badges::{BadgeCatalog, BadgeCatalogClient};
use crate::catalog::{Catalog, CatalogClient};
use crate::config::{AccountSettings, AccountSettingsPatch, GlobalSettings};
use crate::domain::Channel;
use crate::error::{Error, ErrorCode, Result};
use crate::events::{EventContext, EventKind, MinerEvent, PauseReason};
use crate::feed::{Feed, FeedAnswer};
use crate::logs::LogFilter;
use crate::notifications::Notifier;
use crate::requests::{RequestCursor, RequestFilter, RequestLog, REPORT_MAX};
use crate::spam::{BurstProgress, SpamBalance, SpamChannelView};
use crate::store::vault::{random_salt, Vault, VaultMode};
use crate::store::{
    AccountRecord, AccountSource, Credential, HistoryCurveFilter, HistoryExportFilter,
    HistoryFilter, HistoryStatsFilter, Store, VaultStatus,
};
use crate::twitch::auth::{self, DeviceCode, Poll};
use crate::twitch::identity::AccountIdentity;
use crate::twitch::TwitchHttp;
use crate::util::{locked, read_locked, write_locked};
use zeroize::Zeroizing;

const RECENT_EVENTS: usize = 100;
// ! Long enough that a user mashing the button never queues a burst of
// outbound requests, short enough to feel instant on the first try.
const TEST_ACTION_GAP: Duration = Duration::from_secs(5);
const PAUSE_TICK: Duration = Duration::from_secs(30);

fn recent_kinds() -> Vec<EventKind> {
    EventKind::ALL
        .into_iter()
        .filter(|kind| kind.in_recent())
        .collect()
}

fn read_recent(store: &Store) -> Result<Vec<MinerEvent>> {
    let kinds = recent_kinds();
    let filter = HistoryFilter {
        kinds: Some(&kinds),
        ..HistoryFilter::default()
    };

    Ok(store.history(&filter, None, RECENT_EVENTS)?.0)
}
// ! Twitch's search suggests nothing under this many letters.
const SEARCH_MIN_CHARS: usize = 2;
const SEARCH_MAX_CHARS: usize = 60;
// ! Twitch logins are 4 to 25 characters long: shorter text is only a
// fragment, not worth reading apart.
const LOGIN_MIN_CHARS: usize = 4;

// ! A wide curve span (90 days of 5-minute gains) can hold tens of
// thousands of pure balance rows: thinned below, annotations kept in full.
const CURVE_MAX_BALANCE_POINTS: usize = 2_000;

// ! Keeps every annotated event (a raid, a bet, a streak...) whole, and
// decimates the plain balance rows (`PointsEarned`/`PointsSpent`) evenly so
// the line stays representative without shipping every 5-minute tick; the
// first and last balance row of the span are always kept so the curve's
// start and end are exact.
fn thin_curve_events(events: Vec<MinerEvent>) -> Vec<MinerEvent> {
    let is_balance_only =
        |kind: EventKind| matches!(kind, EventKind::PointsEarned | EventKind::PointsSpent);
    let balance_count = events.iter().filter(|e| is_balance_only(e.kind)).count();

    if balance_count <= CURVE_MAX_BALANCE_POINTS {
        return events;
    }

    let stride = balance_count.div_ceil(CURVE_MAX_BALANCE_POINTS);
    let last_balance_index = events.iter().rposition(|e| is_balance_only(e.kind));
    let mut seen = 0usize;

    events
        .into_iter()
        .enumerate()
        .filter(|(index, event)| {
            if !is_balance_only(event.kind) {
                return true;
            }

            let keep = seen.is_multiple_of(stride) || Some(*index) == last_balance_index;
            seen += 1;

            keep
        })
        .map(|(_, event)| event)
        .collect()
}

const HISTORY_PRUNE_EVERY: Duration = Duration::from_secs(3600);
const REQUESTS_MAX_AGE: chrono::Duration = chrono::Duration::days(365);
const REQUESTS_TICK: Duration = Duration::from_secs(60);
const TASK_STOP_TIMEOUT: Duration = Duration::from_secs(5);
const PASSPHRASE_MIN_CHARS: usize = 4;
const PASSPHRASE_TRIES: u32 = 5;
const PASSPHRASE_PAUSE: Duration = Duration::from_secs(30);
const UPDATE_ANNOUNCED_KEY: &str = "update_announced";
const UPDATE_FAILED_KEY: &str = "update_failed";
const FIRST_STARTED_KEY: &str = "first_started_at";
const LAST_RUN_VERSION_KEY: &str = "last_run_version";
const STAR_PROMPT_DONE_KEY: &str = "star_prompt_done";
const STAR_PROMPT_AFTER_HOURS: i64 = 48;

struct LoginFlow {
    view: LoginFlowView,
    cancel: CancellationToken,
}

struct ImportFlow {
    view: ImportTokensFlowView,
    cancel: CancellationToken,
    // ! A `watch`, not a one-shot: every concurrent `cancel_import_tokens`
    // call clones its own receiver off this one and observes the job's
    // own completion, not just whichever caller happened to see the flow
    // first (a `watch` replays its current value to a receiver created
    // after the job already sent it).
    done: watch::Receiver<bool>,
}

// ! Bundled so `finish_login` stays under clippy's argument limit:
// `new_account` only matters for a brand-new account, `existing_patch`
// only when applying a patch over an existing one; `source` always
// applies, see `finish_login`.
struct LoginOutcome<'a> {
    source: AccountSource,
    new_account: AccountSettings,
    // ! The identity of the token this login just produced: for an
    // existing account, `finish_login` only keeps its stored user agent
    // if the persona didn't change, see its own comment there.
    identity: AccountIdentity,
    existing_patch: Option<&'a AccountSettingsPatch>,
}

// ! Bundled so `run_import` stays under clippy's argument limit.
struct ImportJob {
    flow_id: String,
    tokens: Vec<String>,
    http: Arc<TwitchHttp>,
    vault: Vault,
    settings: AccountSettingsPatch,
    apply_existing_settings: bool,
    cancel: CancellationToken,
}

// ! What a single line in an import settles to, as distinct from a
// cancellation: the latter must not be recorded as a failed outcome, the
// line goes back to pending instead (see `run_import`).
enum LineOutcome {
    Settled(ImportTokenOutcome),
    Cancelled,
}

enum ImportLineError {
    Cancelled,
    Failed(Error),
}

impl From<Error> for ImportLineError {
    fn from(error: Error) -> Self {
        Self::Failed(error)
    }
}

#[derive(Default)]
pub(crate) struct WrongPassphrases {
    count: u32,
    paused_until: Option<tokio::time::Instant>,
}

impl WrongPassphrases {
    pub(crate) fn paused(&self, now: tokio::time::Instant) -> bool {
        self.paused_until.is_some_and(|until| now < until)
    }

    pub(crate) fn record(&mut self, right: bool, now: tokio::time::Instant) {
        if right {
            *self = Self::default();

            return;
        }

        self.count += 1;

        if self.count >= PASSPHRASE_TRIES {
            self.count = 0;
            self.paused_until = Some(now + PASSPHRASE_PAUSE);
        }
    }
}

pub struct Manager {
    pub store: Store,
    vault: RwLock<Option<Vault>>,
    // ! Swapped when the default proxy changes: read through `http()` at each use.
    http: std::sync::RwLock<Arc<TwitchHttp>>,
    device_id: String,
    install_id: String,
    catalog: Feed<Catalog>,
    badges: Feed<BadgeCatalog>,
    shell: Shell,
    settings: RwLock<GlobalSettings>,
    accounts: Mutex<HashMap<String, AccountHandle>>,
    carried_stats: std::sync::Mutex<HashMap<String, AccountStats>>,
    events: broadcast::Sender<MinerEvent>,
    notifier: Notifier,
    login: Mutex<Option<LoginFlow>>,
    import: Mutex<Option<ImportFlow>>,
    pub changed: Arc<Notify>,
    pub logs: crate::logs::LogBuffer,
    pub requests: RequestLog,
    cancel: CancellationToken,
    banners: Banners,
    recent: std::sync::Mutex<VecDeque<MinerEvent>>,
    start_failures: std::sync::Mutex<HashMap<String, Error>>,
    announcing: std::sync::Mutex<()>,
    update_available: std::sync::Mutex<Option<String>>,
    update_checked_at: std::sync::Mutex<Option<chrono::DateTime<Utc>>>,
    // ! When the "star the repository" window may show; None once it was closed.
    star_prompt_at: std::sync::Mutex<Option<chrono::DateTime<Utc>>>,
    // ! A key derivation takes 64 MiB and a good part of a second: one at a time.
    deriving: Mutex<()>,
    vault_change: Mutex<()>,
    wrong_passphrases: std::sync::Mutex<WrongPassphrases>,
    restarts: std::sync::Mutex<HashMap<String, Arc<Mutex<()>>>>,
    // ! Last broadcast state: compared each tick so an account only gets a
    // journal line on an actual transition, not every tick it stays the same.
    quiet_window: std::sync::Mutex<bool>,
    proxy_test_at: std::sync::Mutex<Option<tokio::time::Instant>>,
    // ! One gate per target: testing Discord does not block Telegram.
    notification_test_at: std::sync::Mutex<HashMap<NotificationTarget, tokio::time::Instant>>,
    // ! See `single_flight`.
    check_token_busy: std::sync::Mutex<bool>,
    spam: Arc<SpamPoints>,
}

impl Manager {
    pub fn open(data_dir: &Path, logs: crate::logs::LogBuffer, shell: Shell) -> Result<Arc<Self>> {
        crate::logs::log_panics();

        let store = Store::open(data_dir)?;
        let settings = store.global_settings()?;

        let device_id = match store.get_setting("device_id")? {
            Some(id) => id,
            None => {
                let id = crate::util::random_device_id();
                store.set_setting("device_id", &id)?;
                id
            }
        };
        let install_id = match store.get_setting("install_id")? {
            Some(id) => id,
            None => {
                let id = crate::util::random_uuid_v4();
                store.set_setting("install_id", &id)?;
                id
            }
        };

        let vault = store.open_default_vault()?;

        let catalog: Catalog = crate::feed::cached(&store);
        let badges: BadgeCatalog = crate::feed::cached(&store);
        let recent = read_recent(&store)?;
        let (events, _) = broadcast::channel(512);
        let changed = Arc::new(Notify::new());
        let cancel = CancellationToken::new();
        let spam = SpamPoints::new(
            store.clone(),
            events.clone(),
            changed.clone(),
            cancel.clone(),
        );
        let requests = RequestLog::new(Some(store.clone()));
        let http = shared_http(&device_id, &settings.default_proxy, &requests)?;

        Ok(Arc::new(Self {
            notifier: Notifier::new(
                settings.notifications.clone(),
                settings.notification_language(),
                requests.clone(),
                shell,
            )?,
            store,
            vault: RwLock::new(vault),
            http: std::sync::RwLock::new(Arc::new(http)),
            device_id,
            install_id: install_id.clone(),
            catalog: Feed::new(
                catalog,
                CatalogClient::new(requests.clone(), shell, &install_id)?,
                changed.clone(),
            ),
            badges: Feed::new(
                badges,
                BadgeCatalogClient::new(requests.clone(), shell, &install_id)?,
                changed.clone(),
            ),
            shell,
            settings: RwLock::new(settings),
            accounts: Mutex::new(HashMap::new()),
            carried_stats: std::sync::Mutex::new(HashMap::new()),
            events,
            login: Mutex::new(None),
            import: Mutex::new(None),
            changed,
            logs,
            requests,
            cancel,
            banners: Banners::default(),
            recent: std::sync::Mutex::new(recent.into()),
            start_failures: std::sync::Mutex::new(HashMap::new()),
            announcing: std::sync::Mutex::new(()),
            update_available: std::sync::Mutex::new(None),
            update_checked_at: std::sync::Mutex::new(None),
            star_prompt_at: std::sync::Mutex::new(None),
            deriving: Mutex::new(()),
            vault_change: Mutex::new(()),
            wrong_passphrases: std::sync::Mutex::new(WrongPassphrases::default()),
            restarts: std::sync::Mutex::new(HashMap::new()),
            quiet_window: std::sync::Mutex::new(false),
            proxy_test_at: std::sync::Mutex::new(None),
            notification_test_at: std::sync::Mutex::new(HashMap::new()),
            check_token_busy: std::sync::Mutex::new(false),
            spam: Arc::new(spam),
        }))
    }

    pub async fn start(self: &Arc<Self>) {
        if let Err(e) = self.plan_star_prompt() {
            tracing::debug!(error = %e, "star prompt date not read");
        }

        let event_receiver = self.events.subscribe();

        if let Err(e) = self.note_version(crate::VERSION) {
            tracing::debug!(error = %e, "version change not read");
        }

        let this = self.clone();
        tokio::spawn(async move { this.catalog_loop().await });

        let this = self.clone();
        tokio::spawn(async move { this.badges_loop().await });

        let this = self.clone();
        tokio::spawn(async move { this.event_loop(event_receiver).await });

        // ! Seeded before the pause loop or any account spawns: otherwise its
        // first tick can see a false->true transition mid-window at restart
        // and journal "Paused" for whichever accounts already spawned by
        // then, depending on timing.
        *locked(&self.quiet_window) = self.quiet_window_now().await;

        let this = self.clone();
        tokio::spawn(async move { this.pause_loop().await });

        if self.vault.read().await.is_some() {
            self.spawn_all_accounts().await;
        }
    }

    pub async fn shutdown(&self) {
        self.cancel.cancel();
        self.spam.interrupt().await;

        let handles: Vec<AccountHandle> =
            self.accounts.lock().await.drain().map(|(_, h)| h).collect();

        join_all(handles.into_iter().map(stop_task)).await;
        self.notifier.shutdown();
        self.requests.flush();
    }

    pub fn subscribe(&self) -> broadcast::Receiver<MinerEvent> {
        self.events.subscribe()
    }

    pub fn subscribe_bursts(&self) -> broadcast::Receiver<BurstProgress> {
        self.spam.subscribe()
    }

    async fn event_loop(self: Arc<Self>, mut receiver: broadcast::Receiver<MinerEvent>) {
        self.prune_history().await;

        let mut pruned = tokio::time::Instant::now();
        let mut tick = tokio::time::interval(REQUESTS_TICK);

        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => break,
                _ = tick.tick() => {
                    self.requests.close_due();

                    if pruned.elapsed() > HISTORY_PRUNE_EVERY {
                        pruned = tokio::time::Instant::now();
                        self.prune_history().await;
                    }
                }
                event = receiver.recv() => match event {
                    Ok(event) => {
                        self.remember(&event);
                        self.notifier.notify(event);
                        self.changed.notify_waiters();
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        self.reload_recent();
                        self.changed.notify_waiters();
                    }
                    Err(_) => break,
                }
            }
        }
    }

    fn recent(&self) -> std::sync::MutexGuard<'_, VecDeque<MinerEvent>> {
        locked(&self.recent)
    }

    fn remember(&self, event: &MinerEvent) {
        if !event.kind.in_recent() {
            return;
        }

        let mut recent = self.recent();

        recent.push_front(event.clone());
        recent.truncate(RECENT_EVENTS);
    }

    fn reload_recent(&self) {
        match read_recent(&self.store) {
            Ok(events) => *self.recent() = events.into(),
            Err(error) => tracing::warn!(%error, "recent events not read again"),
        }
    }

    fn start_failures(&self) -> std::sync::MutexGuard<'_, HashMap<String, Error>> {
        locked(&self.start_failures)
    }

    async fn catalog_loop(self: Arc<Self>) {
        let mut wait = self.catalog.first_wait().await;

        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => return,
                _ = tokio::time::sleep(wait) => {}
                next = self.catalog.resynced() => {
                    wait = next;

                    continue;
                }
            }

            wait = self.fetch_catalog().await;
        }
    }

    async fn fetch_catalog(&self) -> Duration {
        let (wait, refresh) = self.catalog.tick(&self.store).await;

        if let Some(refresh) = refresh {
            if matches!(refresh.result, Ok((_, true))) {
                self.refresh_all().await;
            }
        }

        wait
    }

    async fn badges_loop(self: Arc<Self>) {
        let mut wait = self.badges.first_wait().await;

        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => return,
                _ = tokio::time::sleep(wait) => {}
                next = self.badges.resynced() => {
                    wait = next;

                    continue;
                }
            }

            wait = self.fetch_badges().await;
        }
    }

    async fn fetch_badges(&self) -> Duration {
        self.badges.tick(&self.store).await.0
    }

    fn next_badges_manual_refresh(&self) -> chrono::DateTime<Utc> {
        self.badges.next_manual_refresh()
    }

    pub async fn refresh_badges(&self) -> Result<RefreshCount> {
        let (count, _) = self.badges.refresh_manual(&self.store).await?;

        Ok(RefreshCount {
            count: u32::try_from(count).unwrap_or(u32::MAX),
        })
    }

    pub async fn badge_list(&self) -> BadgesResponse {
        BadgesResponse {
            badges: self.badges.state.read().await.badges.clone(),
        }
    }

    pub async fn catalog_list(&self) -> CatalogResponse {
        CatalogResponse {
            campaigns: self.catalog.state.read().await.campaigns.clone(),
        }
    }

    // ! A campaign or badge link that only the account's own inventory carries (ended, not
    // crawled, or whose `link_url` differs from the catalogue's) is resolved from here first.
    pub async fn account_snapshot(&self, account_id: &str) -> Option<AccountSnapshot> {
        self.accounts
            .lock()
            .await
            .get(account_id)
            .map(|handle| handle.snapshot.borrow().clone())
    }

    pub fn logs_page(&self, query: &LogsQuery) -> LogsResponse {
        let limit = query
            .limit
            .unwrap_or(LOGS_LIMIT_DEFAULT)
            .clamp(1, LOGS_LIMIT_MAX) as usize;
        let logins = self.log_logins(query.account_ids.as_deref());
        let filter = LogFilter {
            accounts: logins.as_deref(),
            levels: query.levels.as_deref(),
        };
        let page = self.logs.page(&filter, query.before_id, limit);

        LogsResponse {
            lines: page.lines,
            next_before_id: page.next_before_id,
        }
    }

    pub async fn logs_report(&self, query: &LogsQuery) -> LogsReportResponse {
        let limit = query
            .limit
            .map_or(crate::logs::CAPACITY, |limit| limit as usize);

        LogsReportResponse {
            report: self.logs_export(query, limit).await,
        }
    }

    pub fn history_page(&self, query: &HistoryQuery) -> Result<HistoryResponse> {
        let limit = query
            .limit
            .unwrap_or(HISTORY_LIMIT_DEFAULT)
            .clamp(1, HISTORY_LIMIT_MAX) as usize;
        let filter = HistoryFilter {
            account_ids: query.account_ids.as_deref(),
            category: query.category,
            kinds: query.kinds.as_deref(),
        };
        let (events, next_before_id) = self.store.history(&filter, query.before_id, limit)?;

        Ok(HistoryResponse {
            events,
            next_before_id,
        })
    }

    // ! Clamped against `history_retention` and a hard span cap, regardless
    // of what the caller asked for: the stats page and the per-streamer
    // curve never read more than one bounded period at a time.
    async fn clamped_span(
        &self,
        since: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> (DateTime<Utc>, DateTime<Utc>) {
        let retention = self.settings.read().await.history_retention.max_age();
        let now = Utc::now();
        let until = until.min(now);
        let mut since = since.min(until);

        if let Some(age) = retention {
            since = since.max(now - age);
        }

        let max_span = chrono::Duration::days(HISTORY_STATS_MAX_SPAN_DAYS);

        if until - since > max_span {
            since = until - max_span;
        }

        (since, until)
    }

    pub async fn history_stats(&self, query: &HistoryStatsQuery) -> Result<HistoryStatsResponse> {
        let (since, until) = self.clamped_span(query.since, query.until).await;
        let filter = HistoryStatsFilter {
            account_ids: query.account_ids.as_deref(),
            since,
            until,
        };

        // ! A boundary below the clamped `since` (narrow edge: retention
        // equal to the requested period, near a day boundary) is floored to
        // `since` rather than dropped, so the rows between them are not
        // silently left out of every bucket.
        let mut day_boundaries: Vec<DateTime<Utc>> = query
            .day_boundaries
            .iter()
            .copied()
            .filter(|boundary| *boundary < until)
            .map(|boundary| boundary.max(since))
            .collect();
        day_boundaries.sort();
        day_boundaries.dedup();

        let points_by_day = self.store.points_by_day(&filter, &day_boundaries)?;
        let drops_by_category = self.store.drops_by_category(&filter)?;
        let top_streamers = self.store.streamer_points(&filter, HISTORY_STATS_TOP_MAX)?;
        let streamer_predictions = self.store.streamer_predictions_won(&filter)?;
        let predictions_won_total = self.store.predictions_won_total(&filter)?;
        let watch_minutes = self.store.watch_minutes_by_streamer(&filter)?;

        let points_total = points_by_day.iter().map(|(_, _, points)| points).sum();
        let drops_claimed_total = drops_by_category.iter().map(|(_, count)| count).sum();
        let minutes_watched_total = watch_minutes.iter().map(|(_, minutes)| minutes).sum();

        let top_points_streamer = top_streamers.first().map(|(login, points)| StreamerPoints {
            login: login.clone(),
            points: *points,
        });
        let top_category = drops_by_category
            .first()
            .map(|(category, count)| DropsByCategory {
                category: category.clone(),
                count: *count,
            });
        let top_watched_streamer = watch_minutes
            .first()
            .map(|(login, minutes)| StreamerMinutes {
                login: login.clone(),
                minutes: *minutes,
            });
        let top_predictions_streamer =
            streamer_predictions
                .first()
                .map(|(login, won)| StreamerPredictions {
                    login: login.clone(),
                    won: *won,
                });

        Ok(HistoryStatsResponse {
            points_by_day: points_by_day
                .into_iter()
                .map(|(day_start, reason, points)| PointsByDay {
                    day_start,
                    reason,
                    points,
                })
                .collect(),
            drops_by_category: drops_by_category
                .into_iter()
                .map(|(category, count)| DropsByCategory { category, count })
                .collect(),
            top_streamers_by_points: top_streamers
                .into_iter()
                .map(|(login, points)| StreamerPoints { login, points })
                .collect(),
            points_total,
            drops_claimed_total,
            minutes_watched_total,
            predictions_won_total,
            top_points_streamer,
            top_category,
            top_watched_streamer,
            top_predictions_streamer,
        })
    }

    pub async fn history_curve(&self, query: &HistoryCurveQuery) -> Result<HistoryCurveResponse> {
        let (since, until) = self.clamped_span(query.since, query.until).await;
        let filter = HistoryCurveFilter {
            account_id: &query.account_id,
            channel: &query.channel,
            since,
            until,
        };
        let events = thin_curve_events(self.store.balance_curve(&filter)?);

        Ok(HistoryCurveResponse {
            events,
            since,
            until,
        })
    }

    async fn prune_history(&self) {
        let retention = self.settings.read().await.history_retention.max_age();
        let now = Utc::now();
        let history_cutoff = retention.map(|age| now - age);
        let requests_cutoff =
            now - retention.map_or(REQUESTS_MAX_AGE, |age| age.min(REQUESTS_MAX_AGE));
        let store = self.store.clone();
        let pruned = tokio::task::spawn_blocking(move || {
            if let Some(cutoff) = history_cutoff {
                store.prune_history(cutoff)?;
            }

            store.prune_requests(requests_cutoff)
        })
        .await;

        match pruned {
            Ok(Ok(_)) => {}
            Ok(Err(error)) => tracing::warn!(%error, "history pruning failed"),
            Err(error) => tracing::warn!(%error, "history pruning stopped"),
        }

        if let Some(cutoff) = history_cutoff {
            self.recent().retain(|event| event.at >= cutoff);
        }
    }

    pub async fn requests_page(&self, query: RequestsQuery) -> Result<RequestsResponse> {
        let requests = self.requests.clone();

        blocking(move || {
            let limit = query
                .limit
                .unwrap_or(REQUESTS_LIMIT_DEFAULT)
                .clamp(1, REQUESTS_LIMIT_MAX) as usize;
            let cursor = RequestCursor {
                live_id: query.before_live_id,
                stored_id: query.before_stored_id,
            };
            let page = requests.page(&request_filter(&query), cursor, limit)?;

            Ok(RequestsResponse {
                requests: page.requests,
                next_live_id: page.next.live_id,
                next_stored_id: page.next.stored_id,
            })
        })
        .await
    }

    pub async fn requests_report(&self, query: RequestsQuery) -> Result<LogsReportResponse> {
        let requests = self.requests.clone();
        let store = self.store.clone();
        let zone = crate::util::display_time_zone(&self.settings.read().await.time_zone);

        blocking(move || {
            let limit = query
                .limit
                .map_or(REPORT_MAX, |limit| limit as usize)
                .clamp(1, REPORT_MAX);
            let logins: HashMap<String, String> = store
                .accounts()?
                .into_iter()
                .map(|account| (account.id, account.login))
                .collect();

            Ok(LogsReportResponse {
                report: format!(
                    "{}\n{}",
                    report_header(zone),
                    requests.report(&request_filter(&query), &logins, limit, zone)?
                ),
            })
        })
        .await
    }

    pub async fn vault_status(&self) -> VaultStatus {
        VaultStatus {
            mode: self.store.vault_mode().unwrap_or(VaultMode::KeyFile),
            unlocked: self.vault.read().await.is_some(),
        }
    }

    pub async fn unlock(self: &Arc<Self>, passphrase: &str) -> Result<()> {
        if self.vault.read().await.is_some() {
            return Ok(());
        }

        let vault = self.open_with_passphrase(passphrase).await?;

        *self.vault.write().await = Some(vault);
        self.spawn_all_accounts().await;
        self.changed.notify_waiters();

        Ok(())
    }

    pub async fn set_passphrase(
        self: &Arc<Self>,
        current: Option<&str>,
        next: Option<&str>,
    ) -> Result<()> {
        if self.shell == Shell::Server {
            return Err(Error::NotSupported);
        }

        let _one_change_at_a_time = self.vault_change.lock().await;

        // ! Only `None` removes the password: a blank one would silently
        // switch the vault back to the key file.
        if next.is_some_and(|passphrase| passphrase.trim().chars().count() < PASSPHRASE_MIN_CHARS) {
            return Err(Error::InvalidRequest("passphrase too short".to_owned()));
        }

        let mode = self.store.vault_mode()?;

        if mode == VaultMode::KeyFile && next.is_none() {
            return Err(Error::InvalidRequest(
                "no app password to remove".to_owned(),
            ));
        }

        let current_vault = match mode {
            VaultMode::KeyFile => {
                Vault::from_key_file(&self.store.dir, !self.store.has_credentials()?)?
            }
            VaultMode::Passphrase => {
                self.open_with_passphrase(current.unwrap_or_default())
                    .await?
            }
        };

        let unlocked = self.vault.read().await.is_some();

        if !unlocked {
            return Err(Error::VaultLocked);
        }

        // ! Running accounts and a pending sign-in hold the old key: a session
        // they renew during the rekey would be sealed with it, then unreadable.
        self.cancel_login().await;
        self.stop_all_accounts().await;

        let switched = self.switch_vault(&current_vault, next).await;

        self.spawn_all_accounts().await;
        self.changed.notify_waiters();

        switched
    }

    async fn open_with_passphrase(&self, passphrase: &str) -> Result<Vault> {
        let _one_at_a_time = self.deriving.lock().await;

        if self.wrong_passphrases().paused(tokio::time::Instant::now()) {
            return Err(Error::TooManyAttempts);
        }

        let store = self.store.clone();
        let passphrase = Zeroizing::new(passphrase.to_owned());
        let opened = tokio::task::spawn_blocking(move || store.unlock_with_passphrase(&passphrase))
            .await
            .map_err(|_| Error::Storage("the key derivation stopped".to_owned()))?;

        match &opened {
            Ok(_) => self
                .wrong_passphrases()
                .record(true, tokio::time::Instant::now()),
            Err(Error::WrongPassphrase) => self
                .wrong_passphrases()
                .record(false, tokio::time::Instant::now()),
            Err(_) => {}
        }

        opened
    }

    fn wrong_passphrases(&self) -> std::sync::MutexGuard<'_, WrongPassphrases> {
        locked(&self.wrong_passphrases)
    }

    // ! Kept exactly as typed: unlocking compares it as typed too.
    async fn switch_vault(&self, current: &Vault, next: Option<&str>) -> Result<()> {
        let next_vault = match next {
            Some(passphrase) => {
                let salt = random_salt();
                let vault = {
                    let _one_at_a_time = self.deriving.lock().await;
                    let passphrase = Zeroizing::new(passphrase.to_owned());
                    let salt = salt.clone();

                    tokio::task::spawn_blocking(move || Vault::from_passphrase(&passphrase, &salt))
                        .await
                        .map_err(|_| Error::Storage("the key derivation stopped".to_owned()))??
                };

                self.store
                    .rekey(current, &vault, VaultMode::Passphrase, Some(&salt))?;

                vault
            }
            None => {
                crate::store::vault::remove_key_file(&self.store.dir);

                let vault = Vault::from_key_file(&self.store.dir, true)?;

                self.store
                    .rekey(current, &vault, VaultMode::KeyFile, None)?;

                vault
            }
        };

        *self.vault.write().await = Some(next_vault);

        Ok(())
    }

    async fn stop_all_accounts(&self) {
        let handles: Vec<AccountHandle> =
            self.accounts.lock().await.drain().map(|(_, h)| h).collect();

        for handle in &handles {
            self.keep_stats(handle);
        }

        join_all(handles.into_iter().map(stop_task)).await;
    }

    // ! Only with a passphrase: a key-file vault would reopen by itself.
    pub async fn lock(&self) -> Result<()> {
        if self.shell == Shell::Server {
            return Err(Error::NotSupported);
        }

        let _one_change_at_a_time = self.vault_change.lock().await;

        if self.store.vault_mode()? != VaultMode::Passphrase {
            return Err(Error::PassphraseRequired);
        }

        self.cancel_login().await;
        self.cancel_import_tokens().await;
        // ! No result screen survives a lock: drop it outright rather
        // than leave a `Cancelled` view with nothing open to show it.
        self.dismiss_import_tokens().await;
        self.spam.cancel_bursts(None);
        self.stop_all_accounts().await;
        *self.vault.write().await = None;
        self.changed.notify_waiters();

        Ok(())
    }

    fn http(&self) -> Arc<TwitchHttp> {
        read_locked(&self.http).clone()
    }

    async fn vault(&self) -> Result<Vault> {
        self.vault.read().await.clone().ok_or(Error::VaultLocked)
    }

    async fn spawn_all_accounts(self: &Arc<Self>) {
        let Ok(vault) = self.vault().await else {
            return;
        };
        let records = match self.store.accounts() {
            Ok(records) => records,
            Err(error) => {
                tracing::warn!(%error, "no account could start");

                return;
            }
        };

        for record in records {
            let restart = self.restart_lock(&record.id);
            let _one_at_a_time = restart.lock().await;

            if let Err(error) = self.spawn_account(record, &vault).await {
                tracing::warn!(%error, "account could not start");
            }
        }
    }

    async fn spawn_account(self: &Arc<Self>, record: AccountRecord, vault: &Vault) -> Result<()> {
        let mut accounts = self.accounts.lock().await;

        if accounts.contains_key(&record.id) {
            return Ok(());
        }

        let id = record.id.clone();

        match self.start_task(record, vault).await {
            Ok(handle) => {
                self.start_failures().remove(&id);
                accounts.insert(id, handle);

                Ok(())
            }
            Err(error) => {
                self.start_failures().insert(id, error.clone());

                Err(error)
            }
        }
    }

    async fn start_task(
        self: &Arc<Self>,
        record: AccountRecord,
        vault: &Vault,
    ) -> Result<AccountHandle> {
        let credential = self
            .store
            .load_credential(&record.id, vault)?
            .ok_or_else(|| Error::UnknownAccount(record.id.clone()))?;
        let proxy = self.effective_proxy(&record.settings.proxy).await;
        // ! Never without the proxy: the account would reach Twitch from the
        // real address.
        let http = proxied_http(&self.device_id, proxy.as_deref(), &self.requests)?;
        let stats = locked(&self.carried_stats)
            .remove(&record.id)
            .unwrap_or_default();
        let context = AccountContext {
            store: self.store.clone(),
            vault: vault.clone(),
            http: Arc::new(http),
            catalog: self.catalog.state.clone(),
            badges: self.badges.state.clone(),
            install_id: self.install_id.clone(),
            events: self.events.clone(),
            stats,
            spends: self.spam.spends(&record.id),
            quiet_window: self.quiet_window_now().await,
        };
        let handle = account::spawn(record, credential, context);

        let mut watcher = handle.snapshot.clone();
        let this = self.clone();

        tokio::spawn(async move {
            while watcher.changed().await.is_ok() {
                this.changed.notify_waiters();
            }
        });

        Ok(handle)
    }

    async fn effective_proxy(&self, account_proxy: &str) -> Option<String> {
        let own = account_proxy.trim();

        if !own.is_empty() {
            return Some(own.to_owned());
        }

        let default = self.settings.read().await.default_proxy.trim().to_owned();

        (!default.is_empty()).then_some(default)
    }

    fn keep_stats(&self, handle: &AccountHandle) {
        let stats = handle.snapshot.borrow().stats.clone();

        locked(&self.carried_stats).insert(handle.id.clone(), stats);
    }

    fn restart_lock(&self, account_id: &str) -> Arc<Mutex<()>> {
        locked(&self.restarts)
            .entry(account_id.to_owned())
            .or_default()
            .clone()
    }

    async fn respawn_account(self: &Arc<Self>, account_id: &str) -> Result<()> {
        let vault = self.vault().await?;
        let restart = self.restart_lock(account_id);
        let _one_at_a_time = restart.lock().await;

        // ! Taken out in its own statement: in an `if let` the lock would stay
        // held while the task stops, and every snapshot and command would wait.
        let running = self.accounts.lock().await.remove(account_id);

        if let Some(handle) = running {
            self.keep_stats(&handle);
            stop_task(handle).await;
        }

        let record = self
            .store
            .account(account_id)?
            .ok_or_else(|| Error::UnknownAccount(account_id.to_owned()))?;

        self.spawn_account(record, &vault).await
    }

    pub async fn order_campaign(
        self: &Arc<Self>,
        account_id: &str,
        campaign_id: &str,
        position: Option<usize>,
    ) -> Result<()> {
        let record = self
            .store
            .account(account_id)?
            .ok_or_else(|| Error::UnknownAccount(account_id.to_owned()))?;
        let mut settings = record.settings;

        settings.drops.campaign_order.retain(|id| id != campaign_id);

        if let Some(position) = position {
            let position = position.min(settings.drops.campaign_order.len());

            settings
                .drops
                .campaign_order
                .insert(position, campaign_id.to_owned());

            // ! "Mine first" is a choice: with "only my selection" on, it must
            // not leave the campaign out.
            if !settings
                .drops
                .selected_campaign_ids
                .iter()
                .any(|id| id == campaign_id)
            {
                settings
                    .drops
                    .selected_campaign_ids
                    .push(campaign_id.to_owned());
            }
        }

        self.set_account_settings(account_id, settings).await
    }

    async fn refresh_all(&self) {
        let accounts = self.accounts.lock().await;

        for handle in accounts.values() {
            let _ = handle.commands.try_send(AccountCommand::Refresh);
        }
    }

    // ! The queue is waited on outside the lock: an account busy with a long
    // pass must not hold every snapshot and every other command.
    async fn send_command(&self, account_id: &str, command: AccountCommand) -> Result<()> {
        let commands = self
            .accounts
            .lock()
            .await
            .get(account_id)
            .map(|handle| handle.commands.clone())
            .ok_or_else(|| Error::UnknownAccount(account_id.to_owned()))?;

        commands
            .send(command)
            .await
            .map_err(|_| Error::UnknownAccount(account_id.to_owned()))
    }

    pub async fn set_account_settings(
        self: &Arc<Self>,
        account_id: &str,
        settings: AccountSettings,
    ) -> Result<()> {
        settings.validate()?;

        let mut settings = settings;

        settings.proxy = crate::twitch::trim_and_validate_proxy(&settings.proxy)?;

        settings
            .points
            .streamers
            .retain(|s| !s.login.trim().is_empty());

        for streamer in &mut settings.points.streamers {
            streamer.login = crate::util::normalize_login(&streamer.login)
                .ok_or_else(|| Error::InvalidStreamerLogin(streamer.login.clone()))?;
        }

        let mut seen = std::collections::HashSet::new();

        settings
            .points
            .streamers
            .retain(|streamer| seen.insert(streamer.login.clone()));

        let stored = self
            .store
            .account(account_id)?
            .ok_or_else(|| Error::UnknownAccount(account_id.to_owned()))?;
        let previous_proxy = stored.settings.proxy;
        // ! Only `PauseAccount`/`PauseAllAccounts` write this: a settings
        // save from the UI must not erase or resurrect a scheduled pause.
        settings.paused_until = stored.settings.paused_until;

        let defaults = &mut settings.points.streamer_defaults;

        defaults.login = String::new();
        defaults.enabled = true;

        self.store.save_account_settings(account_id, &settings)?;

        if previous_proxy != settings.proxy {
            self.respawn_account(account_id).await?;
        } else {
            let commands = self
                .accounts
                .lock()
                .await
                .get(account_id)
                .map(|handle| handle.commands.clone());

            if let Some(commands) = commands {
                let _ = commands
                    .send(AccountCommand::Settings(Box::new(settings)))
                    .await;
            }
        }

        self.changed.notify_waiters();

        Ok(())
    }

    pub async fn reorder_accounts(&self, ids: &[String]) -> Result<()> {
        self.store.reorder_accounts(ids)?;
        self.changed.notify_waiters();
        Ok(())
    }

    pub async fn remove_account(&self, account_id: &str, revoke_token: bool) -> Result<()> {
        let restart = self.restart_lock(account_id);
        let _one_at_a_time = restart.lock().await;
        let running = self.accounts.lock().await.remove(account_id);

        self.spam.forget_account(account_id);

        if let Some(handle) = running {
            stop_task(handle).await;
        }

        // ! A device-code account is always revoked: only an imported
        // token's owner can tell the miner apart from whatever else that
        // same token signs in, so that choice is theirs.
        let source = self
            .store
            .account(account_id)?
            .map(|record| record.source)
            .unwrap_or_default();
        let should_revoke = revoke_token || source != AccountSource::ImportedToken;

        if should_revoke {
            if let Ok(vault) = self.vault().await {
                if let Ok(Some(credential)) = self.store.load_credential(account_id, &vault) {
                    auth::revoke(
                        &self.http(),
                        Some(account_id),
                        &credential.identity,
                        &credential.tokens.access_token,
                    )
                    .await;
                }
            }
        }

        self.requests.forget(account_id);

        let store = self.store.clone();
        let id = account_id.to_owned();

        blocking(move || store.delete_account(&id)).await?;

        self.recent().retain(|event| event.account_id != account_id);
        self.start_failures().remove(account_id);
        self.changed.notify_waiters();

        Ok(())
    }

    pub async fn refresh_account(&self, account_id: &str) -> Result<()> {
        self.send_command(account_id, AccountCommand::Refresh).await
    }

    pub async fn claim_drop(&self, account_id: &str, instance: &str) -> Result<()> {
        let (reply, answer) = oneshot::channel();

        self.send_command(
            account_id,
            AccountCommand::ClaimDrop(instance.to_owned(), reply),
        )
        .await?;

        answer
            .await
            .map_err(|_| Error::UnknownAccount(account_id.to_owned()))?
    }

    pub async fn login_status(&self) -> Option<LoginFlowView> {
        self.login.lock().await.as_ref().map(|f| f.view.clone())
    }

    pub async fn cancel_login(&self) {
        if let Some(flow) = self.login.lock().await.take() {
            flow.cancel.cancel();
        }

        self.changed.notify_waiters();
    }

    pub async fn start_login(self: &Arc<Self>) -> Result<LoginFlowView> {
        let vault = self.vault().await?;

        self.cancel_login().await;

        let code = auth::start_device_flow(&self.http()).await?;
        let view = LoginFlowView {
            flow_id: crate::util::random_hex(8),
            user_code: code.user_code.clone(),
            verification_uri: code.verification_uri.clone(),
            issued_at: code.issued_at,
            expires_at: code.expires_at,
            status: LoginFlowStatus::Pending,
            problem: None,
            account_id: None,
        };
        let cancel = CancellationToken::new();

        *self.login.lock().await = Some(LoginFlow {
            view: view.clone(),
            cancel: cancel.clone(),
        });

        let this = self.clone();
        let flow_id = view.flow_id.clone();

        tokio::spawn(async move {
            let outcome = tokio::select! {
                _ = cancel.cancelled() => return,
                outcome = this.poll_login(code, &vault) => outcome,
            };

            let mut login = this.login.lock().await;

            if let Some(flow) = login.as_mut().filter(|f| f.view.flow_id == flow_id) {
                match outcome {
                    Ok(account_id) => {
                        flow.view.status = LoginFlowStatus::Completed;
                        flow.view.account_id = Some(account_id);
                    }
                    Err(Error::DeviceCodeExpired) => flow.view.status = LoginFlowStatus::Expired,
                    Err(error) => {
                        flow.view.status = LoginFlowStatus::Failed;
                        tracing::warn!("sign-in failed: {error}");
                        flow.view.problem = Some(error.code());
                    }
                }
            }

            this.changed.notify_waiters();
        });

        self.changed.notify_waiters();

        Ok(view)
    }

    async fn poll_login(self: &Arc<Self>, code: DeviceCode, vault: &Vault) -> Result<String> {
        let interval = Duration::from_secs(code.interval_seconds.max(2));

        loop {
            tokio::time::sleep(interval).await;

            let http = self.http();

            let poll = match auth::poll_device_flow(&http, &code).await {
                Err(error) if auth::missed_poll(&error) => {
                    tracing::debug!(%error, "sign-in poll failed; trying again");

                    continue;
                }
                poll => poll?,
            };

            match poll {
                Poll::Pending => continue,
                Poll::Token(tokens) => {
                    let validation = auth::validate(
                        &http,
                        None,
                        &AccountIdentity::default(),
                        &tokens.access_token,
                    )
                    .await?
                    .ok_or(Error::Unauthorized)?;

                    return self
                        .finish_login(
                            &http,
                            vault,
                            validation,
                            tokens,
                            LoginOutcome {
                                source: AccountSource::DeviceCode,
                                new_account: AccountSettings {
                                    enabled: false,
                                    ..AccountSettings::default()
                                },
                                identity: AccountIdentity::new_tv(),
                                existing_patch: None,
                            },
                        )
                        .await;
                }
            }
        }
    }

    // ! Read-only: a line the user is still editing is checked without
    // touching any account or credential.
    pub async fn check_pasted_token(self: &Arc<Self>, token: &str) -> Result<TokenCheck> {
        let _single_flight = single_flight(&self.check_token_busy)?;

        within_token_line_max(token)?;

        let token = auth::parse_pasted_token(token)?;
        let http = self.http();
        let validation = auth::validate_pasted_token(&http, &token).await?;
        let identity = AccountIdentity::for_client(&validation.client_id);
        let (display_name, avatar_url) = self.describe_token(&http, &validation, &identity).await;
        let already_added = self.store.account(&validation.user_id)?.is_some();
        let persona = identity.persona();
        let scopes_ok = auth::has_required_scopes(&validation.scopes);

        Ok(TokenCheck {
            login: validation.login,
            display_name,
            avatar_url,
            already_added,
            persona,
            scopes_ok,
        })
    }

    // ! Runs as a cancellable background job, followed through the snapshot
    // like a login flow: a long list or a lock mid-import must not leave the
    // caller holding one open request with no way to know where it stopped.
    pub async fn start_import_tokens(
        self: &Arc<Self>,
        tokens: Vec<String>,
        settings: AccountSettingsPatch,
        apply_existing_settings: bool,
    ) -> Result<ImportTokensFlowView> {
        if tokens.len() > TOKENS_IMPORT_MAX {
            return Err(Error::InvalidRequest(format!(
                "{} tokens, at most {TOKENS_IMPORT_MAX}",
                tokens.len()
            )));
        }

        settings.validate()?;

        let mut settings = settings;

        settings.proxy = crate::twitch::trim_and_validate_proxy(&settings.proxy)?;

        self.cancel_import_tokens().await;

        let vault = self.vault().await?;
        let proxy = self.effective_proxy(&settings.proxy).await;
        let http = Arc::new(proxied_http(
            &self.device_id,
            proxy.as_deref(),
            &self.requests,
        )?);
        let flow_id = crate::util::random_hex(8);
        let view = ImportTokensFlowView {
            flow_id: flow_id.clone(),
            lines: tokens.iter().map(|_| ImportTokenOutcome::Pending).collect(),
            status: ImportTokensFlowStatus::Running,
        };
        let cancel = CancellationToken::new();
        let (done_tx, done_rx) = watch::channel(false);

        *self.import.lock().await = Some(ImportFlow {
            view: view.clone(),
            cancel: cancel.clone(),
            done: done_rx,
        });

        let this = self.clone();
        let flow_id_for_task = flow_id.clone();

        tokio::spawn(async move {
            this.run_import(ImportJob {
                flow_id: flow_id_for_task,
                tokens,
                http,
                vault,
                settings,
                apply_existing_settings,
                cancel,
            })
            .await;

            let _ = done_tx.send(true);
        });

        self.changed.notify_waiters();

        Ok(view)
    }

    // ! Each line keeps going after a failure: the other accounts in the
    // batch still import, and the caller sees exactly which line stopped
    // and why instead of a single error for the whole list. Cancellation
    // is checked between lines and around the one network call that
    // identifies a token, never mid-commit: once a line starts writing to
    // the store, it runs to completion rather than being dropped partway
    // between `upsert_account`, `stop_task`, `save_credential` and
    // `spawn_account`.
    async fn run_import(self: Arc<Self>, job: ImportJob) {
        let ImportJob {
            flow_id,
            tokens,
            http,
            vault,
            settings,
            apply_existing_settings,
            cancel,
        } = job;

        for (index, token) in tokens.into_iter().enumerate() {
            if cancel.is_cancelled() {
                break;
            }

            self.set_import_line(&flow_id, index, ImportTokenOutcome::Checking)
                .await;

            match self
                .import_one(
                    &http,
                    &vault,
                    &token,
                    &settings,
                    apply_existing_settings,
                    &cancel,
                )
                .await
            {
                LineOutcome::Settled(outcome) => {
                    self.set_import_line(&flow_id, index, outcome).await;
                }
                LineOutcome::Cancelled => {
                    self.set_import_line(&flow_id, index, ImportTokenOutcome::Pending)
                        .await;
                    break;
                }
            }
        }

        let status = if cancel.is_cancelled() {
            ImportTokensFlowStatus::Cancelled
        } else {
            ImportTokensFlowStatus::Completed
        };

        self.set_import_status(&flow_id, status).await;
    }

    async fn import_one(
        self: &Arc<Self>,
        http: &Arc<TwitchHttp>,
        vault: &Vault,
        token: &str,
        settings: &AccountSettingsPatch,
        apply_existing_settings: bool,
        cancel: &CancellationToken,
    ) -> LineOutcome {
        match self
            .import_one_inner(
                http,
                vault,
                token,
                settings,
                apply_existing_settings,
                cancel,
            )
            .await
        {
            Ok(account_id) => LineOutcome::Settled(ImportTokenOutcome::Imported { account_id }),
            Err(ImportLineError::Cancelled) => LineOutcome::Cancelled,
            Err(ImportLineError::Failed(error)) => {
                tracing::warn!(%error, "a line in a token import failed");

                LineOutcome::Settled(ImportTokenOutcome::Failed {
                    problem: error.code(),
                })
            }
        }
    }

    async fn import_one_inner(
        self: &Arc<Self>,
        default_http: &Arc<TwitchHttp>,
        vault: &Vault,
        token: &str,
        settings: &AccountSettingsPatch,
        apply_existing_settings: bool,
        cancel: &CancellationToken,
    ) -> std::result::Result<String, ImportLineError> {
        within_token_line_max(token)?;

        let token = auth::parse_pasted_token(token)?;

        let validation = tokio::select! {
            _ = cancel.cancelled() => return Err(ImportLineError::Cancelled),
            result = auth::validate_pasted_token(default_http, &token) => result?,
        };

        let http = self
            .import_line_http(&validation.user_id, apply_existing_settings, default_http)
            .await?;

        let token_set = auth::TokenSet {
            access_token: token,
            refresh_token: None,
        };
        let identity = AccountIdentity::for_client(&validation.client_id);

        self.finish_login(
            &http,
            vault,
            validation,
            token_set,
            LoginOutcome {
                source: AccountSource::ImportedToken,
                // ! Always starts paused, same as a fresh device-code
                // account: the setup wizard the import flow runs once
                // results are in decides the real `enabled` state.
                new_account: AccountSettings {
                    enabled: false,
                    ..settings.clone().apply(AccountSettings::default())
                },
                identity,
                existing_patch: apply_existing_settings.then_some(settings),
            },
        )
        .await
        .map_err(ImportLineError::from)
    }

    // ! The batch's own proxy for a new account or one being overwritten;
    // an existing account kept as is uses its own stored proxy instead,
    // so the import never reaches Twitch from an address that account has
    // never used.
    async fn import_line_http(
        &self,
        user_id: &str,
        apply_existing_settings: bool,
        default_http: &Arc<TwitchHttp>,
    ) -> Result<Arc<TwitchHttp>> {
        if apply_existing_settings {
            return Ok(default_http.clone());
        }

        let Some(existing) = self.store.account(user_id)? else {
            return Ok(default_http.clone());
        };

        let proxy = self.effective_proxy(&existing.settings.proxy).await;

        Ok(Arc::new(proxied_http(
            &self.device_id,
            proxy.as_deref(),
            &self.requests,
        )?))
    }

    async fn set_import_line(&self, flow_id: &str, index: usize, outcome: ImportTokenOutcome) {
        let mut import = self.import.lock().await;

        if let Some(flow) = import.as_mut().filter(|flow| flow.view.flow_id == flow_id) {
            if let Some(line) = flow.view.lines.get_mut(index) {
                *line = outcome;
            }
        }

        drop(import);
        self.changed.notify_waiters();
    }

    async fn set_import_status(&self, flow_id: &str, status: ImportTokensFlowStatus) {
        let mut import = self.import.lock().await;

        if let Some(flow) = import.as_mut().filter(|flow| flow.view.flow_id == flow_id) {
            flow.view.status = status;
        }

        drop(import);
        self.changed.notify_waiters();
    }

    pub async fn import_status(&self) -> Option<ImportTokensFlowView> {
        self.import
            .lock()
            .await
            .as_ref()
            .map(|flow| flow.view.clone())
    }

    // ! A running flow is cancelled in place, not removed: the result
    // screen still has imported/failed lines to show once the job settles
    // it to `Cancelled` (see `run_import`). A no-op on an already settled
    // or absent flow; `dismiss_import_tokens` is the only way to clear a
    // settled one, so a cancel landing just after completion cannot drop
    // outcomes the UI has not seen yet.
    pub async fn cancel_import_tokens(&self) {
        let done = {
            let mut import = self.import.lock().await;

            import.as_mut().and_then(|flow| {
                if flow.view.status == ImportTokensFlowStatus::Running {
                    flow.cancel.cancel();

                    Some(flow.done.clone())
                } else {
                    None
                }
            })
        };

        if let Some(mut done) = done {
            let _ = done.wait_for(|done| *done).await;
        }

        self.changed.notify_waiters();
    }

    // ! Clears the flow outright, running or settled: closing the dialog
    // on a finished result, or a caller that wants it gone unconditionally
    // (`lock()`), goes through this instead of `cancel_import_tokens`.
    pub async fn dismiss_import_tokens(&self) {
        *self.import.lock().await = None;

        self.changed.notify_waiters();
    }

    // ! A public, unauthenticated lookup: the login is enough to show a name
    // and an avatar, without an authenticated request logged under an
    // account id that may not exist yet, and without needing a proxy
    // decision before one has been made (the check step, before settings).
    async fn describe_token(
        &self,
        http: &Arc<TwitchHttp>,
        validation: &auth::Validation,
        identity: &AccountIdentity,
    ) -> (String, Option<String>) {
        match crate::twitch::channels::lookup_public(http, identity, &validation.login).await {
            Ok(Some(channel)) => (channel.display_name, channel.avatar_url),
            _ => (validation.login.clone(), None),
        }
    }

    // ! Shared by the device-code flow and a pasted token: both end in the
    // same account record, credential and "signed in" event. `source`
    // always applies (see the field's own comment); `new_account` only
    // matters when no account exists yet, `existing_patch` only when one
    // does and asks to apply a patch over it.
    async fn finish_login(
        self: &Arc<Self>,
        http: &Arc<TwitchHttp>,
        vault: &Vault,
        validation: auth::Validation,
        tokens: auth::TokenSet,
        outcome: LoginOutcome<'_>,
    ) -> Result<String> {
        let existing = self.store.account(&validation.user_id)?;
        let (display_name, avatar_url) = self
            .describe_token(http, &validation, &outcome.identity)
            .await;
        let position = self.store.accounts()?.len() as i64;
        let record = AccountRecord {
            id: validation.user_id.clone(),
            login: validation.login.clone(),
            display_name,
            avatar_url,
            settings: match existing.as_ref() {
                Some(existing) => match outcome.existing_patch {
                    Some(patch) => patch.clone().apply(existing.settings.clone()),
                    None => existing.settings.clone(),
                },
                None => outcome.new_account,
            },
            position: existing.as_ref().map(|e| e.position).unwrap_or(position),
            created_at: existing
                .as_ref()
                .map(|e| e.created_at)
                .unwrap_or_else(Utc::now),
            // ! Always the method that just produced this credential: an
            // account re-authenticated through a different flow must not
            // keep the old source, or `remove_account` would decide whether
            // to auto-revoke based on how it was first added instead of
            // how its current session was actually obtained.
            source: outcome.source,
        };

        self.store.upsert_account(&record)?;

        // ! A token is bound to the client that issued it: the client id
        // always follows the fresh token this login just produced. The
        // user agent only changes if the persona changed too (a
        // same-persona re-login/re-import keeps the device it already
        // picked instead of looking like a new one every time). The
        // existing identity lives inside the *old* credential, read before
        // it is overwritten below.
        let existing_identity = match existing {
            Some(existing) => self
                .store
                .load_credential(&existing.id, vault)?
                .map(|credential| credential.identity),
            None => None,
        };
        let identity = match existing_identity {
            Some(existing_identity)
                if existing_identity.persona() == outcome.identity.persona() =>
            {
                AccountIdentity {
                    client_id: outcome.identity.client_id.clone(),
                    user_agent: existing_identity.user_agent.clone(),
                }
            }
            _ => outcome.identity,
        };

        let restart = self.restart_lock(&record.id);
        let _one_at_a_time = restart.lock().await;

        // ! A re-login replaces the running task, stopped first: a
        // session it renews now would overwrite the new credential.
        let running = self.accounts.lock().await.remove(&record.id);

        if let Some(handle) = running {
            self.keep_stats(&handle);
            stop_task(handle).await;
        }

        let credential = Credential {
            tokens,
            device_id: self.device_id.clone(),
            identity,
        };

        self.store.save_credential(&record.id, &credential, vault)?;

        let id = record.id.clone();
        let context = EventContext {
            account_name: Some(record.display_name.clone()),
            account_avatar: record.avatar_url.clone(),
            ..EventContext::default()
        };

        self.spawn_account(record, vault).await?;

        let event = MinerEvent::new(
            &id,
            &validation.login,
            EventKind::LoginCompleted,
            format!("{} signed in", validation.login),
        )
        .context(context);

        record_event(&self.store, &self.events, event);

        Ok(id)
    }

    fn note_version(&self, version: &str) -> Result<()> {
        let previous = self.store.get_setting(LAST_RUN_VERSION_KEY)?;

        if previous.as_deref() != Some(version) {
            self.store.set_setting(LAST_RUN_VERSION_KEY, version)?;
        }

        let Some(previous) = previous.filter(|previous| previous != version) else {
            return Ok(());
        };

        let event = MinerEvent::new(
            "",
            "",
            EventKind::VersionUpdated,
            format!("Updated to {version} (from {previous})"),
        )
        .context(EventContext {
            version: Some(version.to_owned()),
            previous_version: Some(previous),
            ..EventContext::default()
        });

        record_event(&self.store, &self.events, event);

        Ok(())
    }

    fn plan_star_prompt(&self) -> Result<()> {
        if self.store.get_setting(STAR_PROMPT_DONE_KEY)?.is_some() {
            return Ok(());
        }

        let first = match self
            .store
            .get_setting(FIRST_STARTED_KEY)?
            .and_then(|text| chrono::DateTime::parse_from_rfc3339(&text).ok())
        {
            Some(at) => at.with_timezone(&Utc),
            None => {
                let now = Utc::now();
                self.store
                    .set_setting(FIRST_STARTED_KEY, &now.to_rfc3339())?;
                now
            }
        };

        *locked(&self.star_prompt_at) =
            Some(first + chrono::Duration::hours(STAR_PROMPT_AFTER_HOURS));

        Ok(())
    }

    fn dismiss_star_prompt(&self) -> Result<()> {
        self.store.set_setting(STAR_PROMPT_DONE_KEY, "1")?;
        *locked(&self.star_prompt_at) = None;
        self.changed.notify_waiters();

        Ok(())
    }

    pub fn clear_update(&self) {
        let shown = locked(&self.update_available).take();

        if shown.is_some() {
            self.changed.notify_waiters();
        }
    }

    pub fn update_checked(&self) {
        *locked(&self.update_checked_at) = Some(Utc::now());

        self.changed.notify_waiters();
    }

    pub fn note_failed_update(&self, version: &str) -> Result<()> {
        self.store.set_setting(UPDATE_FAILED_KEY, version)
    }

    pub fn take_failed_update(&self) -> Result<Option<String>> {
        let version = self.store.get_setting(UPDATE_FAILED_KEY)?;

        if version.is_some() {
            self.store.delete_setting(UPDATE_FAILED_KEY)?;
        }

        Ok(version)
    }

    pub fn announce_update(&self, version: &str) -> Result<bool> {
        let _one_at_a_time = locked(&self.announcing);
        let shown = locked(&self.update_available).replace(version.to_owned());

        if shown.as_deref() != Some(version) {
            self.changed.notify_waiters();
        }

        if self.store.get_setting(UPDATE_ANNOUNCED_KEY)?.as_deref() == Some(version) {
            return Ok(false);
        }

        self.store.set_setting(UPDATE_ANNOUNCED_KEY, version)?;

        let event = MinerEvent::new(
            "",
            "",
            EventKind::UpdateAvailable,
            format!("Version {version} is available"),
        )
        .context(EventContext {
            version: Some(version.to_owned()),
            ..EventContext::default()
        });

        record_event(&self.store, &self.events, event);

        Ok(true)
    }

    pub async fn global_settings(&self) -> GlobalSettings {
        self.settings.read().await.clone()
    }

    pub async fn set_global_settings(self: &Arc<Self>, settings: GlobalSettings) -> Result<()> {
        settings.validate()?;

        let mut settings = settings;

        settings.default_proxy = crate::twitch::trim_and_validate_proxy(&settings.default_proxy)?;

        let proxy_changed = self.settings.read().await.default_proxy != settings.default_proxy;
        let http = if proxy_changed {
            Some(shared_http(
                &self.device_id,
                &settings.default_proxy,
                &self.requests,
            )?)
        } else {
            None
        };

        self.store.save_global_settings(&settings)?;
        self.notifier.update(
            settings.notifications.clone(),
            settings.notification_language(),
        );
        *self.settings.write().await = settings;

        if let Some(http) = http {
            *write_locked(&self.http) = Arc::new(http);

            let ids: Vec<String> = self
                .store
                .accounts()?
                .into_iter()
                .filter(|a| a.settings.proxy.trim().is_empty())
                .map(|a| a.id)
                .collect();
            let restarts = ids.iter().map(|id| async move {
                if let Err(error) = self.respawn_account(id).await {
                    tracing::warn!(%error, account = %id, "account could not restart with the new proxy");
                }
            });

            join_all(restarts).await;
        }

        self.changed.notify_waiters();

        Ok(())
    }

    fn log_logins(&self, account_ids: Option<&[String]>) -> Option<Vec<String>> {
        account_ids.map(|ids| {
            ids.iter()
                .filter_map(|id| self.store.account(id).ok().flatten().map(|a| a.login))
                .collect()
        })
    }

    async fn logs_export(&self, query: &LogsQuery, limit: usize) -> String {
        let logins = self.log_logins(query.account_ids.as_deref());
        let filter = LogFilter {
            accounts: logins.as_deref(),
            levels: query.levels.as_deref(),
        };
        let zone = crate::util::display_time_zone(&self.settings.read().await.time_zone);

        format!(
            "{}\n{}",
            report_header(zone),
            self.logs
                .export(&filter, limit.clamp(1, crate::logs::CAPACITY), zone)
        )
    }

    pub async fn snapshot(&self) -> Result<Snapshot> {
        let records = self.store.accounts()?;
        let vault = self.vault_status().await;
        let settings = self.settings.read().await.clone();
        let now = Utc::now();
        let quiet_window_active = settings.quiet_window.active_at(now);
        let history_retention_days = settings.history_retention.days();
        let accounts_lock = self.accounts.lock().await;
        let start_failures: HashMap<String, ErrorCode> = self
            .start_failures()
            .iter()
            .map(|(id, error)| (id.clone(), error.code()))
            .collect();
        let accounts: Vec<AccountSnapshot> = records
            .iter()
            .map(|record| match accounts_lock.get(&record.id) {
                // ! The stored settings, not the task's: it applies a change only after its
                // current pass, and a snapshot taken in between would show the old value (a
                // Pause flipping back).
                Some(handle) => {
                    let mut account = handle.snapshot.borrow().clone();

                    account.settings = record.settings.clone();
                    account.source = record.source;

                    let paused = !account.settings.enabled
                        || quiet_window_active
                        || account
                            .settings
                            .paused_until
                            .is_some_and(|until| now < until);

                    if paused && account.status != AccountStatus::LoginRequired {
                        account.status = AccountStatus::Paused;
                    }

                    account
                }
                None => {
                    let (status, problem) = match (vault.unlocked, start_failures.get(&record.id)) {
                        (false, _) => (AccountStatus::LoginRequired, Some(ErrorCode::VaultLocked)),
                        (true, Some(code)) => (failed_start_status(*code), Some(*code)),
                        (true, None) => (AccountStatus::Starting, None),
                    };

                    AccountSnapshot {
                        id: record.id.clone(),
                        login: record.login.clone(),
                        display_name: record.display_name.clone(),
                        source: record.source,
                        avatar_url: record.avatar_url.clone(),
                        settings: record.settings.clone(),
                        status,
                        problem,
                        slots: vec![],
                        queue: vec![],
                        streamers: vec![],
                        progress: vec![],
                        other_campaigns: vec![],
                        quests: vec![],
                        owned_badges: vec![],
                        badges_read_at: None,
                        stats: Default::default(),
                        updated_at: Utc::now(),
                        spam_channels: vec![],
                    }
                }
            })
            .collect();

        drop(accounts_lock);

        let mut spam_rows = self.store.spam_rows()?;
        let mut accounts = accounts;

        for account in &mut accounts {
            let rows = spam_rows.remove(&account.id).unwrap_or_default();

            account.spam_channels = spam_channels(account, &rows);
        }

        let catalog = self.catalog.state.read().await;
        let badges = self.badges.state.read().await;
        let catalog_interval = Catalog::REFRESH_INTERVAL.as_secs();
        let badges_interval = BadgeCatalog::REFRESH_INTERVAL.as_secs();
        let mut recent_events: Vec<MinerEvent> = self.recent().iter().cloned().collect();

        for event in &mut recent_events {
            if let Some(account) = records.iter().find(|r| r.id == event.account_id) {
                event.account_login = account.login.clone();
            }
        }

        let update_available = locked(&self.update_available).clone();
        let update_checked_at = *locked(&self.update_checked_at);
        let star_prompt = locked(&self.star_prompt_at).is_some_and(|at| Utc::now() >= at);

        Ok(Snapshot {
            version: crate::VERSION.to_owned(),
            build: None,
            update_available,
            update_checked_at,
            star_prompt,
            accounts,
            quiet_window_active,
            catalog: CatalogView {
                read_at: catalog.read_at,
                updated_at: catalog.updated_at,
                fetched_at: catalog.fetched_at,
                problem: catalog.problem,
                interval_seconds: catalog_interval,
            },
            badges: BadgeCatalogView {
                read_at: badges.read_at,
                updated_at: badges.updated_at,
                fetched_at: badges.fetched_at,
                problem: badges.problem,
                interval_seconds: badges_interval,
                next_refresh_at: badges.next_refresh_at,
                next_manual_refresh_at: self.next_badges_manual_refresh(),
            },
            settings,
            history_retention_days,
            vault,
            recent_events,
            login: self.login_status().await,
            import: self.import_status().await,
            token_import_max: TOKENS_IMPORT_MAX as u32,
            token_line_max_chars: TOKEN_LINE_MAX_CHARS as u32,
            import_default_patch: AccountSettingsPatch::defaults(),
        })
    }

    pub async fn execute(self: &Arc<Self>, command: Command) -> Result<()> {
        match command {
            Command::CancelLogin {} => self.cancel_login().await,
            Command::CancelImportTokens {} => self.cancel_import_tokens().await,
            Command::DismissImportTokens {} => self.dismiss_import_tokens().await,
            Command::DismissStarPrompt {} => self.dismiss_star_prompt()?,
            Command::RemoveAccount {
                account_id,
                revoke_token,
            } => self.remove_account(&account_id, revoke_token).await?,
            Command::SetAccountSettings {
                account_id,
                settings,
            } => self.set_account_settings(&account_id, settings).await?,
            Command::ReorderAccounts { account_ids } => self.reorder_accounts(&account_ids).await?,
            Command::SetGlobalSettings { settings } => self.set_global_settings(settings).await?,
            Command::RefreshAccount { account_id } => self.refresh_account(&account_id).await?,
            Command::ClaimDrop {
                account_id,
                drop_instance_id,
            } => self.claim_drop(&account_id, &drop_instance_id).await?,
            Command::SetPassphrase { current, next } => {
                self.set_passphrase(current.as_deref(), next.as_deref())
                    .await?
            }
            Command::Unlock { passphrase } => self.unlock(&passphrase).await?,
            Command::Lock {} => self.lock().await?,
            Command::OrderCampaign {
                account_id,
                campaign_id,
                position,
            } => {
                self.order_campaign(&account_id, &campaign_id, position.map(|p| p as usize))
                    .await?
            }
            Command::AddSpamChannel { account_id, login } => {
                self.add_spam_channel(&account_id, &login)?
            }
            Command::ForgetSpamChannel { account_id, login } => {
                self.forget_spam_channel(&account_id, &login)?
            }
            Command::CancelBurst { burst_id } => self.cancel_burst(&burst_id),
            Command::PauseAccount { account_id, until } => {
                self.pause_account(&account_id, until).await?
            }
            Command::PauseAllAccounts { until } => self.pause_all_accounts(until).await?,
        }

        Ok(())
    }

    pub async fn lookup_channel(&self, login: &str) -> Result<Channel> {
        let login = crate::util::normalize_login(login)
            .ok_or_else(|| Error::UnknownChannel(login.to_owned()))?;

        crate::twitch::channels::lookup_public(&self.http(), &AccountIdentity::default(), &login)
            .await?
            .ok_or(Error::UnknownChannel(login))
    }

    pub async fn channel_banners(
        &self,
        logins: &[String],
    ) -> Result<crate::api::ChannelBannersResponse> {
        let mut seen: HashSet<String> = HashSet::new();
        let wanted: Vec<String> = logins
            .iter()
            .filter_map(|login| crate::util::normalize_login(login))
            .filter(|login| seen.insert(login.clone()))
            .collect();

        within_lookup_max(wanted.len())?;

        let missing = self.banners.missing(&wanted);

        if !missing.is_empty() {
            self.banners
                .keep(crate::twitch::channels::banners(&self.http(), &missing).await?);
        }

        Ok(crate::api::ChannelBannersResponse {
            banners: self.banners.found(wanted),
        })
    }

    pub async fn search(&self, query: &str) -> Result<crate::api::SearchResponse> {
        let query = query.trim();

        if query.chars().count() < SEARCH_MIN_CHARS {
            return Ok(crate::api::SearchResponse::default());
        }

        let fragment: String = query.chars().take(SEARCH_MAX_CHARS).collect();
        let http = self.http();
        let exact =
            crate::util::normalize_login(query).filter(|login| login.len() >= LOGIN_MIN_CHARS);
        let (suggested, read) = tokio::join!(
            crate::twitch::search::suggestions(&http, &fragment),
            async {
                match &exact {
                    // ! Only an extra: a failed read leaves the suggestions as they are.
                    Some(login) => crate::twitch::channels::lookup_public(
                        &http,
                        &AccountIdentity::default(),
                        login,
                    )
                    .await
                    .ok()
                    .flatten(),
                    None => None,
                }
            },
        );
        let (games, channels) = suggested?;

        Ok(crate::api::SearchResponse {
            games,
            channels: crate::twitch::search::exact_first(channels, read),
        })
    }

    pub async fn lookup_channels(&self, entries: &[String]) -> Result<ChannelsLookupResponse> {
        let mut wanted: Vec<String> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        let mut missing: Vec<String> = Vec::new();

        for entry in entries {
            match crate::util::normalize_login(entry) {
                Some(login) => {
                    if seen.insert(login.clone()) {
                        wanted.push(login);
                    }
                }
                None => {
                    let entry = entry.trim();

                    if !entry.is_empty() && seen.insert(entry.to_owned()) {
                        missing.push(entry.to_owned());
                    }
                }
            }
        }

        within_lookup_max(wanted.len())?;

        let read = crate::twitch::channels::lookup_public_many(&self.http(), &wanted).await?;
        let known: HashSet<&str> = read
            .channels
            .iter()
            .map(|c| c.login.as_str())
            .chain(read.unchecked.iter().map(String::as_str))
            .collect();

        missing.extend(
            wanted
                .iter()
                .filter(|login| !known.contains(login.as_str()))
                .cloned(),
        );

        Ok(ChannelsLookupResponse {
            channels: read.channels,
            missing,
            unchecked: read.unchecked,
        })
    }

    pub async fn followed_channels(&self, account_id: &str) -> Result<Vec<Channel>> {
        let access = self.account_access(account_id).await?;

        access.client.followed_channels(FOLLOWS_MAX).await
    }

    pub async fn test_notification_target(
        &self,
        request: NotificationTestRequest,
    ) -> Result<NotificationTest> {
        self.notifier.validate_adhoc(&request)?;
        check_notification_test_rate_limit(&self.notification_test_at, request.target())?;

        Ok(self.notifier.test_adhoc(request).await)
    }

    pub fn cancel_burst(&self, burst_id: &str) {
        self.spam.cancel_burst(burst_id);
    }

    pub fn add_spam_channel(&self, account_id: &str, login: &str) -> Result<()> {
        self.spam.add_channel(account_id, login)
    }

    pub fn forget_spam_channel(&self, account_id: &str, login: &str) -> Result<()> {
        self.spam.forget_channel(account_id, login)
    }

    pub async fn spam_channel(&self, account_id: &str, login: &str) -> Result<SpamChannelView> {
        self.spam
            .channel(account_id, login, self.account_access(account_id))
            .await
    }

    pub async fn spam_balance(&self, account_id: &str, login: &str) -> Result<SpamBalance> {
        self.spam
            .balance(account_id, login, self.account_access(account_id))
            .await
    }

    pub async fn redeem(&self, request: RedeemRequest) -> Result<String> {
        let account_id = request.account_id.clone();
        let delay = Duration::from_millis(self.settings.read().await.spam_points.delay_ms.into());

        self.spam
            .redeem(request, delay, self.account_access(&account_id))
            .await
    }

    // ! A paused account does not renew its token: it checks its session first, as the only spender of the refresh token.
    async fn account_access(&self, account_id: &str) -> Result<SpamAccess> {
        let access = self
            .accounts
            .lock()
            .await
            .get(account_id)
            .map(|handle| SpamAccess {
                client: handle.client.clone(),
                commands: handle.commands.clone(),
                snapshot: handle.snapshot.clone(),
            });

        let access = match access {
            Some(access) => access,
            None if !self.vault_status().await.unlocked => return Err(Error::VaultLocked),
            None => {
                return Err(self
                    .start_failures()
                    .get(account_id)
                    .cloned()
                    .unwrap_or_else(|| Error::UnknownAccount(account_id.to_owned())))
            }
        };
        let (reply, answer) = oneshot::channel();
        let gone = || Error::UnknownAccount(account_id.to_owned());

        access
            .commands
            .send(AccountCommand::Session(reply))
            .await
            .map_err(|_| gone())?;
        answer.await.map_err(|_| gone())??;

        Ok(access)
    }

    pub async fn test_proxy(&self, url: &str) -> Result<ProxyTestResponse> {
        check_test_rate_limit(&self.proxy_test_at)?;

        let latency = crate::twitch::test_proxy(url, self.requests.clone()).await?;

        Ok(ProxyTestResponse {
            latency_ms: u64::try_from(latency.as_millis()).unwrap_or(u64::MAX),
        })
    }

    pub fn history_export(&self, query: &HistoryExportQuery) -> Result<HistoryExportResponse> {
        let filter = HistoryExportFilter {
            account_ids: query.account_ids.as_deref(),
        };
        let events = self.store.claimed_drops(&filter, HISTORY_EXPORT_MAX)?;
        let rows: Vec<ClaimedDropRow> = events
            .iter()
            .map(|event| ClaimedDropRow {
                at: event.at,
                account_login: event.account_login.clone(),
                category: event.context.game.clone().unwrap_or_default(),
                campaign: event.context.campaign_name.clone().unwrap_or_default(),
                drop: event.context.drop_name.clone().unwrap_or_default(),
                minutes: event.context.required,
            })
            .collect();
        let csv = claimed_drops_csv(&rows);
        let (category_counts, total) = self.store.claimed_drops_counts(&filter)?;
        let mut counts: Vec<CategoryCount> = category_counts
            .into_iter()
            .map(|(category, count)| CategoryCount { category, count })
            .collect();

        counts.sort_by(|a, b| {
            b.count
                .cmp(&a.count)
                .then_with(|| a.category.cmp(&b.category))
        });

        Ok(HistoryExportResponse {
            csv,
            counts,
            truncated: total > rows.len() as u64,
        })
    }

    async fn quiet_window_now(&self) -> bool {
        let settings = self.settings.read().await;

        settings.quiet_window.active_at(Utc::now())
    }

    async fn pause_loop(self: Arc<Self>) {
        let mut tick = tokio::time::interval(PAUSE_TICK);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => break,
                _ = tick.tick() => {
                    self.resume_due_accounts().await;
                    self.sync_quiet_window().await;
                }
            }
        }
    }

    // ! Checked on every tick, not only at startup: the app can run for days
    // without a settings change, and a scheduled pause must still lift itself.
    async fn resume_due_accounts(&self) {
        let now = Utc::now();
        let records = match self.store.accounts() {
            Ok(records) => records,
            Err(error) => {
                tracing::warn!(%error, "accounts not read for the pause scheduler");
                return;
            }
        };

        for record in records {
            if record
                .settings
                .paused_until
                .is_some_and(|until| now >= until)
            {
                if let Err(error) = self.set_paused_until(&record.id, None).await {
                    tracing::warn!(account_id = %record.id, %error, "a scheduled pause was not cleared");
                }
            }
        }
    }

    // ! Every running account is told the current state on every tick
    // (idempotent for one already in it), so one just spawned or respawned
    // never waits for the next edge; a journal line is written only on an
    // actual transition, compared against the last one broadcast.
    async fn sync_quiet_window(&self) {
        let now = Utc::now();
        let active = self.quiet_window_now().await;

        let transitioned = {
            let mut stored = locked(&self.quiet_window);
            let transitioned = *stored != active;

            *stored = active;

            transitioned
        };

        let ids: Vec<(String, mpsc::Sender<AccountCommand>)> = {
            let accounts = self.accounts.lock().await;

            accounts
                .iter()
                .map(|(id, handle)| (id.clone(), handle.commands.clone()))
                .collect()
        };

        for (_, commands) in &ids {
            let _ = commands.try_send(AccountCommand::QuietWindow(active));
        }

        if !transitioned {
            return;
        }

        for (id, _) in &ids {
            let Ok(Some(record)) = self.store.account(id) else {
                continue;
            };

            let individually_paused = record
                .settings
                .paused_until
                .is_some_and(|until| now < until);

            if !record.settings.enabled || individually_paused {
                continue;
            }

            if active {
                self.emit_pause_event(id, &record.login, PauseReason::QuietWindow, None);
            } else {
                self.emit_resume_event(id, &record.login);
            }
        }
    }

    pub async fn pause_account(
        &self,
        account_id: &str,
        until: Option<chrono::DateTime<Utc>>,
    ) -> Result<()> {
        validate_pause_until(until, Utc::now())?;
        self.set_paused_until(account_id, until).await
    }

    pub async fn pause_all_accounts(&self, until: Option<chrono::DateTime<Utc>>) -> Result<()> {
        validate_pause_until(until, Utc::now())?;

        let mut failed = Vec::new();

        for record in self.store.accounts()? {
            if let Err(error) = self.set_paused_until(&record.id, until).await {
                tracing::warn!(account_id = %record.id, %error, "account not paused");
                failed.push(record.login);
            }
        }

        if failed.is_empty() {
            Ok(())
        } else {
            Err(Error::Storage(format!(
                "{} account(s) not paused: {}",
                failed.len(),
                failed.join(", ")
            )))
        }
    }

    async fn set_paused_until(
        &self,
        account_id: &str,
        until: Option<chrono::DateTime<Utc>>,
    ) -> Result<()> {
        let record = self
            .store
            .account(account_id)?
            .ok_or_else(|| Error::UnknownAccount(account_id.to_owned()))?;
        let mut settings = record.settings;
        // ! Not compared against `now`: a scheduled pause whose deadline just
        // passed is still a pause to resume from, not a no-op, or the auto
        // clear in `resume_due_accounts` would silently lose its event.
        let was_paused = settings.paused_until.is_some();
        let enabled = settings.enabled;

        settings.paused_until = until;
        self.store.save_account_settings(account_id, &settings)?;

        let commands = self
            .accounts
            .lock()
            .await
            .get(account_id)
            .map(|handle| handle.commands.clone());

        if let Some(commands) = commands {
            let _ = commands
                .send(AccountCommand::Settings(Box::new(settings)))
                .await;
        }

        let resumed = until.is_none() && was_paused && enabled && !self.quiet_window_now().await;

        match until {
            Some(until) => self.emit_pause_event(
                account_id,
                &record.login,
                PauseReason::Scheduled,
                Some(until),
            ),
            // ! Only when this clear actually resumes mining: the manual
            // switch or the quiet window can still keep the account paused,
            // which emits its own Resumed event once it lifts.
            None if resumed => self.emit_resume_event(account_id, &record.login),
            None => {}
        }

        self.changed.notify_waiters();

        Ok(())
    }

    fn emit_pause_event(
        &self,
        account_id: &str,
        login: &str,
        reason: PauseReason,
        until: Option<chrono::DateTime<Utc>>,
    ) {
        let event = MinerEvent::new(
            account_id,
            login,
            EventKind::Paused,
            format!("{login} paused"),
        )
        .context(EventContext {
            pause_reason: Some(reason),
            pause_until: until,
            ..EventContext::default()
        });

        record_event(&self.store, &self.events, event);
    }

    fn emit_resume_event(&self, account_id: &str, login: &str) {
        let event = MinerEvent::new(
            account_id,
            login,
            EventKind::Resumed,
            format!("{login} resumed"),
        );

        record_event(&self.store, &self.events, event);
    }
}

// ! A gate shared by every test-before-saving action: a button mashed fast
// waits instead of sending another round-trip with the typed secret.
fn check_test_rate_limit(gate: &std::sync::Mutex<Option<tokio::time::Instant>>) -> Result<()> {
    let mut last = locked(gate);
    let now = tokio::time::Instant::now();

    if last.is_some_and(|at| now.duration_since(at) < TEST_ACTION_GAP) {
        return Err(Error::ActionRateLimited);
    }

    *last = Some(now);

    Ok(())
}

// ! One check in flight at a time: a second one (another tab, a race) is
// refused instead of paced behind a fixed delay — the caller already walks
// a whole list one line at a time on its own.
fn single_flight(gate: &std::sync::Mutex<bool>) -> Result<SingleFlightGuard<'_>> {
    let mut busy = locked(gate);

    if *busy {
        return Err(Error::ActionRateLimited);
    }

    *busy = true;

    Ok(SingleFlightGuard(gate))
}

struct SingleFlightGuard<'a>(&'a std::sync::Mutex<bool>);

impl Drop for SingleFlightGuard<'_> {
    fn drop(&mut self) {
        *locked(self.0) = false;
    }
}

// ! One slot per `NotificationTarget`: testing Discord does not block
// Telegram or the webhook.
fn check_notification_test_rate_limit(
    gate: &std::sync::Mutex<HashMap<NotificationTarget, tokio::time::Instant>>,
    target: NotificationTarget,
) -> Result<()> {
    let mut gate = locked(gate);
    let now = tokio::time::Instant::now();

    if gate
        .get(&target)
        .is_some_and(|at| now.duration_since(*at) < TEST_ACTION_GAP)
    {
        return Err(Error::ActionRateLimited);
    }

    gate.insert(target, now);

    Ok(())
}

fn validate_pause_until(
    until: Option<chrono::DateTime<Utc>>,
    now: chrono::DateTime<Utc>,
) -> Result<()> {
    if until.is_some_and(|until| until <= now) {
        return Err(Error::InvalidRequest(
            "the pause deadline must be in the future".into(),
        ));
    }

    Ok(())
}

// ! Built from a stored `DROP_CLAIMED` event for the CSV export: the row a
// spreadsheet shows, not wire-shaped for the API on its own (see
// `HistoryExportResponse`, which already holds the finished CSV text).
struct ClaimedDropRow {
    at: DateTime<Utc>,
    account_login: String,
    // ! Empty when the stored event predates the field, or Twitch's category is unknown.
    category: String,
    campaign: String,
    drop: String,
    minutes: Option<u32>,
}

// ! RFC 4180 escaping (a value touching a comma, quote or line break is
// quoted, with any quote inside doubled), a UTF-8 BOM and CRLF endings:
// ready for a spreadsheet to open as-is. A value starting with a character
// a spreadsheet treats as a formula prefix (publisher-controlled campaign,
// drop or category names included) is quoted with a leading `'` guard too,
// per https://owasp.org/www-community/attacks/CSV_Injection.
fn claimed_drops_csv(rows: &[ClaimedDropRow]) -> String {
    fn field(value: &str) -> String {
        let guarded = value.starts_with(['=', '+', '-', '@', '\t', '\r']);

        if guarded || value.contains(['"', ',', '\n', '\r']) {
            let prefix = if guarded { "'" } else { "" };

            format!("\"{prefix}{}\"", value.replace('"', "\"\""))
        } else {
            value.to_owned()
        }
    }

    let mut csv = String::from('\u{feff}');
    csv.push_str("Date,Account,Category,Campaign,Drop,Minutes\r\n");

    for row in rows {
        csv.push_str(&field(&row.at.to_rfc3339()));
        csv.push(',');
        csv.push_str(&field(&row.account_login));
        csv.push(',');
        csv.push_str(&field(&row.category));
        csv.push(',');
        csv.push_str(&field(&row.campaign));
        csv.push(',');
        csv.push_str(&field(&row.drop));
        csv.push(',');

        if let Some(minutes) = row.minutes {
            csv.push_str(&minutes.to_string());
        }

        csv.push_str("\r\n");
    }

    csv
}

// ! A proxy the client refuses leaves these requests direct, rather than
// failing every sign-in and lookup.
fn shared_http(device_id: &str, proxy: &str, requests: &RequestLog) -> Result<TwitchHttp> {
    TwitchHttp::new(device_id.to_owned(), Some(proxy), requests.clone()).or_else(|error| {
        tracing::warn!(%error, "default proxy rejected, continuing without it");
        TwitchHttp::new(device_id.to_owned(), None, requests.clone())
    })
}

// ! A refused proxy is reported as invalid, not swallowed into a different
// error: the setting that caused it is what the user must fix.
fn proxied_http(device_id: &str, proxy: Option<&str>, requests: &RequestLog) -> Result<TwitchHttp> {
    TwitchHttp::new(device_id.to_owned(), proxy, requests.clone()).map_err(|error| {
        match (error, proxy) {
            (Error::InvalidProxy(why), _) => Error::InvalidProxy(why),
            (other, Some(_)) => Error::InvalidProxy(other.to_string()),
            (other, None) => other,
        }
    })
}

// ! Only a credential the task could not use asks for a new sign-in: a
// refused proxy is fixed in the settings.
fn failed_start_status(code: ErrorCode) -> AccountStatus {
    match code {
        ErrorCode::InvalidProxy => AccountStatus::Idle,
        _ => AccountStatus::LoginRequired,
    }
}

fn report_header(zone: chrono_tz::Tz) -> String {
    format!(
        "{} {} ({}, {}) — time zone: {}",
        crate::APP_NAME,
        crate::VERSION,
        std::env::consts::OS,
        std::env::consts::ARCH,
        zone,
    )
}

async fn blocking<T: Send + 'static>(
    work: impl FnOnce() -> Result<T> + Send + 'static,
) -> Result<T> {
    tokio::task::spawn_blocking(work)
        .await
        .map_err(|error| Error::Storage(error.to_string()))?
}

fn request_filter(query: &RequestsQuery) -> RequestFilter<'_> {
    RequestFilter {
        account_ids: query.account_ids.as_deref(),
        kinds: query.kinds.as_deref(),
        errors_only: query.errors_only.unwrap_or(false),
    }
}

fn within_lookup_max(count: usize) -> Result<()> {
    if count > CHANNELS_LOOKUP_MAX {
        return Err(Error::InvalidRequest(format!(
            "{count} channels, at most {CHANNELS_LOOKUP_MAX}"
        )));
    }

    Ok(())
}

fn within_token_line_max(line: &str) -> Result<()> {
    if line.len() > TOKEN_LINE_MAX_CHARS {
        return Err(Error::InvalidToken);
    }

    Ok(())
}

// ! Two tasks of one account must never run together: both would send the
// beacon, and the old one could save a stale session.
async fn stop_task(handle: AccountHandle) {
    handle.cancel.cancel();

    let mut task = handle.task;

    if tokio::time::timeout(TASK_STOP_TIMEOUT, &mut task)
        .await
        .is_err()
    {
        tracing::info!(account = %handle.id, "account task still busy after the stop timeout, aborted");

        task.abort();

        let _ = task.await;
    }
}
