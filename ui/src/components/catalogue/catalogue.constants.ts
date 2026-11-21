import { DAY_MS, HOUR_MS } from "../../lib/time/time.constants";
import { BadgeTone } from "../../lib/tones.constants";

export enum CampaignSort {
  QUEUE = "QUEUE",
  ENDING = "ENDING",
  NEW = "NEW",
  GAME = "GAME",
}

export const CAMPAIGN_SORT_SLUGS: Record<CampaignSort, string> = {
  [CampaignSort.QUEUE]: "queue",
  [CampaignSort.ENDING]: "ending",
  [CampaignSort.NEW]: "new",
  [CampaignSort.GAME]: "category",
};

export const CATALOGUE_SORTS = [CampaignSort.ENDING, CampaignSort.NEW, CampaignSort.GAME] as const;

export const QUEUE_SORTS = [CampaignSort.QUEUE, ...CATALOGUE_SORTS] as const;

export const URGENT_MS = 3 * HOUR_MS;
export const URGENT_TONE = BadgeTone.DANGER;
export const NEW_MS = DAY_MS;

export enum CampaignAction {
  IGNORE = "IGNORE",
  RESTORE = "RESTORE",
  SELECT = "SELECT",
  UNSELECT = "UNSELECT",
  FIRST = "FIRST",
}
