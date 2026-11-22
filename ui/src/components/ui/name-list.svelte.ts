import { getContext, setContext } from "svelte";

import type { NameListKind } from "./link.constants";

const NAME_LIST = Symbol("name-list");

export const provideNameList = (kind: NameListKind): void => {
  setContext(NAME_LIST, kind);
};

export const nameListKind = (): NameListKind | undefined =>
  getContext<NameListKind | undefined>(NAME_LIST);
