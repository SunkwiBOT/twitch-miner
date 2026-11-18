import { BetResult } from "../../lib/bindings/BetResult";
import { EventKind } from "../../lib/bindings/EventKind";
import { IconName } from "../../lib/icons.constants";

export type StatsAnnotationKindType =
  | EventKind.STREAK_RECOVERED
  | EventKind.RAID_JOINED
  | EventKind.MOMENT_CLAIMED
  | EventKind.PREDICTION_PLACED
  | `${EventKind.PREDICTION_RESULT}_${BetResult}`;

export type StatsAnnotationStyleType = { icon: IconName; text: string; bg: string };

// ! Icons reused from the Journal's own per-kind icons (`EVENT_ICONS`), so
// the curve's markers stay consistent with the rest of the app; only the
// prediction result splits further by `bet_result`, which the Journal does
// not need to since its badge text already states win/lose/refund.
// `text` colors the icon (it draws with `currentColor`); `bg` fills the
// marker's backing dot on the chart.
export const STATS_ANNOTATION_STYLES: Record<StatsAnnotationKindType, StatsAnnotationStyleType> = {
  [EventKind.STREAK_RECOVERED]: {
    icon: IconName.FLAME,
    text: "text-warning-emphasis",
    bg: "fill-warning-emphasis",
  },
  [EventKind.RAID_JOINED]: {
    icon: IconName.ZAP,
    text: "text-info-emphasis",
    bg: "fill-info-emphasis",
  },
  [EventKind.MOMENT_CLAIMED]: { icon: IconName.STAR, text: "text-link", bg: "fill-link" },
  [EventKind.PREDICTION_PLACED]: {
    icon: IconName.TARGET,
    text: "text-muted-foreground",
    bg: "fill-muted-foreground",
  },
  [`${EventKind.PREDICTION_RESULT}_${BetResult.WIN}`]: {
    icon: IconName.TROPHY,
    text: "text-success-emphasis",
    bg: "fill-success-emphasis",
  },
  [`${EventKind.PREDICTION_RESULT}_${BetResult.LOSE}`]: {
    icon: IconName.X,
    text: "text-destructive",
    bg: "fill-destructive",
  },
  [`${EventKind.PREDICTION_RESULT}_${BetResult.REFUND}`]: {
    icon: IconName.REFRESH,
    text: "text-muted-foreground",
    bg: "fill-muted-foreground",
  },
};

// ! Derived from the simple (non-templated) keys of `STATS_ANNOTATION_STYLES`
// instead of repeating the member list: the templated prediction-result
// keys are not valid `EventKind` values, so filtering against the real
// enum's own values separates the two without naming them twice.
const EVENT_KIND_VALUES: ReadonlySet<string> = new Set(Object.values(EventKind));

export const ANNOTATION_KINDS: ReadonlySet<EventKind> = new Set(
  Object.keys(STATS_ANNOTATION_STYLES).filter((key): key is EventKind =>
    EVENT_KIND_VALUES.has(key),
  ),
);
