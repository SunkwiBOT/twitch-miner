import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { BadgeRefType } from "../bindings/BadgeRefType";
import { BenefitKind } from "../bindings/BenefitKind";
import type { CampaignType } from "../bindings/CampaignType";
import type { EvolutionType } from "../bindings/EvolutionType";
import type { GlobalBadgeType } from "../bindings/GlobalBadgeType";
import { QueueBlock } from "../bindings/QueueBlock";
import type { QuestGroupType } from "../bindings/QuestGroupType";
import { QuestStatus } from "../bindings/QuestStatus";
import type { QuestType } from "../bindings/QuestType";
import type { RepeatType } from "../bindings/RepeatType";
import type { TimedDropType } from "../bindings/TimedDropType";
import {
  dropDone,
  evolutionBlocked,
  periodTotals,
  subsMissing,
  type PeriodsType,
} from "../catalogue/campaign.utils";
import { campaignOf, catalogueCampaign } from "../catalogue/catalogue.svelte";
import { ProgressState, type AccountProgressType } from "../catalogue/progress.constants";
import { badgeKey } from "./badges.utils";

export type BadgeProgressType = AccountProgressType & {
  done: number;
  repeat: RepeatType | undefined;
};

type WatchedType = {
  id: string;
  periods: PeriodsType;
  claimable: boolean;
  done: boolean;
  subs: boolean;
  reachable: boolean;
  started: boolean;
  evolution: EvolutionType | undefined;
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
  quest: QuestType | undefined,
  badge: GlobalBadgeType,
): QuestGroupType | undefined =>
  quest?.groups.find((group) => group.rewards.some((reward) => isBadge(badge, reward.badges))) ??
  quest?.groups.find((group) => group.name === badge.title);

const watchedProgress = (
  account: AccountSnapshotType,
  campaignId: string,
  item: WatchedType,
): BadgeProgressType => {
  const totals = periodTotals(item.periods);
  const drops = account.settings.drops;
  const queued = account.queue.find((q) => q.campaign_id === campaignId);
  const entry = {
    account,
    current: totals.current,
    required: totals.required,
    minable: false,
    done: item.periods.done,
    repeat: item.periods.repeat,
    setAsideUntil: queued?.set_aside_until ?? null,
  };

  if (item.claimable) {
    return { ...entry, state: ProgressState.CLAIMABLE };
  }

  if (item.done) {
    return { ...entry, state: ProgressState.OWNED };
  }

  if (item.subs) {
    return { ...entry, state: ProgressState.SUB_ONLY };
  }

  const minable =
    item.reachable && !evolutionBlocked(item.evolution, account.settings.badges.evolutions);

  const state = (): ProgressState => {
    if (
      drops.exclude_campaign_ids.includes(campaignId) ||
      drops.exclude_drop_ids.includes(item.id)
    ) {
      return ProgressState.IGNORED;
    }

    if (!item.reachable) {
      return ProgressState.NOT_MINABLE;
    }

    if (queued?.blocked === QueueBlock.SET_ASIDE) {
      return ProgressState.SET_ASIDE;
    }

    if (
      account.slots.some(
        (slot) => slot.drop?.campaign_id === campaignId && slot.drop.drop_id === item.id,
      )
    ) {
      return ProgressState.MINING;
    }

    return item.started ? ProgressState.IN_PROGRESS : ProgressState.NOT_STARTED;
  };

  return { ...entry, state: state(), minable };
};

export const badgeProgress = (
  account: AccountSnapshotType,
  badge: GlobalBadgeType,
): BadgeProgressType => {
  const key = badgeKey(badge.set_id, badge.version_id);
  const none = {
    account,
    current: 0,
    required: 0,
    minable: false,
    done: 0,
    repeat: undefined,
    setAsideUntil: null,
  };

  if (account.owned_badges.some((owned) => badgeKey(owned.set_id, owned.version) === key)) {
    return { ...none, state: ProgressState.OWNED };
  }

  const id = badge.requirement?.campaign_id;
  const quest = account.quests.find((item) => item.id === id);
  const group = badgeGroup(quest, badge);

  if (quest && group) {
    return watchedProgress(account, quest.id, {
      id: group.id,
      periods: {
        current: group.current_minutes,
        required: group.required_minutes,
        done: group.grant_count,
        closed: group.status === QuestStatus.CLAIMED,
        repeat: group.repeat,
      },
      claimable: group.status === QuestStatus.CLAIMABLE,
      done: group.repeat
        ? group.grant_count >= group.repeat.times
        : group.status === QuestStatus.CLAIMED,
      subs: subsMissing(group) || group.required_minutes === 0,
      reachable: quest.sitewide && group.status !== QuestStatus.UNKNOWN,
      started: group.current_minutes > 0 || group.grant_count > 0,
      evolution: group.evolution,
    });
  }

  const campaign = id ? campaignOf(account, id) : undefined;
  const drop = campaign && badgeDrop(campaign, badge);

  if (!campaign || !drop) {
    return { ...none, state: ProgressState.NOT_OFFERED };
  }

  return watchedProgress(account, campaign.id, {
    id: drop.id,
    periods: {
      current: drop.current_minutes,
      required: drop.required_minutes,
      done: drop.periods_done,
      closed: drop.claimed,
      repeat: drop.repeat,
    },
    claimable: drop.instance_id !== undefined && !drop.claimed,
    done: drop.badge_owned || dropDone(drop),
    subs: subsMissing(drop),
    reachable: true,
    started: drop.current_minutes > 0 || drop.periods_done > 0,
    evolution: drop.evolution,
  });
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

  const flagged = accounts.flatMap(
    (account) =>
      badgeGroup(
        account.quests.find((quest) => quest.id === badge.requirement?.campaign_id),
        badge,
      )?.evolution ?? [],
  );

  return flagged.find((evolution) => evolution.from) ?? flagged.at(0) ?? null;
};
