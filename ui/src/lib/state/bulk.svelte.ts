import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import type { CampaignType } from "../bindings/CampaignType";
import { campaignArtUrl, categoryUrl, sitewideQuest } from "../catalogue/campaign.utils";
import { sameValue } from "../compare.utils";
import { number, peopleMarkers } from "../format.utils";
import { IconName } from "../icons.constants";
import { t } from "../i18n/i18n.svelte";
import {
  categoryMarker,
  personMarker,
  strong,
  type TextCategoryType,
  type TextPersonType,
} from "../text.utils";
import { BULK_TOAST_KEY, ChangeKind, ToastKind } from "./state.constants";
import { updateAccountSettings } from "./account-settings.svelte";
import { app } from "./app.svelte";
import { type AccountChangeType, wouldChange } from "./bulk.utils";
import { toast } from "./toasts";

type BulkStateType = { open: boolean; sourceIds: string[]; changes: AccountChangeType[] };

// ! The title says what happened, fixed; the description is one sentence naming the subject once and the account(s) it applies to.
// `people` and `categories` are what `describe` marks (personMarker(0)…, categoryMarker(0)…); the accounts come after the people.
export type ChangeToastType = {
  kind: ChangeKind;
  describe: (accounts: number) => string;
  people?: TextPersonType[];
  categories?: TextCategoryType[];
};

const CHANGE_TITLES: Record<ChangeKind, () => string> = {
  [ChangeKind.SETTINGS]: () => t.change_settings(),
  [ChangeKind.STREAMER_ADDED]: () => t.change_streamer_added(),
  [ChangeKind.STREAMER_EDITED]: () => t.change_streamer_edited(),
  [ChangeKind.STREAMER_REMOVED]: () => t.change_streamer_removed(),
  [ChangeKind.STREAMER_MOVED]: () => t.change_streamer_moved(),
  [ChangeKind.STREAMER_ENABLED]: () => t.change_streamer_enabled(),
  [ChangeKind.STREAMER_DISABLED]: () => t.change_streamer_disabled(),
  [ChangeKind.STREAMERS_EDITED]: () => t.change_streamers_edited(),
  [ChangeKind.STREAMERS_IMPORTED]: () => t.change_streamers_imported(),
  [ChangeKind.STREAMER_DEFAULTS]: () => t.change_streamer_defaults(),
  [ChangeKind.COPIED_DROPS]: () => t.change_copied_drops(),
  [ChangeKind.COPIED_POINTS]: () => t.change_copied_points(),
  [ChangeKind.CAMPAIGN_MOVED]: () => t.change_campaign_moved(),
  [ChangeKind.ORDER_RESET]: () => t.change_order_reset(),
  [ChangeKind.IGNORED]: () => t.change_ignored(),
  [ChangeKind.UNIGNORED]: () => t.change_unignored(),
  [ChangeKind.SELECTED]: () => t.change_selected(),
  [ChangeKind.UNSELECTED]: () => t.change_unselected(),
  [ChangeKind.PRIORITIZED]: () => t.change_prioritized(),
};

type DetailType = (accounts: number) => string;

export type CampaignChangeKind =
  | ChangeKind.CAMPAIGN_MOVED
  | ChangeKind.IGNORED
  | ChangeKind.UNIGNORED
  | ChangeKind.SELECTED
  | ChangeKind.UNSELECTED
  | ChangeKind.PRIORITIZED;

const CAMPAIGN_DETAILS: Record<CampaignChangeKind, DetailType> = {
  [ChangeKind.CAMPAIGN_MOVED]: (accounts) =>
    t.change_campaign_moved_detail({
      campaign: categoryMarker(0),
      account: peopleMarkers(0, accounts),
    }),
  [ChangeKind.IGNORED]: (accounts) =>
    t.change_ignored_detail({ campaign: categoryMarker(0), account: peopleMarkers(0, accounts) }),
  [ChangeKind.UNIGNORED]: (accounts) =>
    t.change_unignored_detail({ campaign: categoryMarker(0), account: peopleMarkers(0, accounts) }),
  [ChangeKind.SELECTED]: (accounts) =>
    t.change_selected_detail({ campaign: categoryMarker(0), account: peopleMarkers(0, accounts) }),
  [ChangeKind.UNSELECTED]: (accounts) =>
    t.change_unselected_detail({
      campaign: categoryMarker(0),
      account: peopleMarkers(0, accounts),
    }),
  [ChangeKind.PRIORITIZED]: (accounts) =>
    t.change_prioritized_detail({
      campaign: categoryMarker(0),
      account: peopleMarkers(0, accounts),
    }),
};

