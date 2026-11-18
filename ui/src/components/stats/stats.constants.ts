import { TwitchPointsReason } from "../../lib/events/events.constants";
import type { StatsPeriodDaysType, StatsReasonToneType } from "./stats.types";

export const STATS_PERIODS: StatsPeriodDaysType[] = [7, 30, 90];

export const STATS_REASON_TONES: StatsReasonToneType[] = [
  { bg: "bg-primary", fill: "fill-primary", text: "text-primary" },
  { bg: "bg-warning-emphasis", fill: "fill-warning-emphasis", text: "text-warning-emphasis" },
  { bg: "bg-success-emphasis", fill: "fill-success-emphasis", text: "text-success-emphasis" },
  { bg: "bg-info-emphasis", fill: "fill-info-emphasis", text: "text-info-emphasis" },
  { bg: "bg-link", fill: "fill-link", text: "text-link" },
  { bg: "bg-destructive", fill: "fill-destructive", text: "text-destructive" },
  { bg: "bg-muted-foreground", fill: "fill-muted-foreground", text: "text-muted-foreground" },
];

// ! Order pins each known Twitch reason code to a stable tone; any other
// code is hashed into the same palette instead of defaulting to one color.
export const STATS_KNOWN_REASONS: string[] = Object.values(TwitchPointsReason);

export const STATS_CATEGORY_TONES: string[] = [
  "bg-primary",
  "bg-info-emphasis",
  "bg-success-emphasis",
  "bg-warning-emphasis",
  "bg-link",
  "bg-destructive",
];
