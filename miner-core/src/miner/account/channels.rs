use super::*;

impl<T: TwitchApi> AccountMiner<T> {
    pub(super) async fn refresh_channels(&mut self) -> Result<()> {
        // ! The listed streamers first: the Drops lookups then see their broadcasts
        // as they are now.
        let streamers = self.refresh_streamers().await;

        if matches!(streamers, Err(Error::Unauthorized)) {
            return streamers;
        }

        if self.settings.mode == Mode::Points {
            self.live_by_campaign.clear();
            self.offered_drops.clear();
        } else {
            self.refresh_drops_channels().await?;
        }

        streamers
    }

    pub(super) async fn refresh_streamers(&mut self) -> Result<()> {
        let logins: Vec<String> = self
            .settings
            .points
            .streamers
            .iter()
            .filter(|s| s.enabled)
            .map(|s| s.login.clone())
            .collect();

        if !logins.is_empty() {
            let read = self.client.channels(&logins).await?;
            // ! A login answered with no channel and no error is gone (banned, renamed): it is offline.
            let gone: Vec<Channel> = logins
                .iter()
                .filter(|login| {
                    !read.unchecked.contains(*login)
                        && !read.channels.iter().any(|c| &c.login == *login)
                })
                .filter_map(|login| self.streamer_channels.get(login))
                .filter(|channel| channel.live)
                .map(|channel| Channel {
                    live: false,
                    broadcast_id: None,
                    viewers: None,
                    ..channel.clone()
                })
                .collect();
            let mut seen: HashSet<String> = HashSet::new();
            let mut went_live: HashSet<String> = HashSet::new();

            for channel in read.channels.into_iter().chain(gone) {
                seen.insert(channel.login.clone());

                let was_live = self
                    .streamer_channels
                    .get(&channel.login)
                    .is_some_and(|c| c.live);

                if channel.live && !was_live {
                    went_live.insert(channel.login.clone());
                }

                if let Some(broadcast) = channel.broadcast_id.as_deref().filter(|_| channel.live) {
                    let known = self
                        .streak_bonuses
                        .get(&channel.login)
                        .is_some_and(|bonus| bonus.broadcast_id == broadcast);

                    if !known {
                        self.streak_bonuses
                            .insert(channel.login.clone(), StreakBonus::new(broadcast));
                    }
                }

                if channel.live != was_live && self.streamer_channels.contains_key(&channel.login) {
                    let kind = if channel.live {
                        EventKind::StreamOnline
                    } else {
                        EventKind::StreamOffline
                    };

                    let title = format!(
                        "{} is {}",
                        channel.display_name,
                        if channel.live { "live" } else { "offline" }
                    );
                    let event = self
                        .emit(kind, title)
                        .context(EventContext::channel(&channel.login));

                    self.publish(event);
                }

                self.streamer_channels
                    .insert(channel.login.clone(), channel);
            }

            let now = tokio::time::Instant::now();
            let stale: Vec<String> = logins
                .iter()
                .filter(|login| seen.contains(*login))
                .filter(|login| {
                    went_live.contains(*login)
                        || self
                            .points_at
                            .get(*login)
                            .is_none_or(|at| now.duration_since(*at) >= POINTS_INTERVAL)
                })
                .cloned()
                .collect();
            let client = &self.client;
            let reads: Vec<_> = futures_util::stream::iter(stale)
                .map(|login| async move {
                    let context = client.points_context(&login).await;
                    let streak = match &context {
                        Ok(Some(_)) => Some(client.watch_streak(&login).await),
                        _ => None,
                    };

                    (login, context, streak)
                })
                .buffer_unordered(POINTS_READS_AT_ONCE)
                .collect()
                .await;

            for (login, context, streak) in reads {
                let context = match context {
                    Ok(Some(context)) => context,
                    Ok(None) => continue,
                    Err(error) => {
                        tracing::debug!(account = %self.login, channel = %login, %error, "points balance read failed");

                        continue;
                    }
                };

                self.points_at.insert(login.clone(), now);

                let state = self.streamer_points.entry(login.clone()).or_default();

                if let Some(balance) = context.balance {
                    state.balance = balance;
                }
                state.enabled = context.enabled;
                state.subscribed = context.subscribed;

                match streak {
                    Some(Ok(streak)) => state.streak = streak,
                    Some(Err(error)) => {
                        tracing::debug!(account = %self.login, channel = %login, %error, "watch streak read failed")
                    }
                    None => {}
                }

                if let Some(claim) = context.available_claim_id {
                    self.claim_bonus(&context.channel_id, &claim).await;
                }
            }
        }

        self.streamer_channels
            .retain(|login, _| logins.contains(login));
        self.streak_bonuses
            .retain(|login, _| logins.contains(login));

        Ok(())
    }

