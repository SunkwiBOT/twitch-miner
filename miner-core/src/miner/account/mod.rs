mod bets;
mod channels;
mod claims;
mod inventory;
mod points;
mod registry;
mod session;
mod setaside;
mod slots;
mod snapshot;
mod sockets;

use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use reqwest::StatusCode;
use tokio::sync::{broadcast, mpsc, oneshot, watch, RwLock};
use tokio_util::sync::CancellationToken;
use url::Url;

use super::record_event;
use super::twitch_api::TwitchApi;
use crate::api::{
    AccountSnapshot, AccountStats, AccountStatus, CampaignProgress, DropProgressView, DropState,
    SlotView, StreamerView, FOLLOWS_MAX,
};
use crate::badges::{attach_badges, mark_owned, BadgeCatalog, OwnedBadges};
use crate::catalog::Catalog;
use crate::config::{AccountSettings, ChatPresence, Mode};
use crate::domain::{
    BadgeRef, Campaign, CampaignKind, Channel, Game, PointsState, PredictionEvent, Repeat,
    TimedDrop,
};
use crate::drops_api;
use crate::error::{Error, ErrorCode, NetworkErrorKind, Result};
use crate::events::{BetResult, EventContext, EventKind, MinerEvent, MinerKind};
use crate::predictions;
use crate::scheduler::{self, QueueItem, QueueReason, SlotPurpose, StreakBonus, StreamerState};
use crate::spam::BurstSpends;
use crate::store::vault::Vault;
use crate::store::{AccountRecord, AccountSource, Credential, Store};
use crate::twitch::chat::{self, Chat, ChatMention};
use crate::twitch::drops::{Award, ClaimOutcome};
use crate::twitch::playback::{self, Playback};
use crate::twitch::points::Goal;
use crate::twitch::pubsub::{self, PubSub, PubSubEvent, RaidJoins};
use crate::twitch::quests::{Quest, QuestGroup, QuestStatus};
use crate::twitch::{
    self, TwitchClient, TwitchHttp, DIRECTORY_READS_AT_ONCE, POINTS_READS_AT_ONCE,
};
use crate::util::sleep_until_some;

use inventory::*;
use registry::ClaimRegistry;

const TICK: Duration = Duration::from_secs(20);
const INVENTORY_INTERVAL: Duration = Duration::from_secs(5 * 60);
const CHANNELS_INTERVAL: Duration = Duration::from_secs(90);
const STREAM_UP_READ: Duration = Duration::from_secs(30);
const BEACON_INTERVAL: Duration = Duration::from_secs(60);
const BEACON_UNREACHABLE_AFTER: u32 = 3;
const VALIDATE_INTERVAL: Duration = Duration::from_secs(55 * 60);
const REFUSAL_RECHECK_GAP: Duration = Duration::from_secs(60);
const FOLLOWS_INTERVAL: Duration = Duration::from_secs(30 * 60);
const POINTS_INTERVAL: Duration = Duration::from_secs(30 * 60);
const RESYNC_GAP: Duration = Duration::from_secs(60);
const GOALS_INTERVAL: Duration = Duration::from_secs(10 * 60);
const LOOKUP_TOP: usize = 8;
const LOOKUP_ROTATING: usize = 4;
const DIRECTORIES_MAX: usize = 10;
const ALLOWED_CHANNELS_MAX: usize = 60;
const CATEGORIES_READ: usize = 3;
const QUEST_OFFERS_ASKED: usize = 20;
const EQUIP_RETRY: Duration = Duration::from_secs(10 * 60);
const MENTION_COOLDOWN: Duration = Duration::from_secs(60);
const OWN_SPEND_WAIT: Duration = Duration::from_secs(2 * 60);
// ! Real minutes sent for a campaign's Drop slot without Twitch moving its
// own progress counter, before the campaign is set aside for the account.
const PROGRESS_STALL_MINUTES: u32 = 30;
// ! How long a set-aside campaign waits before Twitch is asked to credit it again.
const SET_ASIDE_RETRY: chrono::Duration = chrono::Duration::hours(6);
// ! Twitch's answer for a bonus chest another session took first, or that
// expired: nothing went wrong here.
const BONUS_GONE: [&str; 1] = ["NOT_FOUND"];
// ! Parts of Twitch's refusal codes for a bet on a prediction that closed
// before it (a streamer can lock one early): no bet, not an error.
const PREDICTION_CLOSED: [&str; 6] = [
    "LOCKED",
    "NOT_ACTIVE",
    "CLOSED",
    "ENDED",
    "RESOLVED",
    "CANCEL",
];

