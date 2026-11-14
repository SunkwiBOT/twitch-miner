import { t } from "../../lib/i18n/i18n.svelte";
import { IconName } from "../../lib/icons.constants";
import { ProgressState } from "../../lib/catalogue/progress.constants";
import { BadgeTone } from "../../lib/tones.constants";

type ProgressBadgeType = {
  text: () => string;
  tone: BadgeTone;
  icon?: IconName;
  hint?: () => string;
};

export const PROGRESS_BADGES: Record<ProgressState, ProgressBadgeType> = {
  [ProgressState.CLAIMABLE]: {
    text: () => t.reward_state_ready(),
    tone: BadgeTone.PRIMARY,
    icon: IconName.GIFT,
    hint: () => t.badge_hint_ready(),
  },
  [ProgressState.MINING]: {
    text: () => t.queue_mining(),
    tone: BadgeTone.SUCCESS,
    hint: () => t.badge_hint_queue_mining(),
  },
  [ProgressState.IN_PROGRESS]: {
    text: () => t.campaigns_filter_in_progress(),
    tone: BadgeTone.ACCENT,
    hint: () => t.badge_hint_in_progress(),
  },
  [ProgressState.NOT_STARTED]: {
    text: () => t.badge_progress_not_started(),
    tone: BadgeTone.MUTED,
  },
  [ProgressState.SUB_ONLY]: {
    text: () => t.reward_sub_only(),
    tone: BadgeTone.WARNING,
    hint: () => t.badge_hint_sub_only(),
  },
  [ProgressState.NOT_MINABLE]: {
    text: () => t.quest_not_minable(),
    tone: BadgeTone.MUTED,
    hint: () => t.badge_hint_quest_not_minable(),
  },
  [ProgressState.IGNORED]: {
    text: () => t.reward_ignored(),
    tone: BadgeTone.MUTED,
    icon: IconName.EYE_OFF,
  },
  [ProgressState.CLAIMED]: {
    text: () => t.reward_state_claimed(),
    tone: BadgeTone.SUCCESS,
    icon: IconName.CHECK,
    hint: () => t.badge_hint_claimed(),
  },
  [ProgressState.OWNED]: {
    text: () => t.reward_state_owned(),
    tone: BadgeTone.SUCCESS,
    icon: IconName.CHECK,
    hint: () => t.badge_hint_owned(),
  },
  [ProgressState.NOT_OFFERED]: {
    text: () => t.badge_progress_not_offered(),
    tone: BadgeTone.MUTED,
  },
};
