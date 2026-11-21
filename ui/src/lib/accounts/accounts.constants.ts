import { FilterOp } from "../bindings/FilterOp";
import {
  CAMPAIGN_GROUP_SLUGS,
  CAMPAIGN_GROUPS,
  type CampaignGroup,
} from "./campaign-groups.constants";

export enum CopySection {
  DROPS = "DROPS",
  POINTS = "POINTS",
}

export enum ImportSource {
  SEARCH = "SEARCH",
  PASTE = "PASTE",
  ACCOUNT = "ACCOUNT",
  FOLLOWS = "FOLLOWS",
}

export const IMPORT_BATCH = 20;

export const IMPORT_STAGGER_MS = 35;

export const IMPORT_STAGGER_TOTAL_MS = 1500;

export enum ImportEntryStatus {
  PENDING = "PENDING",
  FOUND = "FOUND",
  ALREADY = "ALREADY",
  MISSING = "MISSING",
  UNCHECKED = "UNCHECKED",
  INVALID = "INVALID",
}

export enum SetupStep {
  START = "START",
  MODE = "MODE",
  DROPS = "DROPS",
  STREAMERS = "STREAMERS",
  SUMMARY = "SUMMARY",
}

export enum SetupSource {
  DEFAULTS = "DEFAULTS",
  CUSTOM = "CUSTOM",
}

export const FILTER_OP_SYMBOLS: Record<FilterOp, string> = {
  [FilterOp.GT]: ">",
  [FilterOp.GTE]: "≥",
  [FilterOp.LT]: "<",
  [FilterOp.LTE]: "≤",
};

export const PROXY_CREDENTIALS_MASK = "•••";

export const GROUP_CHIPS_MAX = 16;

export const U32_MAX = 4_294_967_295;

export enum StreamerOption {
  CLAIM_BONUS = "CLAIM_BONUS",
  WATCH_STREAK = "WATCH_STREAK",
  FOLLOW_RAIDS = "FOLLOW_RAIDS",
  CLAIM_MOMENTS = "CLAIM_MOMENTS",
  COMMUNITY_GOALS = "COMMUNITY_GOALS",
  CHAT = "CHAT",
  PREDICTIONS = "PREDICTIONS",
  STRATEGY = "STRATEGY",
  TIMING = "TIMING",
  PERCENTAGE = "PERCENTAGE",
  MAX_POINTS = "MAX_POINTS",
  MIN_POINTS = "MIN_POINTS",
  DELAY_SECONDS = "DELAY_SECONDS",
  TIMING_PERCENT = "TIMING_PERCENT",
  PERCENTAGE_GAP = "PERCENTAGE_GAP",
  STEALTH = "STEALTH",
  FILTER = "FILTER",
  FILTER_BY = "FILTER_BY",
  FILTER_OP = "FILTER_OP",
  FILTER_VALUE = "FILTER_VALUE",
}

export enum SetupEnd {
  START = "START",
  LATER = "LATER",
}

export enum DropsGameList {
  PRIORITY_GAMES = "priority_games",
  EXCLUDE_GAMES = "exclude_games",
}

export enum DropsList {
  EXCLUDE_NAMES = "exclude_names",
}

export enum BadgesList {
  EXCLUDE_NAMES = "exclude_names",
}

export enum DropsTab {
  ORDER = "ORDER",
  SETTINGS = "SETTINGS",
  EXCLUSIONS = "EXCLUSIONS",
}

export type DropsPageTabType = DropsTab | CampaignGroup;

export const DROPS_PAGE_TABS: readonly DropsPageTabType[] = [
  ...Object.values(DropsTab),
  ...CAMPAIGN_GROUPS,
];

export const DROPS_TAB_SLUGS: Record<DropsPageTabType, string> = {
  [DropsTab.ORDER]: "order",
  [DropsTab.SETTINGS]: "settings",
  [DropsTab.EXCLUSIONS]: "exclusions",
  ...CAMPAIGN_GROUP_SLUGS,
};

export enum BadgesTab {
  ORDER = "ORDER",
  SETTINGS = "SETTINGS",
  EXCLUSIONS = "EXCLUSIONS",
  OWNED = "OWNED",
}

export type BadgesPageTabType = BadgesTab | CampaignGroup;

export const BADGES_PAGE_TABS: readonly BadgesPageTabType[] = [
  BadgesTab.ORDER,
  BadgesTab.SETTINGS,
  BadgesTab.EXCLUSIONS,
  ...CAMPAIGN_GROUPS,
  BadgesTab.OWNED,
];

export const BADGES_TAB_SLUGS: Record<BadgesPageTabType, string> = {
  [BadgesTab.ORDER]: "order",
  [BadgesTab.SETTINGS]: "settings",
  [BadgesTab.EXCLUSIONS]: "exclusions",
  [BadgesTab.OWNED]: "owned",
  ...CAMPAIGN_GROUP_SLUGS,
};

export enum InventoryTab {
  CLAIMABLE = "CLAIMABLE",
  WAITING_LINK = "WAITING_LINK",
  IN_PROGRESS = "IN_PROGRESS",
  CLAIMED = "CLAIMED",
}

export const INVENTORY_TAB_SLUGS: Record<InventoryTab, string> = {
  [InventoryTab.CLAIMABLE]: "claimable",
  [InventoryTab.WAITING_LINK]: "waiting-link",
  [InventoryTab.IN_PROGRESS]: "in-progress",
  [InventoryTab.CLAIMED]: "claimed",
};
