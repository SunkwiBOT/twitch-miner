use super::*;

impl<T: TwitchApi> AccountMiner<T> {
    pub(super) async fn claim_available_drops(&mut self) {
        if !self.settings.drops.auto_claim {
            return;
        }

        self.claims.forget_stale_refusals(&self.campaigns);
        self.claims.forget_stale_told_unlinked(&self.campaigns);

        let ignored_drops = &self.settings.drops.exclude_drop_ids;

        // ! See `Campaign::claims_delivered`: told once, not every tick.
        let unlinked: Vec<(String, String, String)> = self
            .campaigns
            .values()
            .filter(|c| !scheduler::claim_excluded(&self.settings, c) && !c.claims_delivered())
            .flat_map(|c| {
                c.drops
                    .iter()
                    .filter(|d| d.claimable() && !ignored_drops.contains(&d.id))
                    .map(move |d| (c.id.clone(), d.id.clone(), d.name.clone()))
            })
            .collect();

        let claimable: Vec<(String, String, String, String, Option<String>)> = self
            .campaigns
            .values()
            .filter(|c| !scheduler::claim_excluded(&self.settings, c) && c.claims_delivered())
            .flat_map(|c| {
                c.drops
                    .iter()
                    .filter(|d| d.claimable() && !ignored_drops.contains(&d.id))
                    .map(move |d| {
                        (
                            c.id.clone(),
                            c.name.clone(),
                            d.id.clone(),
                            d.name.clone(),
                            d.instance_id.clone(),
                        )
                    })
            })
            .collect();

        for (campaign_id, drop_id, drop_name) in unlinked {
            if !self.claims.tell_unlinked(&drop_id) {
                continue;
            }

            let event = self
                .emit(
                    EventKind::Info,
                    format!("{drop_name} is done: link the publisher account to claim it"),
                )
                .context(EventContext {
                    drop_name: Some(drop_name),
                    ..EventContext::drop(&campaign_id, &drop_id)
                });

            self.publish(event);
        }

        for (campaign_id, campaign_name, drop_id, drop_name, instance) in claimable {
            let Some(instance) = instance else {
                // ! `claimable()` above already requires an instance id, on the same
                // unmutated snapshot: unreachable unless that guard changes.
                tracing::debug!(account = %self.login, drop = %drop_name, "claimable drop had no instance id");

                continue;
            };

            match self.client.claim_drop(&instance).await {
                Ok(ClaimOutcome::AlreadyClaimed) => {
                    self.record_claim(&campaign_id, &drop_id, false);

                    let event = self
                        .emit(
                            EventKind::Info,
                            format!("{drop_name} was already claimed on Twitch"),
                        )
                        .context(EventContext {
                            drop_name: Some(drop_name),
                            ..EventContext::drop(&campaign_id, &drop_id)
                        });

                    self.publish(event);
                }
                Ok(ClaimOutcome::Claimed) => {
                    self.record_claim(&campaign_id, &drop_id, true);
                    self.tell_claimed(&campaign_id, &campaign_name, &drop_id, &drop_name);
                }
                Ok(ClaimOutcome::Refused(status)) if self.quest_group(&drop_id).is_some() => {
                    let grant = self.quest_group(&drop_id).map_or(0, |g| g.grant_count);

                    if self.claims.refuse_window(&drop_id, grant) {
                        let event = self
                            .emit(
                                EventKind::Error,
                                format!("Twitch refused the claim of {drop_name} ({status})"),
                            )
                            .context(EventContext {
                                status: Some(status.to_string()),
                                drop_name: Some(drop_name.clone()),
                                ..EventContext::drop(&campaign_id, &drop_id)
                            });

                        self.publish(event);
                    }

                    if let Some(drop) = self
                        .campaigns
                        .get_mut(&campaign_id)
                        .and_then(|c| c.drops.iter_mut().find(|d| d.id == drop_id))
                    {
                        drop.instance_id = None;
                    }
                }
                Ok(ClaimOutcome::Refused(status)) => {
                    let linked = self.campaigns.get(&campaign_id).and_then(|c| c.linked);

                    if self.claims.refuse(&drop_id, linked) {
                        let event = self
                            .emit(
                                EventKind::Error,
                                format!(
                                    "Twitch refused the claim of {drop_name} ({status}): link the publisher account"
                                ),
                            )
                            .context(EventContext {
                                status: Some(status.to_string()),
                                ..EventContext::campaign(&campaign_id)
                            });

                        self.publish(event);
                    } else {
                        tracing::debug!(account = %self.login, drop = %drop_name, %status, "claim still refused");
                    }

                    if let Some(drop) = self
                        .campaigns
                        .get_mut(&campaign_id)
                        .and_then(|c| c.drops.iter_mut().find(|d| d.id == drop_id))
                    {
                        drop.instance_id = None;
                    }
                }
                Err(error) => self.handle_error(error).await,
            }
        }
    }

    // ! "Already claimed" on a repeated Drops reward names a period Twitch counted already: its next inventory says which.
    pub(super) fn record_claim(&mut self, campaign_id: &str, drop_id: &str, fresh: bool) {
        let counted = fresh
            || self.quest_group(drop_id).is_some()
            || self
                .campaigns
                .get(campaign_id)
                .and_then(|c| c.drops.iter().find(|d| d.id == drop_id))
                .is_none_or(|d| d.repeat.is_none());

        if counted {
            self.note_claimed(drop_id);
        } else {
            self.inventory_at = None;
        }

        if let Some(drop) = self
            .campaigns
            .get_mut(campaign_id)
            .and_then(|c| c.drops.iter_mut().find(|d| d.id == drop_id))
        {
            if counted {
                close_period(drop);
            } else {
                drop.claimed = true;
                drop.instance_id = None;
            }

            if fresh {
                drop.current_minutes = drop.required_minutes;
            }
        }
    }

