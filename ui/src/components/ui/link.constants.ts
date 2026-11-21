export enum LinkSize {
  MD = "MD",
  LG = "LG",
}

export const LINK_SIZES: Record<LinkSize, { text: string; icon: number; fit: string }> = {
  [LinkSize.MD]: { text: "", icon: 13, fit: "truncate" },
  [LinkSize.LG]: { text: "text-lg", icon: 16, fit: "break-all" },
};

// ! Where an external link shows its icon on hover. START and END keep the icon's room at rest,
// so a name never shifts; SENTENCE_END keeps none, or the sentence would show a gap.
export enum LinkMark {
  NONE = "NONE",
  START = "START",
  END = "END",
  SENTENCE_END = "SENTENCE_END",
}
