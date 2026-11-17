import { LOCK_WARNING_LEAD_MS } from "./state.constants";

export const warningLeadMs = (idleMs: number | null): number =>
  Math.min(LOCK_WARNING_LEAD_MS, (idleMs ?? 0) / 2);
