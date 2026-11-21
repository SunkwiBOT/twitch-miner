export enum UpdateCheck {
  IDLE = "IDLE",
  CHECKING = "CHECKING",
  CURRENT = "CURRENT",
  AVAILABLE = "AVAILABLE",
  FAILED = "FAILED",
}

export enum UpdateStage {
  IDLE = "IDLE",
  DOWNLOADING = "DOWNLOADING",
  INSTALLING = "INSTALLING",
}

export const UPDATE_TOAST_KEY = "update";

export const UPDATE_NOTIFIED_STORAGE_KEY = "updates:notifiedVersion";

export const APP_ICON_URL = "/app-icon.png";

export enum BrowserPermission {
  DEFAULT = "default",
  GRANTED = "granted",
  DENIED = "denied",
}

export const BROWSER_PERMISSIONS: Record<NotificationPermission, BrowserPermission> = {
  default: BrowserPermission.DEFAULT,
  granted: BrowserPermission.GRANTED,
  denied: BrowserPermission.DENIED,
};

export const IMAGE_UPDATE_COMMANDS: readonly string[] = [
  "docker compose pull",
  "docker compose up -d",
];

// ! The file names, path and service of the README's systemd section.
export const BINARY_ASSET_PREFIX = "twitch-miner-server_";
export const BINARY_PATH = "/usr/local/bin/twitch-miner";
export const BINARY_SERVICE = "twitch-miner";

export const UPDATE_BADGE_CLASSES =
  "rounded-full border-info-emphasis/40 bg-info text-info-foreground hover:bg-info/80 hover:text-info-foreground";

export const MEGABYTE_UNIT = "megabyte";
export const BYTES_PER_MEGABYTE = 1_000_000;
