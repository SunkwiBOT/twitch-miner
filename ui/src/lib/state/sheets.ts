import type { CampaignType } from "../bindings/CampaignType";
import type { GlobalBadgeType } from "../bindings/GlobalBadgeType";
import { app } from "./app.svelte";

// ! One sheet at a time: opening one closes the other, never a second sheet on top.
export const openCampaignSheet = (item: CampaignType, accountId: string | null): void => {
  app.badgeSheet = null;
  app.campaignSheet = { item, accountId };
};

export const openBadgeSheet = (item: GlobalBadgeType, accountId: string | null): void => {
  app.campaignSheet = null;
  app.badgeSheet = { item, accountId };
};

export const closeSheets = (): void => {
  app.campaignSheet = null;
  app.badgeSheet = null;
};