#[derive(Debug)]
pub enum AccountCommand {
    Settings(Box<AccountSettings>),
    Refresh,
    ClaimDrop(String, oneshot::Sender<Result<()>>),
    Balance { login: String, balance: i64 },
    Session(oneshot::Sender<Result<()>>),
    // ! Never persisted on the account: the global quiet window's own state,
    // pushed by the manager whenever it changes.
    QuietWindow(bool),
}

enum Wake {
    Bets,
    Slots,
    Command(AccountCommand),
    PubSub(PubSubEvent),
    Mention(ChatMention),
    Tick,
}

pub struct AccountHandle {
    pub id: String,
    pub commands: mpsc::Sender<AccountCommand>,
    pub snapshot: watch::Receiver<AccountSnapshot>,
    pub cancel: CancellationToken,
    pub task: tokio::task::JoinHandle<()>,
    pub client: TwitchClient,
}

struct ActiveSlot {
    channel: Channel,
    purpose: SlotPurpose,
    since: DateTime<Utc>,
    beacon: Option<Url>,
    last_beacon_at: Option<DateTime<Utc>>,
    next_beacon: tokio::time::Instant,
    playback: Option<Playback>,
    next_playback: tokio::time::Instant,
    beacon_failures: u32,
    playback_accumulated: Duration,
}

struct OwnSpend {
    amount: i64,
    at: tokio::time::Instant,
}

#[derive(Default)]
struct OwnSpends(HashMap<String, Vec<OwnSpend>>);

impl OwnSpends {
    fn add(&mut self, login: &str, amount: i64) {
        self.0.entry(login.to_owned()).or_default().push(OwnSpend {
            amount,
            at: tokio::time::Instant::now(),
        });
    }

    fn pending(&self, login: &str) -> i64 {
        self.0.get(login).map_or(0, |spends| {
            spends
                .iter()
                .filter(|spend| spend.at.elapsed() < OWN_SPEND_WAIT)
                .map(|spend| spend.amount)
                .sum()
        })
    }

    fn settle(&mut self, login: &str, amount: Option<i64>) -> Option<i64> {
        for spends in self.0.values_mut() {
            spends.retain(|spend| spend.at.elapsed() < OWN_SPEND_WAIT);
        }

        self.0.retain(|_, spends| !spends.is_empty());

        let Some(spends) = self.0.get_mut(login) else {
            return amount;
        };

        let left = match amount {
            // ! Twitch sends one message per spend: one whose amount is not
            // known is the oldest spend's.
            None => {
                spends.remove(0);

                0
            }
            Some(mut left) => {
                for spend in spends.iter_mut() {
                    let taken = spend.amount.min(left);

                    spend.amount -= taken;
                    left -= taken;
                }

                spends.retain(|spend| spend.amount > 0);

                left
            }
        };

        if spends.is_empty() {
            self.0.remove(login);
        }

        Some(left)
    }
}

struct OfferedDrops {
    broadcast_id: String,
    campaign_ids: Vec<String>,
}

pub struct AccountContext {
    pub store: Store,
    pub vault: Vault,
    pub http: Arc<TwitchHttp>,
    pub catalog: Arc<RwLock<Catalog>>,
    pub badges: Arc<RwLock<BadgeCatalog>>,
    pub install_id: String,
    pub events: broadcast::Sender<MinerEvent>,
    pub stats: AccountStats,
    pub spends: BurstSpends,
    // ! The current quiet-window state at spawn time: without it, a freshly
    // spawned or respawned account can run a full first pass mining before
    // the manager's own `QuietWindow` command catches up with it.
    pub quiet_window: bool,
}

