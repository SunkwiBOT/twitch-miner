import { KEY_STORAGE_UPDATE_BANNER } from "../state/storage.constants";
import { recall, remember } from "../state/storage.utils";
import {
  askBrowserNotifications,
  browserNotificationPermission,
} from "./update-notification.utils";
import type { BrowserPermission } from "./updates.constants";

type BrowserNotificationsStateType = {
  permission: BrowserPermission | null;
  bannerDismissed: boolean;
};

// ! Shared by the Settings button and the Home banner: an answer given in one hides the other.
export const browserNotifications = $state<BrowserNotificationsStateType>({
  permission: browserNotificationPermission(),
  bannerDismissed: recall(KEY_STORAGE_UPDATE_BANNER) !== null,
});

// ! The answer can also change in the browser's own site settings, or in another tab. A browser
// that cannot report it keeps the answer read at load and after each ask.
if (browserNotifications.permission !== null && "permissions" in navigator) {
  navigator.permissions.query({ name: "notifications" }).then(
    (status) => {
      status.onchange = () => {
        browserNotifications.permission = browserNotificationPermission();
      };
    },
    (e: unknown) => console.error("notification permission not watched", e),
  );
}

export const allowBrowserNotifications = async (): Promise<void> => {
  browserNotifications.permission = await askBrowserNotifications();
};

export const dismissUpdateBanner = (): void => {
  browserNotifications.bannerDismissed = true;
  remember(KEY_STORAGE_UPDATE_BANNER, String(true));
};
