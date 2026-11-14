export enum RewardState {
  CLAIMED = "CLAIMED",
  READY = "READY",
  OWNED = "OWNED",
  IN_PROGRESS = "IN_PROGRESS",
  TO_EARN = "TO_EARN",
  SKIPPED = "SKIPPED",
}

// ! A tile can shrink to RewardTiles' 116 px minimum: its badges wrap there rather than cut their label.
export const TILE_BADGE_CLASS = "h-auto min-h-5 text-center text-balance whitespace-normal";
