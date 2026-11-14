use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};

use crate::domain::{Campaign, CampaignKind};
use crate::twitch::drops::Award;
use crate::twitch::quests::{Quest, QuestStatus};

#[derive(Default)]
pub(super) struct ClaimRegistry {
    pub drops: HashSet<String>,
    // ! A repeated Drops reward claimed here, until the inventory counts it.
    pub periods: HashMap<String, Award>,
    // ! A quest group repeats on rolling windows: a claim or refusal holds for its window, named by the grant count.
    windows: HashMap<String, u32>,
    refused_windows: HashMap<String, u32>,
    refused: HashMap<String, Option<bool>>,
    pub finished: HashSet<String>,
}

impl ClaimRegistry {
    pub fn note_claimed(
        &mut self,
        drop_id: &str,
        quest_grant: Option<u32>,
        periods_done: Option<u32>,
        now: DateTime<Utc>,
    ) {
        if let Some(grant) = quest_grant {
            self.windows.insert(drop_id.to_owned(), grant);
        } else if let Some(done) = periods_done {
            self.periods.insert(
                drop_id.to_owned(),
                Award {
                    at: now,
                    count: done + 1,
                },
            );
        } else {
            self.drops.insert(drop_id.to_owned());
        }
    }

    pub fn refuse_window(&mut self, drop_id: &str, grant: u32) -> bool {
        self.refused_windows
            .insert(drop_id.to_owned(), grant)
            .is_none()
    }

    // ! Twitch refuses it again at every inventory read until the game account is linked: told once.
    pub fn refuse(&mut self, drop_id: &str, linked: Option<bool>) -> bool {
        self.refused.insert(drop_id.to_owned(), linked).is_none()
    }

    pub fn forget_stale_refusals(&mut self, campaigns: &HashMap<String, Campaign>) {
        self.refused.retain(|drop_id, linked| {
            campaigns
                .values()
                .find(|c| c.drops.iter().any(|d| &d.id == drop_id))
                .is_some_and(|c| c.linked == *linked)
        });
    }

    pub fn finish(&mut self, campaign_id: &str) -> bool {
        self.finished.insert(campaign_id.to_owned())
    }

    // ! Twitch's status and minutes rule a quest group: only while it still says CLAIMABLE for the window claimed is the claim held.
    pub fn settle_quest_windows(
        &mut self,
        quests: &[Quest],
        campaigns: &mut HashMap<String, Campaign>,
    ) {
        let claimable: HashMap<&str, u32> = quests
            .iter()
            .flat_map(|quest| &quest.groups)
            .filter(|group| group.status == QuestStatus::Claimable)
            .map(|group| (group.id.as_str(), group.grant_count))
            .collect();
        let same_window =
            |group_id: &String, grant: &mut u32| claimable.get(group_id.as_str()) == Some(grant);

        self.windows.retain(same_window);
        self.refused_windows.retain(same_window);

        for campaign in campaigns
            .values_mut()
            .filter(|c| c.kind == CampaignKind::Quest)
        {
            for drop in &mut campaign.drops {
                if self.windows.contains_key(&drop.id) {
                    drop.claimed = true;
                    drop.instance_id = None;
                } else if self.refused_windows.contains_key(&drop.id) {
                    drop.instance_id = None;
                }
            }
        }
    }
}
