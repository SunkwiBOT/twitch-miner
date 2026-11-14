use super::*;

impl<T: TwitchApi> AccountMiner<T> {
    pub(super) async fn handle_pubsub(&mut self, event: PubSubEvent) {
        match event {
            PubSubEvent::Unauthorized => self.handle_error(Error::Unauthorized).await,
            PubSubEvent::Reconnected => self.pubsub_reconnected(),
            PubSubEvent::StreamUp { channel_id } => self.stream_changed(&channel_id, true),
            PubSubEvent::StreamDown { channel_id } => self.stream_changed(&channel_id, false),
            PubSubEvent::ViewCount {
                channel_id,
                viewers,
            } => {
                for channel in self
                    .streamer_channels
                    .values_mut()
                    .filter(|c| c.id == channel_id)
                {
                    channel.viewers = Some(viewers);
                }
            }
            PubSubEvent::PointsEarned {
                channel_id,
                earned,
                balance,
                reason,
            } => self.points_earned(&channel_id, earned, balance, reason),
            PubSubEvent::PointsSpent {
                channel_id,
                balance,
            } => self.points_spent(&channel_id, balance),
            PubSubEvent::ClaimAvailable {
                channel_id,
                claim_id,
            } => self.claim_bonus(&channel_id, &claim_id).await,
            PubSubEvent::Raid {
                channel_id,
                raid_id,
                target_login,
            } => self.join_raid(&channel_id, &raid_id, &target_login).await,
            PubSubEvent::Moment {
                channel_id,
                moment_id,
            } => self.claim_moment(&channel_id, &moment_id).await,
            PubSubEvent::PredictionCreated(event) | PubSubEvent::PredictionUpdated(event) => {
                self.consider_prediction(event);
            }
            PubSubEvent::PredictionMade {
                event_id,
                outcome_id,
                points,
            } => {
                self.predictions_placed
                    .insert(event_id, (outcome_id, points));
            }
            PubSubEvent::PredictionResult {
                event_id,
                result,
                points_won,
            } => self.prediction_result(&event_id, &result, points_won),
            PubSubEvent::DropProgress {
                drop_id,
                current_minutes,
                required_minutes,
                ..
            } => self.drop_progress(&drop_id, current_minutes, required_minutes),
            PubSubEvent::DropClaim {
                drop_id,
                instance_id,
            } => self.drop_claimable(&drop_id, &instance_id).await,
            PubSubEvent::StreakRecovered { channel_id } => self.streak_recovered(&channel_id),
        }
    }

    // ! A bonus chest, a raid or a stream start may have gone by while a socket was down; the 4 sockets come back together, so the first one asks for all.
    pub(super) fn pubsub_reconnected(&mut self) {
        let now = tokio::time::Instant::now();
        if self
            .resynced_at
            .is_some_and(|at| now.duration_since(at) < RESYNC_GAP)
        {
            return;
        }

        self.resynced_at = Some(now);
        self.points_at.clear();
        self.channels_at = None;
    }

    // ! PubSub's stream-up comes before Twitch's pages show the broadcast: only a read of the channel counts.
    pub(super) fn stream_changed(&mut self, channel_id: &str, live: bool) {
        if live {
            let now = tokio::time::Instant::now();
            let due = now.checked_sub(CHANNELS_INTERVAL.saturating_sub(STREAM_UP_READ));

            self.channels_at = self.channels_at.min(due);

            return;
        }

        if let Some(slot) = self.slots.iter_mut().find(|s| s.channel.id == channel_id) {
            slot.channel.live = false;
        }

        self.channels_at = None;
    }

    pub(super) fn points_earned(
        &mut self,
        channel_id: &str,
        earned: i64,
        balance: Option<i64>,
        reason: String,
    ) {
        let listed = self.streamer_login(channel_id);

        if let Some(login) = &listed {
            let waiting = self.own_spends.pending(login);
            let points = self.streamer_points.entry(login.clone()).or_default();

            // ! Until Twitch's `points-spent` arrives, a balance it gives may or may not
            // count a spend of the miner's own: the earned points are added, and that
            // message settles the balance.
            match balance {
                Some(balance) if waiting == 0 => points.balance = balance,
                _ => points.balance += earned,
            }

            points.earned_session += earned;

            if reason == pubsub::WATCH_STREAK_REASON {
                self.streak_bonus_paid(login);
            }
        }

        let login = listed.or_else(|| {
            self.drops_points_channel(channel_id)
                .map(|channel| channel.login.clone())
        });

        if let Some(login) = login {
            self.stats.points_earned += earned;

            let event = self
                .emit(
                    EventKind::PointsEarned,
                    format!("+{earned} points on {login} ({reason})"),
                )
                .context(EventContext {
                    earned: Some(earned),
                    balance,
                    reason: Some(reason),
                    ..EventContext::channel(&login)
                });

            self.publish(event);
        }
    }

