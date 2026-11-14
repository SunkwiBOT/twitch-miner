export enum LinkLook {
  HOVER = "HOVER",
  ALWAYS = "ALWAYS",
  SENTENCE = "SENTENCE",
}

export enum LinkSize {
  MD = "MD",
  LG = "LG",
}

export const LINK_SIZES: Record<LinkSize, { text: string; icon: number; fit: string }> = {
  [LinkSize.MD]: { text: "", icon: 13, fit: "truncate" },
  [LinkSize.LG]: { text: "text-lg", icon: 16, fit: "break-all" },
};

export enum LinkMark {
  NONE = "NONE",
  START = "START",
  END = "END",
}
