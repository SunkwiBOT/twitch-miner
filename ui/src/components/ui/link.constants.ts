export enum LinkSize {
  SM = "SM",
  MD = "MD",
  LG = "LG",
}

// ! `truncate` clips at the padding box and the underline sits below the line box (offset 3px, 2px on hover):
// the padding keeps it visible, the negative margin keeps the line's height.
const TRUNCATE_WITH_UNDERLINE = "truncate pb-1.5 -mb-1.5";

export const LINK_SIZES: Record<LinkSize, { text: string; icon: number; fit: string }> = {
  [LinkSize.SM]: { text: "text-xs", icon: 12, fit: TRUNCATE_WITH_UNDERLINE },
  [LinkSize.MD]: { text: "", icon: 13, fit: TRUNCATE_WITH_UNDERLINE },
  [LinkSize.LG]: { text: "text-lg", icon: 16, fit: "break-all" },
};

// ! MUTED is the quiet link of a meta line (the version under the sidebar): it keeps its row's muted tone.
export enum LinkTone {
  STRONG = "STRONG",
  MUTED = "MUTED",
}

export const LINK_TONES: Record<LinkTone, string> = {
  [LinkTone.STRONG]: "font-semibold text-foreground",
  [LinkTone.MUTED]: "font-medium text-muted-foreground",
};

// ! Where an external link shows its icon on hover. START and END keep the icon's room at rest,
// so a name never shifts; OVERLAY keeps none and draws the icon over what follows (a sentence's
// final period, the room around a centered line), so nothing shows a gap or moves.
export enum LinkMark {
  NONE = "NONE",
  START = "START",
  END = "END",
  OVERLAY = "OVERLAY",
}