struct AccountMiner<T> {
    id: String,
    login: String,
    display_name: String,
    avatar_url: Option<String>,
    source: AccountSource,
    settings: AccountSettings,
    context: AccountContext,
    client: T,
    credential: Credential,
    status: AccountStatus,
    problem: Option<ErrorCode>,
    reported: HashSet<ErrorCode>,
    noticed: HashMap<ErrorCode, u64>,
    passes: u64,
    stats: AccountStats,
    campaigns: HashMap<String, Campaign>,
    quests: Vec<Quest>,
    owned_badges: Vec<BadgeRef>,
    selected_badge: Option<BadgeRef>,
    equip_tried: HashMap<String, tokio::time::Instant>,
    badges_read_at: Option<DateTime<Utc>>,
    inventory_at: Option<tokio::time::Instant>,
    live_by_campaign: HashMap<String, Vec<Channel>>,
    channels_at: Option<tokio::time::Instant>,
    lookup_offset: usize,
    category_turn: usize,
    streamer_channels: HashMap<String, Channel>,
    streamer_points: HashMap<String, PointsState>,
    streak_bonuses: HashMap<String, StreakBonus>,
    own_spends: OwnSpends,
    offered_drops: HashMap<String, OfferedDrops>,
    points_at: HashMap<String, tokio::time::Instant>,
    resynced_at: Option<tokio::time::Instant>,
    pubsub_left_out: usize,
    goals_at: HashMap<String, tokio::time::Instant>,
    slots: Vec<ActiveSlot>,
    // ! Channel id to when this account last saw it offline or refused by
    // "Drops & More": a list read after that moment is trusted again.
    seen_offline_or_refused: HashMap<String, DateTime<Utc>>,
    // ! The login is known only when this account placed the bet itself
    // (`place_due_bets`); `None` when Twitch's own echo of an external bet
    // (`PredictionMade`) inserts the entry first.
    predictions_placed: HashMap<String, (String, i64, Option<String>)>,
    // ! A bet sent once is never sent again for its prediction: Twitch repeats the prediction at every update.
    bets_tried: HashSet<String>,
    claims: ClaimRegistry,
    set_aside: setaside::SetAside,
    // ! PubSub and the balance read can both bring the same chest.
    bonus_claims: HashMap<String, String>,
    beacon_blocked: bool,
    beacon_connect_failures: u32,
    beacon_blocked_reported: bool,
    raid_joins: RaidJoins,
    pending_bets: Vec<(PredictionEvent, DateTime<Utc>)>,
    last_queue: Vec<QueueItem>,
    snapshot: watch::Sender<AccountSnapshot>,
    pubsub: Option<PubSub>,
    chat: Option<Chat>,
    mentioned_at: HashMap<(String, String), tokio::time::Instant>,
    followed: HashSet<String>,
    followed_at: Option<tokio::time::Instant>,
    pubsub_events: mpsc::Receiver<PubSubEvent>,
    pubsub_sender: mpsc::Sender<PubSubEvent>,
    cancel: CancellationToken,
    session_valid_at: Option<tokio::time::Instant>,
    login_checked_at: Option<tokio::time::Instant>,
    quiet_window: bool,
}

pub fn spawn(
    record: AccountRecord,
    credential: Credential,
    context: AccountContext,
) -> AccountHandle {
    let (commands, receiver) = mpsc::channel(16);
    let cancel = CancellationToken::new();
    let client = TwitchClient::new(
        context.http.clone(),
        record.id.clone(),
        record.login.clone(),
        credential.identity.clone(),
        credential.tokens.access_token.clone(),
    );
    let (mut miner, snapshot) =
        AccountMiner::new(record, credential, context, client.clone(), cancel.clone());
    let id = miner.id.clone();
    let task = tokio::spawn(async move { miner.run(receiver).await });

    AccountHandle {
        id,
        commands,
        snapshot,
        cancel,
        task,
        client,
    }
}

