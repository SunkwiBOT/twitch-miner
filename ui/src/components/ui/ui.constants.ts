import { IconName } from "../../lib/icons.constants";

export enum BalanceAlign {
  START = "START",
  END = "END",
}

export const COPIED_FEEDBACK_MS = 1_800;

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
  XL = "560px",
}

export const CARD_LIFT_CLASSES =
  "transition-[translate,box-shadow] duration-150 group-hover:translate-x-0.75 group-hover:-translate-y-0.75 group-hover:shadow-[-4px_4px_0_var(--primary)] group-focus-visible:translate-x-0.75 group-focus-visible:-translate-y-0.75 group-focus-visible:shadow-[-4px_4px_0_var(--primary)] group-has-focus-visible:translate-x-0.75 group-has-focus-visible:-translate-y-0.75 group-has-focus-visible:shadow-[-4px_4px_0_var(--primary)]";

// ! The button's `::after` covers its nearest positioned row or card, so all of it opens; the other
// controls there sit outside the button, raised with `RAISED_CLASSES`.
export const STRETCHED_BUTTON_CLASSES =
  "text-left outline-hidden after:absolute after:inset-0 focus-visible:after:outline-2 focus-visible:after:-outline-offset-2 focus-visible:after:outline-ring focus-visible:after:outline-solid";

// ! Clicks between the raised children still reach the stretched button under them.
export const RAISED_CLASSES = "pointer-events-none relative z-10 *:pointer-events-auto";

// ! A picture beside a name in running text: its bottom on the baseline, then lowered so its centre meets the x-height's, leaving the line's baseline alone.
export const INLINE_PICTURE_CLASSES =
  "inline-flex shrink-0 self-baseline translate-y-[calc(50%-0.3em)]";

// ! A value in a sentence: its weight follows the text around it (`--value-weight`, theme.css).
export const VALUE_CLASSES = "font-(--value-weight) in-[.text-muted-foreground]:text-foreground";
