import { BadgeTone } from "../../lib/tones.constants";

export enum BadgeVariant {
  TONE = "TONE",
  CHIP = "CHIP",
}

export const BADGE_VARIANTS: Record<BadgeVariant, "outline" | "secondary"> = {
  [BadgeVariant.TONE]: "outline",
  [BadgeVariant.CHIP]: "secondary",
};

// ! The border carries the pill's outline: at least 3:1 against every surface a badge sits on, in both themes.
export const BADGE_TONE_CLASSES: Record<BadgeTone, string> = {
  [BadgeTone.NEUTRAL]: "border-muted-foreground/80 bg-foreground/10 text-foreground",
  [BadgeTone.MUTED]: "border-muted-foreground/80 bg-transparent text-muted-foreground",
  [BadgeTone.PRIMARY]: "border-link bg-primary text-primary-foreground",
  [BadgeTone.ACCENT]: "border-link/80 bg-primary-soft text-link",
  [BadgeTone.SUCCESS]: "border-success-foreground/80 bg-success text-success-foreground",
  [BadgeTone.WARNING]: "border-warning-foreground/80 bg-warning text-warning-foreground",
  [BadgeTone.INFO]: "border-info-foreground/80 bg-info text-info-foreground",
  [BadgeTone.SUBSCRIBED]:
    "border-subscribed-foreground/80 bg-subscribed text-subscribed-foreground",
  [BadgeTone.DANGER]:
    "border-destructive-foreground/80 bg-destructive/15 text-destructive-foreground",
};

export const BADGE_CHIP_CLASSES = "border-muted-foreground/80";

export const DOT_TONE_CLASSES: Record<BadgeTone, string> = {
  [BadgeTone.NEUTRAL]: "bg-muted-foreground",
  [BadgeTone.MUTED]: "bg-muted-foreground",
  [BadgeTone.PRIMARY]: "bg-primary",
  [BadgeTone.ACCENT]: "bg-primary",
  [BadgeTone.SUCCESS]: "bg-success-emphasis ring-3 ring-success",
  [BadgeTone.WARNING]: "bg-warning-emphasis",
  [BadgeTone.INFO]: "bg-info-emphasis",
  [BadgeTone.SUBSCRIBED]: "bg-subscribed-emphasis",
  [BadgeTone.DANGER]: "bg-destructive",
};
