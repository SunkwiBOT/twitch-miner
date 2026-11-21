export enum LinkSize {
  SM = "SM",
  MD = "MD",
  LG = "LG",
}

// ! SM is the quiet link of a meta line (the version under the sidebar): it keeps its row's muted tone.
export const LINK_SIZES: Record<
  LinkSize,
  { text: string; label: string; icon: number; fit: string }
> = {
  [LinkSize.SM]: {
    text: "",
    label: "font-medium text-muted-foreground",
    icon: 12,
    fit: "truncate",
  },
  [LinkSize.MD]: { text: "", label: "font-semibold text-foreground", icon: 13, fit: "truncate" },
  [LinkSize.LG]: {
    text: "text-lg",
    label: "font-semibold text-foreground",
    icon: 16,
    fit: "break-all",
  },
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
