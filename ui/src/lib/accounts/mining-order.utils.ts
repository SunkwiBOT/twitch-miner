import { MiningLaneKind } from "../bindings/MiningLaneKind";
import type { MiningLaneType } from "../bindings/MiningLaneType";
import { t } from "../i18n/i18n.svelte";

export const MINING_LANE_LABELS: Record<MiningLaneKind, () => string> = {
  [MiningLaneKind.PRIORITY_GAMES]: () => t.mining_lane_priority_games(),
  [MiningLaneKind.BADGES]: () => t.mining_lane_badges(),
  [MiningLaneKind.OTHER_CAMPAIGNS]: () => t.mining_lane_other_campaigns(),
};

export const MINING_LANE_HINTS: Record<MiningLaneKind, () => string> = {
  [MiningLaneKind.PRIORITY_GAMES]: () => t.mining_lane_priority_games_hint(),
  [MiningLaneKind.BADGES]: () => t.mining_lane_badges_hint(),
  [MiningLaneKind.OTHER_CAMPAIGNS]: () => t.mining_lane_other_campaigns_hint(),
};

export const sameMiningOrder = (a: MiningLaneType[], b: MiningLaneType[]): boolean =>
  a.length === b.length &&
  a.every((lane, index) => lane.kind === b[index].kind && lane.enabled === b[index].enabled);

export const miningOrderSummary = (order: MiningLaneType[]): string => {
  const on = order.filter((lane) => lane.enabled).map((lane) => MINING_LANE_LABELS[lane.kind]());

  return on.length > 0 ? on.join(", ") : t.mining_order_none();
};

export const laneEnabled = (order: MiningLaneType[], kind: MiningLaneKind): boolean =>
  order.some((lane) => lane.kind === kind && lane.enabled);
