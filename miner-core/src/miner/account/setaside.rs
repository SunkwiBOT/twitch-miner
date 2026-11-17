use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};

use crate::domain::Campaign;

use super::{PROGRESS_STALL_MINUTES, SET_ASIDE_RETRY};

pub(super) type SetAside = HashMap<String, CampaignWatch>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct CampaignWatch {
    fingerprint: Option<(String, u32)>,
    minutes_without_progress: u32,
    watched_since_observed: u32,
    set_aside_until: Option<DateTime<Utc>>,
    pending_retry: bool,
}

impl CampaignWatch {
    fn record_minute(&mut self) {
        self.watched_since_observed = self.watched_since_observed.saturating_add(1);
    }

    // ! A lift (natural expiry, fresh progress, or a finished sibling of the
    // same category) flags a pending retry: the next time this campaign
    // gets a Drop slot, that slot assignment clears the flag and announces
    // the retry, however many plans it waited behind another campaign first.
    fn lift(&mut self) {
        if self.set_aside_until.take().is_some() {
            self.pending_retry = true;
        }
    }

    // ! A change in the drop or its minutes is real progress: it clears a
    // pending or active set-aside and resets the stall count.
    fn observe(
        &mut self,
        drop_id: &str,
        current_minutes: u32,
        now: DateTime<Utc>,
    ) -> Option<DateTime<Utc>> {
        let fingerprint = Some((drop_id.to_owned(), current_minutes));
        let watched = std::mem::take(&mut self.watched_since_observed);

        if self.fingerprint != fingerprint {
            self.fingerprint = fingerprint;
            self.minutes_without_progress = 0;
            self.lift();

            return None;
        }

        if self.set_aside_until.is_some_and(|at| at > now) {
            return None;
        }

        self.lift();
        self.minutes_without_progress += watched;

        if self.minutes_without_progress < PROGRESS_STALL_MINUTES {
            return None;
        }

        let retry_at = now + SET_ASIDE_RETRY;

        self.set_aside_until = Some(retry_at);
        self.minutes_without_progress = 0;

        Some(retry_at)
    }
}

pub(super) fn record_minute(set_aside: &mut SetAside, campaign_id: &str) {
    set_aside
        .entry(campaign_id.to_owned())
        .or_default()
        .record_minute();
}

// ! Consumed once, by whichever plan first gives this campaign a Drop slot:
// a campaign still queued behind another one keeps the flag across every
// plan in between.
pub(super) fn take_pending_retry(set_aside: &mut SetAside, campaign_id: &str) -> bool {
    set_aside
        .get_mut(campaign_id)
        .is_some_and(|watch| std::mem::take(&mut watch.pending_retry))
}

// ! Every known campaign's own progress is watched here, not just the one
// currently mined: a repeated drop's period or a quest's window can move
// while this account briefly is not the one credited. `stale_quests` names
// quest campaigns whose quests were not freshly read this pass: their
// `watched_since_observed` accumulates instead of being judged on stale data.
pub(super) fn observe_all(
    set_aside: &mut SetAside,
    campaigns: &HashMap<String, Campaign>,
    excluded: &[String],
    priority: &[crate::domain::BenefitKind],
    stale_quests: &HashSet<String>,
    lifted_categories: &HashSet<String>,
    now: DateTime<Utc>,
) -> Vec<(String, String, DateTime<Utc>)> {
    set_aside.retain(|id, _| campaigns.contains_key(id));

    // ! If Twitch credited a sibling campaign of the same category instead,
    // and that sibling just finished in this pass (a real transition, not
    // one still finished from before), there is nothing left for this one
    // to wait for: lift its set-aside now. Applied before the stall
    // evaluation below, so a set-aside just made in this very pass stays.
    for (campaign_id, watch) in set_aside.iter_mut() {
        if watch.set_aside_until.is_none() {
            continue;
        }

        let Some(campaign) = campaigns.get(campaign_id) else {
            continue;
        };

        if !lifted_categories.contains(&campaign.game.id) {
            continue;
        }

        watch.lift();
        watch.minutes_without_progress = 0;
    }

    let newly_set_aside: Vec<(String, String, DateTime<Utc>)> = campaigns
        .values()
        .filter(|campaign| !stale_quests.contains(&campaign.id))
        .filter_map(|campaign| {
            let drop = campaign.next_drop(now, excluded, priority)?;
            let retry_at = set_aside.entry(campaign.id.clone()).or_default().observe(
                &drop.id,
                drop.current_minutes,
                now,
            )?;

            Some((campaign.id.clone(), drop.id.clone(), retry_at))
        })
        .collect();

    newly_set_aside
}

// ! Called from `apply_plan`, right before `active()` reads the live clock:
// a retry time already passed is lifted here, flagging a pending retry, so
// the very same plan that reclaims the slot also announces it. Without
// this, the slot could be reclaimed up to a tick (20 s) before the next
// inventory read (up to 5 min later) would otherwise call `lift()` through
// `observe()`, and the retry would surface late, on an unrelated plan.
pub(super) fn expire(set_aside: &mut SetAside, now: DateTime<Utc>) {
    for watch in set_aside.values_mut() {
        if watch.set_aside_until.is_some_and(|at| at <= now) {
            watch.lift();
        }
    }
}

// ! Filtered live by `now`: an expired entry stops blocking immediately,
// not at the next inventory read.
pub(super) fn active(set_aside: &SetAside, now: DateTime<Utc>) -> HashMap<String, DateTime<Utc>> {
    set_aside
        .iter()
        .filter_map(|(id, watch)| {
            watch
                .set_aside_until
                .filter(|at| *at > now)
                .map(|at| (id.clone(), at))
        })
        .collect()
}
