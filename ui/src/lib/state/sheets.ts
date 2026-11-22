import { badgeKey } from "../badges/badges.utils";
import type { CampaignType } from "../bindings/CampaignType";
import type { GlobalBadgeType } from "../bindings/GlobalBadgeType";
import { app } from "./app.svelte";

// ! One sheet at a time: opening one closes the other, never a second sheet on top.
export const openCampaignSheet = (campaign: CampaignType, accountId: string | null): void => {
  app.badgeSheet = null;
  app.campaignSheet = { key: campaign.id, accountId };
};

export const openBadgeSheet = (badge: GlobalBadgeType, accountId: string | null): void => {
  app.campaignSheet = null;
  app.badgeSheet = { key: badgeKey(badge.set_id, badge.version_id), accountId };
};

export const closeSheets = (): void => {
  app.campaignSheet = null;
  app.badgeSheet = null;
};