    pub(super) async fn refresh_drops_channels(&mut self) -> Result<()> {
        let campaigns: Vec<Campaign> = self.campaigns.values().cloned().collect();
        let ranked = scheduler::rank_campaigns(&self.settings, &campaigns, Utc::now());
        let now = Utc::now();

        self.seen_offline_or_refused
            .retain(|_, at| now - *at < drops_api::LIVE_CHANNELS_MAX_AGE);

        let watched = self.drops_watch();
        let chosen = self.campaigns_to_look_up(&ranked, watched.as_ref());
        let api_channels = self.fresh_api_channels(&ranked).await;
        let need_lookup: Vec<&Campaign> = chosen
            .iter()
            .copied()
            .filter(|c| !api_channels.contains_key(&c.id))
            .collect();
        let lookup = lookup_targets(&need_lookup, self.category_turn);

        self.category_turn = self.category_turn.wrapping_add(CATEGORIES_READ - 1);

        let directory = self.read_directories(lookup.games).await?;
        let missing = self.unlisted_watch(
            watched.as_ref(),
            &chosen,
            &directory,
            &lookup.by_directory,
            &api_channels,
        );
        let allowed_logins = lookup.logins;
        let mut logins = allowed_logins.clone();

        if let Some((_, channel)) =
            missing.filter(|(_, channel)| !self.streamer_channels.contains_key(&channel.login))
        {
            logins.push(channel.login.clone());
        }

        let read = self.read_channels(&logins).await?;
        let missing_now: Option<(String, Option<Channel>)> =
            missing.map(|(campaign_id, channel)| {
                let now = self
                    .streamer_channels
                    .get(&channel.login)
                    .cloned()
                    .or_else(|| {
                        read.as_ref()
                            .and_then(|read| read.iter().find(|c| c.id == channel.id).cloned())
                    });

                (campaign_id.clone(), now)
            });
        let allowed: Option<Vec<Channel>> = read.map(|read| {
            read.into_iter()
                .filter(|channel| allowed_logins.contains(&channel.login))
                .collect()
        });
        let recheck: Option<Channel> = missing_now.as_ref().and_then(|(campaign_id, now)| {
            let campaign = self.campaigns.get(campaign_id)?;

            now.clone()
                .filter(|channel| channel.live && counts_for(channel, campaign))
        });

        self.ask_offered_drops(&chosen, &directory, recheck.clone())
            .await;

        let mut kept: Option<(String, Channel)> = None;

        if let Some((campaign_id, now)) = missing_now.as_ref() {
            if let Some(campaign) = self.campaigns.get(campaign_id).cloned() {
                let result = self.still_watched(&campaign, watched.as_ref(), now.as_ref());

                if result.is_none() && now.is_some() {
                    if let Some((_, channel)) = watched.as_ref() {
                        self.seen_offline_or_refused
                            .insert(channel.id.clone(), Utc::now());
                    }
                }

                kept = result.map(|channel| (campaign_id.clone(), channel));
            }
        }
        let looked_up: HashSet<&str> = chosen.iter().map(|c| c.id.as_str()).collect();

        let sources = ChannelSources {
            directory: &directory,
            by_directory: &lookup.by_directory,
            allowed: allowed.as_deref(),
            api_channels: &api_channels,
        };

        self.live_by_campaign =
            self.live_channels(&ranked, &looked_up, &sources, missing, kept.as_ref());
        self.forget_stale_offers(&directory, recheck.as_ref());

        Ok(())
    }

