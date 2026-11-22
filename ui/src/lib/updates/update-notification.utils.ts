import { isTauri } from "../api/tauri";
import { EventKind } from "../bindings/EventKind";
import { EVENT_KIND_LABELS } from "../events/labels.utils";
import { t } from "../i18n/i18n.svelte";
import { app } from "../state/app.svelte";
import { recall, remember } from "../state/storage.utils";
import { KEY_STORAGE_UPDATE_NOTIFIED } from "../state/storage.constants";
import { APP_ICON_URL } from "../partners.constants";
import {
  BROWSER_PERMISSIONS,
  BrowserPermission,
  UPDATE_NOTIFICATION_TAG,
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
// receives the event or a snapshot naming the version, and the stored version keeps it to one notification.
export const notifyUpdate = (version: string | undefined): void => {
  if (
    isTauri() ||
    version === undefined ||
    !app.snapshot?.settings.notifications.system_update ||
    browserNotificationPermission() !== BrowserPermission.GRANTED ||
    recall(KEY_STORAGE_UPDATE_NOTIFIED) === version
  ) {
    return;
  }

  remember(KEY_STORAGE_UPDATE_NOTIFIED, version);

  // ! Chrome for Android grants the permission yet refuses the constructor: the version still counts as notified.
  try {
    const notification = new Notification(EVENT_KIND_LABELS[EventKind.UPDATE_AVAILABLE](), {
      body: t.ev_update_available({ version }),
      icon: APP_ICON_URL,
      tag: UPDATE_NOTIFICATION_TAG,
    });

    notification.onclick = () => window.focus();
  } catch (e: unknown) {
    console.error("update notification not shown", e);
  }
};
