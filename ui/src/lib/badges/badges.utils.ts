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
import type { IgnoredType } from "../accounts/drops.utils";
import { BadgeFilter, HIDDEN_BADGE_SETS, NEW_BADGE_MS } from "./badges.constants";

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
  dropId: string;
  campaignId: string;
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
  ignored: IgnoredType | null;
};

// ! One row per campaign: its badges are mined, moved and ignored together.
export type BadgeRowType = {
  campaignId: string;
  campaign: CampaignType | null;
  items: AccountBadgeType[];
  current: number;
  required: number;
};

export type AccountBadgesType = {
  minable: AccountBadgeType[];
  notMined: AccountBadgeType[];
  ignored: AccountBadgeType[];
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

const ignoredOf = (
  account: AccountSnapshotType,
  campaign: CampaignType | null,
  campaignId: string,
  dropId: string,
  name: string,
): IgnoredType | null => {
  const drops = account.settings.drops;

  if (drops.exclude_drop_ids.includes(dropId)) {
    return { id: dropId, name, detail: "", drop: true, campaign };
  }

  if (drops.exclude_campaign_ids.includes(campaignId)) {
    return { id: campaignId, name: campaign?.name ?? name, detail: "", drop: false, campaign };
  }

  return null;
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
        dropId: drop.id,
        campaignId: campaign.id,
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
        ignored: ignoredOf(account, campaign, campaign.id, drop.id, drop.name),
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
      const title = group.name || reward?.name || quest.name;
      const campaign = account.campaigns.find((item) => item.id === quest.id) ?? null;

      items.push({
        key: `${quest.id}/${group.id}`,
        dropId: group.id,
        campaignId: quest.id,
        title,
        image: reward?.badge?.image_url ?? reward?.image_url ?? quest.image_url,
        source: quest.name,
        current: group.current_minutes,
        required: group.required_minutes,
        endsAt: quest.ends_at,
        state: quest.sitewide ? groupState(group, now) : AccountBadgeState.NOT_MINABLE,
        quest,
        campaign,
        nextAt: group.next_eligible_at,
        ignored: ignoredOf(account, campaign, quest.id, group.id, title),
      });
    }
  }

  const byEnd = (a: AccountBadgeType, b: AccountBadgeType): number =>
    time(a.endsAt) - time(b.endsAt) || a.title.localeCompare(b.title);

  const notMinable = (item: AccountBadgeType): boolean =>
    [AccountBadgeState.SUB_ONLY, AccountBadgeState.NOT_MINABLE].includes(item.state);

  return {
    minable: items.filter((item) => !item.ignored && !notMinable(item)).sort(byEnd),
    notMined: items.filter((item) => !item.ignored && notMinable(item)).sort(byEnd),
    ignored: items.filter((item) => item.ignored).sort(byEnd),
  };
};

// ! Rows follow the account's mining order; the ones it does not hold yet
// (waiting for a window, badges lane off) come after, by end date.
export const badgeRows = (items: AccountBadgeType[], order: string[]): BadgeRowType[] => {
  const rows = new Map<string, BadgeRowType>();

  for (const item of items) {
    const row = rows.get(item.campaignId) ?? {
      campaignId: item.campaignId,
      campaign: item.campaign,
      items: [],
      current: 0,
      required: 0,
    };

    row.items.push(item);
    row.current += Math.min(item.current, item.required);
    row.required += item.required;
    rows.set(item.campaignId, row);
  }

  const rank = (row: BadgeRowType): number => {
    const at = order.indexOf(row.campaignId);

    return at === -1 ? Number.MAX_SAFE_INTEGER : at;
  };

  return [...rows.values()].sort((a, b) => rank(a) - rank(b));
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
    .filter((owned) => !HIDDEN_BADGE_SETS.has(owned.set_id))
    .map((owned) => ({ owned, known: global.get(badgeKey(owned.set_id, owned.version)) }))
    .filter(({ known }) => catalogue.length === 0 || known)
    .map(({ owned, known }) => ({
      key: badgeKey(owned.set_id, owned.version),
      title: known?.title ?? owned.title,
      image: known?.image_url ?? owned.image_url,
    }))
    .sort((a, b) => a.title.localeCompare(b.title));
};
