import { getContext, setContext } from "svelte";

const NAME_LIST = Symbol("name-list");

// ! A list of names that wraps shows no external icon: its room at rest would open gaps between the names.
export const provideNameList = (): void => {
  setContext(NAME_LIST, true);
};

export const inNameList = (): boolean => getContext<boolean | undefined>(NAME_LIST) === true;
