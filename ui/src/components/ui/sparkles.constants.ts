export type SparkleRangeType = { min: number; max: number };

// ! One sparkle between two others at rest, quicker under the pointer.
export const SPARKLE_REST_DELAY_MS: SparkleRangeType = { min: 380, max: 760 };
export const SPARKLE_HOVER_DELAY_MS: SparkleRangeType = { min: 110, max: 220 };

// ! A press throws a burst: a few sparkles at once, then the rest one step apart, so it feels instant.
export const SPARKLE_BURST_COUNT = 10;
export const SPARKLE_BURST_INSTANT_COUNT = 4;
export const SPARKLE_BURST_STEP_MS = 25;

// ! The keyframes' duration too (`--sparkle-lifetime`): a sparkle grows, turns and shrinks back, then leaves.
export const SPARKLE_LIFETIME_MS = 900;
export const SPARKLE_REMOVAL_MARGIN_MS = 50;
export const SPARKLE_SIZE_PX: SparkleRangeType = { min: 7, max: 15 };
export const SPARKLE_SIZE_STEPS_PER_PX = 10;

// ! A bound for a long hover or repeated clicks.
export const MAX_SPARKLES = 24;

// ! In % of the button: born around it, beyond its edges, never on its label.
export const SPARKLE_AREA: Record<"x" | "y", SparkleRangeType> = {
  x: { min: -6, max: 106 },
  y: { min: -40, max: 140 },
};
export const SPARKLE_LABEL_BOX: Record<"x" | "y", SparkleRangeType> = {
  x: { min: 14, max: 86 },
  y: { min: 20, max: 80 },
};

// ! The label box covers about a fifth of the area: a few draws are enough.
export const SPARKLE_POSITION_DRAWS = 32;

export const SPARKLE_PRIMARY_BUTTON = 0;

export enum SparkleTone {
  SPARKLE = "SPARKLE",
  GOLD = "GOLD",
  BRAND = "BRAND",
}

export const SPARKLE_TONES: readonly SparkleTone[] = [
  SparkleTone.SPARKLE,
  SparkleTone.GOLD,
  SparkleTone.BRAND,
];

export const SPARKLE_TONE_CLASSES: Record<SparkleTone, string> = {
  [SparkleTone.SPARKLE]: "text-sparkle",
  [SparkleTone.GOLD]: "text-sparkle-gold",
  [SparkleTone.BRAND]: "text-link",
};

export const SPARKLE_PATH =
  "M80 0C80 0 84.28 41.29 101.5 58.5C118.71 75.72 160 80 160 80C160 80 118.71 84.28 101.5 101.5C84.28 118.71 80 160 80 160C80 160 75.72 118.71 58.5 101.5C41.29 84.28 0 80 0 80C0 80 41.29 75.72 58.5 58.5C75.72 41.29 80 0 80 0Z";
export const SPARKLE_VIEW_BOX = "0 0 160 160";
