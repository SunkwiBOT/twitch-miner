import { isTauri } from "../api/tauri";
import type { MinerEventType } from "../bindings/MinerEventType";
import { EVENT_KIND_LABELS } from "../events/labels.utils";
import { t } from "../i18n/i18n.svelte";
import { app } from "../state/app.svelte";
import { recall, remember } from "../state/storage.utils";
import {
  APP_ICON_URL,
  BROWSER_PERMISSIONS,
  BrowserPermission,
  UPDATE_NOTIFIED_STORAGE_KEY,
} from "./updates.constants";

// ! Browsers only offer notifications in a secure context: a dashboard opened
// over plain HTTP on another machine of the network has none.
export const browserNotificationPermission = (): BrowserPermission | null =>
  "Notification" in window && window.isSecureContext
    ? BROWSER_PERMISSIONS[Notification.permission]
    : null;

export const askBrowserNotifications = async (): Promise<BrowserPermission | null> => {
  if (browserNotificationPermission() === BrowserPermission.DEFAULT) {
    await Notification.requestPermission();
  }

  return browserNotificationPermission();
};

// ! The desktop app shows it from the backend; in a browser, every open tab
// receives the event, and the stored version keeps it to one notification.
export const notifyUpdate = (event: MinerEventType): void => {
  const version = event.context.version;

  if (
    isTauri() ||
    version === undefined ||
    !app.snapshot?.settings.notifications.system_update ||
    browserNotificationPermission() !== BrowserPermission.GRANTED ||
    recall(UPDATE_NOTIFIED_STORAGE_KEY) === version
  ) {
    return;
  }

  remember(UPDATE_NOTIFIED_STORAGE_KEY, version);

  const notification = new Notification(EVENT_KIND_LABELS[event.kind](), {
    body: t.ev_update_available({ version }),
    icon: APP_ICON_URL,
    tag: UPDATE_NOTIFIED_STORAGE_KEY,
  });

  notification.onclick = () => window.focus();
};
