use super::*;

// ! One real minute of Drops progress, accumulated from playback polls that
// returned segments: independent of the beacon timer, which can back off to
// once every 10 minutes while still blocked.
const WATCHED_MINUTE: Duration = Duration::from_secs(60);

impl<T: TwitchApi> AccountMiner<T> {
    pub(super) async fn apply_plan(&mut self, now: DateTime<Utc>) {
        self.settle_streak_bonuses(now);

        let campaigns: Vec<Campaign> = self.campaigns.values().cloned().collect();
        setaside::expire(&mut self.set_aside, now);
        let aside = setaside::active(&self.set_aside, now);
        let empty = PointsState::default();
        let states: Vec<StreamerState<'_>> = self
            .settings
            .points
            .streamers
            .iter()
            .map(|s| StreamerState {
                settings: s,
                channel: self.streamer_channels.get(&s.login),
                points: self.streamer_points.get(&s.login).unwrap_or(&empty),
                streak_bonus: self.streak_bonuses.get(&s.login),
            })
            .collect();
        let current = self.slots.iter().find_map(|s| match &s.purpose {
            SlotPurpose::Drop { campaign_id, .. } => Some(scheduler::CurrentDrop {
                campaign_id,
                channel_id: &s.channel.id,
            }),
            SlotPurpose::Points { .. } => None,
        });
        let holds = scheduler::Holds {
            aside: &aside,
            drops_held: self.drops_hold.flagged(),
        };
        let mut plan = scheduler::plan(
            &self.settings,
            &campaigns,
            &holds,
            &self.live_by_campaign,
            &states,
            current,
            now,
        );

        for item in &mut plan.queue {
            if item.blocked == Some(scheduler::QueueBlock::SetAside) {
                item.set_aside_stall_minutes = Some(PROGRESS_STALL_MINUTES);
            }
        }

        self.last_queue = plan.queue.clone();
        self.last_left_out = plan.left_out.clone();
        self.tell_passed_over(&plan.passed_over);
        self.equip_for(&plan).await;

        // ! Twitch pays the streak bonus to a Drops watch too: the bonus window counts
        // from the first minute any slot watches this broadcast.
        for slot in &plan.slots {
            if let Some(bonus) = self.streak_bonuses.get_mut(&slot.channel.login) {
                bonus.watched_since.get_or_insert(now);
            }
        }

        let mut index = 0;

        while index < self.slots.len() {
            let wanted = plan.slots.iter().find(|wanted| {
                wanted.channel.id == self.slots[index].channel.id
                    && self.slots[index].purpose.keeps_stream(&wanted.purpose)
            });

            match wanted {
                Some(wanted) => {
                    let told = !self.slots[index].purpose.same_watch(&wanted.purpose);
                    let live = self.slots[index].channel.live;

                    self.slots[index].channel = wanted.channel.clone();
                    self.slots[index].channel.live &= live;
                    self.slots[index].purpose = wanted.purpose.clone();

                    if told {
                        if let SlotPurpose::Drop {
                            campaign_id,
                            drop_id,
                        } = &wanted.purpose
                        {
                            if setaside::take_pending_retry(&mut self.set_aside, campaign_id) {
                                self.retry_campaign(campaign_id, drop_id);
                            }
                        }

                        let event = self.watch_started(&wanted.channel, &wanted.purpose);

                        self.publish(event);
                    }

                    index += 1;
                }
                None => {
                    let watched = &self.slots[index];
                    let live = watched.channel.live
                        && self
                            .streamer_channels
                            .get(&watched.channel.login)
                            .is_none_or(|channel| channel.live);
                    let reason = stop_reason(watched, live, &plan, &campaigns, now);
                    let slot = self.slots.remove(index);
                    let event = self
                        .emit(
                            EventKind::WatchStopped,
                            format!("Stopped watching {} ({reason})", slot.channel.display_name),
                        )
                        .context(EventContext {
                            avatar: slot.channel.avatar_url.clone(),
                            miner: Some(miner_of(&slot.purpose, &self.campaigns)),
                            ..EventContext::channel(&slot.channel.login)
                        });

                    self.publish(event);
                }
            }
        }

        for slot in plan.slots {
            if self.slots.iter().any(|s| s.channel.id == slot.channel.id) {
                continue;
            }

            if let SlotPurpose::Drop {
                campaign_id,
                drop_id,
            } = &slot.purpose
            {
                if setaside::take_pending_retry(&mut self.set_aside, campaign_id) {
                    self.retry_campaign(campaign_id, drop_id);
                }
            }

            let event = self.watch_started(&slot.channel, &slot.purpose);

            self.publish(event);

            let playback = Some(Playback::new(&slot.channel.login));

            self.slots.push(ActiveSlot {
                channel: slot.channel,
                purpose: slot.purpose,
                since: now,
                beacon: None,
                last_beacon_at: None,
                next_beacon: tokio::time::Instant::now(),
                playback,
                next_playback: tokio::time::Instant::now(),
                beacon_failures: 0,
                playback_accumulated: Duration::ZERO,
            });
        }
    }

