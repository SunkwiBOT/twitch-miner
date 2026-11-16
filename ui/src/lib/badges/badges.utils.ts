import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { BadgeRefType } from "../bindings/BadgeRefType";
import { BenefitKind } from "../bindings/BenefitKind";
import type { CampaignType } from "../bindings/CampaignType";
import { CampaignKind } from "../bindings/CampaignKind";
import type { EvolutionType } from "../bindings/EvolutionType";
import type { GlobalBadgeType } from "../bindings/GlobalBadgeType";
import type { QuestGroupType } from "../bindings/QuestGroupType";
import { QuestStatus } from "../bindings/QuestStatus";
import type { QuestType } from "../bindings/QuestType";
import type { RepeatType } from "../bindings/RepeatType";
import type { TimedDropType } from "../bindings/TimedDropType";
import { dropDone, periodTotals, subsMissing } from "../catalogue/campaign.utils";
import { campaignOf, campaignsOf } from "../catalogue/catalogue.svelte";
import { gameArt } from "../catalogue/games.utils";
import {
  toTimelineRows,
  type TimelineHeadType,
  type TimelineRowType,
} from "../catalogue/timeline.utils";
import { BoxArtSize } from "../format.constants";
import { boxArt } from "../format.utils";
import { matchesSearch } from "../search/search.utils";
import { t } from "../i18n/i18n.svelte";
import type { BadgeRequirementType } from "../bindings/BadgeRequirementType";
import type { IgnoredType } from "../accounts/drops.utils";
import { badgeUrl } from "./badge-slug.utils";
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

// ! A collection or a sitewide quest spans many games: its badges share the collection's row, a Drops campaign's the game's.
const badgeRow = (badge: GlobalBadgeType, rule: BadgeRequirementType): TimelineHeadType => {
  const collection = rule.collection !== undefined || rule.categories.length > 1;

  if (collection && rule.campaign_name) {
    return {
      key: `campaign/${rule.campaign_name.toLowerCase()}`,
      name: rule.campaign_name,
      art: badge.image_url ?? null,
      contain: true,
    };
  }

  const name = rule.game_name || rule.campaign_name || badge.title;
  const game = rule.categories.find((category) => category.name === name);

  return {
    key: `game/${name.toLowerCase()}`,
    name,
    art: boxArt(gameArt(name, game?.box_art_url), BoxArtSize.ROW),
    contain: false,
  };
};

export const badgeTimelineRows = (
  badges: GlobalBadgeType[],
  now: number,
): TimelineRowType<GlobalBadgeType>[] =>
  toTimelineRows(
    badges.flatMap((badge) => {
      const rule = badge.requirement;
      const start = time(rule?.starts_at);
      const end = time(rule?.ends_at);

      if (!rule || Number.isNaN(start) || !(end > now)) {
        return [];
      }

      return [
        {
          head: badgeRow(badge, rule),
          item: {
            key: badgeKey(badge.set_id, badge.version_id),
            name: badge.title,
            image: badge.image_url,
            start,
            end,
            value: badge,
          },
        },
      ];
    }),
    (items) => t.badges_count({ count: items.length }),
  );

export enum AccountBadgeState {
  IN_PROGRESS = "IN_PROGRESS",
  CLAIMABLE = "CLAIMABLE",
  AVAILABLE = "AVAILABLE",
  SUB_ONLY = "SUB_ONLY",
  NOT_MINABLE = "NOT_MINABLE",
  WAITING = "WAITING",
  EVOLUTION_UNKNOWN = "EVOLUTION_UNKNOWN",
  EVOLUTION_LOCKED = "EVOLUTION_LOCKED",
  EVOLUTION_OFF = "EVOLUTION_OFF",
}

const NOT_MINED_STATES: ReadonlySet<AccountBadgeState> = new Set([
  AccountBadgeState.SUB_ONLY,
  AccountBadgeState.NOT_MINABLE,
  AccountBadgeState.EVOLUTION_UNKNOWN,
  AccountBadgeState.EVOLUTION_LOCKED,
  AccountBadgeState.EVOLUTION_OFF,
]);