    pub(super) fn points_spent(&mut self, channel_id: &str, balance: i64) {
        let listed = self.streamer_login(channel_id);
        // ! Twitch sends only the new balance: the amount is what the balance it gave
        // last lost. The miner's own spends are off the local balance already, so
        // they are added back to find it.
        let mut spent = None;

        if let Some(login) = &listed {
            let points = self.streamer_points.entry(login.clone()).or_default();
            let before = points.balance + self.own_spends.pending(login);
            let amount = Some(before - balance).filter(|amount| *amount > 0);

            spent = self.own_spends.settle(login, amount);
            points.balance = balance - self.own_spends.pending(login);
        }

        let login = listed
            .or_else(|| {
                self.drops_points_channel(channel_id)
                    .map(|channel| channel.login.clone())
            })
            .filter(|_| spent != Some(0))
            .filter(|login| !self.context.spends.covers(login));

        if let Some(login) = login {
            let title = match spent {
                Some(amount) => format!("-{amount} points on {login}"),
                None => format!("Points spent on {login} (balance {balance})"),
            };
            let event = self
                .emit(EventKind::PointsSpent, title)
                .context(EventContext {
                    spent,
                    balance: Some(balance),
                    ..EventContext::channel(&login)
                });

            self.publish(event);
        }
    }

    pub(super) async fn join_raid(&mut self, channel_id: &str, raid_id: &str, target_login: &str) {
        let login = self.streamer_login(channel_id);
        let allowed = login.as_ref().is_some_and(|l| {
            self.settings
                .points
                .streamers
                .iter()
                .any(|s| &s.login == l && s.follow_raids)
        });

        if !allowed
            || !self
                .raid_joins
                .due(channel_id, raid_id, tokio::time::Instant::now())
        {
            return;
        }

        match self.client.join_raid(raid_id).await {
            Ok(()) => {
                self.raid_joins.joined(channel_id, raid_id);

                let event = self
                    .emit(
                        EventKind::RaidJoined,
                        format!(
                            "Joined the raid of {} to {target_login}",
                            login.clone().unwrap_or_default()
                        ),
                    )
                    .context(EventContext {
                        channel: login,
                        ..EventContext::default()
                    });

                self.publish(event);
            }
            Err(error) => {
                tracing::debug!(account = %self.login, %error, "raid join failed")
            }
        }
    }

    pub(super) async fn claim_moment(&mut self, channel_id: &str, moment_id: &str) {
        let login = self.streamer_login(channel_id);
        let allowed = login.as_ref().is_some_and(|l| {
            self.settings
                .points
                .streamers
                .iter()
                .any(|s| &s.login == l && s.claim_moments)
        });

        if !allowed {
            return;
        }

        match self.client.claim_moment(moment_id).await {
            Ok(()) => {
                let event = self
                    .emit(
                        EventKind::MomentClaimed,
                        format!("Moment claimed on {}", login.clone().unwrap_or_default()),
                    )
                    .context(EventContext {
                        channel: login,
                        ..EventContext::default()
                    });

                self.publish(event);
            }
            Err(error) => {
                tracing::debug!(account = %self.login, %error, "moment claim failed")
            }
        }
    }

    pub(super) fn prediction_result(&mut self, event_id: &str, result: &str, points_won: i64) {
        let placed = self.predictions_placed.remove(event_id);
        let staked = placed.map(|(_, p)| p).unwrap_or(0);
        let result = BetResult::from_twitch(result);

        let title = match result {
            BetResult::Win => {
                self.stats.predictions_won += 1;

                format!("Prediction won: +{points_won} points (staked {staked})")
            }
            BetResult::Lose => {
                self.stats.predictions_lost += 1;

                format!("Prediction lost: -{staked} points")
            }
            BetResult::Refund => format!("Prediction refunded: {staked} points returned"),
        };

        let event = self
            .emit(EventKind::PredictionResult, title)
            .context(EventContext {
                bet_result: Some(result),
                points_won: Some(points_won),
                staked: Some(staked),
                ..EventContext::default()
            });

        self.publish(event);
    }

