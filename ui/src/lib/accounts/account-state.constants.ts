import { IconName } from "../icons.constants";
import { BadgeTone } from "../tones.constants";

export enum AccountState {
  RUNNING = "RUNNING",
  STARTING = "STARTING",
  IDLE = "IDLE",
  PAUSED = "PAUSED",
  PROBLEM = "PROBLEM",
}

export const ACCOUNT_STATE_ICONS: Record<AccountState, IconName> = {
  [AccountState.RUNNING]: IconName.PLAY,
  [AccountState.STARTING]: IconName.LOADER,
  [AccountState.IDLE]: IconName.HOURGLASS,
  [AccountState.PAUSED]: IconName.PAUSE,
  [AccountState.PROBLEM]: IconName.ALERT,
};

export const ACCOUNT_STATE_TONES: Record<AccountState, BadgeTone> = {
  [AccountState.RUNNING]: BadgeTone.SUCCESS,
  [AccountState.STARTING]: BadgeTone.INFO,
  [AccountState.IDLE]: BadgeTone.NEUTRAL,
  [AccountState.PAUSED]: BadgeTone.MUTED,
  [AccountState.PROBLEM]: BadgeTone.WARNING,
};

export const ACCOUNT_STATE_ICON_CLASSES: Record<AccountState, string> = {
  [AccountState.RUNNING]: "text-success-emphasis",
  [AccountState.STARTING]: "text-info-emphasis",
  [AccountState.IDLE]: "text-foreground",
  [AccountState.PAUSED]: "text-muted-foreground",
  [AccountState.PROBLEM]: "text-warning-emphasis",
};

export enum PauseState {
  RUNNING = "RUNNING",
  PAUSED = "PAUSED",
  MIXED = "MIXED",
}

export enum PauseScope {
  ACCOUNT = "ACCOUNT",
  ALL = "ALL",
}
