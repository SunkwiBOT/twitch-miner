import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";

export enum ProgressState {
  CLAIMABLE = "CLAIMABLE",
  MINING = "MINING",
  SET_ASIDE = "SET_ASIDE",
  IN_PROGRESS = "IN_PROGRESS",
  NOT_STARTED = "NOT_STARTED",
  SUB_ONLY = "SUB_ONLY",
  NOT_MINABLE = "NOT_MINABLE",
  IGNORED = "IGNORED",
  CLAIMED = "CLAIMED",
  OWNED = "OWNED",
  NOT_OFFERED = "NOT_OFFERED",
}

export const NO_BAR_STATES: ReadonlySet<ProgressState> = new Set([
  ProgressState.SUB_ONLY,
  ProgressState.CLAIMED,
  ProgressState.OWNED,
  ProgressState.NOT_OFFERED,
]);

export type AccountProgressType = {
  account: AccountSnapshotType;
  state: ProgressState;
  current: number;
  required: number;
  minable: boolean;
  setAsideUntil: string | null;
};