    pub(super) fn drop_progress(
        &mut self,
        drop_id: &str,
        current_minutes: u32,
        required_minutes: u32,
    ) {
        let mut matched: Option<(String, String, String, u32)> = None;

        for campaign in self.campaigns.values_mut() {
            if let Some(drop) = campaign.drops.iter_mut().find(|d| d.id == drop_id) {
                drop.current_minutes = if drop.repeat.is_some() {
                    current_minutes
                } else {
                    drop.current_minutes.max(current_minutes)
                };

                if required_minutes > 0 {
                    drop.required_minutes = required_minutes;
                }

                matched = Some((
                    campaign.id.clone(),
                    campaign.name.clone(),
                    drop.name.clone(),
                    drop.required_minutes,
                ));
                break;
            }
        }

        let Some((campaign_id, campaign_name, drop_name, required)) = matched else {
            return;
        };
        let event = self
            .emit(
                EventKind::DropProgress,
                format!("{drop_name}: {current_minutes}/{required} min ({campaign_name})"),
            )
            .context(EventContext {
                current: Some(current_minutes),
                required: Some(required),
                ..EventContext::drop(&campaign_id, drop_id)
            });

        self.publish(event);

        if current_minutes >= required {
            self.inventory_at = None;
        }
    }

    pub(super) async fn drop_claimable(&mut self, drop_id: &str, instance_id: &str) {
        for campaign in self.campaigns.values_mut() {
            if let Some(drop) = campaign.drops.iter_mut().find(|d| d.id == drop_id) {
                drop.instance_id = Some(instance_id.to_owned());
                drop.current_minutes = drop.required_minutes;
            }
        }

        self.claim_available_drops().await;
    }

    pub(super) fn streak_recovered(&mut self, channel_id: &str) {
        let login = self.streamer_login(channel_id);
        let name = login
            .as_deref()
            .map_or_else(|| channel_id.to_owned(), |l| self.channel_name(l));
        let event = self
            .emit(
                EventKind::StreakRecovered,
                format!("Watch streak recovered on {name}"),
            )
            .context(login.map(EventContext::channel).unwrap_or_default());

        self.publish(event);
    }

    pub(super) fn sync_chat(&mut self) {
        let wanted: BTreeSet<String> = if self.settings.mode == Mode::Drops {
            BTreeSet::new()
        } else {
            self.settings
                .points
                .streamers
                .iter()
                .filter(|s| s.enabled && s.chat != ChatPresence::Never)
                .filter(|s| {
                    let live = self.streamer_channels.get(&s.login).map(|c| c.live);

                    match s.chat {
                        ChatPresence::Always => true,
                        ChatPresence::Online => live == Some(true),
                        ChatPresence::Offline => live == Some(false),
                        ChatPresence::Never => false,
                    }
                })
                .map(|s| s.login.clone())
                .take(chat::MAX_CHANNELS)
                .collect()
        };

        if wanted.is_empty() && self.chat.is_none() {
            return;
        }

        let chat = self
            .chat
            .get_or_insert_with(|| self.client.start_chat(self.cancel.child_token()));

        chat.set_channels(wanted);
    }

    pub(super) fn sync_pubsub(&mut self) {
        if self.pubsub.is_none() {
            self.pubsub = Some(
                self.client
                    .start_pubsub(self.pubsub_sender.clone(), self.cancel.child_token()),
            );
        }

        let mut channels: Vec<String> = self.slots.iter().map(|s| s.channel.id.clone()).collect();

        for streamer in &self.settings.points.streamers {
            if let Some(channel) = self.streamer_channels.get(&streamer.login) {
                if streamer.enabled && !channels.contains(&channel.id) {
                    channels.push(channel.id.clone());
                }
            }
        }

        let left_out = channels.len().saturating_sub(pubsub::MAX_CHANNELS);
        if left_out != self.pubsub_left_out {
            self.pubsub_left_out = left_out;
            if left_out > 0 {
                tracing::warn!(
                    account = %self.login,
                    left_out,
                    followed = pubsub::MAX_CHANNELS,
                    "too many streamers for live updates: the last ones of the list get no raids, moments, predictions or stream starts"
                );
            }
        }

        let predictions: Vec<String> = self
            .settings
            .points
            .streamers
            .iter()
            .filter(|s| s.enabled && s.predictions.enabled)
            .filter_map(|s| self.streamer_channels.get(&s.login).map(|c| c.id.clone()))
            .collect();

        if let Some(pubsub) = &mut self.pubsub {
            pubsub.set_topics(pubsub::topics_for(&self.id, &channels, &predictions));
        }
    }
}
