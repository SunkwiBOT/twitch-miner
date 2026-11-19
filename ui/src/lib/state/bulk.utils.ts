import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import { sameValue } from "../compare.utils";

export type AccountChangeType = (
  settings: AccountSettingsType,
  target: AccountSnapshotType,
) => void;

export const wouldChange = (target: AccountSnapshotType, changes: AccountChangeType[]): boolean => {
  const before = target.settings;
  const after: AccountSettingsType = JSON.parse(JSON.stringify(before));

  for (const change of changes) {
    change(after, target);
  }

  return !sameValue(before, after);
};