    fn retry_campaign(&mut self, campaign_id: &str, drop_id: &str) {
        let name = self
            .campaigns
            .get(campaign_id)
            .map(|c| c.name.clone())
            .unwrap_or_default();

        tracing::info!(account = %self.login, campaign_id, "campaign retried after being set aside");

        let event = self
            .emit(EventKind::CampaignRetried, format!("Retrying {name}"))
            .context(EventContext::drop(campaign_id, drop_id));

        self.publish(event);
    }

    pub(super) fn set_aside_campaign(
        &mut self,
        campaign_id: &str,
        drop_id: &str,
        retry_at: DateTime<Utc>,
    ) {
        let name = self
            .campaigns
            .get(campaign_id)
            .map(|c| c.name.clone())
            .unwrap_or_default();

        tracing::info!(
            account = %self.login,
            campaign_id,
            retry_at = %retry_at,
            "campaign set aside: no progress in {PROGRESS_STALL_MINUTES} real minutes",
        );

        let event = self
            .emit(
                EventKind::CampaignSetAside,
                format!(
                    "{name} is set aside: no progress in {PROGRESS_STALL_MINUTES} minutes, retrying at {} UTC",
                    retry_at.format("%Y-%m-%d %H:%M"),
                ),
            )
            .context(EventContext {
                required: Some(PROGRESS_STALL_MINUTES),
                set_aside_until: Some(retry_at),
                ..EventContext::drop(campaign_id, drop_id)
            });

        self.publish(event);
    }

    fn tell_passed_over(&mut self, passed_over: &[PassedOver]) {
        // ! A campaign no longer passed over is forgotten: passed over again
        // later, its reason is told again, even the same one.
        self.passed_over_told.retain(|campaign_id, _| {
            passed_over
                .iter()
                .any(|passed| &passed.campaign_id == campaign_id)
        });

        for PassedOver { campaign_id, block } in passed_over {
            if self.passed_over_told.insert(campaign_id.clone(), *block) == Some(*block) {
                continue;
            }

            let name = self
                .campaigns
                .get(campaign_id)
                .map(|c| c.name.clone())
                .unwrap_or_default();
            let title = match block {
                QueueBlock::NoLiveChannel => {
                    format!("{name} is not mined: none of its channels is live in its category")
                }
                QueueBlock::OneCampaignAtATime => {
                    format!("{name} waits: only one campaign is mined at a time")
                }
                QueueBlock::DropsHeld => {
                    format!("{name} waits: this account's Drops are held by Twitch")
                }
                // ! Already told by its own event.
                QueueBlock::SetAside => continue,
            };
            let event = self.emit(EventKind::Info, title).context(EventContext {
                queue_block: Some(*block),
                ..EventContext::campaign(campaign_id)
            });

            self.publish(event);
        }
    }