export type AccountBadgeType = {
  key: string;
  dropId: string;
  campaignId: string;
  title: string;
  image: string | undefined;
  source: string;
  current: number;
  required: number;
  repeat: RepeatType | undefined;
  periodsDone: number;
  closed: boolean;
  badges: BadgeRefType[];
  chest: number | undefined;
  endsAt: string;
  state: AccountBadgeState;
  quest: QuestType | null;
  campaign: CampaignType | null;
  nextAt: string | undefined;
  ignored: IgnoredType | null;
  evolution: EvolutionType | undefined;
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

// ! An evolution is mined only once the badge it evolves from is named and owned, and only with evolutions switched on.
const evolutionHold = (
  evolution: EvolutionType | undefined,
  enabled: boolean,
): AccountBadgeState | null => {
  if (!evolution) {
    return null;
  }

  if (!evolution.from) {
    return AccountBadgeState.EVOLUTION_UNKNOWN;
  }

  if (!evolution.owned) {
    return AccountBadgeState.EVOLUTION_LOCKED;
  }

  return enabled ? null : AccountBadgeState.EVOLUTION_OFF;
};

const dropState = (
  drop: TimedDropType,
  now: number,
  hold: AccountBadgeState | null,
): AccountBadgeState => {
  if (drop.instance_id && !drop.claimed) {
    return AccountBadgeState.CLAIMABLE;
  }

  if (subsMissing(drop)) {
    return AccountBadgeState.SUB_ONLY;
  }

  if (hold) {
    return hold;
  }

  if (time(drop.next_period_at) > now) {
    return AccountBadgeState.WAITING;
  }

  return drop.current_minutes > 0 ? AccountBadgeState.IN_PROGRESS : AccountBadgeState.AVAILABLE;
};

const groupState = (
  group: QuestGroupType,
  now: number,
  hold: AccountBadgeState | null,
): AccountBadgeState => {
  if (group.status === QuestStatus.CLAIMABLE) {
    return AccountBadgeState.CLAIMABLE;
  }

  if (subsMissing(group) || group.required_minutes === 0) {
    return AccountBadgeState.SUB_ONLY;
  }

  if (hold) {
    return hold;
  }

  if (group.status === QuestStatus.UNKNOWN) {
    return AccountBadgeState.NOT_MINABLE;
  }

  if (time(group.next_eligible_at) > now) {
    return AccountBadgeState.WAITING;
  }

  return group.status === QuestStatus.IN_PROGRESS
    ? AccountBadgeState.IN_PROGRESS
    : AccountBadgeState.AVAILABLE;
};

const groupDone = (group: QuestGroupType): boolean =>
  group.repeat ? group.grant_count >= group.repeat.times : group.status === QuestStatus.CLAIMED;

// ! Same lanes as `Quest::lanes` in the core: watch groups are one track; subscription groups and the whole-collection badge stand apart.
export const questLanes = (quest: QuestType): QuestGroupType[] => {
  const lane = (group: QuestGroupType): boolean => group.required_subs === 0 && !group.after_all;
  const started = quest.groups.some(
    (group) => lane(group) && group.status !== QuestStatus.NOT_STARTED,
  );

  return quest.groups.filter(
    (group) => !started || !lane(group) || group.status !== QuestStatus.NOT_STARTED,
  );
};

const groupOwned = (
  account: AccountSnapshotType,
  quest: QuestType,
  group: QuestGroupType,
): boolean => {
  const owned = new Set(account.owned_badges.map((badge) => badgeKey(badge.set_id, badge.version)));
  const drop = campaignOf(account, quest.id)?.drops.find((item) => item.id === group.id);

  return (
    drop?.badge_owned === true ||
    (group.rewards.length > 0 &&
      group.rewards.every(
        (reward) =>
          reward.badges.length > 0 &&
          reward.badges.every((badge) => owned.has(badgeKey(badge.set_id, badge.version))),
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
  const evolutions = account.settings.badges.evolutions;

  for (const campaign of campaignsOf(account)) {
    if (campaign.kind === CampaignKind.QUEST || time(campaign.ends_at) <= now) {
      continue;
    }

    for (const drop of campaign.drops) {
      if (!dropGivesBadge(drop) || dropDone(drop) || drop.badge_owned) {
        continue;
      }

      const benefit = drop.benefits.find((b) => b.kind === BenefitKind.BADGE);

      items.push({
        key: `${campaign.id}/${drop.id}`,
        dropId: drop.id,
        campaignId: campaign.id,
        title: drop.name,
        image: benefit?.image_url,
        source: `${campaign.game.name} · ${campaign.name}`,
        current: drop.current_minutes,
        required: drop.required_minutes,
        repeat: drop.repeat,
        periodsDone: drop.periods_done,
        closed: drop.claimed,
        badges: benefit?.badges ?? [],
        chest: benefit?.chest_size,
        endsAt: drop.ends_at < campaign.ends_at ? drop.ends_at : campaign.ends_at,
        state: dropState(drop, now, evolutionHold(drop.evolution, evolutions)),
        quest: null,
        campaign,
        nextAt: drop.next_period_at,
        ignored: ignoredOf(account, campaign, campaign.id, drop.id, drop.name),
        evolution: drop.evolution,
      });
    }
  }

  for (const quest of account.quests) {
    if (quest.completed || time(quest.ends_at) <= now) {
      continue;
    }

    for (const group of questLanes(quest)) {
      if (groupDone(group) || groupOwned(account, quest, group)) {
        continue;
      }

      const reward = group.rewards.at(0);
      const title = group.name || reward?.name || quest.name;
      const campaign = campaignOf(account, quest.id) ?? null;
      const single = reward?.badges.length === 1 ? reward.badges[0] : undefined;

      items.push({
        key: `${quest.id}/${group.id}`,
        dropId: group.id,
        campaignId: quest.id,
        title,
        image: single?.image_url ?? reward?.image_url ?? quest.image_url,
        source: quest.name,
        current: group.current_minutes,
        required: group.required_minutes,
        repeat: group.repeat,
        periodsDone: group.grant_count,
        closed: group.status === QuestStatus.CLAIMED,
        badges: reward?.badges ?? [],
        chest: reward?.chest_size,
        endsAt: quest.ends_at,
        state: quest.sitewide
          ? groupState(group, now, evolutionHold(group.evolution, evolutions))
          : AccountBadgeState.NOT_MINABLE,
        quest,
        campaign,
        nextAt: group.next_eligible_at,
        ignored: ignoredOf(account, campaign, quest.id, group.id, title),
        evolution: group.evolution,
      });
    }
  }

  const byEnd = (a: AccountBadgeType, b: AccountBadgeType): number =>
    time(a.endsAt) - time(b.endsAt) || a.title.localeCompare(b.title);

  const notMinable = (item: AccountBadgeType): boolean => NOT_MINED_STATES.has(item.state);

  return {
    minable: items.filter((item) => !item.ignored && !notMinable(item)).sort(byEnd),
    notMined: items.filter((item) => !item.ignored && notMinable(item)).sort(byEnd),
    ignored: items.filter((item) => item.ignored).sort(byEnd),
  };
};

// ! Rows follow the account's mining order; the ones it does not hold yet (waiting for a window, badges lane off) come after, by end date.
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
    const totals = periodTotals({
      current: item.current,
      required: item.required,
      done: item.periodsDone,
      closed: item.closed,
      repeat: item.repeat,
    });

    row.items.push(item);
    row.current += totals.current;
    row.required += totals.required;
    rows.set(item.campaignId, row);
  }

  const rank = (row: BadgeRowType): number => {
    const at = order.indexOf(row.campaignId);

    return at === -1 ? Number.MAX_SAFE_INTEGER : at;
  };

  return [...rows.values()].sort((a, b) => rank(a) - rank(b));
};

export type OwnedBadgeType = { key: string; title: string; image: string | undefined; url: string };

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
      url: badgeUrl(owned.set_id, owned.version),
    }))
    .sort((a, b) => a.title.localeCompare(b.title));
};

export const collectionText = (rule: BadgeRequirementType): string => {
  const name = rule.campaign_name ?? "";

  if (rule.collection?.complete_all) {
    return t.badges_collection_all({ name });
  }

  return rule.collection?.chest_size
    ? t.badges_collection_chest({ name, count: rule.collection.chest_size })
    : t.badges_collection({ name });
};