    pub(super) fn tell_claimed(
        &mut self,
        campaign_id: &str,
        campaign_name: &str,
        drop_id: &str,
        drop_name: &str,
    ) {
        self.stats.drops_claimed += 1;

        let required = self
            .campaigns
            .get(campaign_id)
            .and_then(|c| c.drops.iter().find(|d| d.id == drop_id))
            .map(|d| d.required_minutes);
        let event = self
            .emit(
                EventKind::DropClaimed,
                format!("Claimed {drop_name} ({campaign_name})"),
            )
            .context(EventContext {
                required,
                ..EventContext::drop(campaign_id, drop_id)
            });

        self.publish(event);
        self.tell_finished(campaign_id, campaign_name);
    }

    pub(super) fn tell_finished(&mut self, campaign_id: &str, campaign_name: &str) {
        let now = Utc::now();
        let finished = self
            .campaigns
            .get(campaign_id)
            .is_some_and(|c| c.finished(now, &self.settings.drops.exclude_drop_ids));

        if finished && self.claims.finish(campaign_id) {
            let event = self
                .emit(
                    EventKind::CampaignFinished,
                    format!("Finished {campaign_name}"),
                )
                .context(EventContext::campaign(campaign_id));

            self.publish(event);
            self.channels_at = None;
        }
    }

    pub(super) async fn claim_instance(&mut self, instance: &str) -> Result<()> {
        let unlinked = self.campaigns.values().any(|campaign| {
            !campaign.claims_delivered()
                && campaign
                    .drops
                    .iter()
                    .any(|drop| drop.instance_id.as_deref() == Some(instance))
        });

        if unlinked {
            return Err(Error::ClaimRefused(
                "the publisher account is not linked".to_owned(),
            ));
        }

        let target = self.campaigns.values().find_map(|campaign| {
            campaign
                .drops
                .iter()
                .find(|drop| drop.instance_id.as_deref() == Some(instance))
                .map(|drop| {
                    (
                        campaign.id.clone(),
                        campaign.name.clone(),
                        drop.id.clone(),
                        drop.name.clone(),
                    )
                })
        });

        match self.client.claim_drop(instance).await {
            Ok(outcome @ (ClaimOutcome::Claimed | ClaimOutcome::AlreadyClaimed)) => {
                let fresh = outcome == ClaimOutcome::Claimed;

                self.inventory_at = None;

                match target {
                    Some((campaign_id, campaign_name, drop_id, drop_name)) => {
                        self.record_claim(&campaign_id, &drop_id, fresh);

                        if fresh {
                            self.tell_claimed(&campaign_id, &campaign_name, &drop_id, &drop_name);
                        }
                    }
                    None if fresh => {
                        self.stats.drops_claimed += 1;

                        let event = self.emit(EventKind::DropClaimed, "Drop claimed");

                        self.publish(event);
                    }
                    None => {}
                }
            }
            Ok(ClaimOutcome::Refused(status)) => {
                let event = self.emit(
                    EventKind::Error,
                    format!("Twitch refused the claim ({status})"),
                );

                self.publish(event);
                self.push_snapshot();

                return Err(Error::ClaimRefused(status));
            }
            Err(error) => {
                match &error {
                    Error::Unauthorized => self.session_refused().await,
                    other => self.set_problem(other.code()),
                }

                self.push_snapshot();

                return Err(error);
            }
        }

        self.push_snapshot();

        Ok(())
    }

    pub(super) async fn claim_bonus(&mut self, channel_id: &str, claim_id: &str) {
        let login = match self.streamer_login(channel_id) {
            Some(login) => {
                let allowed = self
                    .settings
                    .points
                    .streamers
                    .iter()
                    .any(|s| s.login == login && s.claim_bonus);

                if !allowed {
                    return;
                }

                login
            }
            None => match self.drops_points_channel(channel_id) {
                Some(channel) => channel.login.clone(),
                None => return,
            },
        };

        if self
            .bonus_claims
            .get(channel_id)
            .is_some_and(|claimed| claimed == claim_id)
        {
            return;
        }

        self.bonus_claims
            .insert(channel_id.to_owned(), claim_id.to_owned());
        // ! Twitch's `points-earned` message with reason `CLAIM` is the
        // single source for this chest's earned points; only the balance
        // (for the curve) comes from the claim mutation itself.
        match self.client.claim_bonus(channel_id, claim_id).await {
            Ok(balance) => {
                self.stats.bonuses_claimed += 1;

                let name = self.channel_name(&login);

                if let Some(points) = self.streamer_points.get_mut(&login) {
                    if let Some(balance) = balance {
                        points.balance = balance;
                    }

                    points.last_bonus_at = Some(Utc::now());
                }

                let event = self
                    .emit(
                        EventKind::BonusClaimed,
                        format!("Bonus chest claimed on {name}"),
                    )
                    .context(EventContext {
                        balance,
                        ..EventContext::channel(&login)
                    });

                self.publish(event);
            }
            Err(Error::Gql { code, .. }) if BONUS_GONE.contains(&code.as_str()) => {
                tracing::debug!(account = %self.login, channel = %login, %code, "bonus chest already gone");
            }
            Err(error) => {
                self.bonus_claims.remove(channel_id);
                self.handle_error(error).await;
            }
        }
    }

    pub(super) fn streamer_login(&self, channel_id: &str) -> Option<String> {
        self.streamer_channels
            .values()
            .find(|channel| channel.id == channel_id)
            .map(|channel| channel.login.clone())
    }
}
