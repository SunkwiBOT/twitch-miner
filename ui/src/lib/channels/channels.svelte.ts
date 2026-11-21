import { fetchBanners, lookupChannels } from "../api/client";
import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { ChannelType } from "../bindings/ChannelType";
import type { SearchChannelType } from "../bindings/SearchChannelType";
import type { SnapshotType } from "../bindings/SnapshotType";
import { app } from "../state/app.svelte";
import { coreLimits } from "../state/limits";

export const knownChannels: Partial<Record<string, ChannelType>> = $state({});

const asked = new Set<string>();
let queued: string[] = [];
let flushing = false;

export const rememberChannels = (channels: ChannelType[]): void => {
  for (const channel of channels) {
    knownChannels[channel.login] = channel;
  }
};

export const accountChannel = (
  account: AccountSnapshotType,
  login: string,
): ChannelType | undefined =>
  account.streamers.find((s) => s.login === login)?.channel ?? knownChannels[login];

export const searchedChannel = (found: SearchChannelType): ChannelType => ({
  id: "",
  login: found.login,
  display_name: found.display_name,
  avatar_url: found.avatar_url,
  live: found.live,
  drops_enabled: false,
});

const flush = async (): Promise<void> => {
  flushing = true;

  try {
    while (queued.length > 0) {
      const batch = queued.splice(0, coreLimits().channels_lookup_max);

      try {
        rememberChannels((await lookupChannels(batch)).channels);
      } catch (e: unknown) {
        // ! Pictures are a nicety: the names show anyway.
        for (const login of batch) {
          asked.delete(login);
        }
      }
    }
  } finally {
    flushing = false;
  }
};

// ! A session that ends drops what it still had to look up: the next one asks again.
export const forgetChannelLookups = (): void => {
  for (const login of queued) {
    asked.delete(login);
  }

  queued = [];
};

// ! A channel known only from a search (no id) is read in full: its game and viewers.
export const ensureChannels = (logins: string[]): void => {
  const wanted = logins.filter((login) => !knownChannels[login]?.id && !asked.has(login));

  if (wanted.length === 0) {
    return;
  }

  for (const login of wanted) {
    asked.add(login);
  }

  queued.push(...wanted);

  if (!flushing) {
    queueMicrotask(() => void flush());
  }
};

const liveIndexes = new WeakMap<SnapshotType, Map<string, boolean>>();

const liveIndexOf = (snapshot: SnapshotType): Map<string, boolean> => {
  const known = liveIndexes.get(snapshot);

  if (known) {
    return known;
  }

  const index = new Map<string, boolean>();

  for (const account of snapshot.accounts) {
    for (const channel of [
      ...account.slots.map((slot) => slot.channel),
      ...account.streamers.map((view) => view.channel),
    ]) {
      if (channel && (channel.live || !index.has(channel.login))) {
        index.set(channel.login, channel.live);
      }
    }
  }

  liveIndexes.set(snapshot, index);

  return index;
};

export const isLive = (login: string | null | undefined): boolean => {
  if (!login) {
    return false;
  }

  const reported = app.snapshot ? liveIndexOf(app.snapshot).get(login) : undefined;

  return reported ?? knownChannels[login]?.live === true;
};

export const knownBanners: Partial<Record<string, string>> = $state({});
const bannersAsked = new Set<string>();

const fetchBannerBatches = async (logins: string[]): Promise<void> => {
  const lookupMax = coreLimits().channels_lookup_max;

  for (let from = 0; from < logins.length; from += lookupMax) {
    const batch = logins.slice(from, from + lookupMax);

    try {
      Object.assign(knownBanners, await fetchBanners(batch));
    } catch (e: any) {
      for (const login of batch) {
        bannersAsked.delete(login);
      }
    }
  }
};

export const ensureBanners = (logins: string[]): void => {
  const wanted = logins.filter((login) => !bannersAsked.has(login));

  if (wanted.length === 0) {
    return;
  }

  for (const login of wanted) {
    bannersAsked.add(login);
  }

  void fetchBannerBatches(wanted);
};
