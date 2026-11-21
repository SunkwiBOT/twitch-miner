import { PauseScope } from "../accounts/account-state.constants";
import { scheduledPause } from "../accounts/account-state.utils";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import { CommandName } from "../bindings/CommandName";
import { updateAccountSettings } from "./account-settings.svelte";
import { run } from "./session";

const setEnabled = (accounts: AccountSnapshotType[], enabled: boolean): Promise<boolean>[] =>
  accounts.map((account) =>
    updateAccountSettings(account.id, (settings) => (settings.enabled = enabled)),
  );

const schedule = (
  scope: PauseScope,
  accounts: AccountSnapshotType[],
  until?: string,
): Promise<boolean>[] =>
  scope === PauseScope.ALL
    ? [run({ command: CommandName.PAUSE_ALL_ACCOUNTS, until })]
    : accounts.map((account) =>
        run({ command: CommandName.PAUSE_ACCOUNT, account_id: account.id, until }),
      );

// ! "Until I resume" is the account's `enabled` switch, a timed pause its server-owned
// `paused_until`: resuming lifts whichever of the two holds each account.
export const pauseAccounts = async (
  scope: PauseScope,
  accounts: AccountSnapshotType[],
  until: string | null,
): Promise<boolean> => {
  const done = await Promise.all(
    until === null ? setEnabled(accounts, false) : schedule(scope, accounts, until),
  );

  return done.every(Boolean);
};

export const resumeAccounts = async (
  scope: PauseScope,
  accounts: AccountSnapshotType[],
  now: number,
): Promise<boolean> => {
  const scheduled = accounts.filter((account) => scheduledPause(account.settings, now));
  const done = await Promise.all([
    ...setEnabled(
      accounts.filter((account) => !account.settings.enabled),
      true,
    ),
    ...(scheduled.length > 0 ? schedule(scope, scheduled) : []),
  ]);

  return done.every(Boolean);
};
