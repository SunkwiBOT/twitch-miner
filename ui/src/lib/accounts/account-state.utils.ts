import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import { AccountStatus } from "../bindings/AccountStatus";
import { AccountState, PauseState } from "./account-state.constants";

type PauseSettingsType = Pick<AccountSettingsType, "enabled" | "paused_until">;

export const scheduledPause = (settings: PauseSettingsType, now: number): boolean =>
  settings.paused_until !== undefined && Date.parse(settings.paused_until) > now;

export const userPaused = (settings: PauseSettingsType, now: number): boolean =>
  !settings.enabled || scheduledPause(settings, now);

export const pauseState = (settings: PauseSettingsType[], now: number): PauseState => {
  const paused = settings.filter((item) => userPaused(item, now)).length;

  if (paused === 0) {
    return PauseState.RUNNING;
  }

  return paused === settings.length ? PauseState.PAUSED : PauseState.MIXED;
};

export const accountState = (
  account: Pick<AccountSnapshotType, "status" | "problem" | "settings">,
  now: number,
): AccountState => {
  if (account.status === AccountStatus.LOGIN_REQUIRED || account.problem) {
    return AccountState.PROBLEM;
  }

  if (account.status === AccountStatus.PAUSED || userPaused(account.settings, now)) {
    return AccountState.PAUSED;
  }

  return AccountState.RUNNING;
};
