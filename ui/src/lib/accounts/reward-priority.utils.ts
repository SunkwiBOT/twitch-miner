import { BenefitKind } from "../bindings/BenefitKind";
import { t } from "../i18n/i18n.svelte";

export const REWARD_KIND_LABELS: Record<BenefitKind, () => string> = {
  [BenefitKind.BADGE]: () => t.reward_kind_badge(),
  [BenefitKind.EMOTE]: () => t.reward_kind_emote(),
  [BenefitKind.IN_GAME_ITEM]: () => t.reward_kind_item(),
  [BenefitKind.OTHER]: () => t.reward_kind_other(),
};

export const REWARD_KIND_HINTS: Record<BenefitKind, () => string> = {
  [BenefitKind.BADGE]: () => t.reward_kind_badge_hint(),
  [BenefitKind.EMOTE]: () => t.reward_kind_emote_hint(),
  [BenefitKind.IN_GAME_ITEM]: () => t.reward_kind_item_hint(),
  [BenefitKind.OTHER]: () => t.reward_kind_other_hint(),
};
