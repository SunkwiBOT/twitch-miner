import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { BadgeRefType } from "../bindings/BadgeRefType";
import { BenefitKind } from "../bindings/BenefitKind";
import type { CampaignType } from "../bindings/CampaignType";
import type { EvolutionType } from "../bindings/EvolutionType";
import type { GlobalBadgeType } from "../bindings/GlobalBadgeType";
import type { QuestGroupType } from "../bindings/QuestGroupType";
import { QuestStatus } from "../bindings/QuestStatus";
import type { RepeatType } from "../bindings/RepeatType";
import type { TimedDropType } from "../bindings/TimedDropType";
import { dropDone, subsMissing } from "../catalogue/campaign.utils";
import { campaignOf, catalogueCampaign } from "../catalogue/catalogue.svelte";
import { badgeKey } from "./badges.utils";

export enum BadgeProgressState {
  OWNED = "OWNED",
  CLAIMABLE = "CLAIMABLE",
  SUB_ONLY = "SUB_ONLY",
  IN_PROGRESS = "IN_PROGRESS",
  NOT_STARTED = "NOT_STARTED",
  NOT_OFFERED = "NOT_OFFERED",
}

export type BadgeProgressType = {
  account: AccountSnapshotType;
  state: BadgeProgressState;
  current: number;
  required: number;
  done: number;
  closed: boolean;
  repeat: RepeatType | undefined;
};

const isBadge = (badge: GlobalBadgeType, refs: BadgeRefType[]): boolean =>
  refs.some((ref) => ref.set_id === badge.set_id && ref.version === badge.version_id);

const badgeDrop = (campaign: CampaignType, badge: GlobalBadgeType): TimedDropType | undefined => {
  const giving = campaign.drops.filter((drop) =>
    drop.benefits.some((benefit) => benefit.kind === BenefitKind.BADGE),
  );

  return (
    giving.find((drop) => drop.benefits.some((benefit) => isBadge(badge, benefit.badges))) ??
    giving.find((drop) => drop.name === badge.title) ??
    (giving.length === 1 ? giving[0] : undefined)
  );
};

const badgeGroup = (
  account: AccountSnapshotType,
  badge: GlobalBadgeType,
): QuestGroupType | undefined => {
  const groups = account.quests.find(
    (quest) => quest.id === badge.requirement?.campaign_id,
  )?.groups;

  return (
    groups?.find((group) => group.rewards.some((reward) => isBadge(badge, reward.badges))) ??
    groups?.find((group) => group.name === badge.title)
  );
};

const progress = (
  account: AccountSnapshotType,
  state: BadgeProgressState,
  rest: Partial<BadgeProgressType> = {},
): BadgeProgressType => ({
  account,
  state,
  current: 0,
  required: 0,
  done: 0,
  closed: false,
  repeat: undefined,
  ...rest,
});

const groupProgress = (account: AccountSnapshotType, group: QuestGroupType): BadgeProgressType => {
  const totals = {
    current: group.current_minutes,
    required: group.required_minutes,
    done: group.grant_count,
    closed: group.status === QuestStatus.CLAIMED,
    repeat: group.repeat,
  };

  if (group.status === QuestStatus.CLAIMABLE) {
    return progress(account, BadgeProgressState.CLAIMABLE, totals);
  }

  if (group.repeat ? group.grant_count >= group.repeat.times : totals.closed) {
    return progress(account, BadgeProgressState.OWNED);
  }

  if (subsMissing(group)) {
    return progress(account, BadgeProgressState.SUB_ONLY, totals);
  }

  return progress(
    account,
    group.current_minutes > 0 || group.grant_count > 0
      ? BadgeProgressState.IN_PROGRESS
      : BadgeProgressState.NOT_STARTED,
    totals,
  );
};

const dropProgress = (account: AccountSnapshotType, drop: TimedDropType): BadgeProgressType => {
  const totals = {
    current: drop.current_minutes,
    required: drop.required_minutes,
    done: drop.periods_done,
    closed: drop.claimed,
    repeat: drop.repeat,
  };

  if (drop.badge_owned || dropDone(drop)) {
    return progress(account, BadgeProgressState.OWNED);
  }

  if (drop.instance_id && !drop.claimed) {
    return progress(account, BadgeProgressState.CLAIMABLE, totals);
  }

  if (subsMissing(drop)) {
    return progress(account, BadgeProgressState.SUB_ONLY, totals);
  }

  return progress(
    account,
    drop.current_minutes > 0 || drop.periods_done > 0
      ? BadgeProgressState.IN_PROGRESS
      : BadgeProgressState.NOT_STARTED,
    totals,
  );
};

export const badgeProgress = (
  account: AccountSnapshotType,
  badge: GlobalBadgeType,
): BadgeProgressType => {
  const key = badgeKey(badge.set_id, badge.version_id);

  if (account.owned_badges.some((owned) => badgeKey(owned.set_id, owned.version) === key)) {
    return progress(account, BadgeProgressState.OWNED);
  }

  const group = badgeGroup(account, badge);

  if (group) {
    return groupProgress(account, group);
  }

  const id = badge.requirement?.campaign_id;
  const campaign = id ? campaignOf(account, id) : undefined;
  const drop = campaign && badgeDrop(campaign, badge);

  return drop ? dropProgress(account, drop) : progress(account, BadgeProgressState.NOT_OFFERED);
};

export const badgeCampaign = (
  badge: GlobalBadgeType,
  accounts: AccountSnapshotType[],
): CampaignType | null => {
  const id = badge.requirement?.campaign_id;

  if (!id) {
    return null;
  }

  for (const account of accounts) {
    const campaign = campaignOf(account, id);

    if (campaign) {
      return campaign;
    }
  }

  return catalogueCampaign(id) ?? null;
};

export const chestBadges = (campaign: CampaignType | null): BadgeRefType[] =>
  (campaign?.drops ?? [])
    .flatMap((drop) => drop.benefits)
    .filter((benefit) => benefit.chest_size && benefit.badges.length > 1)
    .flatMap((benefit) => benefit.badges);

// ! The badge list names the badge an evolution comes from; until it does, an account's quest may still flag the evolution.
export const badgeEvolution = (
  badge: GlobalBadgeType,
  list: GlobalBadgeType[],
  accounts: AccountSnapshotType[],
): EvolutionType | null => {
  const from = badge.requirement?.evolves_from;
  const named = from
    ? list.find((other) => badgeKey(other.set_id, other.version_id) === from)
    : null;

  if (named) {
    return {
      from: {
        set_id: named.set_id,
        version: named.version_id,
        title: named.title,
        image_url: named.image_url,
      },
      owned: false,
    };
  }

  const flagged = accounts.flatMap((account) => badgeGroup(account, badge)?.evolution ?? []);

  return flagged.find((evolution) => evolution.from) ?? flagged.at(0) ?? null;
};
