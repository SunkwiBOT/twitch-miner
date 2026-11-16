use super::*;

impl<T: TwitchApi> AccountMiner<T> {
    pub(super) fn consider_prediction(&mut self, event: PredictionEvent) {
        if event.status != "ACTIVE" {
            self.pending_bets.retain(|(p, _)| p.id != event.id);
            self.bets_tried.remove(&event.id);

            return;
        }

        let Some(login) = self.streamer_login(&event.channel_id) else {
            return;
        };
        let Some(settings) = self
            .settings
            .points
            .streamers
            .iter()
            .find(|s| s.login == login)
        else {
            return;
        };

        if !settings.predictions.enabled
            || self.predictions_placed.contains_key(&event.id)
            || self.bets_tried.contains(&event.id)
        {
            return;
        }

        let due = predictions::bet_time(&event, &settings.predictions);

        if let Some(pending) = self.pending_bets.iter_mut().find(|(p, _)| p.id == event.id) {
            pending.0 = event;
        } else {
            self.pending_bets.push((event, due));
        }
    }

    pub(super) fn next_bet_at(&self) -> Option<tokio::time::Instant> {
        let due = self.pending_bets.iter().map(|(_, at)| *at).min()?;
        let wait = (due - Utc::now()).to_std().unwrap_or(Duration::ZERO);
        Some(tokio::time::Instant::now() + wait)
    }

    pub(super) async fn place_due_bets(&mut self) {
        let now = Utc::now();
        let due: Vec<PredictionEvent> = self
            .pending_bets
            .iter()
            .filter(|(_, at)| *at <= now)
            .map(|(e, _)| e.clone())
            .collect();

        self.pending_bets.retain(|(_, at)| *at > now);

        for event in due {
            if now > event.closes_at() {
                continue;
            }

            let Some(login) = self.streamer_login(&event.channel_id) else {
                continue;
            };
            let Some(settings) = self
                .settings
                .points
                .streamers
                .iter()
                .find(|s| s.login == login)
                .map(|s| s.predictions.clone())
            else {
                continue;
            };
            let balance = self
                .streamer_points
                .get(&login)
                .map(|p| p.balance)
                .unwrap_or(0);
            let Some(decision) = predictions::decide(&event, &settings, balance) else {
                continue;
            };

            self.bets_tried.insert(event.id.clone());

            match self
                .client
                .make_prediction(&event.id, &decision.outcome_id, decision.points)
                .await
            {
                Ok(()) => {
                    self.spent_here(&login, decision.points);
                    self.predictions_placed.insert(
                        event.id.clone(),
                        (decision.outcome_id.clone(), decision.points),
                    );

                    let outcome = event
                        .outcomes
                        .get(decision.outcome_index)
                        .map(|o| o.title.clone())
                        .unwrap_or_default();
                    let title = format!(
                        "Bet {} points on \"{outcome}\" ({}) for {login}",
                        decision.points, event.title
                    );
                    let miner_event =
                        self.emit(EventKind::PredictionPlaced, title)
                            .context(EventContext {
                                points: Some(decision.points),
                                outcome: Some(outcome),
                                ..EventContext::channel(&login)
                            });

                    self.publish(miner_event);
                }
                Err(Error::Gql { code, .. })
                    if PREDICTION_CLOSED.iter().any(|part| code.contains(part)) =>
                {
                    let miner_event = self
                        .emit(
                            EventKind::Info,
                            format!(
                                "Bet not placed on {login}: the prediction closed first ({code})"
                            ),
                        )
                        .context(EventContext {
                            miner: Some(MinerKind::Points),
                            ..EventContext::channel(&login)
                        });

                    self.publish(miner_event);
                }
                Err(error) => {
                    let miner_event = self
                        .emit(
                            EventKind::Error,
                            format!("Prediction on {login} failed: {error}"),
                        )
                        .context(EventContext {
                            miner: Some(MinerKind::Points),
                            ..EventContext::channel(&login)
                        });

                    self.publish(miner_event);
                }
            }
        }
    }
}
