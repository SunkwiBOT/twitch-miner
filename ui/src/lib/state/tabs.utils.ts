import { Page, TAB_SEPARATOR } from "./state.constants";

export type SavedTabsType = Partial<Record<Page, string[]>>;

const PAGES: readonly Page[] = Object.values(Page);

export const tabParts = (tab: string): string[] =>
  tab.split(TAB_SEPARATOR).filter((part) => part !== "");

export const joinTabs = (parts: readonly string[]): string => parts.join(TAB_SEPARATOR);

export const pickTab = <T extends string | number>(
  parts: readonly string[],
  tabs: readonly T[],
  slugs: Record<T, string>,
): T | undefined => tabs.find((tab) => parts.includes(slugs[tab]));

// ! One page can hold several tab rows (view, zoom, sort…): each row only
// replaces its own slug, in place, so the others keep theirs and their order.
export const withTab = <T extends string | number>(
  parts: readonly string[],
  slugs: Record<T, string>,
  tab: T,
): string[] => {
  const own: ReadonlySet<string> = new Set(Object.values<string>(slugs));
  const at = parts.findIndex((part) => own.has(part));

  if (at === -1) {
    return [...parts, slugs[tab]];
  }

  return parts.flatMap((part, index) => {
    if (index === at) {
      return [slugs[tab]];
    }

    return own.has(part) ? [] : [part];
  });
};

const isSlugList = (value: unknown): value is string[] =>
  Array.isArray(value) && value.every((part) => typeof part === "string" && part !== "");

export const readSavedTabs = (raw: string | null): SavedTabsType => {
  let parsed: unknown;

  try {
    parsed = JSON.parse(raw ?? "{}");
  } catch (e: unknown) {
    return {};
  }

  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
    return {};
  }

  const saved: SavedTabsType = {};

  for (const page of PAGES) {
    const parts: unknown = Reflect.get(parsed, page);

    if (isSlugList(parts)) {
      saved[page] = parts;
    }
  }

  return saved;
};