type StreamerChangeKind =
  | ChangeKind.STREAMER_ADDED
  | ChangeKind.STREAMER_EDITED
  | ChangeKind.STREAMER_REMOVED
  | ChangeKind.STREAMER_ENABLED
  | ChangeKind.STREAMER_DISABLED;

const STREAMER_DETAILS: Record<StreamerChangeKind, DetailType> = {
  [ChangeKind.STREAMER_ADDED]: (accounts) =>
    t.change_streamer_added_detail({
      streamer: personMarker(0),
      account: peopleMarkers(1, accounts),
    }),
  [ChangeKind.STREAMER_EDITED]: (accounts) =>
    t.change_streamer_edited_detail({
      streamer: personMarker(0),
      account: peopleMarkers(1, accounts),
    }),
  [ChangeKind.STREAMER_REMOVED]: (accounts) =>
    t.change_streamer_removed_detail({
      streamer: personMarker(0),
      account: peopleMarkers(1, accounts),
    }),
  [ChangeKind.STREAMER_ENABLED]: (accounts) =>
    t.change_streamer_enabled_detail({
      streamer: personMarker(0),
      account: peopleMarkers(1, accounts),
    }),
  [ChangeKind.STREAMER_DISABLED]: (accounts) =>
    t.change_streamer_disabled_detail({
      streamer: personMarker(0),
      account: peopleMarkers(1, accounts),
    }),
};

type AccountChangeKind =
  ChangeKind.SETTINGS | ChangeKind.STREAMER_DEFAULTS | ChangeKind.ORDER_RESET;

const ACCOUNT_DETAILS: Record<AccountChangeKind, DetailType> = {
  [ChangeKind.SETTINGS]: (accounts) =>
    t.change_settings_detail({ account: peopleMarkers(0, accounts) }),
  [ChangeKind.STREAMER_DEFAULTS]: (accounts) =>
    t.change_streamer_defaults_detail({ account: peopleMarkers(0, accounts) }),
  [ChangeKind.ORDER_RESET]: (accounts) =>
    t.change_order_reset_detail({ account: peopleMarkers(0, accounts) }),
};

export const accountChange = (kind: AccountChangeKind): ChangeToastType => ({
  kind,
  describe: ACCOUNT_DETAILS[kind],
});

// ! A campaign or badge shows its campaign's picture inline, as its rows do.
export const campaignChange = (
  kind: CampaignChangeKind,
  name: string,
  campaign: CampaignType | null | undefined,
): ChangeToastType => ({
  kind,
  describe: CAMPAIGN_DETAILS[kind],
  categories: [
    {
      name,
      art: campaign ? campaignArtUrl(campaign) : undefined,
      url:
        campaign && !sitewideQuest(campaign)
          ? categoryUrl(campaign.game.name, campaign.game.slug)
          : null,
    },
  ],
});

export const streamerChange = (
  kind: StreamerChangeKind,
  streamer: TextPersonType,
): ChangeToastType => ({ kind, describe: STREAMER_DETAILS[kind], people: [streamer] });

export const streamerMoved = (streamer: TextPersonType, index: number): ChangeToastType => ({
  kind: ChangeKind.STREAMER_MOVED,
  describe: (accounts) =>
    t.change_streamer_moved_detail({
      streamer: personMarker(0),
      position: strong(number(index + 1)),
      account: peopleMarkers(1, accounts),
    }),
  people: [streamer],
});

export const streamersCounted = (
  kind: ChangeKind.STREAMERS_EDITED | ChangeKind.STREAMERS_IMPORTED,
  count: number,
): ChangeToastType => ({
  kind,
  describe: (accounts) =>
    kind === ChangeKind.STREAMERS_EDITED
      ? t.change_streamers_edited_detail({
          count,
          streamerCount: strong(number(count)),
          account: peopleMarkers(0, accounts),
        })
      : t.change_streamers_imported_detail({
          count,
          streamerCount: strong(number(count)),
          account: peopleMarkers(0, accounts),
        }),
});

