import { DAY_MS, HOUR_MS } from "../../lib/time/time.constants";
import { BadgeTone } from "../../lib/tones.constants";

export enum CatalogueSort {
  ENDING = "ENDING",
  NEW = "NEW",
  GAME = "GAME",
}

export const URGENT_MS = 3 * HOUR_MS;
export const URGENT_TONE = BadgeTone.DANGER;
export const NEW_MS = DAY_MS;

export enum CampaignAction {
  IGNORE = "IGNORE",
  SELECT = "SELECT",
  FIRST = "FIRST",
}
