export enum ButtonVariant {
  DEFAULT = "DEFAULT",
  PRIMARY = "PRIMARY",
  DANGER = "DANGER",
  OUTLINE = "OUTLINE",
  GHOST = "GHOST",
  DANGER_GHOST = "DANGER_GHOST",
}

export const BUTTON_VARIANTS: Record<
  ButtonVariant,
  "default" | "outline" | "ghost" | "destructive"
> = {
  [ButtonVariant.DEFAULT]: "outline",
  [ButtonVariant.PRIMARY]: "default",
  [ButtonVariant.DANGER]: "destructive",
  [ButtonVariant.OUTLINE]: "outline",
  [ButtonVariant.GHOST]: "ghost",
  [ButtonVariant.DANGER_GHOST]: "ghost",
};

export const BUTTON_VARIANT_CLASSES: Record<ButtonVariant, string> = {
  [ButtonVariant.DEFAULT]: "",
  [ButtonVariant.PRIMARY]: "",
  [ButtonVariant.DANGER]: "",
  [ButtonVariant.OUTLINE]: "",
  [ButtonVariant.GHOST]: "text-muted-foreground",
  [ButtonVariant.DANGER_GHOST]:
    "text-destructive hover:bg-destructive/10 hover:text-destructive dark:hover:bg-destructive/20",
};

export enum ButtonSize {
  MD = "MD",
  SM = "SM",
}

export const BUTTON_SIZES: Record<ButtonSize, "default" | "sm"> = {
  [ButtonSize.MD]: "default",
  [ButtonSize.SM]: "sm",
};

export const BUTTON_ICON_SIZES: Record<ButtonSize, number> = {
  [ButtonSize.MD]: 14,
  [ButtonSize.SM]: 13,
};

export const ICON_BUTTON_SIZES: Record<ButtonSize, "icon" | "icon-sm"> = {
  [ButtonSize.MD]: "icon",
  [ButtonSize.SM]: "icon-sm",
};

export enum ButtonType {
  BUTTON = "button",
  SUBMIT = "submit",
}