    // ! Twitch counts an evolution only while the badge it evolves from is equipped, and a group on
    // a track only while its lane is the one equipped in the Drops panel. The old badge is never
    // put back; another lane is equipped only once the equipped one has nothing left to earn.
    pub(super) async fn equip_for(&mut self, plan: &scheduler::Plan) {
        let Some((campaign_id, drop)) = plan.slots.iter().find_map(|slot| match &slot.purpose {
            SlotPurpose::Drop {
                campaign_id,
                drop_id,
            } => {
                let drop = self
                    .campaigns
                    .get(campaign_id)?
                    .drops
                    .iter()
                    .find(|d| &d.id == drop_id)?;

                Some((campaign_id.clone(), drop.clone()))
            }
            SlotPurpose::Points { .. } => None,
        }) else {
            return;
        };
        let badge = drop
            .evolution
            .as_ref()
            .and_then(|evolution| evolution.from.clone())
            .filter(|_| self.settings.badges.evolutions);

        if let Some(badge) = badge {
            if !self.equip_badge(&campaign_id, &drop, badge).await {
                return;
            }
        }

        if let Some(lane) = drop.lane.clone().filter(|l| !l.equipped && !l.refused) {
            self.equip_lane(&campaign_id, &drop, lane).await;
        }
    }

    // ! `false` once the session was refused: nothing else is sent for it.
    async fn equip_badge(&mut self, campaign_id: &str, drop: &TimedDrop, badge: BadgeRef) -> bool {
        let key = badge.key();
        let now = tokio::time::Instant::now();

        if self.selected_badge.as_ref().is_some_and(|b| b.key() == key)
            || self
                .equip_tried
                .get(&key)
                .is_some_and(|at| now.duration_since(*at) < EQUIP_RETRY)
        {
            return true;
        }

        self.equip_tried.insert(key.clone(), now);

        let drop_name = &drop.name;
        let context = EventContext {
            badge: Some(badge.title.clone()),
            badge_key: Some(key.clone()),
            drop_name: Some(drop_name.clone()),
            miner: Some(MinerKind::Badges),
            ..EventContext::drop(campaign_id, &drop.id)
        };
        let event = match self.client.select_badge(&badge).await {
            Ok(Some(selected)) if selected.key() == key => {
                self.selected_badge = Some(selected);

                self.emit(
                    EventKind::Info,
                    format!("Equipped the {} badge to evolve {drop_name}", badge.title),
                )
                .context(context)
            }
            Ok(_) => self
                .emit(
                    EventKind::Error,
                    format!("Twitch did not equip the {} badge", badge.title),
                )
                .context(context),
            Err(Error::Unauthorized) => {
                self.session_refused().await;

                return false;
            }
            Err(error) => self
                .emit(
                    EventKind::Error,
                    format!("The {} badge could not be equipped: {error}", badge.title),
                )
                .context(context),
        };

        self.publish(event);

        true
    }

    // ! Twitch's answer, a refusal or an error in its reply, keeps the lane out for a while; a
    // request that never got its answer is tried again on the next equip turn.
    async fn equip_lane(&mut self, campaign_id: &str, drop: &TimedDrop, lane: TrackLane) {
        let now = tokio::time::Instant::now();

        if self
            .equip_tried
            .get(&lane.lane)
            .is_some_and(|at| now.duration_since(*at) < EQUIP_RETRY)
        {
            return;
        }

        self.equip_tried.insert(lane.lane.clone(), now);

        let drop_name = &drop.name;
        let context = EventContext {
            lane: Some(lane.lane.clone()),
            drop_name: Some(drop_name.clone()),
            miner: Some(MinerKind::Badges),
            ..EventContext::drop(campaign_id, &drop.id)
        };
        let refusal = match self.client.select_lane(&lane).await {
            Ok(None) => {
                self.note_lane(&lane, true);

                let event = self
                    .emit(
                        EventKind::Info,
                        format!("Equipped {drop_name} in the Drops panel"),
                    )
                    .context(context);

                self.publish(event);

                return;
            }
            Ok(Some(code)) => code,
            Err(Error::Gql { code, .. }) => code,
            Err(Error::Unauthorized) => {
                self.session_refused().await;

                return;
            }
            Err(error) => {
                let event = self
                    .emit(
                        EventKind::Error,
                        format!("{drop_name} could not be equipped in the Drops panel: {error}"),
                    )
                    .context(context);

                self.publish(event);

                return;
            }
        };

        self.lane_refused
            .insert(lane.lane.clone(), Utc::now() + LANE_REFUSED_RETRY);
        self.note_lane(&lane, false);

        let event = self
            .emit(
                EventKind::Error,
                format!("Twitch did not equip {drop_name} in the Drops panel ({refusal})"),
            )
            .context(EventContext {
                reason: Some(refusal),
                ..context
            });

        self.publish(event);
    }