    // ! Only a non-quest campaign: a Quest still needs the "Drops & More"
    // cross-check (`ask_offered_drops`), which the catalogue's `liveChannels`
    // cannot answer. Costs no Twitch call, so every ranked campaign is served,
    // not just the ones this pass looks up.
    async fn fresh_api_channels(
        &self,
        ranked: &[(&Campaign, QueueReason)],
    ) -> HashMap<String, Vec<Channel>> {
        let catalog = self.context.catalog.read().await;
        let now = Utc::now();

        ranked
            .iter()
            .map(|(c, _)| *c)
            .filter(|c| c.kind != CampaignKind::Quest)
            .filter_map(|c| {
                let entry = catalog.live_channels.get(&c.id)?;

                if entry.read_at > now
                    || now - entry.read_at >= drops_api::LIVE_CHANNELS_MAX_AGE
                    || entry.channels.is_empty()
                {
                    return None;
                }

                // ! A channel seen offline or refused since the list was last
                // read is dropped until a fresher list vouches for it again:
                // otherwise it comes right back and is watched in a loop.
                let channels: Vec<Channel> = entry
                    .channels
                    .iter()
                    .filter(|channel| {
                        self.seen_offline_or_refused
                            .get(&channel.id)
                            .is_none_or(|seen_at| entry.read_at > *seen_at)
                    })
                    .cloned()
                    .collect();

                (!channels.is_empty()).then_some((c.id.clone(), channels))
            })
            .collect()
    }

    pub(super) fn campaigns_to_look_up<'a>(
        &mut self,
        ranked: &[(&'a Campaign, QueueReason)],
        watched: Option<&(String, Channel)>,
    ) -> Vec<&'a Campaign> {
        let mut chosen: Vec<&Campaign> = ranked
            .iter()
            .map(|(c, _)| *c)
            .filter(|c| watched.is_some_and(|(id, _)| *id == c.id))
            .collect();

        for (campaign, _) in ranked.iter().take(LOOKUP_TOP) {
            if !chosen.iter().any(|c| c.id == campaign.id) {
                chosen.push(*campaign);
            }
        }

        let rest: Vec<&Campaign> = ranked
            .iter()
            .skip(LOOKUP_TOP)
            .map(|(c, _)| *c)
            .filter(|c| !chosen.iter().any(|known| known.id == c.id))
            .collect();

        if !rest.is_empty() {
            for step in 0..LOOKUP_ROTATING.min(rest.len()) {
                chosen.push(rest[(self.lookup_offset + step) % rest.len()]);
            }

            self.lookup_offset = (self.lookup_offset + LOOKUP_ROTATING) % rest.len();
        }

        chosen
    }

    pub(super) async fn read_directories(
        &self,
        games: Vec<Game>,
    ) -> Result<HashMap<String, Vec<Channel>>> {
        let client = &self.client;
        let reads: Vec<(Game, Result<Vec<Channel>>)> = futures_util::stream::iter(games)
            .map(|game| async move {
                let read = client.game_directory(&game).await;

                (game, read)
            })
            .buffered(DIRECTORY_READS_AT_ONCE)
            .collect()
            .await;
        let mut directory: HashMap<String, Vec<Channel>> = HashMap::new();

        for (game, read) in reads {
            match read {
                Ok(channels) => {
                    directory.insert(game.id, channels);
                }
                Err(Error::Unauthorized) => return Err(Error::Unauthorized),
                Err(error) => {
                    tracing::debug!(account = %self.login, game = %game.name, %error, "directory lookup failed")
                }
            }
        }

        Ok(directory)
    }

