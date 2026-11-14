export const FOCUS_INSIDE_CLASSES =
  "outline-hidden focus-visible:outline-solid focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-ring";

const FIELD_BUTTON_LOOK = `grid cursor-pointer place-items-center text-muted-foreground transition-colors hover:text-foreground focus-visible:text-foreground ${FOCUS_INSIDE_CLASSES}`;

export const FIELD_BUTTON_CLASSES = `absolute inset-y-0 right-0 w-8 rounded-r-lg ${FIELD_BUTTON_LOOK}`;

export const FIELD_ICON_CLASSES =
  "pointer-events-none absolute inset-y-0 right-0 grid w-8 place-items-center text-muted-foreground";

export const SEARCH_FIELD_PADDING = "pr-9";

export enum KeyboardKey {
  ENTER = "Enter",
  ESCAPE = "Escape",
  ARROW_UP = "ArrowUp",
  ARROW_DOWN = "ArrowDown",
}

export const CLEAR_SEARCH_KEY = KeyboardKey.ESCAPE;
