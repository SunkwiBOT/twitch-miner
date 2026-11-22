import { relativeTime } from "../../lib/time/time.utils";
import { t } from "../../lib/i18n/i18n.svelte";
import { strong } from "../../lib/text.utils";
import type { IconName } from "../../lib/icons.constants";
import { ProgressState, type AccountProgressType } from "../../lib/catalogue/progress.constants";
import {
  CLAIMED_VIEW,
  IGNORED_VIEW,
  MINING_VIEW,
  NOT_MINABLE_VIEW,
  OWNED_VIEW,
  READY_VIEW,
  SET_ASIDE_VIEW,
} from "../../lib/badge-views.constants";
import { BadgeTone } from "../../lib/tones.constants";

type ProgressBadgeType = {
  text: () => string;
  tone: BadgeTone;
  icon?: IconName;
  hint?: (entry: AccountProgressType, now: number) => string;
};

export const PROGRESS_BADGES: Record<ProgressState, ProgressBadgeType> = {
  [ProgressState.CLAIMABLE]: READY_VIEW,
  [ProgressState.MINING]: MINING_VIEW,
  [ProgressState.SET_ASIDE]: {
    ...SET_ASIDE_VIEW,
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
  [ProgressState.NOT_MINABLE]: NOT_MINABLE_VIEW,
  [ProgressState.IGNORED]: IGNORED_VIEW,
  [ProgressState.CLAIMED]: CLAIMED_VIEW,
  [ProgressState.OWNED]: OWNED_VIEW,
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
