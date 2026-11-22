import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { CampaignType } from "../bindings/CampaignType";
import type { DropsSettingsType } from "../bindings/DropsSettingsType";
import type { GameEntryType } from "../bindings/GameEntryType";
import { campaignOf } from "../catalogue/catalogue.svelte";
import { gameEntryKey, gameKeysLike, sameGame } from "../catalogue/games.utils";
import { accountChange, campaignChange, changedWhenSaved } from "../state/bulk.svelte";
import { updateAccountSettings } from "../state/account-settings.svelte";
import { ChangeKind } from "../state/state.constants";
import type { BadgesList, DropsGameList, DropsList } from "./accounts.constants";
import {
  applyDelta,
  deltaOf,
  miningOrder,
  moveLike,
  pinnedPrefix,
  placeOf,
  setCampaignIgnoredIn,
  setCampaignSelectedIn,
  setDropIgnoredIn,
} from "./changes.utils";
import type { IgnoredType } from "./drops.utils";

const saveDrops = (
  account: AccountSnapshotType,
  mutate: (drops: DropsSettingsType) => void,
): Promise<boolean> => updateAccountSettings(account.id, (settings) => mutate(settings.drops));

export const editDropsList = (
  account: AccountSnapshotType,
  field: DropsList,
  next: string[],
): void => {
  const delta = deltaOf(account.settings.drops[field], next);

  void changedWhenSaved(
    saveDrops(account, (drops) => (drops[field] = next)),
    account.id,
    accountChange(ChangeKind.SETTINGS),
    (settings) => {
      settings.drops[field] = applyDelta(settings.drops[field], delta);
    },
  );
};

type GameListType = {
  read: (settings: AccountSettingsType) => GameEntryType[];
  write: (settings: AccountSettingsType, list: GameEntryType[]) => void;
};

const editGames = (
  account: AccountSnapshotType,
  list: GameListType,
  next: GameEntryType[],
): void => {
  const before = list.read(account.settings);
  const added = next.filter((entry) => !before.some((other) => sameGame(other, entry)));
  const removed = before.filter((entry) => !next.some((other) => sameGame(other, entry)));

  void changedWhenSaved(
    updateAccountSettings(account.id, (settings) => list.write(settings, next)),
    account.id,
    accountChange(ChangeKind.SETTINGS),
    (settings) => {
      const current = list.read(settings);

      list.write(settings, [
        ...current.filter((entry) => !removed.some((other) => sameGame(other, entry))),
        ...added.filter((entry) => !current.some((other) => sameGame(other, entry))),
      ]);
    },
  );
};

export const editDropsGames = (
  account: AccountSnapshotType,
  field: DropsGameList,
  next: GameEntryType[],
): void =>
  editGames(
    account,
    {
      read: (settings) => settings.drops[field],
      write: (settings, list) => (settings.drops[field] = list),
    },
    next,
  );

export const editBadgesExcludedGames = (
  account: AccountSnapshotType,
  next: GameEntryType[],
): void =>
  editGames(
    account,
    {
      read: (settings) => settings.badges.exclude_games,
      write: (settings, list) => (settings.badges.exclude_games = list),
    },
    next,
  );

export const editBadgesList = (
  account: AccountSnapshotType,
  field: BadgesList,
  next: string[],
): void => {
  const delta = deltaOf(account.settings.badges[field], next);

  void changedWhenSaved(
    updateAccountSettings(account.id, (settings) => (settings.badges[field] = next)),
    account.id,
    accountChange(ChangeKind.SETTINGS),
    (settings) => {
      settings.badges[field] = applyDelta(settings.badges[field], delta);
    },
  );
};

export const setSelected = (
  account: AccountSnapshotType,
  campaignId: string,
  name: string,
  selected: boolean,
): void => {
  const apply = (settings: AccountSettingsType): void =>
    setCampaignSelectedIn(settings, campaignId, selected);

  void changedWhenSaved(
    updateAccountSettings(account.id, apply),
    account.id,
    campaignChange(
      selected ? ChangeKind.SELECTED : ChangeKind.UNSELECTED,
      name,
      campaignOf(account, campaignId),
    ),
    apply,
  );
};

export const movePriorityGame = (
  account: AccountSnapshotType,
  list: GameEntryType[],
  moved: GameEntryType,
): void => {
  const place = placeOf(list.map(gameEntryKey), gameEntryKey(moved));

  void changedWhenSaved(
    saveDrops(account, (drops) => (drops.priority_games = list)),
    account.id,
    accountChange(ChangeKind.SETTINGS),
    (settings) => {
      const current = settings.drops.priority_games;
      const keys = gameKeysLike(current, list);
      const next = moveLike(keys, gameEntryKey(moved), place);

      if (next) {
        const byKey = new Map(current.map((entry, index) => [keys[index], entry]));

        settings.drops.priority_games = next.flatMap((key) => byKey.get(key) ?? []);
      }
    },
  );
};

const reorderCampaigns = (account: AccountSnapshotType, order: string[], id: string): void => {
  const place = placeOf(order, id);

  const campaign = campaignOf(account, id);

  void changedWhenSaved(
    saveDrops(account, (drops) => {
      drops.campaign_order = pinnedPrefix(order, [...drops.campaign_order, id]);
    }),
    account.id,
    campaignChange(ChangeKind.CAMPAIGN_MOVED, campaign?.name ?? id, campaign),
    (settings, target) => {
      const next = moveLike(miningOrder(settings, target), id, place);

      if (next) {
        settings.drops.campaign_order = pinnedPrefix(next, [...settings.drops.campaign_order, id]);
      }
    },
  );
};

// ! A move inside a part of the order (Drops queue, badges) lands in the whole order beside the same neighbour, keeping campaigns pinned outside that part.
export const reorderWithin = (account: AccountSnapshotType, part: string[], id: string): void => {
  const whole = miningOrder(account.settings, account);
  const rest = [...whole, ...part.filter((other) => !whole.includes(other))].filter(
    (other) => other !== id,
  );
  const at = part.indexOf(id);
  const index = at + 1 < part.length ? rest.indexOf(part[at + 1]) : rest.indexOf(part[at - 1]) + 1;

  rest.splice(index, 0, id);
  reorderCampaigns(account, rest, id);
};

export const setBadgesIgnored = (
  account: AccountSnapshotType,
  campaign: CampaignType | null,
  dropIds: string[],
  name: string,
): void => {
  const apply = (settings: AccountSettingsType): void => {
    for (const id of dropIds) {
      setDropIgnoredIn(settings, id, true);
    }
  };

  void changedWhenSaved(
    updateAccountSettings(account.id, apply),
    account.id,
    campaignChange(ChangeKind.IGNORED, name, campaign),
    apply,
  );
};

export const resetQueue = (account: AccountSnapshotType): void => {
  void changedWhenSaved(
    saveDrops(account, (drops) => (drops.campaign_order = [])),
    account.id,
    accountChange(ChangeKind.ORDER_RESET),
    (settings) => (settings.drops.campaign_order = []),
  );
};

export const setIgnored = (
  account: AccountSnapshotType,
  item: IgnoredType,
  ignored: boolean,
): void => {
  const apply = (settings: AccountSettingsType): void =>
    item.drop
      ? setDropIgnoredIn(settings, item.id, ignored)
      : setCampaignIgnoredIn(settings, item.id, ignored);

  void changedWhenSaved(
    updateAccountSettings(account.id, apply),
    account.id,
    campaignChange(ignored ? ChangeKind.IGNORED : ChangeKind.UNIGNORED, item.name, item.campaign),
    apply,
  );
};
