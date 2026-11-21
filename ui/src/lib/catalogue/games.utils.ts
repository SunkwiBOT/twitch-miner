import type { GameEntryType } from "../bindings/GameEntryType";
import type { GameType } from "../bindings/GameType";
import type { TagSuggestionType } from "../search/tag-suggestions.utils";
import { app } from "../state/app.svelte";
import { categoryUrl } from "./campaign.utils";
import { catalogue } from "./catalogue.svelte";

export type RunningGameType = { game: GameType; drops: number };

export const gameEntryKey = (entry: GameEntryType): string => entry.id ?? entry.name;

export const catalogueGame = (name: string): GameType | undefined =>
  catalogue.campaigns.find((c) => c.game.name.toLowerCase() === name.toLowerCase())?.game;

export const catalogueGameTags = (): TagSuggestionType[] => {
  const byName = new Map<string, TagSuggestionType>();

  for (const { game } of catalogue.campaigns) {
    byName.set(game.name.toLowerCase(), {
      label: game.name,
      id: game.id,
      image: game.box_art_url,
    });
  }

  return [...byName.values()].sort((a, b) => a.label.localeCompare(b.label));
};

// ! The image an account kept for a priority or excluded category from the
// search it was added through, shared across every account so a category
// without a running campaign still shows art once any account knows it.
const knownGameArt = (name: string): string | undefined => {
  const needle = name.toLowerCase();

  for (const account of app.snapshot?.accounts ?? []) {
    for (const entry of [
      ...account.settings.drops.priority_games,
      ...account.settings.drops.exclude_games,
      ...account.settings.badges.exclude_games,
    ]) {
      if (entry.box_art_url && entry.name.toLowerCase() === needle) {
        return entry.box_art_url;
      }
    }
  }

  return undefined;
};

export const gameArt = (name: string, art: string | undefined): string | undefined =>
  art ?? catalogueGame(name)?.box_art_url ?? knownGameArt(name);

export const gameUrl = (name: string, slug: string | undefined): string =>
  categoryUrl(name, slug ?? catalogueGame(name)?.slug);

export const runningGames = (now: number): RunningGameType[] => {
  const byName = new Map<string, RunningGameType>();

  for (const c of catalogue.campaigns) {
    if (new Date(c.ends_at).getTime() <= now) {
      continue;
    }

    const key = c.game.name.toLowerCase();
    const known = byName.get(key) ?? { game: c.game, drops: 0 };

    known.drops += c.drops.length;
    byName.set(key, known);
  }

  return [...byName.values()].sort((a, b) => a.game.name.localeCompare(b.game.name));
};
