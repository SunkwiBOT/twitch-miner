use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use chrono::{DateTime, Utc};

use super::{
    bucket_end, bucket_of, Pending, RequestCursor, RequestEntry, RequestFilter, RequestGroup,
    RequestKind, RequestOutcome, RequestPage, RequestSource, SocketEvent, LIVE_CAPACITY,
};
use crate::error::Result;
use crate::logs::redact;
use crate::store::Store;

const UNSAVED_MAX: usize = 10_000;
const OPENED_KEPT: usize = 6;
const DETAIL_MAX_CHARS: usize = 160;

fn clipped(text: &str) -> String {
    let redacted = redact(text).into_owned();

    if redacted.chars().count() <= DETAIL_MAX_CHARS {
        return redacted;
    }

    let mut cut: String = redacted.chars().take(DETAIL_MAX_CHARS - 1).collect();

    cut.push('…');

    cut
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GroupKey {
    account_id: Option<String>,
    kind: RequestKind,
    target: String,
    socket: Option<SocketEvent>,
    outcome: RequestOutcome,
    status: Option<u16>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Sum {
    count: u32,
    total_ms: u64,
    max_ms: u64,
}

#[derive(Default)]
struct State {
    live: VecDeque<RequestEntry>,
    next_id: u64,
    bucket: Option<DateTime<Utc>>,
    opened: VecDeque<(DateTime<Utc>, u64)>,
    groups: HashMap<GroupKey, Sum>,
    failures: Vec<RequestEntry>,
    unsaved: VecDeque<RequestEntry>,
    saving_fails: bool,
    merged: HashMap<GroupKey, (u64, u64)>,
}

impl State {
    fn grouped(&mut self, bucket: DateTime<Utc>) -> Vec<RequestEntry> {
        self.groups
            .drain()
            .map(|(key, sum)| RequestEntry {
                id: 0,
                source: RequestSource::Stored,
                at: bucket,
                account_id: key.account_id,
                kind: key.kind,
                target: key.target,
                socket: key.socket,
                outcome: key.outcome,
                status: key.status,
                detail: None,
                duration_ms: sum.total_ms / u64::from(sum.count.max(1)),
                group: Some(RequestGroup {
                    count: sum.count,
                    max_ms: sum.max_ms,
                    until: bucket_end(bucket),
                }),
            })
            .collect()
    }

    fn roll(&mut self, now: DateTime<Utc>) -> Vec<RequestEntry> {
        let current = bucket_of(now);
        let closed = match self.bucket {
            Some(open) if open < current => {
                let closed = self.grouped(open);

                self.failures.clear();
                self.merged.clear();

                closed
            }
            Some(_) => return Vec::new(),
            None => Vec::new(),
        };

        self.bucket = Some(current);
        self.opened.push_back((current, self.next_id));

        if self.opened.len() > OPENED_KEPT {
            self.opened.pop_front();
        }

        closed
    }

    fn opened_with(&self, live_id: u64) -> Option<DateTime<Utc>> {
        self.opened
            .iter()
            .rev()
            .find(|(_, first_id)| *first_id <= live_id)
            .map(|(start, _)| *start)
    }

    fn keep_unsaved(&mut self, rows: Vec<RequestEntry>) {
        self.unsaved.extend(rows);

        let over = self.unsaved.len().saturating_sub(UNSAVED_MAX);

        if over > 0 {
            self.unsaved.drain(..over);
            tracing::warn!(rows = over, "requests dropped: they could not be saved");
        }
    }

    // ! Successes add to their live row like the stored groups do, or balance reads would push all else out of the ring.
    fn merge_live(&mut self, key: &GroupKey, done: &Done, now: DateTime<Utc>) -> bool {
        let Some((id, total_ms)) = self.merged.get_mut(key) else {
            return false;
        };
        let Ok(index) = self.live.binary_search_by_key(id, |entry| entry.id) else {
            return false;
        };
        let row = &mut self.live[index];
        let (count, max_ms) = row
            .group
            .as_ref()
            .map_or((1, row.duration_ms), |group| (group.count, group.max_ms));
        let count = count.saturating_add(1);

        *total_ms = total_ms.saturating_add(done.duration_ms);
        row.duration_ms = *total_ms / u64::from(count);
        row.group = Some(RequestGroup {
            count,
            max_ms: max_ms.max(done.duration_ms),
            until: now,
        });

        true
    }

    // ! Live rows are the bucket's unstored ones; a failure pushed out of the ring still shows from the bucket's list.
    fn fresh(&self, since: DateTime<Utc>) -> Vec<RequestEntry> {
        let oldest_live = self.live.front().map_or(u64::MAX, |entry| entry.id);
        let mut fresh: Vec<RequestEntry> = self
            .failures
            .iter()
            .filter(|entry| entry.id < oldest_live)
            .chain(self.live.iter().filter(|entry| entry.at >= since))
            .cloned()
            .collect();

        fresh.sort_by_key(|entry| std::cmp::Reverse(entry.id));

        fresh
    }
}

struct Shared {
    state: Mutex<State>,
    store: Option<Store>,
    // ! The last row stored before this run: after a restart, the open bucket's rows are stored ones, not live.
    earlier_run: u64,
    wake: Option<SyncSender<()>>,
    writing: Mutex<()>,
}

impl Shared {
    fn state(&self) -> MutexGuard<'_, State> {
        crate::util::locked(&self.state)
    }

    fn wake(&self) {
        if let Some(wake) = &self.wake {
            let _ = wake.try_send(());
        }
    }

    fn save(&self) {
        let Some(store) = &self.store else {
            return;
        };
        let _one_writer = crate::util::locked(&self.writing);
        let rows: Vec<RequestEntry> = self.state().unsaved.drain(..).collect();

        if rows.is_empty() {
            return;
        }

        let saved = store.push_requests(&rows);
        let mut state = self.state();

        match saved {
            Ok(()) => state.saving_fails = false,
            Err(error) => {
                if !std::mem::replace(&mut state.saving_fails, true) {
                    tracing::warn!(%error, rows = rows.len(), "requests not saved, tried again at the next tick");
                }

                let newer: Vec<RequestEntry> = state.unsaved.drain(..).collect();

                state.keep_unsaved(rows);
                state.keep_unsaved(newer);
            }
        }
    }
}

// ! SQLite is written here, off the async workers and the state's lock: a slow disk never delays a request.
fn write_when_woken(shared: Weak<Shared>, woken: Receiver<()>) {
    while woken.recv().is_ok() {
        let Some(shared) = shared.upgrade() else {
            return;
        };

        shared.save();
    }
}

#[derive(Clone)]
pub struct RequestLog {
    shared: Arc<Shared>,
}

impl Default for RequestLog {
    fn default() -> Self {
        Self::new(None)
    }
}

impl RequestLog {
    pub fn new(store: Option<Store>) -> Self {
        let (wake, woken) = match store {
            Some(_) => {
                let (wake, woken) = sync_channel(1);

                (Some(wake), Some(woken))
            }
            None => (None, None),
        };
        let earlier_run = store
            .as_ref()
            .map_or(Ok(0), Store::last_request_id)
            .unwrap_or_else(|error| {
                tracing::warn!(%error, "the requests of the previous run stay hidden until their bucket closes");
                0
            });
        let shared = Arc::new(Shared {
            state: Mutex::new(State::default()),
            store,
            earlier_run,
            wake,
            writing: Mutex::new(()),
        });

        if let Some(woken) = woken {
            let writer = Arc::downgrade(&shared);
            let spawned = std::thread::Builder::new()
                .name("requests".into())
                .spawn(move || write_when_woken(writer, woken));

            if let Err(error) = spawned {
                tracing::warn!(%error, "requests are saved only when read");
            }
        }

        Self { shared }
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.shared.state()
    }

    fn keep(&self, state: &mut State, rows: Vec<RequestEntry>) -> bool {
        if self.shared.store.is_none() || rows.is_empty() {
            return false;
        }

        state.keep_unsaved(rows);

        true
    }

    pub fn start(&self, kind: RequestKind, target: impl Into<String>) -> Pending {
        Pending::new(self.clone(), kind, target.into())
    }

    pub(super) fn record(&self, done: Done) {
        self.record_at(done, Utc::now());
    }

    fn record_at(&self, done: Done, now: DateTime<Utc>) {
        let target = redact(&done.target).into_owned();
        let detail = done.detail.as_deref().map(clipped);
        let grouped = done.kind == RequestKind::Segment || done.outcome == RequestOutcome::Ok;
        let mut state = self.state();
        let closed = state.roll(now);
        let mut unsaved = self.keep(&mut state, closed);

        let key = GroupKey {
            account_id: done.account_id.clone(),
            kind: done.kind,
            target: target.clone(),
            socket: done.socket,
            outcome: done.outcome,
            status: done.status,
        };

        if grouped {
            let sum = state.groups.entry(key.clone()).or_default();

            sum.count = sum.count.saturating_add(1);
            sum.total_ms = sum.total_ms.saturating_add(done.duration_ms);
            sum.max_ms = sum.max_ms.max(done.duration_ms);
        }

        if done.kind == RequestKind::Segment || (grouped && state.merge_live(&key, &done, now)) {
            drop(state);

            if unsaved {
                self.shared.wake();
            }

            return;
        }

        let id = state.next_id;

        state.next_id += 1;

        let entry = RequestEntry {
            id,
            source: RequestSource::Live,
            at: now,
            account_id: done.account_id,
            kind: done.kind,
            target,
            socket: done.socket,
            outcome: done.outcome,
            status: done.status,
            detail,
            duration_ms: done.duration_ms,
            group: None,
        };

        if grouped {
            state.merged.insert(key, (id, entry.duration_ms));
        } else {
            unsaved |= self.keep(&mut state, vec![entry.clone()]);
            state.failures.push(entry.clone());
        }

        if state.live.len() == LIVE_CAPACITY {
            state.live.pop_front();
        }

        state.live.push_back(entry);
        drop(state);

        if unsaved {
            self.shared.wake();
        }
    }

    // ! Waits for a save on its way, or rows it took would be written after the account's rows are deleted.
    pub fn forget(&self, account_id: &str) {
        let _one_writer = crate::util::locked(&self.shared.writing);
        let mut state = self.state();
        let other = |entry: &RequestEntry| entry.account_id.as_deref() != Some(account_id);
        let other_key = |key: &GroupKey| key.account_id.as_deref() != Some(account_id);

        state.live.retain(other);
        state.failures.retain(other);
        state.unsaved.retain(other);
        state.groups.retain(|key, _| other_key(key));
        state.merged.retain(|key, _| other_key(key));
    }

    pub fn close_due(&self) {
        let mut state = self.state();
        let closed = state.roll(Utc::now());

        self.keep(&mut state, closed);

        let unsaved = !state.unsaved.is_empty();

        drop(state);

        if unsaved {
            self.shared.wake();
        }
    }

    pub fn flush(&self) {
        let mut state = self.state();

        if let Some(bucket) = state.bucket {
            let open = state.grouped(bucket);

            self.keep(&mut state, open);
        }

        drop(state);
        self.shared.save();
    }

    pub fn page(
        &self,
        filter: &RequestFilter,
        cursor: RequestCursor,
        limit: usize,
    ) -> Result<RequestPage> {
        self.shared.save();

        let (fresh, since) = {
            let state = self.state();
            let open = state.bucket.unwrap_or_else(|| bucket_of(Utc::now()));
            // ! A page going down the live list stays in its first bucket: a bucket closed since is on screen already.
            let since = match cursor {
                RequestCursor {
                    live_id: Some(before),
                    stored_id: None,
                } => state.opened_with(before).unwrap_or(open),
                _ => open,
            };

            (state.fresh(since), since)
        };
        let mut page = RequestPage::default();

        if cursor.stored_id.is_none() {
            let mut live = fresh
                .into_iter()
                .filter(|entry| cursor.live_id.is_none_or(|before| entry.id < before))
                .filter(|entry| filter.keeps(entry));

            page.requests.extend(live.by_ref().take(limit));

            if page.requests.len() == limit {
                if live.next().is_some() {
                    page.next.live_id = page.requests.last().map(|entry| entry.id);
                } else {
                    page.next.stored_id = self
                        .stored(filter, since, None, 1)?
                        .0
                        .first()
                        .map(|entry| entry.id.saturating_add(1));
                }

                return Ok(page);
            }
        }

        let room = limit - page.requests.len();
        let (stored, next) = self.stored(filter, since, cursor.stored_id, room)?;

        page.requests.extend(stored);
        page.next.stored_id = next;

        Ok(page)
    }

    fn stored(
        &self,
        filter: &RequestFilter,
        until: DateTime<Utc>,
        before_id: Option<u64>,
        limit: usize,
    ) -> Result<(Vec<RequestEntry>, Option<u64>)> {
        match &self.shared.store {
            Some(store) => store.requests(filter, until, self.shared.earlier_run, before_id, limit),
            None => Ok((Vec::new(), None)),
        }
    }

    pub fn report(
        &self,
        filter: &RequestFilter,
        logins: &HashMap<String, String>,
        limit: usize,
    ) -> Result<String> {
        let page = self.page(filter, RequestCursor::default(), limit)?;

        Ok(page
            .requests
            .iter()
            .rev()
            .map(|entry| {
                let login = entry
                    .account_id
                    .as_ref()
                    .and_then(|id| logins.get(id))
                    .map(String::as_str);

                entry.report_line(login)
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
}

pub(super) struct Done {
    pub(super) account_id: Option<String>,
    pub(super) kind: RequestKind,
    pub(super) target: String,
    pub(super) socket: Option<SocketEvent>,
    pub(super) outcome: RequestOutcome,
    pub(super) status: Option<u16>,
    pub(super) detail: Option<String>,
    pub(super) duration_ms: u64,
}