    // ! A game's directory lists only 30 channels, by relevance: the channel the
    // Drops slot watches can be missing from it. A campaign the catalogue's
    // `liveChannels` covers skips the directory entirely, so its watched
    // channel is verified directly every pass instead: that list only
    // refreshes every 5 minutes and could still call it live after it drops.
    pub(super) fn unlisted_watch<'w>(
        &self,
        watched: Option<&'w (String, Channel)>,
        chosen: &[&Campaign],
        directory: &HashMap<String, Vec<Channel>>,
        by_directory: &HashSet<String>,
        api_channels: &HashMap<String, Vec<Channel>>,
    ) -> Option<&'w (String, Channel)> {
        watched.filter(|(campaign_id, channel)| {
            chosen.iter().any(|campaign| campaign.id == *campaign_id)
                && self.campaigns.get(campaign_id).is_some_and(|campaign| {
                    api_channels.contains_key(&campaign.id)
                        || ((campaign.allowed_channels.is_empty()
                            || by_directory.contains(&campaign.id))
                            && listed(directory, campaign, &channel.id) == Some(false))
                })
        })
    }

    pub(super) async fn read_channels(&self, logins: &[String]) -> Result<Option<Vec<Channel>>> {
        if logins.is_empty() {
            return Ok(Some(Vec::new()));
        }

        match self.client.channels(logins).await {
            Ok(read) => Ok(Some(read.channels)),
            Err(Error::Unauthorized) => Err(Error::Unauthorized),
            Err(error) => {
                tracing::debug!(account = %self.login, %error, "channel lookup failed");

                Ok(None)
            }
        }
    }

    pub(super) fn live_channels(
        &self,
        ranked: &[(&Campaign, QueueReason)],
        looked_up: &HashSet<&str>,
        sources: &ChannelSources<'_>,
        missing: Option<&(String, Channel)>,
        kept: Option<&(String, Channel)>,
    ) -> HashMap<String, Vec<Channel>> {
        let ChannelSources {
            directory,
            by_directory,
            allowed,
            api_channels,
        } = *sources;

        let mut live_by_campaign: HashMap<String, Vec<Channel>> = HashMap::new();

        for (campaign, _) in ranked {
            let is_api = api_channels.contains_key(&campaign.id);
            let fresh = if let Some(channels) = api_channels.get(&campaign.id) {
                Some(channels.clone())
            } else if !looked_up.contains(campaign.id.as_str()) {
                None
            } else if campaign.allowed_channels.is_empty() || by_directory.contains(&campaign.id) {
                let mut found: Option<Vec<Channel>> = None;

                for game in campaign.played() {
                    if let Some(channels) = directory.get(&game.id) {
                        found.get_or_insert_with(Vec::new).extend(
                            channels
                                .iter()
                                .filter(|c| campaign.accepts_channel(&c.id))
                                .cloned(),
                        );
                    }
                }

                found
            } else {
                allowed.map(|allowed| {
                    allowed
                        .iter()
                        .filter(|c| c.live && campaign.accepts_channel(&c.id))
                        .cloned()
                        .collect()
                })
            };

            let Some(mut live) = fresh else {
                if let Some(previous) = self.live_by_campaign.get(&campaign.id) {
                    live_by_campaign.insert(campaign.id.clone(), previous.clone());
                }

                continue;
            };

            // ! A campaign's `liveChannels` can still be stale about the one
            // channel just verified above: drop any copy of it before maybe
            // putting the verified state back.
            if let Some((campaign_id, channel)) = missing {
                if campaign_id == &campaign.id {
                    live.retain(|c| c.id != channel.id);
                }
            }

            if let Some((_, channel)) = kept.filter(|(campaign_id, _)| *campaign_id == campaign.id)
            {
                live.push(channel.clone());
            }

            // ! The own read is authoritative for a login this account lists:
            // it replaces a stale copy, and for an API list (whose entries can
            // only be as fresh as the last catalogue fetch) removes it outright
            // once it is no longer live on a counted category.
            for channel in self.streamer_channels.values() {
                if !campaign.accepts_channel(&channel.id) {
                    continue;
                }

                let counts = channel.live
                    && channel
                        .game
                        .as_ref()
                        .is_some_and(|game| campaign.plays(game));

                if !counts {
                    if is_api {
                        live.retain(|c| c.id != channel.id);
                    }

                    continue;
                }

                let offered = self.offers(channel, &campaign.id);

                match live.iter_mut().find(|c| c.id == channel.id) {
                    Some(known) => {
                        *known = Channel {
                            drops_enabled: offered || known.drops_enabled,
                            ..channel.clone()
                        };
                    }
                    None => live.push(Channel {
                        drops_enabled: offered,
                        ..channel.clone()
                    }),
                }
            }

            // ! A Twitch event counts only on a channel listing it in "Drops & More", whatever its category.
            if campaign.kind == CampaignKind::Quest {
                let kept_id = kept
                    .filter(|(campaign_id, _)| *campaign_id == campaign.id)
                    .map(|(_, channel)| channel.id.as_str());

                live.retain(|c| Some(c.id.as_str()) == kept_id || self.offers(c, &campaign.id));
            }

            if !is_api {
                live.sort_by_cached_key(|channel| {
                    drops_api::shuffle_key(&self.context.install_id, &channel.id)
                });
            }

            live_by_campaign.insert(campaign.id.clone(), live);
        }

        live_by_campaign
    }

    // ! A channel out of every list may come back on the same broadcast:
    // its answer goes only once the channel is seen offline or on another.
    pub(super) fn forget_stale_offers(
        &mut self,
        directory: &HashMap<String, Vec<Channel>>,
        rechecked: Option<&Channel>,
    ) {
        let streamers = &self.streamer_channels;

        self.offered_drops.retain(|id, offered| {
            let seen = streamers
                .values()
                .chain(rechecked)
                .chain(directory.values().flatten())
                .find(|channel| &channel.id == id);

            seen.is_none_or(|channel| {
                channel.live
                    && channel.broadcast_id.as_deref() == Some(offered.broadcast_id.as_str())
            })
        });
    }

    pub(super) fn still_watched(
        &self,
        campaign: &Campaign,
        watched: Option<&(String, Channel)>,
        now: Option<&Channel>,
    ) -> Option<Channel> {
        let (_, before) = watched?;

        let Some(now) = now else {
            return Some(before.clone());
        };

        if !now.live || !counts_for(now, campaign) {
            return None;
        }

        let drops_enabled = match self.offered(now) {
            Some(offered) if !offered.campaign_ids.contains(&campaign.id) => return None,
            Some(_) => true,
            None => before.drops_enabled,
        };

        Some(Channel {
            drops_enabled,
            ..now.clone()
        })
    }

    pub(super) fn drops_watch(&self) -> Option<(String, Channel)> {
        self.slots.iter().find_map(|slot| match &slot.purpose {
            SlotPurpose::Drop { campaign_id, .. } => {
                Some((campaign_id.clone(), slot.channel.clone()))
            }
            SlotPurpose::Points { .. } => None,
        })
    }

    pub(super) async fn ask_offered_drops(
        &mut self,
        campaigns: &[&Campaign],
        directory: &HashMap<String, Vec<Channel>>,
        recheck: Option<Channel>,
    ) {
        if self.settings.mode == Mode::Points {
            return;
        }

        let mut asked: Vec<Channel> = Vec::new();

        if self.settings.drops.streamers_first {
            asked = self
                .settings
                .points
                .streamers
                .iter()
                .filter(|streamer| streamer.enabled)
                .filter_map(|streamer| self.streamer_channels.get(&streamer.login))
                .filter(|channel| {
                    campaigns.iter().any(|campaign| {
                        campaign.allowed_channels.is_empty()
                            && counts_for(channel, campaign)
                            && (campaign.kind == CampaignKind::Quest
                                || listed(directory, campaign, &channel.id) != Some(true))
                    })
                })
                .cloned()
                .collect();
        }

        if let Some(channel) = recheck {
            if !asked.iter().any(|c| c.id == channel.id) {
                asked.push(channel);
            }
        }

        let unknown = |channel: &Channel| {
            channel.live && channel.broadcast_id.is_some() && self.offered(channel).is_none()
        };
        let mut quest_channels: Vec<Channel> = Vec::new();

        // ! The answers stay for the broadcast: a few directory channels of a Twitch event are asked each pass.
        for campaign in campaigns
            .iter()
            .filter(|c| c.kind == CampaignKind::Quest && c.allowed_channels.is_empty())
        {
            for channel in campaign
                .played()
                .filter_map(|game| directory.get(&game.id))
                .flatten()
            {
                if quest_channels.len() < QUEST_OFFERS_ASKED
                    && unknown(channel)
                    && !asked
                        .iter()
                        .chain(&quest_channels)
                        .any(|c| c.id == channel.id)
                {
                    quest_channels.push(channel.clone());
                }
            }
        }

        asked.extend(quest_channels);
        asked.retain(unknown);

        if asked.is_empty() {
            return;
        }

        let ids: Vec<String> = asked.iter().map(|channel| channel.id.clone()).collect();
        let answers = match self.client.available_drops(&ids).await {
            Ok(answers) => answers,
            Err(error) => {
                tracing::debug!(account = %self.login, %error, "available drops lookup failed");

                return;
            }
        };

        for (channel_id, campaign_ids) in answers {
            let broadcast = asked
                .iter()
                .find(|channel| channel.id == channel_id)
                .and_then(|channel| channel.broadcast_id.clone());

            if let Some(broadcast_id) = broadcast {
                self.offered_drops.insert(
                    channel_id,
                    OfferedDrops {
                        broadcast_id,
                        campaign_ids,
                    },
                );
            }
        }
    }

    pub(super) fn offered(&self, channel: &Channel) -> Option<&OfferedDrops> {
        self.offered_drops.get(&channel.id).filter(|offered| {
            channel.broadcast_id.as_deref() == Some(offered.broadcast_id.as_str())
        })
    }

    pub(super) fn offers(&self, channel: &Channel, campaign_id: &str) -> bool {
        self.offered(channel)
            .is_some_and(|offered| offered.campaign_ids.iter().any(|id| id == campaign_id))
    }
}

