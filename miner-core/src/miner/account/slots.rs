use super::*;

impl<T: TwitchApi> AccountMiner<T> {
    pub(super) async fn apply_plan(&mut self) {
        let now = Utc::now();

        self.settle_streak_bonuses(now);

        let campaigns: Vec<Campaign> = self.campaigns.values().cloned().collect();
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
        let plan = scheduler::plan(
            &self.settings,
            &campaigns,
            &self.live_by_campaign,
            &states,
            current,
            now,
        );

        self.last_queue = plan.queue.clone();
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

            let event = self.watch_started(&slot.channel, &slot.purpose);

            self.publish(event);

            let playback = slot
                .channel
                .broadcast_id
                .is_some()
                .then(|| Playback::new(&slot.channel.login));

            self.slots.push(ActiveSlot {
                channel: slot.channel,
                purpose: slot.purpose,
                since: Utc::now(),
                beacon: None,
                last_beacon_at: None,
                next_beacon: tokio::time::Instant::now(),
                playback,
                next_playback: tokio::time::Instant::now(),
                beacon_failures: 0,
            });
        }
    }

    // ! Twitch counts an evolution only while the badge it evolves from is equipped; the old badge is never put back.
    pub(super) async fn equip_for(&mut self, plan: &scheduler::Plan) {
        if !self.settings.badges.evolutions {
            return;
        }

        let Some((campaign_id, drop_id, drop_name, badge)) =
            plan.slots.iter().find_map(|slot| match &slot.purpose {
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
                    let badge = drop.evolution.as_ref()?.from.clone()?;

                    Some((
                        campaign_id.clone(),
                        drop_id.clone(),
                        drop.name.clone(),
                        badge,
                    ))
                }
                SlotPurpose::Points { .. } => None,
            })
        else {
            return;
        };
        let key = badge.key();
        let now = tokio::time::Instant::now();

        if self.selected_badge.as_ref().is_some_and(|b| b.key() == key)
            || self
                .equip_tried
                .get(&key)
                .is_some_and(|at| now.duration_since(*at) < EQUIP_RETRY)
        {
            return;
        }

        self.equip_tried.insert(key.clone(), now);

        let context = EventContext {
            badge: Some(badge.title.clone()),
            drop_name: Some(drop_name.clone()),
            miner: Some(MinerKind::Badges),
            ..EventContext::drop(&campaign_id, &drop_id)
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

                return;
            }
            Err(error) => self
                .emit(
                    EventKind::Error,
                    format!("The {} badge could not be equipped: {error}", badge.title),
                )
                .context(context),
        };

        self.publish(event);
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
            if self.slots[index].next_beacon <= now {
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
                            self.beacon_blocked = false;
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

                let backoff = if self.slots[index].beacon_failures >= 3 {
                    BEACON_INTERVAL * 10
                } else {
                    BEACON_INTERVAL
                };

                self.slots[index].next_beacon = after(self.slots[index].next_beacon, backoff, now);
            }

            if self.slots[index].next_playback <= now {
                if let Some(mut playback) = self.slots[index].playback.take() {
                    match self.client.poll_playback(&mut playback).await {
                        Ok(_) => {}
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
            self.beacon_blocked = false;
            self.beacon_connect_failures = 0;
        }

        if self.beacon_blocked {
            self.problem = Some(ErrorCode::BeaconBlocked);
        }

        removed
    }

    // ! Channel points are not credited while the beacon is blocked; drops
    // still progress, through the video playlist.
    pub(super) async fn beacon_unreachable(&mut self, beacon: &Url) {
        let message = match self.client.beacon_blocked_by(beacon).await {
            Some(address) => format!(
                "spade.twitch.tv is blocked by your DNS (it answers {address}): allowing spade.twitch.tv alone is not enough, also allow the names it points to (spade.sci.twitch.tv and science-edge-external-prod-*.elb.amazonaws.com), otherwise channel points are not credited"
            ),
            None if self.beacon_connect_failures >= BEACON_UNREACHABLE_AFTER => format!(
                "spade.twitch.tv could not be reached {BEACON_UNREACHABLE_AFTER} times in a row: check your firewall, proxy or DNS, otherwise channel points are not credited"
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

        self.beacon_blocked = true;

        if !self.beacon_blocked_reported {
            self.beacon_blocked_reported = true;

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
        return "another channel comes first".to_owned();
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

    if !plan
        .queue
        .iter()
        .any(|item| &item.campaign_id == campaign_id)
    {
        return "the campaign left the mining order: done, ignored or out of time".to_owned();
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