    // ! Equipped, a lane becomes its track's only one; refused, it alone is kept out.
    fn note_lane(&mut self, chosen: &TrackLane, equipped: bool) {
        let lanes = self
            .quests
            .iter_mut()
            .flat_map(|quest| &mut quest.groups)
            .filter_map(|group| group.lane.as_mut())
            .chain(
                self.campaigns
                    .values_mut()
                    .flat_map(|campaign| &mut campaign.drops)
                    .filter_map(|drop| drop.lane.as_mut()),
            )
            .filter(|lane| lane.track == chosen.track);

        for lane in lanes {
            if equipped {
                lane.equipped = lane.lane == chosen.lane;
            } else if lane.lane == chosen.lane {
                lane.refused = true;
            }
        }
    }

    pub(super) fn watch_started(&mut self, channel: &Channel, purpose: &SlotPurpose) -> MinerEvent {
        let title = match purpose {
            SlotPurpose::Drop { campaign_id, .. } => {
                let name = self
                    .campaigns
                    .get(campaign_id)
                    .map(|c| c.name.clone())
                    .unwrap_or_default();

                format!("Watching {} for {}", channel.display_name, name)
            }
            SlotPurpose::Points { reason } => {
                format!(
                    "Watching {} for channel points ({})",
                    channel.display_name,
                    reason.describe()
                )
            }
        };

        self.emit(EventKind::WatchStarted, title)
            .context(match purpose {
                SlotPurpose::Drop {
                    campaign_id,
                    drop_id,
                } => EventContext {
                    channel: Some(channel.login.clone()),
                    miner: Some(drops_miner(self.campaigns.get(campaign_id), Some(drop_id))),
                    ..EventContext::drop(campaign_id, drop_id)
                },
                SlotPurpose::Points { reason } => EventContext {
                    miner: Some(MinerKind::Points),
                    watch_reason: Some(*reason),
                    ..EventContext::channel(&channel.login)
                },
            })
    }

    pub(super) fn next_slot_at(&self) -> Option<tokio::time::Instant> {
        self.slots
            .iter()
            .flat_map(|slot| [slot.next_beacon, slot.next_playback])
            .min()
    }