impl<T: TwitchApi> AccountMiner<T> {
    fn new(
        record: AccountRecord,
        credential: Credential,
        context: AccountContext,
        client: T,
        cancel: CancellationToken,
    ) -> (Self, watch::Receiver<AccountSnapshot>) {
        let initial = AccountSnapshot {
            id: record.id.clone(),
            login: record.login.clone(),
            display_name: record.display_name.clone(),
            source: record.source,
            avatar_url: record.avatar_url.clone(),
            settings: record.settings.clone(),
            status: AccountStatus::Starting,
            problem: None,
            slots: vec![],
            queue: vec![],
            streamers: vec![],
            progress: vec![],
            other_campaigns: vec![],
            quests: vec![],
            owned_badges: vec![],
            badges_read_at: None,
            stats: context.stats.clone(),
            updated_at: Utc::now(),
            spam_channels: vec![],
        };
        let (snapshot, receiver) = watch::channel(initial);
        let (pubsub_sender, pubsub_events) = mpsc::channel(256);
        let stats = context.stats.clone();
        let quiet_window = context.quiet_window;
        let miner = Self {
            id: record.id,
            login: record.login,
            display_name: record.display_name,
            avatar_url: record.avatar_url,
            source: record.source,
            settings: record.settings,
            context,
            client,
            credential,
            status: AccountStatus::Starting,
            problem: None,
            reported: HashSet::new(),
            noticed: HashMap::new(),
            passes: 0,
            stats,
            campaigns: HashMap::new(),
            quests: Vec::new(),
            owned_badges: Vec::new(),
            selected_badge: None,
            equip_tried: HashMap::new(),
            badges_read_at: None,
            inventory_at: None,
            live_by_campaign: HashMap::new(),
            channels_at: None,
            lookup_offset: 0,
            category_turn: 0,
            streamer_channels: HashMap::new(),
            streamer_points: HashMap::new(),
            streak_bonuses: HashMap::new(),
            own_spends: OwnSpends::default(),
            offered_drops: HashMap::new(),
            points_at: HashMap::new(),
            resynced_at: None,
            pubsub_left_out: 0,
            goals_at: HashMap::new(),
            slots: Vec::new(),
            seen_offline_or_refused: HashMap::new(),
            predictions_placed: HashMap::new(),
            bets_tried: HashSet::new(),
            claims: ClaimRegistry::default(),
            set_aside: HashMap::new(),
            bonus_claims: HashMap::new(),
            beacon_blocked: false,
            beacon_connect_failures: 0,
            beacon_blocked_reported: false,
            raid_joins: RaidJoins::default(),
            pending_bets: Vec::new(),
            last_queue: Vec::new(),
            snapshot,
            pubsub: None,
            chat: None,
            mentioned_at: HashMap::new(),
            followed: HashSet::new(),
            followed_at: None,
            pubsub_events,
            quiet_window,
            pubsub_sender,
            cancel,
            session_valid_at: None,
            login_checked_at: None,
        };

        (miner, receiver)
    }

    fn emit(&mut self, kind: EventKind, title: impl Into<String>) -> MinerEvent {
        MinerEvent::new(&self.id, &self.login, kind, title)
    }

    fn illustrate(&self, event: &mut MinerEvent) {
        let context = &mut event.context;

        context.account_name = Some(self.display_name.clone());
        context.account_avatar = self.avatar_url.clone();

        if let Some(login) = context.channel.clone() {
            let channel = self
                .streamer_channels
                .get(&login)
                .or_else(|| {
                    self.slots
                        .iter()
                        .map(|s| &s.channel)
                        .find(|c| c.login == login)
                })
                .or_else(|| {
                    self.live_by_campaign
                        .values()
                        .flatten()
                        .find(|c| c.login == login)
                });

            if let Some(avatar) = channel.and_then(|c| c.avatar_url.clone()) {
                context.avatar = Some(avatar);
            }

            context.channel_name = Some(channel.map(|c| c.display_name.clone()).unwrap_or(login));
        }

        if let Some(campaign) = context
            .campaign_id
            .as_deref()
            .and_then(|id| self.campaigns.get(id))
        {
            if let Some(art) = &campaign.game.box_art_url {
                context.art = Some(art.clone());
            }

            context.game = Some(campaign.game.name.clone());
            context.game_slug = Some(campaign.game.slug()).filter(|slug| !slug.is_empty());
            context.campaign_name = Some(campaign.name.clone());

            let drop = context
                .drop_id
                .as_deref()
                .and_then(|id| campaign.drops.iter().find(|d| d.id == id));

            if let Some(drop) = drop {
                context.drop_name = Some(drop.name.clone());

                if let Some(reward) = drop.benefits.first().and_then(|b| b.image_url.clone()) {
                    context.reward = Some(reward);
                }
            }
        }
    }

