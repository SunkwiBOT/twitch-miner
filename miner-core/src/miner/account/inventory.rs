use super::*;
use crate::badges::GlobalBadge;
use crate::twitch::drops::Inventory;

pub(super) struct Held<'a> {
    pub user_id: &'a str,
    pub quests: &'a [Quest],
    pub badges: &'a [GlobalBadge],
    pub owned: &'a OwnedBadges,
    pub campaigns: &'a HashMap<String, Campaign>,
    pub claims: &'a ClaimRegistry,
}

pub(super) struct Merged {
    pub campaigns: HashMap<String, Campaign>,
    pub claimed_periods: HashMap<String, Award>,
    pub drop_dates: HashMap<String, DateTime<Utc>>,
    pub awards: HashMap<String, Award>,
}

impl<T: TwitchApi> AccountMiner<T> {
    pub(super) async fn refresh_inventory(&mut self) -> Result<()> {
        let mut inventory = self.client.inventory().await?;
        let names = std::mem::take(&mut inventory.names);
        let mut quests_read = true;

        match self.client.quests().await {
            Ok(mut read) => {
                {
                    let badges = self.context.badges.read().await;
                    let owned = OwnedBadges::new(&read.owned);

                    for quest in &mut read.quests {
                        quest.attach_badges(&badges.badges, &owned);
                    }
                }

                let first = self.badges_read_at.is_none();
                let before = std::mem::replace(&mut self.quests, read.quests);

                self.owned_badges = read.owned;
                self.selected_badge = read.selected;
                self.badges_read_at = Some(Utc::now());

                if !first {
                    self.tell_quest_progress(&before);
                }
            }
            Err(Error::Unauthorized) => return Err(Error::Unauthorized),
            Err(error) => {
                quests_read = false;

                tracing::debug!(account = %self.login, %error, "quests and badges not read")
            }
        }

        let catalog = self.context.catalog.read().await.campaigns.clone();
        let badges = self.context.badges.read().await;
        let merged = merge_inventory(
            catalog,
            inventory,
            &Held {
                user_id: &self.id,
                quests: &self.quests,
                badges: &badges.badges,
                owned: &OwnedBadges::new(&self.owned_badges),
                campaigns: &self.campaigns,
                claims: &self.claims,
            },
            Utc::now(),
        );

        drop(badges);

        let mut campaigns = merged.campaigns;

        self.badge_dates = award_dates(
            &self.owned_badges,
            campaigns.values(),
            &merged.awards,
            &names,
        );
        self.claims.periods = merged.claimed_periods;
        self.drop_dates = merged.drop_dates;
        self.claims
            .settle_quest_windows(&self.quests, &mut campaigns);

        let now = Utc::now();
        let excluded = &self.settings.drops.exclude_drop_ids;
        // ! A real category id only: quests carry an empty `game.id` and
        // never count as siblings of one another.
        let lifted_categories: HashSet<String> = campaigns
            .values()
            .filter(|campaign| !campaign.game.id.is_empty())
            .filter(|campaign| campaign.finished(now, excluded))
            .filter(|campaign| {
                self.campaigns
                    .get(&campaign.id)
                    .is_none_or(|previous| !previous.finished(now, excluded))
            })
            .map(|campaign| campaign.game.id.clone())
            .collect();

        self.campaigns = campaigns;

        let stale_quests: HashSet<String> = if quests_read {
            HashSet::new()
        } else {
            self.quests.iter().map(|quest| quest.id.clone()).collect()
        };

        for (campaign_id, drop_id, retry_at) in setaside::observe_all(
            &mut self.set_aside,
            &self.campaigns,
            excluded,
            &self.settings.drops.reward_priority,
            &stale_quests,
            &lifted_categories,
            now,
        ) {
            self.set_aside_campaign(&campaign_id, &drop_id, retry_at);
        }

        Ok(())
    }

