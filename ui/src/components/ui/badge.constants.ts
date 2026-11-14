import { BadgeTone } from "../../lib/tones.constants";

export enum BadgeVariant {
  TONE = "TONE",
  CHIP = "CHIP",
}

export const BADGE_VARIANTS: Record<BadgeVariant, "outline" | "secondary"> = {
  [BadgeVariant.TONE]: "outline",
  [BadgeVariant.CHIP]: "secondary",
};

export const BADGE_TONE_CLASSES: Record<BadgeTone, string> = {
  [BadgeTone.NEUTRAL]: "border-transparent bg-foreground/10 text-foreground",
  [BadgeTone.MUTED]: "border-border bg-transparent text-muted-foreground",
  [BadgeTone.PRIMARY]: "bg-primary text-primary-foreground",
  [BadgeTone.ACCENT]: "border-primary/40 bg-primary/15 text-link",
  [BadgeTone.SUCCESS]: "border-success-emphasis/30 bg-success text-success-foreground",
  [BadgeTone.WARNING]: "border-warning-emphasis/30 bg-warning text-warning-foreground",
  [BadgeTone.INFO]: "border-info-emphasis/30 bg-info text-info-foreground",
  [BadgeTone.DANGER]: "border-destructive/30 bg-destructive/15 text-destructive-foreground",
};

export const DOT_TONE_CLASSES: Record<BadgeTone, string> = {
  [BadgeTone.NEUTRAL]: "bg-muted-foreground",
  [BadgeTone.MUTED]: "bg-muted-foreground",
  [BadgeTone.PRIMARY]: "bg-primary",
  [BadgeTone.ACCENT]: "bg-primary",
  [BadgeTone.SUCCESS]: "bg-success-emphasis ring-3 ring-success",
  [BadgeTone.WARNING]: "bg-warning-emphasis",
  [BadgeTone.INFO]: "bg-info-emphasis",
  [BadgeTone.DANGER]: "bg-destructive",
};
