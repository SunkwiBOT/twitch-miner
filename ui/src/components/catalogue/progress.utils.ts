import { relativeTime } from "../../lib/time/time.utils";
import { t } from "../../lib/i18n/i18n.svelte";
import { strong } from "../../lib/text.utils";
import { IconName } from "../../lib/icons.constants";
import { ProgressState, type AccountProgressType } from "../../lib/catalogue/progress.constants";
import { BadgeTone } from "../../lib/tones.constants";

type ProgressBadgeType = {
  text: () => string;
  tone: BadgeTone;
  icon?: IconName;
  hint?: (entry: AccountProgressType, now: number) => string;
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
    icon: IconName.PLAY,
    hint: () => t.badge_hint_queue_mining(),
  },
  [ProgressState.SET_ASIDE]: {
    text: () => t.badge_progress_set_aside(),
    tone: BadgeTone.WARNING,
    icon: IconName.CLOCK,
    hint: (entry, now) =>
      entry.setAsideUntil
        ? t.badge_hint_blocked_set_aside({
            retryTime: strong(relativeTime(entry.setAsideUntil, now)),
          })
        : "",
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

export type ProgressCountType = { state: ProgressState; count: number };

export const progressSummary = (entries: readonly AccountProgressType[]): ProgressCountType[] =>
  Object.values(ProgressState)
    .map((state) => ({ state, count: entries.filter((entry) => entry.state === state).length }))
    .filter((item) => item.count > 0);