    pub(super) async fn pump_slots(&mut self) -> bool {
        let now = tokio::time::Instant::now();
        let mut errors: Vec<Error> = Vec::new();
        let mut timeouts: Vec<Error> = Vec::new();
        let mut minutes = 0u64;

        for index in 0..self.slots.len() {
            if self.slots[index].next_playback <= now {
                let mut progressed = false;

                if let Some(mut playback) = self.slots[index].playback.take() {
                    match self.client.poll_playback(&mut playback).await {
                        Ok(count) => progressed = count > 0,
                        Err(Error::Status(status)) if status == StatusCode::NOT_FOUND => {
                            self.slots[index].channel.live = false;
                        }
                        Err(Error::Unauthorized) => errors.push(Error::Unauthorized),
                        Err(error) => {
                            tracing::debug!(account = %self.login, %error, "playback poll failed")
                        }
                    }

                    self.slots[index].playback = Some(playback);
                }

                self.slots[index].next_playback = after(
                    self.slots[index].next_playback,
                    playback::POLL_INTERVAL,
                    now,
                );

                if progressed {
                    self.slots[index].playback_accumulated += playback::POLL_INTERVAL;
                }

                // ! Counted from accumulated playback polls that returned segments,
                // not from the beacon timer: a DNS-blocked beacon still lets Drops
                // progress through the video playlist, and that timer can back off
                // to once every 10 minutes.
                while self.slots[index].playback_accumulated >= WATCHED_MINUTE {
                    self.slots[index].playback_accumulated -= WATCHED_MINUTE;

                    if let SlotPurpose::Drop { campaign_id, .. } = &self.slots[index].purpose {
                        let campaign_id = campaign_id.clone();
                        let credited = self.campaigns.get(&campaign_id).is_some_and(|campaign| {
                            scheduler::credits_drops(&self.slots[index].channel, campaign)
                        });

                        if credited {
                            setaside::record_minute(&mut self.set_aside, &campaign_id);
                        }
                    }
                }
            }

            if self.slots[index].next_beacon <= now {
                let mut retry_soon = false;

                if self.slots[index].beacon.is_none() {
                    match self
                        .client
                        .resolve_beacon(&self.slots[index].channel.login)
                        .await
                    {
                        Ok(url) => self.slots[index].beacon = Some(url),
                        Err(error) => {
                            self.slots[index].beacon_failures += 1;

                            // ! Without its beacon a slot earns no points: a page Twitch changed must show, not be read again every minute.
                            if self.slots[index].beacon_failures >= BEACON_UNREACHABLE_AFTER {
                                errors.push(error);
                            } else {
                                tracing::debug!(account = %self.login, %error, "beacon lookup failed");
                            }
                        }
                    }
                }

                if let Some(beacon) = self.slots[index].beacon.clone() {
                    match self
                        .client
                        .send_minute_watched(&beacon, &self.slots[index].channel)
                        .await
                    {
                        Ok(true) => {
                            self.slots[index].last_beacon_at = Some(Utc::now());
                            self.slots[index].beacon_failures = 0;
                            self.beacon_connect_failures = 0;
                            self.beacon_problem = None;
                            minutes += 1;
                        }
                        Ok(false) => {
                            self.slots[index].beacon_failures += 1;

                            if self.slots[index].beacon_failures >= 3 {
                                self.slots[index].beacon = None;
                            }
                        }
                        Err(Error::Transport(NetworkErrorKind::Connect)) => {
                            self.slots[index].beacon_failures += 1;
                            self.beacon_connect_failures += 1;
                            self.beacon_unreachable(&beacon).await;
                            // ! Only a DNS block stays until the user acts; any other refused connection is retried every minute so the banner clears soon after the network is back.
                            retry_soon = self.beacon_problem != Some(ErrorCode::BeaconBlocked);
                        }
                        Err(error @ Error::Transport(NetworkErrorKind::Timeout)) => {
                            self.slots[index].beacon_failures += 1;

                            if self.slots[index].beacon_failures >= BEACON_UNREACHABLE_AFTER {
                                timeouts.push(error);
                            } else {
                                tracing::debug!(account = %self.login, %error, "beacon timed out, retried next minute");
                            }
                        }
                        Err(error) => errors.push(error),
                    }
                }

                let backoff = if self.slots[index].beacon_failures >= 3 && !retry_soon {
                    BEACON_INTERVAL * 10
                } else {
                    BEACON_INTERVAL
                };

                self.slots[index].next_beacon = after(self.slots[index].next_beacon, backoff, now);
            }
        }

        self.stats.minutes_watched += minutes;

        for error in errors {
            self.handle_error(error).await;
        }

        for error in timeouts {
            self.report(error, false);
        }

        let mut index = 0;
        let mut removed = false;

        while index < self.slots.len() {
            if !self.slots[index].channel.live {
                let slot = self.slots.remove(index);
                removed = true;

                self.channels_at = None;
                self.seen_offline_or_refused
                    .insert(slot.channel.id.clone(), Utc::now());

                if let Some(channel) = self.streamer_channels.get_mut(&slot.channel.login) {
                    channel.live = false;
                }

                for list in self.live_by_campaign.values_mut() {
                    list.retain(|c| c.id != slot.channel.id);
                }

                let event = self
                    .emit(
                        EventKind::WatchStopped,
                        format!("{} went offline", slot.channel.display_name),
                    )
                    .context(EventContext {
                        avatar: slot.channel.avatar_url.clone(),
                        miner: Some(miner_of(&slot.purpose, &self.campaigns)),
                        ..EventContext::channel(&slot.channel.login)
                    });

                self.publish(event);
            } else {
                index += 1;
            }
        }

        if self.slots.is_empty() {
            self.beacon_problem = None;
            self.beacon_connect_failures = 0;
        }

        if let Some(code) = self.beacon_problem {
            self.problem = Some(code);
        }

        let progressed = self.read_drop_session(now).await;

        removed || progressed
    }

