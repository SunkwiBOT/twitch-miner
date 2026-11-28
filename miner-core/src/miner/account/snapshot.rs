use super::*;

impl<T: TwitchApi> AccountMiner<T> {
    pub(super) fn push_snapshot(&mut self) {
        let now = Utc::now();
        let slots = self
            .slots
            .iter()
            .map(|slot| {
                let drop = match &slot.purpose {
                    SlotPurpose::Drop {
                        campaign_id,
                        drop_id,
                    } => self.campaigns.get(campaign_id).and_then(|c| {
                        c.drops
                            .iter()
                            .find(|d| &d.id == drop_id)
                            .map(|d| DropProgressView {
                                campaign_id: c.id.clone(),
                                campaign_name: c.name.clone(),
                                game: c.game.name.clone(),
                                drop_id: d.id.clone(),
                                drop_name: d.name.clone(),
                                current_minutes: d.current_minutes,
                                required_minutes: d.required_minutes,
                                image_url: d
                                    .benefits
                                    .first()
                                    .and_then(|b| b.image_url.clone())
                                    .or_else(|| c.image_url.clone()),
                            })
                    }),
                    SlotPurpose::Points { .. } => None,
                };

                SlotView {
                    channel: slot.channel.clone(),
                    purpose: slot.purpose.clone(),
                    miner: miner_of(&slot.purpose, &self.campaigns),
                    since: slot.since,
                    last_beacon_at: slot.last_beacon_at,
                    segments_requested: slot
                        .playback
                        .as_ref()
                        .map(|p| p.segments_requested)
                        .unwrap_or(0),
                    drop,
                    points: self.streamer_points.get(&slot.channel.login).cloned(),
                }
            })
            .collect();
        let streamers = self
            .settings
            .points
            .streamers
            .iter()
            .map(|s| {
                let points = self
                    .streamer_points
                    .get(&s.login)
                    .cloned()
                    .unwrap_or_default();

                StreamerView {
                    login: s.login.clone(),
                    channel: self.streamer_channels.get(&s.login).cloned(),
                    streak_at_risk: points
                        .streak
                        .as_ref()
                        .is_some_and(|streak| streak.at_risk(now)),
                    points,
                    watching: self.slots.iter().any(|slot| slot.channel.login == s.login),
                }
            })
            .collect();
        let mut shown: Vec<&Campaign> = self
            .campaigns
            .values()
            .filter(|c| c.active_at(now) || c.in_inventory)
            .collect();

        shown.sort_by_key(|c| c.ends_at);

        // ! A catalogue being replaced is not waited for: the campaigns go whole this once.
        let catalog = self.context.catalog.try_read().ok();
        let (progress, other_campaigns) = split_campaigns(
            &shown,
            catalog.as_ref().map_or(&[][..], |c| c.campaigns.as_slice()),
        );

        drop(catalog);

        let snapshot = AccountSnapshot {
            id: self.id.clone(),
            login: self.login.clone(),
            display_name: self.display_name.clone(),
            source: self.source,
            avatar_url: self.avatar_url.clone(),
            settings: self.settings.clone(),
            status: self.status,
            problem: self.problem,
            slots,
            queue: self.last_queue.clone(),
            left_out: self.last_left_out.clone(),
            streamers,
            progress,
            other_campaigns,
            quests: self.quests.clone(),
            owned_badges: self.owned_badges.clone(),
            owned_badge_dates: self.badge_dates.clone(),
            drop_dates: self.drop_dates.clone(),
            refused_claims: self.claims.refused_drops(),
            badges_read_at: self.badges_read_at,
            stats: self.stats.clone(),
            updated_at: now,
            spam_channels: vec![],
        };

        let _ = self.snapshot.send(snapshot);
    }
}

pub(super) fn split_campaigns(
    shown: &[&Campaign],
    catalog: &[Campaign],
) -> (Vec<CampaignProgress>, Vec<Campaign>) {
    let by_id: HashMap<&str, &Campaign> = catalog.iter().map(|c| (c.id.as_str(), c)).collect();
    let mut progress = Vec::new();
    let mut others = Vec::new();

    for campaign in shown {
        match by_id
            .get(campaign.id.as_str())
            .and_then(|known| progress_over(known, campaign))
        {
            Some(state) => progress.push(state),
            None => others.push((*campaign).clone()),
        }
    }

    (progress, others)
}

// ! Every field is named so a new one cannot be left out of the comparison.
pub(super) fn progress_over(catalog: &Campaign, account: &Campaign) -> Option<CampaignProgress> {
    let Campaign {
        id,
        name,
        kind,
        game,
        categories,
        sitewide,
        status,
        linked,
        link_url,
        image_url,
        // ! Catalogue-merged campaigns carry it too: skipped here because the catalogue's own copy is what gets shown.
        web_url: _,
        starts_at,
        ends_at,
        allowed_channels,
        drops,
        in_inventory,
    } = account;

    let same = *id == catalog.id
        && *name == catalog.name
        && *kind == catalog.kind
        && *game == catalog.game
        && *categories == catalog.categories
        && *sitewide == catalog.sitewide
        && *status == catalog.status
        && *image_url == catalog.image_url
        && *starts_at == catalog.starts_at
        && *ends_at == catalog.ends_at
        && *allowed_channels == catalog.allowed_channels
        && drops.len() == catalog.drops.len();

    if !same {
        return None;
    }

    let mut states = Vec::new();

    for (drop, known) in drops.iter().zip(&catalog.drops) {
        let TimedDrop {
            id,
            name,
            starts_at,
            ends_at,
            required_minutes,
            required_subs,
            current_subs,
            current_minutes,
            instance_id,
            claimed,
            preconditions_met,
            precondition_drop_ids,
            benefits,
            badge_owned,
            repeat,
            after_all,
            periods_done,
            next_period_at,
            evolution,
            lane,
        } = drop;

        let same = *id == known.id
            && *name == known.name
            && *starts_at == known.starts_at
            && *ends_at == known.ends_at
            && *required_minutes == known.required_minutes
            && *required_subs == known.required_subs
            && *precondition_drop_ids == known.precondition_drop_ids
            && *benefits == known.benefits
            && *repeat == known.repeat
            && *after_all == known.after_all
            && *evolution == known.evolution
            && *lane == known.lane;

        if !same {
            return None;
        }

        let instance_id = over(instance_id, &known.instance_id)?;
        let preconditions_met = over(preconditions_met, &known.preconditions_met)?;

        if *current_minutes != known.current_minutes
            || *claimed != known.claimed
            || *badge_owned != known.badge_owned
            || *current_subs != known.current_subs
            || *periods_done != known.periods_done
            || *next_period_at != known.next_period_at
            || instance_id.is_some()
            || preconditions_met.is_some()
        {
            states.push(DropState {
                id: id.clone(),
                current_minutes: *current_minutes,
                instance_id,
                claimed: *claimed,
                preconditions_met,
                badge_owned: *badge_owned,
                current_subs: *current_subs,
                periods_done: *periods_done,
                next_period_at: *next_period_at,
            });
        }
    }

    Some(CampaignProgress {
        id: id.clone(),
        linked: over(linked, &catalog.linked)?,
        link_url: over(link_url, &catalog.link_url)?,
        in_inventory: *in_inventory,
        drops: states,
    })
}

pub(super) fn over<T: Clone + PartialEq>(mine: &Option<T>, known: &Option<T>) -> Option<Option<T>> {
    match mine {
        _ if mine == known => Some(None),
        Some(value) => Some(Some(value.clone())),
        None => None,
    }
}
