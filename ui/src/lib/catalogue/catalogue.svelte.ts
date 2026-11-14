import { fetchCatalog } from "../api/client";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { CampaignProgressType } from "../bindings/CampaignProgressType";
import type { CampaignType } from "../bindings/CampaignType";

class Catalogue {
  campaigns: CampaignType[] = $state.raw([]);
}

export const catalogue = new Catalogue();

let asked: string | null = null;

export const loadCatalog = async (revision: string | undefined): Promise<void> => {
  const key = revision ?? "";

  if (asked === key) {
    return;
  }

  asked = key;

  try {
    const { campaigns } = await fetchCatalog();

    if (asked === key) {
      catalogue.campaigns = campaigns;
    }
  } catch (e: any) {
    if (asked === key) {
      asked = null;
    }
  }
};

const withProgress = (base: CampaignType, progress: CampaignProgressType): CampaignType => {
  const states = new Map(progress.drops.map((state) => [state.id, state]));

  return {
    ...base,
    linked: progress.linked ?? base.linked,
    link_url: progress.link_url ?? base.link_url,
    in_inventory: progress.in_inventory,
    drops: base.drops.map((drop) => {
      const state = states.get(drop.id);

      return state
        ? {
            ...drop,
            current_minutes: state.current_minutes,
            instance_id: state.instance_id ?? drop.instance_id,
            claimed: state.claimed,
            preconditions_met: state.preconditions_met ?? drop.preconditions_met,
            badge_owned: state.badge_owned,
            current_subs: state.current_subs,
            periods_done: state.periods_done,
            next_period_at: state.next_period_at,
          }
        : drop;
    }),
  };
};

type CombinedType = {
  catalog: CampaignType[];
  progress: CampaignProgressType[];
  others: CampaignType[];
  campaigns: CampaignType[];
  byId: Map<string, CampaignType>;
};

const byIds = new WeakMap<CampaignType[], Map<string, CampaignType>>();
const combined = new WeakMap<AccountSnapshotType, CombinedType>();

const byIdOf = (catalog: CampaignType[]): Map<string, CampaignType> => {
  let found = byIds.get(catalog);

  if (!found) {
    found = new Map(catalog.map((campaign) => [campaign.id, campaign]));
    byIds.set(catalog, found);
  }

  return found;
};

export const catalogueCampaign = (id: string): CampaignType | undefined =>
  byIdOf(catalogue.campaigns).get(id);

const combinedOf = (account: AccountSnapshotType): CombinedType => {
  const catalog = catalogue.campaigns;
  const progress = account.progress;
  const others = account.other_campaigns;
  const known = combined.get(account);

  if (known?.catalog === catalog && known.progress === progress && known.others === others) {
    return known;
  }

  const byId = byIdOf(catalog);
  const campaigns = progress.flatMap((state) => {
    const base = byId.get(state.id);

    return base ? [withProgress(base, state)] : [];
  });

  campaigns.push(...others);
  campaigns.sort(
    (a, b) => Date.parse(a.ends_at) - Date.parse(b.ends_at) || a.id.localeCompare(b.id),
  );

  const entry: CombinedType = {
    catalog,
    progress,
    others,
    campaigns,
    byId: new Map(campaigns.map((campaign) => [campaign.id, campaign])),
  };

  combined.set(account, entry);

  return entry;
};

export const campaignsOf = (account: AccountSnapshotType): CampaignType[] =>
  combinedOf(account).campaigns;

export const campaignOf = (account: AccountSnapshotType, id: string): CampaignType | undefined =>
  combinedOf(account).byId.get(id) ?? catalogueCampaign(id);
