import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import { BenefitKind } from "../bindings/BenefitKind";
import type { CampaignType } from "../bindings/CampaignType";
import { CampaignKind } from "../bindings/CampaignKind";
import type { GlobalBadgeType } from "../bindings/GlobalBadgeType";
import type { QuestGroupType } from "../bindings/QuestGroupType";
import { QuestStatus } from "../bindings/QuestStatus";
import type { QuestType } from "../bindings/QuestType";
import type { TimedDropType } from "../bindings/TimedDropType";
import { matchesSearch } from "../search/search.utils";
import { BadgeFilter, NEW_BADGE_MS } from "./badges.constants";

const time = (iso: string | undefined): number => (iso ? new Date(iso).getTime() : Number.NaN);

export const badgeKey = (setId: string, version: string): string => `${setId}/${version}`;

export const badgeEnded = (badge: GlobalBadgeType, now: number): boolean =>
  time(badge.requirement?.ends_at) <= now;

export const badgeMinable = (badge: GlobalBadgeType, now: number): boolean =>
  badge.requirement?.watch_time_only === true &&
  time(badge.requirement.starts_at) <= now &&
  time(badge.requirement.ends_at) > now;

export const badgeNeedsSub = (badge: GlobalBadgeType): boolean =>
  (badge.requirement?.required_subs ?? 0) > 0;

export const badgeIsNew = (badge: GlobalBadgeType, now: number): boolean =>
  now - time(badge.first_seen_at) < NEW_BADGE_MS;

export const minableBadgeCount = (badges: GlobalBadgeType[], now: number): number =>
  badges.filter((badge) => badgeMinable(badge, now)).length;

const filtered: Record<BadgeFilter, (badge: GlobalBadgeType, now: number) => boolean> = {
  [BadgeFilter.ALL]: () => true,
  [BadgeFilter.WATCH]: badgeMinable,
  [BadgeFilter.SUB]: (badge) => badgeNeedsSub(badge),
};

export const visibleBadges = (
  badges: GlobalBadgeType[],
  filter: BadgeFilter,
  needle: string,
  now: number,
): GlobalBadgeType[] =>
  badges
    .filter((badge) => filtered[filter](badge, now))
    .filter(
      (badge) =>
        !needle ||
        matchesSearch(badge.title, needle) ||
        matchesSearch(badge.requirement?.game_name ?? "", needle) ||
        matchesSearch(badge.requirement?.campaign_name ?? "", needle),
    )
    .sort(
      (a, b) =>
        Number(badgeMinable(b, now)) - Number(badgeMinable(a, now)) ||
        (time(b.first_seen_at) || 0) - (time(a.first_seen_at) || 0) ||
        a.title.localeCompare(b.title),
    );

export enum AccountBadgeState {
  IN_PROGRESS = "IN_PROGRESS",
  CLAIMABLE = "CLAIMABLE",
  AVAILABLE = "AVAILABLE",
  SUB_ONLY = "SUB_ONLY",
  NOT_MINABLE = "NOT_MINABLE",
  WAITING = "WAITING",
}

export type AccountBadgeType = {
  key: string;
  title: string;
  image: string | undefined;
  source: string;
  current: number;
  required: number;
  endsAt: string;
  state: AccountBadgeState;
  quest: QuestType | null;
  campaign: CampaignType | null;
  nextAt: string | undefined;
};

export type AccountBadgesType = {
  inProgress: AccountBadgeType[];
  available: AccountBadgeType[];
  notMined: AccountBadgeType[];
};

const dropGivesBadge = (drop: TimedDropType): boolean =>
  drop.benefits.some((benefit) => benefit.kind === BenefitKind.BADGE);

const dropState = (drop: TimedDropType): AccountBadgeState => {
  if (drop.required_subs > 0) {
    return AccountBadgeState.SUB_ONLY;
  }

  if (drop.instance_id) {
    return AccountBadgeState.CLAIMABLE;
  }

  return drop.current_minutes > 0 ? AccountBadgeState.IN_PROGRESS : AccountBadgeState.AVAILABLE;
};

const groupState = (group: QuestGroupType, now: number): AccountBadgeState => {
  if (group.required_subs > 0 || group.required_minutes === 0) {
    return AccountBadgeState.SUB_ONLY;
  }

  if (group.status === QuestStatus.UNKNOWN) {
    return AccountBadgeState.NOT_MINABLE;
  }

  if (group.status === QuestStatus.CLAIMABLE) {
    return AccountBadgeState.CLAIMABLE;
  }

  if (time(group.next_eligible_at) > now) {
    return AccountBadgeState.WAITING;
  }

  return group.status === QuestStatus.IN_PROGRESS
    ? AccountBadgeState.IN_PROGRESS
    : AccountBadgeState.AVAILABLE;
};