    // ! Twitch's own counter for the watched drop, once a minute: it covers a
    // progress message pubsub left out, and names the drop Twitch credits when
    // several campaigns share a category. A failed read changes nothing and
    // waits its minute like any other.
    pub(super) async fn read_drop_session(&mut self, now: tokio::time::Instant) -> bool {
        if self
            .drop_session_at
            .is_some_and(|at| now.duration_since(at) < DROP_SESSION_INTERVAL)
        {
            return false;
        }

        let watched = self.slots.iter().find_map(|slot| match &slot.purpose {
            SlotPurpose::Drop { campaign_id, .. }
                if self
                    .campaigns
                    .get(campaign_id)
                    .is_some_and(|campaign| campaign.kind == CampaignKind::Drops) =>
            {
                Some(slot.channel.id.clone())
            }
            _ => None,
        });
        let Some(channel_id) = watched else {
            return false;
        };

        self.drop_session_at = Some(now);

        match self.client.drop_session(&channel_id).await {
            // ! The session may be one left from an earlier watch on another
            // channel: only the watched channel's counter is taken.
            Ok(Some(session))
                if session
                    .channel_id
                    .as_ref()
                    .is_none_or(|id| *id == channel_id) =>
            {
                self.drop_progress(
                    &session.drop_id,
                    session.current_minutes,
                    session.required_minutes,
                )
            }
            Ok(_) => false,
            Err(error) => {
                tracing::debug!(account = %self.login, %error, "drop session read failed");

                false
            }
        }
    }

    // ! Channel points are not credited while the beacon is blocked or
    // unreachable; drops still progress, through the video playlist.
    pub(super) async fn beacon_unreachable(&mut self, beacon: &Url) {
        let spade = BeaconHost::Spade.name();
        let (code, message) = match self.client.beacon_blocked_by(beacon).await {
            Some(address) => (
                ErrorCode::BeaconBlocked,
                format!(
                    "{spade} is blocked by your DNS (it answers {address}): allowing {spade} alone is not enough, also allow the names it points to ({} and {}), otherwise channel points are not credited",
                    BeaconHost::SpadeScience.name(),
                    BeaconHost::ScienceEdge.name(),
                ),
            ),
            None if self.beacon_connect_failures >= BEACON_UNREACHABLE_AFTER => (
                ErrorCode::BeaconUnreachable,
                format!(
                    "{spade} could not be reached {BEACON_UNREACHABLE_AFTER} times in a row: check this account's proxy, your firewall or your connection, otherwise channel points are not credited"
                ),
            ),
            None => {
                tracing::debug!(
                    account = %self.login,
                    failures = self.beacon_connect_failures,
                    "beacon connection failed, retried next minute"
                );

                return;
            }
        };

        self.beacon_problem = Some(code);

        if self.beacon_reported.insert(code) {
            let event = self
                .emit(EventKind::Error, message)
                .detail("Drops still progress through the video playlist.");

            self.publish(event);
        }
    }

