import { sendCommand } from "../api/client";
import { setDropIgnoredIn } from "../accounts/changes.utils";
import type { AccountSettingsType } from "../bindings/AccountSettingsType";
import { CommandName } from "../bindings/CommandName";
import type { SnapshotType } from "../bindings/SnapshotType";
import { t } from "../i18n/i18n.svelte";
import { app } from "./app.svelte";
import { refresh, reportFailure, run } from "./session";
import { SETTINGS_SAVE_DELAY_MS, SETTINGS_SETTLE_MS } from "./state.constants";

type PendingSaveType = {
  settings: AccountSettingsType;
  timer: ReturnType<typeof setTimeout> | null;
  saving: boolean;
  version: number;
  saved: number;
  waiters: ((saved: boolean) => void)[];
};

const pendingSettings = new Map<string, PendingSaveType>();

export const settingsPending = (): boolean => pendingSettings.size > 0;

// ! A saved state stays on screen a moment: a snapshot sent before the core applied it must not flip the control back.
const finishSave = (accountId: string, entry: PendingSaveType, saved: boolean): void => {
  const waiters = entry.waiters;

  entry.waiters = [];

  if (entry.timer) {
    clearTimeout(entry.timer);
    entry.timer = null;
  }

  if (saved) {
    const version = entry.version;

    entry.timer = setTimeout(() => {
      if (pendingSettings.get(accountId) === entry && entry.version === version) {
        pendingSettings.delete(accountId);
      }
    }, SETTINGS_SETTLE_MS);
  } else {
    pendingSettings.delete(accountId);
  }

  for (const resolve of waiters) {
    resolve(saved);
  }
};

const flushSettings = async (accountId: string): Promise<void> => {
  const entry = pendingSettings.get(accountId);

  if (!entry || entry.saving) {
    return;
  }

  entry.saving = true;

  const sent = entry.version;

  try {
    const settings = { ...entry.settings };
    delete settings.paused_until;

    await sendCommand({
      command: CommandName.SET_ACCOUNT_SETTINGS,
      account_id: accountId,
      settings,
    });
    entry.saving = false;
    entry.saved = sent;

    if (entry.version !== sent) {
      if (entry.timer) {
        clearTimeout(entry.timer);
        entry.timer = null;
      }

      await flushSettings(accountId);
    } else {
      finishSave(accountId, entry, true);
    }
  } catch (e: any) {
    finishSave(accountId, entry, false);

    if (reportFailure(e)) {
      await refresh();
    }
  }
};

export const forgetSavedSettings = (accountId: string): boolean => {
  const entry = pendingSettings.get(accountId);

  if (!entry || entry.saving || entry.version !== entry.saved) {
    return false;
  }

  clearTimeout(entry.timer ?? undefined);

  pendingSettings.delete(accountId);

  return true;
};

export const updateAccountSettings = (
  accountId: string,
  mutate: (settings: AccountSettingsType) => void,
): Promise<boolean> => {
  const target = app.snapshot?.accounts.find((a) => a.id === accountId);

  if (!target) {
    return Promise.resolve(false);
  }

  const settings = $state.snapshot(target.settings);

  mutate(settings);
  target.settings = settings;

  const entry = pendingSettings.get(accountId) ?? {
    settings,
    timer: null,
    saving: false,
    version: 0,
    saved: 0,
    waiters: [],
  };

  entry.settings = settings;
  entry.version += 1;
  pendingSettings.set(accountId, entry);

  clearTimeout(entry.timer ?? undefined);

  entry.timer = setTimeout(() => {
    entry.timer = null;
    void flushSettings(accountId);
  }, SETTINGS_SAVE_DELAY_MS);

  return new Promise((resolve) => entry.waiters.push(resolve));
};

export const settleSettings = async (accountId: string): Promise<void> => {
  const entry = pendingSettings.get(accountId);

  if (!entry || (entry.version === entry.saved && !entry.saving)) {
    return;
  }

  if (entry.timer) {
    clearTimeout(entry.timer);
    entry.timer = null;
  }

  const saved = new Promise((resolve) => entry.waiters.push(resolve));

  void flushSettings(accountId);
  await saved;
};

export const keepPendingSettings = (snapshot: SnapshotType): void => {
  for (const item of snapshot.accounts) {
    const entry = pendingSettings.get(item.id);

    if (entry) {
      // ! `paused_until` is server-owned (see `set_account_settings`): kept
      // from the snapshot, never from the locally edited draft.
      item.settings = { ...entry.settings, paused_until: item.settings.paused_until };
    }
  }
};

export const saveAccountSettings = (
  id: string,
  settings: AccountSettingsType,
  success: string | undefined = t.common_saved(),
): Promise<boolean> => {
  const rest = { ...settings };
  delete rest.paused_until;

  return run(
    { command: CommandName.SET_ACCOUNT_SETTINGS, account_id: id, settings: rest },
    success,
  );
};

export const setDropIgnored = (
  accountIds: string[],
  dropId: string,
  ignored: boolean,
): Promise<boolean[]> =>
  Promise.all(
    accountIds.map((accountId) =>
      updateAccountSettings(accountId, (settings) => setDropIgnoredIn(settings, dropId, ignored)),
    ),
  );
