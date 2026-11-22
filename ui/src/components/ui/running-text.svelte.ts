import { getContext, setContext } from "svelte";

const RUNNING_TEXT = Symbol("running-text");

// ! Inside a sentence, a name with its picture sits on the sentence's baseline; in a row it stays centred.
export const provideRunningText = (): void => {
  setContext(RUNNING_TEXT, true);
};

export const inRunningText = (): boolean => getContext<boolean | undefined>(RUNNING_TEXT) === true;