#[derive(Clone, Copy)]
pub(super) struct ChannelSources<'a> {
    pub(super) directory: &'a HashMap<String, Vec<Channel>>,
    pub(super) by_directory: &'a HashSet<String>,
    pub(super) allowed: Option<&'a [Channel]>,
    pub(super) api_channels: &'a HashMap<String, Vec<Channel>>,
}

pub(super) struct Lookup {
    pub(super) games: Vec<Game>,
    pub(super) logins: Vec<String>,
    pub(super) by_directory: HashSet<String>,
}

// ! A list longer than what is left of the channel read goes through its game's Drops directory.
// ! Every chosen campaign's game is read first; the other categories share the places left, in turn.
pub(super) fn lookup_targets(chosen: &[&Campaign], turn: usize) -> Lookup {
    let mut lookup = Lookup {
        games: Vec::new(),
        logins: Vec::new(),
        by_directory: HashSet::new(),
    };
    let mut others: Vec<Vec<&Game>> = Vec::new();
    let add = |games: &mut Vec<Game>, game: &Game| {
        if !games.iter().any(|known| known.id == game.id) {
            games.push(game.clone());
        }
    };

    for campaign in chosen {
        let mut logins: Vec<&String> = Vec::new();

        for channel in &campaign.allowed_channels {
            if !channel.login.is_empty()
                && !lookup.logins.contains(&channel.login)
                && !logins.contains(&&channel.login)
            {
                logins.push(&channel.login);
            }
        }

        if !campaign.allowed_channels.is_empty()
            && lookup.logins.len() + logins.len() <= ALLOWED_CHANNELS_MAX
        {
            lookup.logins.extend(logins.into_iter().cloned());

            continue;
        }

        if !campaign.allowed_channels.is_empty() {
            lookup.by_directory.insert(campaign.id.clone());
        }

        add(&mut lookup.games, &campaign.game);
        others.push(campaign.played().skip(1).collect());
    }

    for categories in &others {
        let read = categories.len().min(CATEGORIES_READ - 1);
        let start = if categories.len() > read {
            turn % categories.len()
        } else {
            0
        };

        for step in 0..read {
            if lookup.games.len() >= DIRECTORIES_MAX {
                break;
            }

            add(
                &mut lookup.games,
                categories[(start + step) % categories.len()],
            );
        }
    }

    lookup.games.truncate(DIRECTORIES_MAX);

    lookup
}

pub(super) fn counts_for(channel: &Channel, campaign: &Campaign) -> bool {
    channel
        .game
        .as_ref()
        .is_some_and(|game| campaign.plays(game))
}

pub(super) fn listed(
    directory: &HashMap<String, Vec<Channel>>,
    campaign: &Campaign,
    channel_id: &str,
) -> Option<bool> {
    let mut read = campaign
        .played()
        .filter_map(|game| directory.get(&game.id))
        .peekable();

    read.peek()?;

    Some(read.any(|channels| channels.iter().any(|c| c.id == channel_id)))
}
