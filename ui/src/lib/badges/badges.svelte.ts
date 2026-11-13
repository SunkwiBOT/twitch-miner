import { fetchBadges } from "../api/client";
import type { GlobalBadgeType } from "../bindings/GlobalBadgeType";

type BadgeListType = { badges: GlobalBadgeType[]; loaded: boolean };

export const badgeList: BadgeListType = $state({ badges: [], loaded: false });

let asked: string | null = null;

export const loadBadges = async (revision: string | undefined): Promise<void> => {
  const key = revision ?? "";

  if (asked === key) {
    return;
  }

  asked = key;

  try {
    badgeList.badges = (await fetchBadges()).badges;
    badgeList.loaded = true;
  } catch (e: any) {
    asked = null;
  }
};
