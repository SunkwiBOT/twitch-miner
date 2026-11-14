use super::*;

impl<T: TwitchApi> AccountMiner<T> {
    pub(super) async fn contribute_goals(&mut self, now: tokio::time::Instant) {
        if self.settings.mode == Mode::Drops {
            return;
        }

        let due: Vec<(String, String)> = self
            .settings
            .points
            .streamers
            .iter()
            .filter(|s| s.enabled && s.community_goals)
            .filter_map(|s| {
                let channel = self.streamer_channels.get(&s.login).filter(|c| c.live)?;
                Some((s.login.clone(), channel.id.clone()))
            })
            .filter(|(login, _)| {
                self.goals_at
                    .get(login)
                    .is_none_or(|at| now.duration_since(*at) >= GOALS_INTERVAL)
            })
            .collect();

        for (login, channel_id) in due {
            self.goals_at.insert(login.clone(), now);

            let goals = match self.client.community_goals(&login).await {
                Ok(goals) => goals,
                Err(error) => {
                    self.handle_error(error).await;
                    continue;
                }
            };

            let open: Vec<Goal> = goals.into_iter().filter(Goal::open).collect();

            if open.is_empty() {
                continue;
            }

            let given = match self.client.goal_contributions(&login).await {
                Ok(given) => given,
                Err(error) => {
                    self.handle_error(error).await;
                    continue;
                }
            };

            for goal in open {
                let balance = self.streamer_points.get(&login).map_or(0, |p| p.balance);
                let left_for_user =
                    goal.per_stream_user_maximum - given.get(&goal.id).copied().unwrap_or(0);
                let amount = (goal.amount_needed - goal.points_contributed)
                    .min(left_for_user)
                    .min(balance);

                if amount <= 0 {
                    continue;
                }

                match self
                    .client
                    .contribute_goal(&channel_id, &goal.id, amount)
                    .await
                {
                    Ok(()) => {
                        self.spent_here(&login, amount);

                        let name = self.channel_name(&login);
                        let event = self
                            .emit(
                                EventKind::GoalContributed,
                                format!(
                                    "{amount} points given to the goal {} of {name}",
                                    goal.title
                                ),
                            )
                            .context(EventContext {
                                points: Some(amount),
                                goal: Some(goal.title.clone()),
                                ..EventContext::channel(&login)
                            });

                        self.publish(event);
                    }
                    Err(error) => self.handle_error(error).await,
                }
            }
        }
    }

    pub(super) fn streak_bonus_paid(&mut self, login: &str) {
        let Some(broadcast) = self
            .streamer_channels
            .get(login)
            .and_then(|channel| channel.broadcast_id.clone())
        else {
            return;
        };

        let bonus = self
            .streak_bonuses
            .entry(login.to_owned())
            .or_insert_with(|| StreakBonus::new(&broadcast));

        if bonus.broadcast_id != broadcast {
            *bonus = StreakBonus::new(&broadcast);
        }

        bonus.settled = true;
    }

    pub(super) fn settle_streak_bonuses(&mut self, now: DateTime<Utc>) {
        for (login, bonus) in &mut self.streak_bonuses {
            if !bonus.settled && bonus.window_passed(now) {
                bonus.settled = true;

                tracing::debug!(
                    account = %self.login,
                    channel = %login,
                    "no watch streak bonus on this broadcast within {} minutes",
                    scheduler::STREAK_BONUS_WINDOW.num_minutes()
                );
            }
        }
    }

    pub(super) fn drops_points_channel(&self, channel_id: &str) -> Option<&Channel> {
        let drops = &self.settings.drops;

        if !drops.mine_points {
            return None;
        }

        self.slots
            .iter()
            .filter(|slot| matches!(slot.purpose, SlotPurpose::Drop { .. }))
            .map(|slot| &slot.channel)
            .find(|channel| channel.id == channel_id)
            .filter(|channel| !drops.points_followed_only || self.followed.contains(&channel.login))
    }

    pub(super) fn balance_read(&mut self, login: &str, balance: i64) {
        let listed = self
            .settings
            .points
            .streamers
            .iter()
            .any(|s| s.login == login);

        if !listed {
            return;
        }

        let read = balance - self.own_spends.pending(login);
        let points = self.streamer_points.entry(login.to_owned()).or_default();

        if points.balance != read {
            points.balance = read;
            self.push_snapshot();
        }
    }

    pub(super) fn spent_here(&mut self, login: &str, amount: i64) {
        if let Some(points) = self.streamer_points.get_mut(login) {
            points.balance -= amount;
        }

        self.own_spends.add(login, amount);
    }

    pub(super) fn channel_name(&self, login: &str) -> String {
        self.streamer_channels
            .get(login)
            .or_else(|| {
                self.slots
                    .iter()
                    .map(|s| &s.channel)
                    .find(|c| c.login == login)
            })
            .map(|c| c.display_name.clone())
            .unwrap_or_else(|| login.to_owned())
    }

    pub(super) async fn refresh_follows(&mut self, now: tokio::time::Instant) {
        let drops = &self.settings.drops;
        let needed =
            drops.mine_points && drops.points_followed_only && self.settings.mode != Mode::Points;

        if !needed
            || self
                .followed_at
                .is_some_and(|at| now.duration_since(at) < FOLLOWS_INTERVAL)
        {
            return;
        }

        match self.client.follows(FOLLOWS_MAX).await {
            Ok(channels) => {
                self.followed = channels.into_iter().map(|c| c.login).collect();
                self.followed_at = Some(now);
            }
            Err(error) => self.handle_error(error).await,
        }
    }
}
