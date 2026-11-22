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
  [BadgeTone.MUTED]: "border-muted-foreground/70 bg-foreground/5 text-foreground",
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

// ! A chip (streamer, tag, « +N ») wears the neutral tone over the shadcn secondary base, its link hover included.
export const BADGE_CHIP_CLASSES = `${BADGE_TONE_CLASSES[BadgeTone.NEUTRAL]} [a]:hover:bg-foreground/15`;

export const BADGE_MARQUEE_PX_PER_SECOND = 30;

// ! The share of each lap spent moving: `badge-marquee` in `theme.css` rests from 0 to 20 % and from 80 to 100 %.
export const BADGE_MARQUEE_TRAVEL_SHARE = 0.6;

// ! The opaque pill under a badge laid over a picture. A flex box: a block wrapper
// would size to its line box, strut included, and paint past the badge's border.
export const BADGE_BACKDROP_CLASSES = "flex max-w-full rounded-4xl bg-background";

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
