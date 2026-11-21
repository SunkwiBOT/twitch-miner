export enum BadgeFilter {
  ALL = "ALL",
  WATCH = "WATCH",
  SUB = "SUB",
}

export const BADGE_FILTER_SLUGS: Record<BadgeFilter, string> = {
  [BadgeFilter.ALL]: "all",
  [BadgeFilter.WATCH]: "watch-time",
  [BadgeFilter.SUB]: "subscription",
};

// ! Every channel owner has the Broadcaster badge: listing it says nothing.
export const HIDDEN_BADGE_SETS: ReadonlySet<string> = new Set(["broadcaster"]);

export const NEW_BADGE_MS = 7 * 24 * 60 * 60 * 1000;

export const BADGE_TILE_CLASSES = "flex min-w-0 items-center gap-2.5 rounded-lg border p-2";
export const BADGE_TILE_INTERACTIVE_CLASSES =
  "group/tile relative text-left transition-colors hover:bg-accent focus-visible:bg-accent";
