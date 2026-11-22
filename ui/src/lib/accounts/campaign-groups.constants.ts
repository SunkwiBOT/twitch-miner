import { AccountStatus } from "../bindings/AccountStatus";
import { LeftOutReason } from "../bindings/LeftOutReason";

export enum CampaignGroup {
  TO_MINE = "TO_MINE",
  BLOCKED = "BLOCKED",
  CLAIMED = "CLAIMED",
}

export const CAMPAIGN_GROUPS: readonly CampaignGroup[] = [
  CampaignGroup.TO_MINE,
  CampaignGroup.BLOCKED,
  CampaignGroup.CLAIMED,
];

export const CAMPAIGN_GROUP_SLUGS: Record<CampaignGroup, string> = {
  [CampaignGroup.TO_MINE]: "to-mine",
  [CampaignGroup.BLOCKED]: "blocked",
  [CampaignGroup.CLAIMED]: "claimed",
};

export enum CampaignState {
  MINING = "MINING",
  QUEUED = "QUEUED",
  NO_LIVE_CHANNEL = "NO_LIVE_CHANNEL",
  SET_ASIDE = "SET_ASIDE",
  CLAIMED_LINKED = "CLAIMED_LINKED",
  CLAIMED = "CLAIMED",
  DONE = "DONE",
  DONE_NOT_LINKED = "DONE_NOT_LINKED",
  CLAIM_FAILED = "CLAIM_FAILED",
  OWNED = "OWNED",
  IGNORED = "IGNORED",
  FILTERED = "FILTERED",
  EVOLUTIONS_OFF = "EVOLUTIONS_OFF",
  POINTS_ONLY = "POINTS_ONLY",
  SUB_REQUIRED = "SUB_REQUIRED",
  DONE_SUB_REQUIRED = "DONE_SUB_REQUIRED",
  EVOLUTION_LOCKED = "EVOLUTION_LOCKED",
  WAITING = "WAITING",
  UNFINISHABLE = "UNFINISHABLE",
  NOT_SELECTED = "NOT_SELECTED",
  NOT_LINKED = "NOT_LINKED",
  LANE_OFF = "LANE_OFF",
  NOT_ACTIVE = "NOT_ACTIVE",
}

export const STATE_GROUPS: Record<CampaignState, CampaignGroup> = {
  [CampaignState.MINING]: CampaignGroup.TO_MINE,
  [CampaignState.QUEUED]: CampaignGroup.TO_MINE,
  [CampaignState.NO_LIVE_CHANNEL]: CampaignGroup.TO_MINE,
  [CampaignState.SET_ASIDE]: CampaignGroup.TO_MINE,
  [CampaignState.CLAIMED_LINKED]: CampaignGroup.CLAIMED,
  [CampaignState.CLAIMED]: CampaignGroup.CLAIMED,
  [CampaignState.DONE]: CampaignGroup.CLAIMED,
  [CampaignState.DONE_NOT_LINKED]: CampaignGroup.CLAIMED,
  [CampaignState.CLAIM_FAILED]: CampaignGroup.CLAIMED,
  [CampaignState.OWNED]: CampaignGroup.CLAIMED,
  [CampaignState.IGNORED]: CampaignGroup.BLOCKED,
  [CampaignState.FILTERED]: CampaignGroup.BLOCKED,
  [CampaignState.EVOLUTIONS_OFF]: CampaignGroup.BLOCKED,
  [CampaignState.POINTS_ONLY]: CampaignGroup.BLOCKED,
  [CampaignState.SUB_REQUIRED]: CampaignGroup.BLOCKED,
  [CampaignState.DONE_SUB_REQUIRED]: CampaignGroup.BLOCKED,
  [CampaignState.EVOLUTION_LOCKED]: CampaignGroup.BLOCKED,
  [CampaignState.WAITING]: CampaignGroup.BLOCKED,
  [CampaignState.UNFINISHABLE]: CampaignGroup.BLOCKED,
  [CampaignState.NOT_SELECTED]: CampaignGroup.BLOCKED,
  [CampaignState.NOT_LINKED]: CampaignGroup.BLOCKED,
  [CampaignState.LANE_OFF]: CampaignGroup.BLOCKED,
  [CampaignState.NOT_ACTIVE]: CampaignGroup.BLOCKED,
};

// ! `FINISHED` alone needs the rewards themselves to say how the campaign ended.
export const LEFT_OUT_STATES: Record<
  Exclude<LeftOutReason, LeftOutReason.FINISHED>,
  CampaignState
> = {
  [LeftOutReason.OWNED]: CampaignState.OWNED,
  [LeftOutReason.NOT_ACTIVE]: CampaignState.NOT_ACTIVE,
  [LeftOutReason.IGNORED]: CampaignState.IGNORED,
  [LeftOutReason.EVOLUTIONS_OFF]: CampaignState.EVOLUTIONS_OFF,
  [LeftOutReason.FILTERED]: CampaignState.FILTERED,
  [LeftOutReason.POINTS_ONLY]: CampaignState.POINTS_ONLY,
  [LeftOutReason.SUB_REQUIRED]: CampaignState.SUB_REQUIRED,
  [LeftOutReason.EVOLUTION_LOCKED]: CampaignState.EVOLUTION_LOCKED,
  [LeftOutReason.WAITING]: CampaignState.WAITING,
  [LeftOutReason.UNFINISHABLE]: CampaignState.UNFINISHABLE,
  [LeftOutReason.NOT_SELECTED]: CampaignState.NOT_SELECTED,
  [LeftOutReason.NOT_LINKED]: CampaignState.NOT_LINKED,
  [LeftOutReason.LANE_OFF]: CampaignState.LANE_OFF,
};

// ! The queue is planned only while the account runs: otherwise its expected starts are stale.
export const PLANNING_STATUSES: ReadonlySet<AccountStatus> = new Set([
  AccountStatus.STARTING,
  AccountStatus.MINING,
  AccountStatus.IDLE,
]);

export enum CampaignFacet {
  STATE = "STATE",
  REASON = "REASON",
  LINK = "LINK",
  REQUIREMENT = "REQUIREMENT",
  REWARD = "REWARD",
}

export const CAMPAIGN_FACETS: readonly CampaignFacet[] = [
  CampaignFacet.STATE,
  CampaignFacet.REASON,
  CampaignFacet.LINK,
  CampaignFacet.REQUIREMENT,
  CampaignFacet.REWARD,
];

export enum LinkFacet {
  LINKED = "LINKED",
  NOT_LINKED = "NOT_LINKED",
  NO_LINK = "NO_LINK",
}

export enum RequirementFacet {
  WATCH_TIME = "WATCH_TIME",
  SUBSCRIPTION = "SUBSCRIPTION",
}

export const FACET_SEPARATOR = ":";