    // ! Twitch sends no live progress for a quest: each inventory read writes the groups that moved.
    pub(super) fn tell_quest_progress(&mut self, before: &[Quest]) {
        struct Moved {
            quest_id: String,
            group_id: String,
            text: String,
            current: u32,
            required: u32,
        }

        let now = Utc::now();
        let seen: HashMap<&str, (u32, QuestStatus)> = before
            .iter()
            .flat_map(|quest| &quest.groups)
            .map(|group| (group.id.as_str(), (group.current_minutes, group.status)))
            .collect();
        let moved: Vec<Moved> = self
            .quests
            .iter()
            .filter(|quest| quest.mined_at(now))
            .flat_map(|quest| quest.groups.iter().map(move |group| (quest, group)))
            .filter(|(_, group)| {
                matches!(
                    group.status,
                    QuestStatus::InProgress | QuestStatus::Claimable
                ) && seen.get(group.id.as_str()) != Some(&(group.current_minutes, group.status))
            })
            .map(|(quest, group)| {
                let (current, required) = (group.current_minutes, group.required_minutes);
                let mut text = format!("{}: {current}/{required} min", group.label());

                if let Some(repeat) = group.repeat {
                    text.push_str(&format!(
                        ", period {}/{}",
                        (group.grant_count + 1).min(repeat.times),
                        repeat.times
                    ));
                }

                text.push_str(&format!(" ({})", quest.name));

                if let Some(at) = group.waiting_until(now) {
                    text.push_str(&format!(
                        ", next window at {} UTC",
                        at.format("%Y-%m-%d %H:%M")
                    ));
                }

                Moved {
                    quest_id: quest.id.clone(),
                    group_id: group.id.clone(),
                    text,
                    current,
                    required,
                }
            })
            .collect();

        for moved in moved {
            let event = self
                .emit(EventKind::DropProgress, moved.text)
                .context(EventContext {
                    current: Some(moved.current),
                    required: Some(moved.required),
                    ..EventContext::drop(&moved.quest_id, &moved.group_id)
                });

            self.publish(event);
        }
    }

    pub(super) fn quest_group(&self, group_id: &str) -> Option<&QuestGroup> {
        self.quests
            .iter()
            .flat_map(|quest| &quest.groups)
            .find(|group| group.id == group_id)
    }

    pub(super) fn note_claimed(&mut self, drop_id: &str) {
        let quest_grant = self.quest_group(drop_id).map(|group| group.grant_count);
        let periods_done = self
            .campaigns
            .values()
            .flat_map(|c| &c.drops)
            .find(|d| d.id == drop_id && d.repeat.is_some())
            .map(|drop| drop.periods_done);

        self.claims
            .note_claimed(drop_id, quest_grant, periods_done, Utc::now());
    }
}