// ! A quest of several groups is a set of lanes: once one is started, the
// others no longer count.
export const questLanes = (quest: QuestType): QuestGroupType[] => {
  const started = quest.groups.filter((group) => group.status !== QuestStatus.NOT_STARTED);

  return started.length > 0 ? started : quest.groups;
};

const groupOwned = (
  account: AccountSnapshotType,
  quest: QuestType,
  group: QuestGroupType,
): boolean => {
  const owned = new Set(account.owned_badges.map((badge) => badgeKey(badge.set_id, badge.version)));
  const drop = account.campaigns
    .find((campaign) => campaign.id === quest.id)
    ?.drops.find((item) => item.id === group.id);

  return (
    drop?.badge_owned === true ||
    (group.rewards.length > 0 &&
      group.rewards.every(
        (reward) => reward.badge && owned.has(badgeKey(reward.badge.set_id, reward.badge.version)),
      ))
  );
};

export const accountBadges = (account: AccountSnapshotType, now: number): AccountBadgesType => {
  const items: AccountBadgeType[] = [];

  for (const campaign of account.campaigns) {
    if (campaign.kind === CampaignKind.QUEST || time(campaign.ends_at) <= now) {
      continue;
    }

    for (const drop of campaign.drops) {
      if (!dropGivesBadge(drop) || drop.claimed || drop.badge_owned) {
        continue;
      }

      items.push({
        key: `${campaign.id}/${drop.id}`,
        title: drop.name,
        image: drop.benefits.find((b) => b.kind === BenefitKind.BADGE)?.image_url,
        source: `${campaign.game.name} · ${campaign.name}`,
        current: drop.current_minutes,
        required: drop.required_minutes,
        endsAt: drop.ends_at < campaign.ends_at ? drop.ends_at : campaign.ends_at,
        state: dropState(drop),
        quest: null,
        campaign,
        nextAt: undefined,
      });
    }
  }

  for (const quest of account.quests) {
    if (quest.completed || time(quest.ends_at) <= now) {
      continue;
    }

    for (const group of questLanes(quest)) {
      if (group.status === QuestStatus.CLAIMED || groupOwned(account, quest, group)) {
        continue;
      }

      const reward = group.rewards.at(0);

      items.push({
        key: `${quest.id}/${group.id}`,
        title: group.name || reward?.name || quest.name,
        image: reward?.badge?.image_url ?? reward?.image_url ?? quest.image_url,
        source: quest.name,
        current: group.current_minutes,
        required: group.required_minutes,
        endsAt: quest.ends_at,
        state: quest.sitewide ? groupState(group, now) : AccountBadgeState.NOT_MINABLE,
        quest,
        campaign: null,
        nextAt: group.next_eligible_at,
      });
    }
  }

  const byEnd = (a: AccountBadgeType, b: AccountBadgeType): number =>
    time(a.endsAt) - time(b.endsAt) || a.title.localeCompare(b.title);

  return {
    inProgress: items
      .filter((item) =>
        [
          AccountBadgeState.IN_PROGRESS,
          AccountBadgeState.CLAIMABLE,
          AccountBadgeState.WAITING,
        ].includes(item.state),
      )
      .sort(byEnd),
    available: items.filter((item) => item.state === AccountBadgeState.AVAILABLE).sort(byEnd),
    notMined: items
      .filter((item) =>
        [AccountBadgeState.SUB_ONLY, AccountBadgeState.NOT_MINABLE].includes(item.state),
      )
      .sort(byEnd),
  };
};

export type OwnedBadgeType = { key: string; title: string; image: string | undefined };

export const ownedBadges = (
  account: AccountSnapshotType,
  catalogue: GlobalBadgeType[],
): OwnedBadgeType[] => {
  const global = new Map(
    catalogue.map((badge) => [badgeKey(badge.set_id, badge.version_id), badge]),
  );

  return account.owned_badges
    .map((owned) => ({ owned, known: global.get(badgeKey(owned.set_id, owned.version)) }))
    .filter(({ known }) => catalogue.length === 0 || known)
    .map(({ owned, known }) => ({
      key: badgeKey(owned.set_id, owned.version),
      title: known?.title ?? owned.title,
      image: known?.image_url ?? owned.image_url,
    }))
    .sort((a, b) => a.title.localeCompare(b.title));
};
