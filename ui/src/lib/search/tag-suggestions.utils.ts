import { SEARCH_MIN_CHARS } from "./search.constants";
import { matchesSearch, normalizeSearch } from "./search.utils";

export type TagSuggestionType = {
  label: string;
  id: string | undefined;
  image: string | undefined;
};

export const typedTag = (draft: string, known: readonly TagSuggestionType[]): TagSuggestionType => {
  const label = draft.trim();
  const name = label.toLowerCase();

  return (
    known.find((item) => item.label.toLowerCase() === name) ?? {
      label,
      id: undefined,
      image: undefined,
    }
  );
};

// ! The app's own suggestions come first, then Twitch's answer: one entry per name, none already taken.
export const tagSuggestions = (
  draft: string,
  taken: readonly string[],
  known: readonly TagSuggestionType[],
  found: readonly TagSuggestionType[],
  max: number,
): TagSuggestionType[] => {
  if (draft.trim().length < SEARCH_MIN_CHARS) {
    return [];
  }

  const needle = normalizeSearch(draft);
  const seen = new Set(taken.map((label) => label.toLowerCase()));
  const out: TagSuggestionType[] = [];

  for (const suggestion of [
    ...known.filter((item) => matchesSearch(item.label, needle)),
    ...found,
  ]) {
    const name = suggestion.label.toLowerCase();

    if (!seen.has(name)) {
      seen.add(name);
      out.push(suggestion);
    }
  }

  return out.slice(0, max);
};