export const settingsCopied = (drops: boolean, source: TextPersonType): ChangeToastType => ({
  kind: drops ? ChangeKind.COPIED_DROPS : ChangeKind.COPIED_POINTS,
  describe: (accounts) =>
    drops
      ? t.change_copied_drops_detail({
          sourceAccount: personMarker(0),
          account: peopleMarkers(1, accounts),
        })
      : t.change_copied_points_detail({
          sourceAccount: personMarker(0),
          account: peopleMarkers(1, accounts),
        }),
  people: [source],
});

export const bulk = $state<BulkStateType>({ open: false, sourceIds: [], changes: [] });

export const closeBulk = (): void => {
  bulk.open = false;
  bulk.sourceIds = [];
  bulk.changes = [];
};

export const changedWhenSaved = async (
  saved: Promise<boolean>,
  sourceId: string,
  said: ChangeToastType,
  change: AccountChangeType,
): Promise<boolean> => {
  if (!(await saved)) {
    return false;
  }

  changed([sourceId], said, change);

  return true;
};

export const changeAccounts = async (
  accountIds: string[],
  said: ChangeToastType,
  change: (settings: AccountSettingsType) => void,
): Promise<void> => {
  const saved = await Promise.all(accountIds.map((id) => updateAccountSettings(id, change)));
  const sources = accountIds.filter((_, index) => saved[index]);

  if (sources.length > 0) {
    changed(sources, said, change);
  }
};

export const changed = (
  sourceIds: string[],
  said: ChangeToastType,
  change: AccountChangeType,
): void => {
  const people = said.people ?? [];
  const accountPeople: TextPersonType[] = (app.snapshot?.accounts ?? [])
    .filter((a) => sourceIds.includes(a.id))
    .map((a) => ({ name: a.display_name, avatar: a.avatar_url, channel: null }));
  const others = (app.snapshot?.accounts ?? []).filter((a) => !sourceIds.includes(a.id));

  if (others.length === 0) {
    toast(ToastKind.SUCCESS, CHANGE_TITLES[said.kind](), {
      description: said.describe(accountPeople.length),
      people: [...people, ...accountPeople],
      categories: said.categories,
    });

    return;
  }

  if (!sameValue(bulk.sourceIds, sourceIds)) {
    bulk.sourceIds = sourceIds;
    bulk.changes = [];
  }

  bulk.changes.push(change);

  const count = bulk.changes.length;
  const applicable = others.some((a) => wouldChange(a, bulk.changes));

  const single = count === 1;

  toast(ToastKind.SUCCESS, CHANGE_TITLES[single ? said.kind : ChangeKind.SETTINGS](), {
    key: BULK_TOAST_KEY,
    description: single
      ? said.describe(accountPeople.length)
      : t.bulk_changes({
          count,
          formattedCount: strong(number(count)),
          account: peopleMarkers(0, accountPeople.length),
        }),
    people: single ? [...people, ...accountPeople] : accountPeople,
    categories: single ? said.categories : undefined,
    action: applicable
      ? { label: t.bulk_apply_others(), icon: IconName.USERS, run: () => (bulk.open = true) }
      : undefined,
    onend: () => {
      if (!bulk.open) {
        closeBulk();
      }
    },
  });
};

export const applyToAccounts = async (accountIds: string[]): Promise<void> => {
  const changes = [...bulk.changes];
  const results = await Promise.all(
    accountIds.map((id) => {
      const target = app.snapshot?.accounts.find((a) => a.id === id);

      if (!target || !wouldChange(target, changes)) {
        return Promise.resolve(false);
      }

      return updateAccountSettings(id, (settings) => {
        for (const change of changes) {
          change(settings, target);
        }
      });
    }),
  );

  const applied = results.filter(Boolean).length;

  if (applied > 0) {
    toast(ToastKind.SUCCESS, t.bulk_applied_title(), {
      description: t.common_accounts_count({ accountCount: strong(number(applied)) }),
    });
  }
};