    pub(super) async fn stop_all_slots(&mut self, reason: &str) {
        let stopped: Vec<(String, String, MinerKind)> = self
            .slots
            .iter()
            .map(|s| {
                (
                    s.channel.login.clone(),
                    s.channel.display_name.clone(),
                    miner_of(&s.purpose, &self.campaigns),
                )
            })
            .collect();

        self.slots.clear();

        for (login, name, miner) in stopped {
            let event = self
                .emit(
                    EventKind::WatchStopped,
                    format!("Stopped watching {name} ({reason})"),
                )
                .context(EventContext {
                    miner: Some(miner),
                    ..EventContext::channel(login)
                });

            self.publish(event);
        }

        if let Some(pubsub) = &mut self.pubsub {
            pubsub.set_topics(Vec::new());
        }

        if let Some(chat) = &self.chat {
            chat.set_channels(BTreeSet::new());
        }
    }
}

pub(super) fn stop_reason(
    slot: &ActiveSlot,
    live: bool,
    plan: &scheduler::Plan,
    campaigns: &[Campaign],
    now: DateTime<Utc>,
) -> String {
    if !live {
        return "the stream ended".to_owned();
    }

    let SlotPurpose::Drop {
        campaign_id,
        drop_id,
    } = &slot.purpose
    else {
        // ! Points watches no campaign: a replacement is only true once the new
        // plan actually wants another points channel, never assumed from the stop alone.
        return if plan
            .slots
            .iter()
            .any(|wanted| matches!(wanted.purpose, SlotPurpose::Points { .. }))
        {
            "another channel comes first".to_owned()
        } else if let Some(id) = plan.slots.iter().find_map(|wanted| match &wanted.purpose {
            SlotPurpose::Drop {
                campaign_id: id, ..
            } => Some(id),
            SlotPurpose::Points { .. } => None,
        }) {
            format!(
                "{} comes first",
                campaigns
                    .iter()
                    .find(|c| &c.id == id)
                    .map_or(id.as_str(), |c| c.name.as_str())
            )
        } else {
            "no live channel offers channel points".to_owned()
        };
    };

    if let Some(drop) = campaigns
        .iter()
        .filter(|c| &c.id == campaign_id)
        .flat_map(|c| &c.drops)
        .find(|d| &d.id == drop_id && d.waiting(now))
    {
        let at = drop
            .next_period_at
            .map(|at| at.format("%Y-%m-%d %H:%M").to_string())
            .unwrap_or_default();

        return match drop.repeat {
            Some(repeat) => format!(
                "{}/{} periods done, the next opens at {at} UTC",
                drop.periods_done, repeat.times
            ),
            None => format!("the quest's next window opens at {at} UTC"),
        };
    }

    let Some(item) = plan
        .queue
        .iter()
        .find(|item| &item.campaign_id == campaign_id)
    else {
        return "the campaign left the mining order: done, ignored or out of time".to_owned();
    };

    if item.blocked == Some(scheduler::QueueBlock::SetAside) {
        return "Twitch is not crediting it: set aside until it is retried".to_owned();
    }

    match plan.slots.iter().find_map(|wanted| match &wanted.purpose {
        SlotPurpose::Drop {
            campaign_id: id, ..
        } => Some(id),
        SlotPurpose::Points { .. } => None,
    }) {
        Some(id) if id == campaign_id => "another channel of the campaign".to_owned(),
        Some(id) => format!(
            "{} comes first",
            campaigns
                .iter()
                .find(|c| &c.id == id)
                .map_or(id.as_str(), |c| c.name.as_str())
        ),
        None => "no live channel offers the campaign".to_owned(),
    }
}
