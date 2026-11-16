import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";

export const accountOptions = (accounts: AccountSnapshotType[]) =>
  accounts.map((a) => ({ value: a.id, label: a.display_name, image: a.avatar_url }));

export const visibleAccountIds = (accounts: AccountSnapshotType[], hidden: string[]): string[] =>
  accounts.map((a) => a.id).filter((id) => !hidden.includes(id));

export const queryAccountIds = (accountIds: string[], total: number): string[] | undefined =>
  accountIds.length === total ? undefined : accountIds;
