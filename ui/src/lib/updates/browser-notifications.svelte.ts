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

export const allowBrowserNotifications = async (): Promise<void> => {
  browserNotifications.permission = await askBrowserNotifications();
};

export const dismissUpdateBanner = (): void => {
  browserNotifications.bannerDismissed = true;
  remember(KEY_STORAGE_UPDATE_BANNER, String(true));
};
