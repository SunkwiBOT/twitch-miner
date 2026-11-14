export enum BadgeFilter {
  ALL = "ALL",
  WATCH = "WATCH",
  SUB = "SUB",
}

// ! Every channel owner has the Broadcaster badge: listing it says nothing.
export const HIDDEN_BADGE_SETS: ReadonlySet<string> = new Set(["broadcaster"]);

export const NEW_BADGE_MS = 7 * 24 * 60 * 60 * 1000;
