import { IconName } from "../../lib/icons.constants";

export enum BalanceAlign {
  START = "START",
  END = "END",
}

export const COPIED_FEEDBACK_MS = 1_800;

export const TOOLTIP_DELAY_MS = 400;

export enum CopyLayout {
  ICON = "ICON",
  LABELLED = "LABELLED",
}

export enum MetaLayout {
  ROW = "ROW",
  STACKED = "STACKED",
}

export enum NoticeTone {
  INFO = "INFO",
  WARNING = "WARNING",
  DANGER = "DANGER",
}

export const NOTICE_TONE_CLASSES: Record<NoticeTone, string> = {
  [NoticeTone.INFO]: "border-info-emphasis/40 bg-info text-info-foreground",
  [NoticeTone.WARNING]: "border-warning-emphasis/40 bg-warning text-warning-foreground",
  [NoticeTone.DANGER]: "border-destructive/40 bg-destructive/10 text-destructive-foreground",
};

export const NOTICE_TONE_ICONS: Record<NoticeTone, IconName> = {
  [NoticeTone.INFO]: IconName.BELL,
  [NoticeTone.WARNING]: IconName.ALERT,
  [NoticeTone.DANGER]: IconName.ALERT,
};

export enum PanelWidth {
  MD = "420px",
  LG = "460px",
}

export const CARD_LIFT_CLASSES =
  "transition-[translate,box-shadow] duration-150 group-hover:translate-x-0.75 group-hover:-translate-y-0.75 group-hover:shadow-[-4px_4px_0_var(--primary)] group-focus-visible:translate-x-0.75 group-focus-visible:-translate-y-0.75 group-focus-visible:shadow-[-4px_4px_0_var(--primary)] group-has-focus-visible:translate-x-0.75 group-has-focus-visible:-translate-y-0.75 group-has-focus-visible:shadow-[-4px_4px_0_var(--primary)]";
