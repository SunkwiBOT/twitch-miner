export enum MenuAlign {
  LEFT = "LEFT",
  RIGHT = "RIGHT",
}

export const MENU_EDGE_GAP = 8;

// ! A menu of short labels grows to its longest label, never narrower than its button or 14rem, never past
// the room on screen; a menu whose options carry descriptions keeps a fixed width so they wrap instead.
export const MENU_FIT_WIDTH_CLASSES =
  "max-h-(--bits-popover-content-available-height) w-max min-w-[max(var(--bits-popover-anchor-width),14rem)] max-w-(--bits-popover-content-available-width) p-0";

export const MENU_DESCRIBED_WIDTH_CLASSES =
  "max-h-(--bits-popover-content-available-height) w-(--bits-popover-anchor-width) min-w-80 p-0";

export const MENU_ITEM_CLASSES =
  "cursor-pointer gap-2.5 rounded-md px-2 py-1.5 data-selected:bg-accent data-selected:text-accent-foreground data-[checked=true]:bg-selected data-[checked=true]:font-medium data-[checked=true]:text-selected-foreground [&>svg:last-child]:hidden data-[checked=true]:[&>svg:last-child]:block";

export const MULTI_ITEM_CLASSES =
  "cursor-pointer gap-2.5 rounded-md px-2 py-1.5 data-selected:bg-accent data-selected:text-accent-foreground [&>svg:last-child]:hidden";

export const MULTI_ALL_VALUE = "__all__";

export const CHECKED_OPTION = '[data-checked="true"]';

export const MULTI_CHECKED_OPTION = `[aria-checked="true"]:not([data-value="${MULTI_ALL_VALUE}"])`;

export const MULTI_PICTURES_MAX = 3;

export const SELECT_SEARCH_FROM = 5;
