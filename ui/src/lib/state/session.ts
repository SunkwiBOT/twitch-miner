import {
  fetchAppInfo,
  fetchAuthStatus,
  fetchSnapshot,
  fetchStartupProblem,
  onCloseRequested,
  requestCatalogRefresh,
  sendCommand,
  subscribe,
} from "../api/client";
import { isTauri } from "../api/tauri";
import type { UnsubscribeType } from "../api/tauri";
import type { CommandType } from "../bindings/CommandType";
import { ErrorApiMessage } from "../bindings/ErrorApiMessage";
import { EventKind } from "../bindings/EventKind";
import type { SnapshotType } from "../bindings/SnapshotType";
import { describeError, toApiError } from "../errors/errors.utils";
import { eventText } from "../events/event-text.utils";
import { number } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { noteBurstProgress } from "../spam/spam.svelte";
import { strong } from "../text.utils";
import {
  forgetSavedSettings,
  keepPendingSettings,
  settingsPending,
  settleSettings,
} from "./account-settings.svelte";
import { app } from "./app.svelte";
import { overlay } from "./overlay.svelte";
import { keepPendingPreferences, preferencesPending, showPreferences } from "./preferences.svelte";
import { followSnapshotAccounts, startRouting } from "./router";
import { LIVE_EVENTS_MAX, RELOAD_WAIT_MS, ToastKind } from "./state.constants";
import { accountImage, dismissToasts, toast } from "./toasts";

export const isDashboardLocked = (): boolean => !isTauri() && !app.auth.authorized;

export const isVaultLocked = (): boolean => app.snapshot?.vault.unlocked === false;

export const isLocked = (): boolean => isDashboardLocked() || isVaultLocked();

let pageBuild: string | null = null;
let reloadWait: ReturnType<typeof setInterval> | undefined;
let unsubscribe: UnsubscribeType | null = null;

const busy = (): boolean =>
  overlay.unsaved ||
  settingsPending() ||
  preferencesPending() ||
  document.querySelector('[role="dialog"]') !== null;

// ! A dashboard tab open through an update keeps the old interface: it reloads once nothing is edited or saved, a toast offers it meanwhile.
const followServerBuild = (snapshot: SnapshotType): void => {
  const build = snapshot.build ?? snapshot.version;

  if (isTauri() || build === pageBuild) {
    return;
  }

  if (pageBuild === null) {
    pageBuild = build;

    return;
  }

  pageBuild = build;

  if (!busy()) {
    location.reload();

    return;
  }

  toast(ToastKind.INFO, t.dashboard_updated({ version: snapshot.version }), {
    key: "dashboard-updated",
    action: { label: t.dashboard_reload(), run: () => location.reload() },
  });

  clearInterval(reloadWait);
  reloadWait = setInterval(() => {
    if (!busy()) {
      location.reload();
    }
  }, RELOAD_WAIT_MS);
};

const applySnapshot = (snapshot: SnapshotType): void => {
  followServerBuild(snapshot);
  keepPendingPreferences(snapshot);
  keepPendingSettings(snapshot);
  app.snapshot = snapshot;
  showPreferences(snapshot.settings);
  followSnapshotAccounts(snapshot);
};

export const refresh = async (): Promise<void> => {
  try {
    applySnapshot(await fetchSnapshot());
  } catch (e: any) {
    reportFailure(e);
  }
};

// ! The toasts go too: a toast may name an account or reopen a dialog.
export const sessionEnded = (): void => {
  app.auth = { ...app.auth, authorized: false, checked: true };
  unsubscribe?.();
  unsubscribe = null;
  app.snapshot = null;
  app.liveEvents = [];
  app.logsAccount = null;
  app.loginDialog = false;
  app.feedbackDialog = false;
  app.navDrawer = false;
  dismissToasts();
};

export const sessionEndedBy = (e: unknown): boolean => {
  if (toApiError(e).code !== ErrorApiMessage.UNAUTHORIZED) {
    return false;
  }

  sessionEnded();

  return true;
};

// ! False when a 401 ended the session instead: nothing is left to refresh.
export const reportFailure = (e: unknown): boolean => {
  if (sessionEndedBy(e)) {
    return false;
  }

  toast(ToastKind.ERROR, describeError(e));

  return true;
};

// ! An account command waits for the pending save, then drops the copy kept: some (Mine first) change the settings.
export const run = async (command: CommandType, success?: string): Promise<boolean> => {
  try {
    if ("account_id" in command) {
      await settleSettings(command.account_id);
    }

    await sendCommand(command);

    if ("account_id" in command && forgetSavedSettings(command.account_id)) {
      await refresh();
    }

    if (success) {
      toast(ToastKind.SUCCESS, success);
    }

    return true;
  } catch (e: any) {
    reportFailure(e);

    return false;
  }
};

export const refreshCatalog = async (): Promise<void> => {
  try {
    const refreshed = await requestCatalogRefresh();

    const count = refreshed.campaign_count;

    toast(ToastKind.SUCCESS, t.catalogue_refreshed({ count, shown: strong(number(count)) }));
  } catch (e: any) {
    if (!reportFailure(e)) {
      return;
    }
  }

  await refresh();
};

export const checkSession = async (): Promise<boolean> => {
  try {
    const status = await fetchAuthStatus();

    if (!status.authorized) {
      sessionEnded();
    }

    return true;
  } catch (e: any) {
    return false;
  }
};

export const connect = async (): Promise<void> => {
  await refresh();
  unsubscribe?.();
  unsubscribe = await subscribe({
    onSnapshot: applySnapshot,
    onEvent: (event) => {
      app.liveEvents = [event, ...app.liveEvents].slice(0, LIVE_EVENTS_MAX);

      const known = accountImage(event.account_id, event.account_login);
      const image = {
        url: event.context.account_avatar ?? known.url,
        name: event.context.account_name ?? known.name,
        channel: null,
      };
      const title = image.name;

      if (event.kind === EventKind.DROP_CLAIMED) {
        toast(ToastKind.SUCCESS, eventText(event, false).text, { image, title });
      }

      if (event.kind === EventKind.LOGIN_COMPLETED) {
        toast(ToastKind.SUCCESS, eventText(event, false).text, { image });
      }

      if (event.kind === EventKind.SESSION_EXPIRED) {
        toast(ToastKind.ERROR, eventText(event, false).text, { image, title });
      }

      if (event.kind === EventKind.CHAT_MENTION) {
        toast(ToastKind.INFO, eventText(event, false).text, { image, title });
      }
    },
    onBurst: noteBurstProgress,
    onConnection: (connected) => {
      app.connected = connected;

      if (connected) {
        void refresh();
      } else {
        void checkSession();
      }
    },
  });
};

export const start = async (): Promise<void> => {
  app.startupProblem = await fetchStartupProblem();

  if (app.startupProblem) {
    return;
  }

  startRouting();

  try {
    app.info = await fetchAppInfo();
    await onCloseRequested(() => {
      app.closeDialog = true;
    });
  } catch (e: any) {
    console.warn("desktop details unavailable", e);
  }

  try {
    const status = await fetchAuthStatus();

    app.auth = { ...status, checked: true };
  } catch (e: any) {
    app.auth = { ...app.auth, checked: true };
    toast(ToastKind.ERROR, describeError(e));
  }

  if (app.auth.authorized) {
    await connect();
  }
};
