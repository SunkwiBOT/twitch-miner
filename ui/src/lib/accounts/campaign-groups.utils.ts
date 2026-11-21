import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import { BenefitKind } from "../bindings/BenefitKind";
import { CampaignKind } from "../bindings/CampaignKind";
import type { CampaignType } from "../bindings/CampaignType";
import { LeftOutReason } from "../bindings/LeftOutReason";
import { QueueBlock } from "../bindings/QueueBlock";
import type { QueueItemType } from "../bindings/QueueItemType";
import { QueueReason } from "../bindings/QueueReason";
import { minesCampaign } from "../catalogue/campaign-progress.utils";
import { dropDone, needsLink, subsMissing } from "../catalogue/campaign.utils";
import { MINUTE_MS } from "../time/time.constants";
import {
  CAMPAIGN_FACETS,
  CampaignFacet,
  CampaignGroup,
  CampaignState,
  FACET_SEPARATOR,
  LEFT_OUT_STATES,
  LinkFacet,
  PLANNING_STATUSES,
  RequirementFacet,
  STATE_GROUPS,
} from "./campaign-groups.constants";

export type AccountCampaignType = {
  campaign: CampaignType;
  state: CampaignState;
  queued: QueueItemType | null;
};

export type CampaignGroupsType = Record<CampaignGroup, AccountCampaignType[]>;

const QUEUE_BLOCK_STATES: Record<QueueBlock, CampaignState> = {
  [QueueBlock.NO_LIVE_CHANNEL]: CampaignState.NO_LIVE_CHANNEL,
  [QueueBlock.ONE_CAMPAIGN_AT_A_TIME]: CampaignState.QUEUED,
  [QueueBlock.SET_ASIDE]: CampaignState.SET_ASIDE,
};

const queueState = (account: AccountSnapshotType, item: QueueItemType): CampaignState => {
  if (minesCampaign(account, item.campaign_id)) {
    return CampaignState.MINING;
  }

  return item.blocked ? QUEUE_BLOCK_STATES[item.blocked] : CampaignState.QUEUED;
};

// ! A reward watched to the end waits for its claim until Twitch counts it as claimed.
export const collectedState = (
  account: AccountSnapshotType,
  campaign: CampaignType,
): CampaignState => {
  const refused = new Set(account.refused_claims);
  const unclaimed = campaign.drops.some(
    (drop) =>
      !dropDone(drop) &&
      !drop.badge_owned &&
      !subsMissing(drop) &&
      (drop.instance_id !== undefined || drop.current_minutes >= drop.required_minutes),
  );
  const linking = needsLink(campaign);

  if (campaign.drops.some((drop) => refused.has(drop.id))) {
    return CampaignState.CLAIM_FAILED;
  }

  if (unclaimed) {
    return linking && campaign.linked === false
      ? CampaignState.DONE_NOT_LINKED
      : CampaignState.DONE;
  }

  if (campaign.drops.some(subsMissing)) {
    return CampaignState.DONE_SUB_REQUIRED;
  }

  return linking && campaign.linked === true
    ? CampaignState.COLLECTED_LINKED
    : CampaignState.COLLECTED;
};

// ! The queue and `left_out` come from one plan: a campaign in neither appeared since, and waits for the next one.
export const accountCampaigns = (
  account: AccountSnapshotType,
  campaigns: readonly CampaignType[],
): CampaignGroupsType => {
  const groups: CampaignGroupsType = {
    [CampaignGroup.TO_MINE]: [],
    [CampaignGroup.BLOCKED]: [],
    [CampaignGroup.COLLECTED]: [],
  };
  const byId = new Map(campaigns.map((campaign) => [campaign.id, campaign]));
  const reasons = new Map(account.left_out.map((item) => [item.campaign_id, item.reason]));

  for (const item of account.queue) {
    const campaign = byId.get(item.campaign_id);

    if (campaign) {
      groups[CampaignGroup.TO_MINE].push({
        campaign,
        state: queueState(account, item),
        queued: item,
      });
    }
  }

  const queued = new Set(account.queue.map((item) => item.campaign_id));

  for (const campaign of campaigns) {
    const reason = reasons.get(campaign.id);

    if (reason === undefined || queued.has(campaign.id)) {
      continue;
    }

    const state =
      reason === LeftOutReason.FINISHED
        ? collectedState(account, campaign)
        : LEFT_OUT_STATES[reason];

    groups[STATE_GROUPS[state]].push({ campaign, state, queued: null });
  }

  return groups;
};

