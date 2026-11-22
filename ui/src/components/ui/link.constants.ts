export enum LinkSize {
  SM = "SM",
  MD = "MD",
  LG = "LG",
}

// ! The underline sits below the line box (offset 3px, 2px on hover): only the width is clipped, never the height.
// `clip` makes no scroll container, so a flex item keeps its text's width as its minimum: `min-w-0` lets it shrink.
const TRUNCATE_WITH_UNDERLINE = "min-w-0 overflow-x-clip text-ellipsis whitespace-nowrap";

export const LINK_SIZES: Record<LinkSize, { text: string; icon: number; fit: string }> = {
  [LinkSize.SM]: { text: "text-xs", icon: 12, fit: TRUNCATE_WITH_UNDERLINE },
  [LinkSize.MD]: { text: "", icon: 13, fit: TRUNCATE_WITH_UNDERLINE },
  [LinkSize.LG]: { text: "text-lg", icon: 16, fit: "break-all" },
};

// ! MUTED is the quiet link of a meta line (the version under the sidebar): it keeps its row's muted tone.
// BRAND is the app's name at the top of the sidebar, as heavy as a title.
export enum LinkTone {
  STRONG = "STRONG",
  MUTED = "MUTED",
  BRAND = "BRAND",
}

export const LINK_TONES: Record<LinkTone, string> = {
  [LinkTone.STRONG]: "font-semibold text-foreground",
  [LinkTone.MUTED]: "font-medium text-muted-foreground",
  [LinkTone.BRAND]: "font-bold text-foreground",
};

// ! A link is underlined at rest; PLAIN is the one exception, for the app's own marks (its name and version in
// the sidebar): no underline at rest, the same reaction as any link on hover.
export enum LinkRest {
  UNDERLINED = "UNDERLINED",
  PLAIN = "PLAIN",
}

export const LINK_RESTS: Record<LinkRest, string> = {
  [LinkRest.UNDERLINED]: "",
  [LinkRest.PLAIN]: "decoration-transparent link-active:decoration-link",
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