    fn publish(&mut self, mut event: MinerEvent) {
        if event.context.channel.is_none() {
            if let Some(campaign_id) = event.context.campaign_id.as_deref() {
                event.context.channel = self
                    .slots
                    .iter()
                    .find(|slot| {
                        matches!(&slot.purpose, SlotPurpose::Drop { campaign_id: id, .. } if id == campaign_id)
                    })
                    .map(|slot| slot.channel.login.clone());
            }
        }

        self.illustrate(&mut event);

        // ! A quest's progress is told before the quest joins the campaigns.
        if event.context.miner.is_none() {
            let drops_campaign =
                event.context.campaign_id.as_deref().filter(|_| {
                    event.kind.miner() == Some(MinerKind::Drops) || event.kind.generic()
                });

            event.context.miner = match drops_campaign {
                Some(id) if self.quests.iter().any(|quest| quest.id == id) => {
                    Some(MinerKind::Badges)
                }
                Some(id) if self.campaigns.contains_key(id) => Some(drops_miner(
                    self.campaigns.get(id),
                    event.context.drop_id.as_deref(),
                )),
                _ if event.kind.generic() => None,
                _ => event
                    .context
                    .channel
                    .as_deref()
                    .and_then(|login| self.slots.iter().find(|s| s.channel.login == login))
                    .map(|slot| miner_of(&slot.purpose, &self.campaigns))
                    .or_else(|| event.kind.miner()),
            };
        }

        event.categorize();
        record_event(&self.context.store, &self.context.events, event);
    }

    fn on_mention(&mut self, mention: ChatMention) {
        let now = tokio::time::Instant::now();

        self.mentioned_at
            .retain(|_, at| now.duration_since(*at) < MENTION_COOLDOWN);

        let key = (mention.channel.clone(), mention.chatter.clone());

        if self.mentioned_at.contains_key(&key) {
            return;
        }

        self.mentioned_at.insert(key, now);

        let known = self
            .streamer_channels
            .get(&mention.channel)
            .or_else(|| {
                self.slots
                    .iter()
                    .map(|slot| &slot.channel)
                    .find(|channel| channel.login == mention.channel)
            })
            .map(|channel| (channel.display_name.clone(), channel.avatar_url.clone()));

        let (channel_name, avatar) = match known {
            Some((name, avatar)) => (Some(name), avatar),
            None => (None, None),
        };

        let event = self
            .emit(
                EventKind::ChatMention,
                format!(
                    "{} mentioned the account in {}'s chat",
                    mention.chatter, mention.channel
                ),
            )
            .detail(mention.text)
            .context(EventContext {
                chatter: Some(mention.chatter),
                miner: Some(MinerKind::Points),
                channel_name,
                avatar,
                ..EventContext::channel(mention.channel)
            });

        self.publish(event);
    }

    fn set_status(&mut self, status: AccountStatus, problem: Option<ErrorCode>) {
        self.status = status;
        self.problem = problem;
    }

    async fn run(&mut self, mut commands: mpsc::Receiver<AccountCommand>) {
        if !self.effectively_mining() {
            self.set_status(AccountStatus::Paused, None);
        }

        self.push_snapshot();

        let mut tick = tokio::time::interval(TICK);
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        let cancel = self.cancel.clone();

        loop {
            let next_bet = self.next_bet_at();
            let next_slot = self.next_slot_at();

            let wake = tokio::select! { biased;
                _ = cancel.cancelled() => break,
                _ = sleep_until_some(next_bet) => Wake::Bets,
                _ = sleep_until_some(next_slot) => Wake::Slots,
                command = commands.recv() => match command {
                    Some(command) => Wake::Command(command),
                    None => break,
                },
                Some(event) = self.pubsub_events.recv() => Wake::PubSub(event),
                mention = next_mention(&mut self.chat) => Wake::Mention(mention),
                _ = tick.tick() => Wake::Tick,
            };

            tokio::select! { biased;
                _ = cancel.cancelled() => break,
                _ = self.wake(wake) => {}
            }
        }

        self.stop_all_slots("stopped").await;
        self.close_sockets().await;
    }

