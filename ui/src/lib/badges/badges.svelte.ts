import { fetchBadges } from "../api/client";
import type { GlobalBadgeType } from "../bindings/GlobalBadgeType";
import { HIDDEN_BADGE_SETS } from "./badges.constants";

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
    badgeList.badges = (await fetchBadges()).badges.filter(
      (badge) => !HIDDEN_BADGE_SETS.has(badge.set_id),
    );
    badgeList.loaded = true;
  } catch (e: any) {
    asked = null;
  }
};
