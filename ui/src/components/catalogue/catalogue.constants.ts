import { DAY_MS, HOUR_MS } from "../../lib/time/time.constants";
import { BadgeTone } from "../../lib/tones.constants";

export enum CatalogueSort {
  ENDING = "ENDING",
  NEW = "NEW",
  GAME = "GAME",
}

export const CATALOGUE_SORT_SLUGS: Record<CatalogueSort, string> = {
  [CatalogueSort.ENDING]: "ending",
  [CatalogueSort.NEW]: "new",
  [CatalogueSort.GAME]: "category",
};

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