    async fn wake(&mut self, wake: Wake) {
        match wake {
            Wake::Bets => {
                self.place_due_bets().await;
                self.push_snapshot();
            }
            Wake::Slots => {
                if self.pump_slots().await {
                    self.push_snapshot();
                }
            }
            Wake::Command(AccountCommand::Settings(settings)) => {
                self.apply_settings(*settings).await
            }
            Wake::Command(AccountCommand::QuietWindow(active)) => {
                self.apply_quiet_window(active).await
            }
            Wake::Command(AccountCommand::Refresh) => {
                self.inventory_at = None;
                self.channels_at = None;
                self.tick().await;
            }
            Wake::Command(AccountCommand::ClaimDrop(instance, reply)) => {
                let _ = reply.send(self.claim_instance(&instance).await);
            }
            Wake::Command(AccountCommand::Balance { login, balance }) => {
                self.balance_read(&login, balance)
            }
            Wake::Command(AccountCommand::Session(reply)) => {
                if self.status == AccountStatus::LoginRequired {
                    let _ = reply.send(Err(Error::Unauthorized));

                    return;
                }

                let checked = self.ensure_session().await;

                if matches!(checked, Err(Error::Unauthorized)) {
                    self.require_login().await;
                    self.push_snapshot();
                }

                let _ = reply.send(checked);
            }
            Wake::PubSub(event) => {
                let mut shown = shows(&event);

                self.handle_pubsub(event).await;

                for _ in 0..self.pubsub_events.len() {
                    let Ok(event) = self.pubsub_events.try_recv() else {
                        break;
                    };

                    shown |= shows(&event);
                    self.handle_pubsub(event).await;
                }

                if shown {
                    self.push_snapshot();
                }
            }
            Wake::Mention(mention) => self.on_mention(mention),
            Wake::Tick => self.tick().await,
        }
    }

    async fn serve_waiting(&mut self) {
        self.place_due_bets().await;

        // ! Only what is queued now: a busy socket does not hold the pass.
        for _ in 0..self.pubsub_events.len() {
            let Ok(event) = self.pubsub_events.try_recv() else {
                break;
            };

            self.handle_pubsub(event).await;
        }
    }

    async fn apply_settings(&mut self, settings: AccountSettings) {
        let was_mining = self.effectively_mining();
        let channels_changed = watched_logins(&self.settings) != watched_logins(&settings)
            || self.settings.mode != settings.mode
            || self.settings.slots != settings.slots;

        self.settings = settings;
        self.apply_mining_transition(was_mining).await;

        if channels_changed {
            self.channels_at = None;
        }

        self.tick().await;
    }

    async fn apply_quiet_window(&mut self, active: bool) {
        if self.quiet_window == active {
            return;
        }

        let was_mining = self.effectively_mining();

        self.quiet_window = active;
        self.apply_mining_transition(was_mining).await;
        self.tick().await;
    }

    // ! Shared by a settings save and the quiet window edge: flips status
    // only on an actual mining-state change, and never while waiting for
    // sign-in. Doing so would hide the prompt behind "Paused" and, once the
    // window lifted, send the account through `require_login` again and
    // re-notify `SessionExpired` for a login it is still waiting on.
    async fn apply_mining_transition(&mut self, was_mining: bool) {
        if self.effectively_mining() == was_mining || self.status == AccountStatus::LoginRequired {
            return;
        }

        if self.effectively_mining() {
            self.set_status(AccountStatus::Starting, None);
        } else {
            self.stop_all_slots("paused").await;
            self.set_status(AccountStatus::Paused, None);
        }
    }

    // ! `enabled` is the manual switch; a scheduled pause or the global quiet
    // window stop mining too, independently of it.
    fn effectively_mining(&self) -> bool {
        self.settings.enabled
            && !self.quiet_window
            && !self
                .settings
                .paused_until
                .is_some_and(|until| Utc::now() < until)
    }