pub(super) fn merge_inventory(
    catalog: Vec<Campaign>,
    inventory: Inventory,
    held: &Held,
    now: DateTime<Utc>,
) -> Merged {
    let mut merged: HashMap<String, Campaign> = catalog
        .into_iter()
        .map(|campaign| (campaign.id.clone(), campaign))
        .collect();
    let from_inventory: HashSet<String> = inventory
        .campaigns
        .iter()
        .flat_map(|c| c.drops.iter().map(|d| d.id.clone()))
        .collect();

    for campaign in inventory.campaigns {
        match merged.get_mut(&campaign.id) {
            Some(existing) => {
                existing.linked = campaign.linked.or(existing.linked);
                existing.in_inventory = true;

                if existing.link_url.is_none() {
                    existing.link_url = campaign.link_url.clone();
                }

                for drop in &campaign.drops {
                    if let Some(target) = existing.drops.iter_mut().find(|d| d.id == drop.id) {
                        if target.repeat.is_some() {
                            target.current_minutes = drop.current_minutes;
                            target.claimed = drop.claimed;
                        } else {
                            target.current_minutes =
                                target.current_minutes.max(drop.current_minutes);
                            target.claimed = target.claimed || drop.claimed;
                        }

                        target.current_subs = target.current_subs.max(drop.current_subs);
                        target.instance_id =
                            drop.instance_id.clone().or(target.instance_id.clone());
                        target.preconditions_met =
                            drop.preconditions_met.or(target.preconditions_met);
                    } else {
                        existing.drops.push(drop.clone());
                    }
                }

                if existing.allowed_channels.is_empty() {
                    existing.allowed_channels = campaign.allowed_channels.clone();
                }
            }
            None => {
                merged.insert(campaign.id.clone(), campaign);
            }
        }
    }

    for quest in held.quests.iter().filter(|q| q.mined_at(now)) {
        if let Some(campaign) = quest.to_campaign(held.user_id, now) {
            merged.insert(campaign.id.clone(), campaign);
        }
    }

    for campaign in merged.values_mut() {
        if campaign.kind != CampaignKind::Quest {
            attach_badges(campaign, held.badges);
        }

        mark_owned(campaign, held.owned);
    }

    let mut claimed_periods: HashMap<String, Award> = HashMap::new();
    let mut drop_dates: HashMap<String, DateTime<Utc>> = HashMap::new();

    // ! A reward id can serve several campaigns: an award counts only out of the inventory and inside the drop's window.
    for campaign in merged
        .values_mut()
        .filter(|c| c.kind != CampaignKind::Quest)
    {
        for drop in &mut campaign.drops {
            let awarded: Vec<&Award> = drop
                .benefits
                .iter()
                .filter_map(|b| inventory.awards.get(&b.id))
                .filter(|award| drop.starts_at <= award.at && award.at < drop.ends_at)
                .collect();
            let every_benefit = !drop.benefits.is_empty() && awarded.len() == drop.benefits.len();

            if let Some(at) = awarded
                .iter()
                .map(|award| award.at)
                .max()
                .filter(|_| every_benefit)
            {
                drop_dates.insert(drop.id.clone(), at);
            }

            if let Some(repeat) = drop.repeat {
                let award = every_benefit
                    .then(|| awarded.iter().min_by_key(|award| award.count).copied())
                    .flatten()
                    .map(|award| periods_held(drop, repeat, *award));
                let claimed = held
                    .claims
                    .periods
                    .get(&drop.id)
                    .filter(|claimed| award.is_none_or(|award| award.count < claimed.count));

                if let Some(claimed) = claimed {
                    claimed_periods.insert(drop.id.clone(), *claimed);
                }

                settle_period(drop, award.as_ref(), claimed, now);
            } else if !drop.claimed && !from_inventory.contains(&drop.id) && every_benefit {
                drop.claimed = true;
            }
        }
    }

    for (id, previous) in held.campaigns {
        if let Some(next) = merged.get_mut(id).filter(|c| c.kind != CampaignKind::Quest) {
            // ! An answer without the link state keeps the last one Twitch gave: it decides whether a claim is worth sending.
            if previous.in_inventory {
                next.linked = next.linked.or(previous.linked);
            }

            for drop in next.drops.iter_mut().filter(|d| d.repeat.is_none()) {
                if let Some(old) = previous.drops.iter().find(|d| d.id == drop.id) {
                    drop.current_minutes = drop.current_minutes.max(old.current_minutes);
                    drop.claimed = drop.claimed || old.claimed;
                }
            }
        }
    }

    // ! The link is per game account: a campaign never started takes the state of another of its game.
    let mut linked_games: HashMap<String, bool> = HashMap::new();

    for campaign in merged.values().filter(|c| c.kind != CampaignKind::Quest) {
        if let Some(linked) = campaign.linked.filter(|_| !campaign.game.id.is_empty()) {
            *linked_games.entry(campaign.game.id.clone()).or_default() |= linked;
        }
    }

    for campaign in merged
        .values_mut()
        .filter(|c| c.kind != CampaignKind::Quest && c.linked.is_none() && c.needs_link())
    {
        campaign.linked = linked_games.get(&campaign.game.id).copied();
    }

    for campaign in merged
        .values_mut()
        .filter(|c| c.kind != CampaignKind::Quest)
    {
        for drop in &mut campaign.drops {
            if drop.repeat.is_none() && held.claims.drops.contains(&drop.id) {
                drop.claimed = true;
            }
        }
    }

    Merged {
        campaigns: merged,
        claimed_periods,
        drop_dates,
        awards: inventory.awards,
    }
}

// ! Twitch counts a repeated reward's periods with its award; a claim made here counts until it does, and lapses with its period.
pub(super) fn settle_period(
    drop: &mut TimedDrop,
    award: Option<&Award>,
    claimed: Option<&Award>,
    now: DateTime<Utc>,
) {
    let Some(repeat) = drop.repeat else {
        return;
    };
    let Some(last) = award
        .into_iter()
        .chain(claimed)
        .max_by_key(|award| (award.count, award.at))
    else {
        return;
    };

    drop.periods_done = last.count;
    drop.next_period_at = None;

    match repeat
        .every()
        .and_then(|every| last.at.checked_add_signed(every))
    {
        Some(next) if next > now => {
            drop.next_period_at = Some(next);
            drop.claimed = true;
        }
        Some(_) if drop.claimed => {
            drop.claimed = false;
            drop.current_minutes = 0;
            drop.instance_id = None;
        }
        _ => {}
    }
}

pub(super) fn close_period(drop: &mut TimedDrop) {
    drop.claimed = true;
    drop.instance_id = None;

    if let Some(repeat) = drop.repeat {
        drop.periods_done += 1;
        drop.next_period_at = repeat
            .every()
            .and_then(|every| Utc::now().checked_add_signed(every));
    }
}

// ! Twitch's count covers the reward id in every campaign: never more periods than this drop's window held by the award.
pub(super) fn periods_held(drop: &TimedDrop, repeat: Repeat, award: Award) -> Award {
    let held = repeat.every().map_or(repeat.times, |every| {
        let spans = (award.at - drop.starts_at).num_seconds() / every.num_seconds().max(1);

        u32::try_from(spans).unwrap_or(0).saturating_add(1)
    });

    Award {
        count: award.count.min(repeat.times).min(held),
        ..award
    }
}