export const expectedStartIn = (
  account: AccountSnapshotType,
  item: QueueItemType,
  now: number,
): number | null =>
  item.expected_start && PLANNING_STATUSES.has(account.status)
    ? Math.max(MINUTE_MS, Date.parse(item.expected_start) - now)
    : null;

export type FacetValueType =
  | { facet: CampaignFacet.STATE; value: CampaignState }
  | { facet: CampaignFacet.REASON; value: QueueReason }
  | { facet: CampaignFacet.LINK; value: LinkFacet }
  | { facet: CampaignFacet.REQUIREMENT; value: RequirementFacet }
  | { facet: CampaignFacet.REWARD; value: BenefitKind };

export const facetKey = (option: FacetValueType): string =>
  `${option.facet}${FACET_SEPARATOR}${option.value}`;

const FACET_VALUES: readonly FacetValueType[] = [
  ...Object.values(CampaignState).map((value): FacetValueType => ({
    facet: CampaignFacet.STATE,
    value,
  })),
  ...Object.values(QueueReason).map((value): FacetValueType => ({
    facet: CampaignFacet.REASON,
    value,
  })),
  ...Object.values(LinkFacet).map((value): FacetValueType => ({
    facet: CampaignFacet.LINK,
    value,
  })),
  ...Object.values(RequirementFacet).map((value): FacetValueType => ({
    facet: CampaignFacet.REQUIREMENT,
    value,
  })),
  ...Object.values(BenefitKind).map((value): FacetValueType => ({
    facet: CampaignFacet.REWARD,
    value,
  })),
];

export const linkFacet = (campaign: CampaignType): LinkFacet | null => {
  if (campaign.kind === CampaignKind.QUEST || !needsLink(campaign)) {
    return LinkFacet.NO_LINK;
  }

  if (campaign.linked === undefined) {
    return null;
  }

  return campaign.linked ? LinkFacet.LINKED : LinkFacet.NOT_LINKED;
};

export const campaignFacets = (entry: AccountCampaignType): FacetValueType[] => {
  const { campaign } = entry;
  const link = linkFacet(campaign);
  const kinds = new Set(campaign.drops.flatMap((drop) => drop.benefits.map((b) => b.kind)));
  const facets: FacetValueType[] = [
    { facet: CampaignFacet.STATE, value: entry.state },
    {
      facet: CampaignFacet.REQUIREMENT,
      value: campaign.drops.some((drop) => drop.required_subs > 0)
        ? RequirementFacet.SUBSCRIPTION
        : RequirementFacet.WATCH_TIME,
    },
    ...[...kinds].map((value): FacetValueType => ({ facet: CampaignFacet.REWARD, value })),
  ];

  if (entry.queued) {
    facets.push({ facet: CampaignFacet.REASON, value: entry.queued.reason });
  }

  if (link) {
    facets.push({ facet: CampaignFacet.LINK, value: link });
  }

  return facets;
};

export const facetOptions = (entries: readonly AccountCampaignType[]): FacetValueType[] => {
  const present = new Set(entries.flatMap((entry) => campaignFacets(entry).map(facetKey)));

  return FACET_VALUES.filter((option) => present.has(facetKey(option)));
};

// ! Values of one facet widen the choice, facets narrow it: a campaign shows when each of its facets keeps one value ticked.
export const facetsShown = (entry: AccountCampaignType, hidden: ReadonlySet<string>): boolean => {
  const values = campaignFacets(entry);

  return CAMPAIGN_FACETS.every((facet) => {
    const own = values.filter((value) => value.facet === facet);

    return own.length === 0 || own.some((value) => !hidden.has(facetKey(value)));
  });
};
