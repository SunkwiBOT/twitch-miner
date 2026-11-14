import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import { CampaignStatus } from "../bindings/CampaignStatus";
import type { CampaignType } from "../bindings/CampaignType";
import { dropTotals, subsMissing } from "../catalogue/campaign.utils";
import { campaignOf, campaignsOf } from "../catalogue/catalogue.svelte";
import { percent } from "../format.utils";

export type IgnoredType = {
  id: string;
  name: string;
  detail: string;
  drop: boolean;
  campaign: CampaignType | null;
};

export const queueProgress = (account: AccountSnapshotType, id: string): number => {
  const excluded = account.settings.drops.exclude_drop_ids;
  const totals = (campaignOf(account, id)?.drops ?? [])
    .filter((d) => !subsMissing(d) && !excluded.includes(d.id))
    .map(dropTotals);
  const required = totals.reduce((sum, d) => sum + d.required, 0);
  const done = totals.reduce((sum, d) => sum + d.current, 0);

  return percent(done, required);
};

export const ignoredCampaign = (account: AccountSnapshotType, id: string): IgnoredType => {
  const c = campaignOf(account, id);

  return { id, name: c?.name ?? id, detail: c?.game.name ?? "", drop: false, campaign: c ?? null };
};

// ! Nothing purges the ids of campaigns gone from Twitch: they are left out rather than shown as raw ids.
export const ignoredItems = (account: AccountSnapshotType): IgnoredType[] => [
  ...account.settings.drops.exclude_campaign_ids
    .filter((id) => campaignOf(account, id))
    .map((id) => ignoredCampaign(account, id)),
  ...account.settings.drops.exclude_drop_ids.flatMap((id) => {
    const c = campaignsOf(account).find((item) => item.drops.some((d) => d.id === id));

    return c
      ? [
          {
            id,
            name: c.drops.find((d) => d.id === id)?.name ?? id,
            detail: `${c.game.name} · ${c.name}`,
            drop: true,
            campaign: c,
          },
        ]
      : [];
  }),
];

export const ongoingCampaigns = (account: AccountSnapshotType, now: number): CampaignType[] =>
  campaignsOf(account).filter(
    (c) =>
      new Date(c.ends_at).getTime() > now && (c.in_inventory || c.status === CampaignStatus.ACTIVE),
  );
