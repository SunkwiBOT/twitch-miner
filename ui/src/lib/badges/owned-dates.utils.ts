import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { BadgeRefType } from "../bindings/BadgeRefType";
import { BenefitKind } from "../bindings/BenefitKind";
import type { GlobalBadgeType } from "../bindings/GlobalBadgeType";
import type { MinerEventType } from "../bindings/MinerEventType";
import { campaignOf } from "../catalogue/catalogue.svelte";
import { badgeKey } from "./badges.utils";

const refKeys = (badges: readonly BadgeRefType[]): string[] =>
  badges.map((badge) => badgeKey(badge.set_id, badge.version));

// ! What a claimed reward could have given: its drop's badges, a quest group's, or, once the
// campaign is gone, the badges the catalogue names for it.
const claimedBadges = (
  account: AccountSnapshotType,
  catalogue: readonly GlobalBadgeType[],
  campaignId: string,
  dropId: string | undefined,
): string[] => {
  const drop = campaignOf(account, campaignId)?.drops.find((item) => item.id === dropId);

  if (drop) {
    return refKeys(
      drop.benefits
        .filter((benefit) => benefit.kind === BenefitKind.BADGE)
        .flatMap((benefit) => benefit.badges),
    );
  }

  const group = account.quests
    .find((quest) => quest.id === campaignId)
    ?.groups.find((item) => item.id === dropId);

  if (group) {
    return refKeys(group.rewards.flatMap((reward) => reward.badges));
  }

  return catalogue
    .filter((badge) => badge.requirement?.campaign_id === campaignId)
    .map((badge) => badgeKey(badge.set_id, badge.version_id));
};

// ! A claim dates a badge only when exactly one owned badge can come from it: a chest
// with several owned badges, or a badge earned elsewhere, stays undated.
export const unlockDates = (
  account: AccountSnapshotType,
  claims: readonly MinerEventType[],
  catalogue: readonly GlobalBadgeType[],
): Map<string, string> => {
  const owned = new Set(refKeys(account.owned_badges));
  const dates = new Map<string, string>();

  for (const claim of claims) {
    const campaignId = claim.context.campaign_id;

    if (!campaignId) {
      continue;
    }

    const candidates = claimedBadges(account, catalogue, campaignId, claim.context.drop_id).filter(
      (key) => owned.has(key),
    );
    const only = new Set(candidates);

    if (only.size === 1) {
      const [key] = only;
      const known = dates.get(key);

      if (!known || Date.parse(claim.at) > Date.parse(known)) {
        dates.set(key, claim.at);
      }
    }
  }

  return dates;
};
