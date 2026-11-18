import { EventKind } from "../bindings/EventKind";
import type { IconName } from "../icons.constants";
import { t } from "../i18n/i18n.svelte";
import { EVENT_ICONS, isTwitchPointsReason, TwitchPointsReason } from "./events.constants";

// ! Twitch's `POINTS_EARNED` reasons (`context.reason` on that event), shared
// between the Journal (`event-text.utils.ts`, `events.utils.ts`) and the
// Statistics page (`stats.utils.ts`): a single source for the label and the
// icon a reason maps to, so both stay consistent with each other and with
// the Journal's own per-kind icons.
const REASON_LABELS: Record<TwitchPointsReason, () => string> = {
  [TwitchPointsReason.WATCH]: () => t.stats_reason_watch(),
  [TwitchPointsReason.WATCH_STREAK]: () => t.stats_reason_watch_streak(),
  [TwitchPointsReason.CLAIM]: () => t.stats_reason_claim(),
  [TwitchPointsReason.RAID]: () => t.stats_reason_raid(),
  [TwitchPointsReason.COMMUNITY_MOMENT_COMPLETE]: () => t.stats_reason_community_moment(),
  [TwitchPointsReason.PREDICTION]: () => t.stats_reason_prediction(),
  [TwitchPointsReason.REFUND]: () => t.stats_reason_refund(),
};

const titleCase = (value: string): string =>
  value
    .toLowerCase()
    .split("_")
    .filter(Boolean)
    .map((word) => word[0].toUpperCase() + word.slice(1))
    .join(" ");

export const reasonLabel = (reason: string): string =>
  isTwitchPointsReason(reason) ? REASON_LABELS[reason]() : titleCase(reason);

// ! Only the reasons that have a more telling icon than the generic points
// one: `RAID` and `COMMUNITY_MOMENT_COMPLETE` reuse the Journal's own icon
// for that action (`RAID_JOINED`, `MOMENT_CLAIMED`); `CLAIM` reuses
// `BONUS_CLAIMED`'s (currently the same channel-points icon, kept through
// `EVENT_ICONS` rather than duplicated so a future change to one follows
// through). `WATCH_STREAK` already has its own icon through `eventIcon`'s
// existing streak check; `WATCH`, `PREDICTION`, `REFUND` and any unknown
// reason keep the default points icon.
const POINTS_EARNED_REASON_KINDS: Partial<Record<TwitchPointsReason, EventKind>> = {
  [TwitchPointsReason.RAID]: EventKind.RAID_JOINED,
  [TwitchPointsReason.CLAIM]: EventKind.BONUS_CLAIMED,
  [TwitchPointsReason.COMMUNITY_MOMENT_COMPLETE]: EventKind.MOMENT_CLAIMED,
};

export const pointsEarnedIcon = (reason: string | undefined): IconName | undefined => {
  if (reason === undefined || !isTwitchPointsReason(reason)) {
    return undefined;
  }

  const kind = POINTS_EARNED_REASON_KINDS[reason];

  return kind === undefined ? undefined : EVENT_ICONS[kind];
};

export const POINTS_EARNED_REASON_TEXT: ReadonlySet<string> = new Set(
  Object.keys(POINTS_EARNED_REASON_KINDS),
);
