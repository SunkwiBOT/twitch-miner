import { REWARD_KIND_LABELS } from "../../lib/accounts/reward-priority.utils";
import { BenefitKind } from "../../lib/bindings/BenefitKind";

export const BENEFIT_KIND_LABELS: Record<BenefitKind, (() => string) | null> = {
  ...REWARD_KIND_LABELS,
  [BenefitKind.OTHER]: null,
};