    async fn tick(&mut self) {
        self.passes = self.passes.wrapping_add(1);

        if self.status != AccountStatus::LoginRequired {
            self.problem = None;
        }

        if !self.effectively_mining() {
            self.push_snapshot();

            return;
        }

        if self.status == AccountStatus::LoginRequired && !self.session_back().await {
            self.push_snapshot();

            return;
        }

        if let Err(error) = self.ensure_session().await {
            match error {
                Error::Unauthorized => self.require_login().await,
                other => self.report_problem(other),
            }

            self.push_snapshot();

            return;
        }

        let now = tokio::time::Instant::now();

        if self
            .inventory_at
            .is_none_or(|at| now.duration_since(at) >= INVENTORY_INTERVAL)
        {
            if let Err(error) = self.refresh_inventory().await {
                self.handle_error(error).await;
            } else {
                self.inventory_at = Some(now);
            }

            self.serve_waiting().await;
        }

        if self
            .channels_at
            .is_none_or(|at| now.duration_since(at) >= CHANNELS_INTERVAL)
        {
            if let Err(error) = self.refresh_channels().await {
                self.handle_error(error).await;
            } else {
                self.channels_at = Some(now);
            }

            self.serve_waiting().await;
        }

        // ! A sign-in asked by a step ends the pass: the steps after it would
        // watch again and reopen the sockets `require_login` closed.
        if self.status == AccountStatus::LoginRequired {
            self.push_snapshot();

            return;
        }

        self.apply_plan(Utc::now()).await;
        self.refresh_follows(now).await;
        self.pump_slots().await;
        self.serve_waiting().await;
        self.contribute_goals(now).await;
        self.serve_waiting().await;
        self.claim_available_drops().await;

        if self.status == AccountStatus::LoginRequired {
            self.push_snapshot();

            return;
        }

        self.sync_pubsub();
        self.sync_chat();

        let status = if self.slots.is_empty() {
            AccountStatus::Idle
        } else {
            AccountStatus::Mining
        };

        if self.problem.is_none() {
            self.reported.clear();
            self.noticed.clear();
        }

        self.set_status(status, self.problem);
        self.push_snapshot();
    }

    async fn handle_error(&mut self, error: Error) {
        match error {
            Error::Unauthorized => self.session_refused().await,
            other => self.report_problem(other),
        }
    }

    // ! As for PubSub and chat: a Twitch hiccup that the next poll clears is
    // no news to the operator, only one that comes back is.
    fn report_problem(&mut self, error: Error) {
        let transient = twitch::transient(&error);

        self.report(error, transient);
    }

    fn report(&mut self, error: Error, transient: bool) {
        let code = error.code();

        match self.verdict(code, transient) {
            tracing::Level::WARN => tracing::warn!(account = %self.login, "{error}"),
            tracing::Level::INFO => tracing::info!(account = %self.login, "{error}"),
            _ => tracing::debug!(account = %self.login, "{error}"),
        }

        self.set_problem(code);
    }

    fn verdict(&mut self, code: ErrorCode, transient: bool) -> tracing::Level {
        match self.noticed.get(&code).copied() {
            Some(pass) if transient && pass == self.passes => tracing::Level::DEBUG,
            Some(_) => {
                self.noticed.remove(&code);

                tracing::Level::WARN
            }
            None if self.reported.contains(&code) => tracing::Level::DEBUG,
            None if transient => {
                self.noticed.insert(code, self.passes);

                tracing::Level::INFO
            }
            None => tracing::Level::WARN,
        }
    }

    fn set_problem(&mut self, code: ErrorCode) {
        self.reported.insert(code);

        if self.problem != Some(ErrorCode::BeaconBlocked) {
            self.problem = Some(code);
        }
    }
}

fn watched_logins(settings: &AccountSettings) -> BTreeSet<&str> {
    settings
        .points
        .streamers
        .iter()
        .filter(|streamer| streamer.enabled)
        .map(|streamer| streamer.login.as_str())
        .collect()
}

fn shows(event: &PubSubEvent) -> bool {
    !matches!(
        event,
        PubSubEvent::ViewCount { .. } | PubSubEvent::Reconnected
    )
}

// ! A badge is mined in a Drops slot: its watch and its events say so, for
// the status, the activity card and the journal.
fn drops_miner(campaign: Option<&Campaign>, drop_id: Option<&str>) -> MinerKind {
    if campaign.is_some_and(|campaign| campaign.mined_for_badge(drop_id)) {
        MinerKind::Badges
    } else {
        MinerKind::Drops
    }
}

fn miner_of(purpose: &SlotPurpose, campaigns: &HashMap<String, Campaign>) -> MinerKind {
    match purpose {
        SlotPurpose::Drop {
            campaign_id,
            drop_id,
        } => drops_miner(campaigns.get(campaign_id), Some(drop_id)),
        SlotPurpose::Points { .. } => MinerKind::Points,
    }
}

async fn next_mention(chat: &mut Option<Chat>) -> ChatMention {
    match chat {
        Some(chat) => chat.next_mention().await,
        None => std::future::pending().await,
    }
}

// ! Counted from when the last one was due, so a late pass does not stretch the minute.
fn after(
    due: tokio::time::Instant,
    interval: Duration,
    now: tokio::time::Instant,
) -> tokio::time::Instant {
    let next = due + interval;

    if next <= now {
        now + interval
    } else {
        next
    }
}
