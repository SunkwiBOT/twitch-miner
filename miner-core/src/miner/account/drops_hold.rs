// ! Three different classic (non-Quest) Drops campaigns set aside in a row,
// each with zero minutes credited and no classic Drop progressing meanwhile,
// says Twitch is not crediting this account's Drops at all: its Quests,
// badges and channel points are unaffected and keep working, so they never
// feed this streak. Kept only in memory, like the set-aside state itself
// (`setaside::SetAside`): lost on a restart, but carried in every snapshot so
// a page refresh never loses it.
const HOLD_STREAK: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum HoldChange {
    Flagged,
    Cleared,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct DropsHold {
    // ! Distinct classic campaigns set aside in a row with zero progress,
    // oldest first; any classic Drop earning a minute clears it.
    streak: Vec<(String, String)>,
    flagged: bool,
}

impl DropsHold {
    pub(super) fn flagged(&self) -> bool {
        self.flagged
    }

    // ! How many campaigns were watched without a single minute, once flagged.
    pub(super) fn count(&self) -> Option<u32> {
        self.flagged.then_some(self.streak.len() as u32)
    }

    pub(super) fn campaign_names(&self) -> Vec<&str> {
        self.streak.iter().map(|(_, name)| name.as_str()).collect()
    }

    // ! Called once per inventory pass with every classic Drops campaign
    // newly set aside with zero progress this pass (a Quest campaign is
    // filtered out before it ever reaches here), and whether any classic
    // Drop anywhere on the account gained a minute since the previous pass.
    pub(super) fn observe(
        &mut self,
        newly_set_aside: &[(String, String)],
        classic_progress: bool,
    ) -> Option<HoldChange> {
        if classic_progress {
            let was_flagged = self.flagged;

            self.streak.clear();
            self.flagged = false;

            return was_flagged.then_some(HoldChange::Cleared);
        }

        for (id, name) in newly_set_aside {
            if !self.streak.iter().any(|(known, _)| known == id) {
                self.streak.push((id.clone(), name.clone()));
            }
        }

        if !self.flagged && self.streak.len() >= HOLD_STREAK {
            self.flagged = true;

            return Some(HoldChange::Flagged);
        }

        None
    }
}
