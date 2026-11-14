export enum ThumbSize {
  XS = "XS",
  SM = "SM",
  MD = "MD",
  LG = "LG",
  XL = "XL",
}

export enum ThumbShape {
  SQUARE = "SQUARE",
  BOX_ART = "BOX_ART",
}

export const THUMB_PIXELS: Record<ThumbSize, number> = {
  [ThumbSize.XS]: 20,
  [ThumbSize.SM]: 28,
  [ThumbSize.MD]: 36,
  [ThumbSize.LG]: 48,
  [ThumbSize.XL]: 64,
};

export const ART_TILES: Record<
  ThumbShape,
  { width: number; height: number; stepX: number; stepY: number }
> = {
  [ThumbShape.BOX_ART]: { width: 27, height: 36, stepX: 30, stepY: 39 },
  [ThumbShape.SQUARE]: { width: 30, height: 30, stepX: 33, stepY: 33 },
};

export enum AvatarSize {
  XS = "XS",
  SM = "SM",
  MD = "MD",
  LG = "LG",
}

export const AVATAR_CLASSES: Record<AvatarSize, string> = {
  [AvatarSize.XS]: "size-5 text-initial-xs",
  [AvatarSize.SM]: "size-6.5 text-initial-sm",
  [AvatarSize.MD]: "size-8 text-xs",
  [AvatarSize.LG]: "size-11 text-base",
};

// ! Never translated, as on Twitch, so it fits the picture; screen readers hear the translated "live".
export const LIVE_TAG = "LIVE";

export const LIVE_TAG_CLASSES: Partial<Record<AvatarSize, string>> = {
  [AvatarSize.SM]: "px-0.5 text-live-tag-sm/3",
  [AvatarSize.MD]: "px-1 text-live-tag/3.5",
  [AvatarSize.LG]: "px-1 text-live-tag/3.5",
};
