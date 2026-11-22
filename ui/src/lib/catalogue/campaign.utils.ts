import { BenefitKind } from "../bindings/BenefitKind";
import { CampaignKind } from "../bindings/CampaignKind";
import type { CampaignType } from "../bindings/CampaignType";
import type { EvolutionType } from "../bindings/EvolutionType";
import type { RepeatType } from "../bindings/RepeatType";
import type { TimedDropType } from "../bindings/TimedDropType";
import { t } from "../i18n/i18n.svelte";
import { matchesSearch } from "../search/search.utils";
import { TWITCH_CATEGORY_URL, TWITCH_SEARCH_URL } from "../partners.constants";
import { strong } from "../text.utils";
import { DAY_HOURS } from "../time/time.constants";
import { relativeTime } from "../time/time.utils";

export type RewardType = { campaign: CampaignType; drop: TimedDropType };

// ! Only Twitch's own count says a subscription was made: until then the reward waits.
export const subsMissing = (item: { required_subs: number; current_subs: number }): boolean =>
  item.current_subs < item.required_subs;

export const dropDone = (drop: TimedDropType): boolean =>
  drop.repeat ? drop.periods_done >= drop.repeat.times : drop.claimed;

export const evolutionBlocked = (evolution: EvolutionType | undefined, enabled: boolean): boolean =>
  evolution !== undefined && (!evolution.from || !evolution.owned || !enabled);

export const watchText = (duration: string, repeat: RepeatType | undefined): string => {
  const watchTime = strong(duration);

  if (!repeat) {
    return t.reward_watch({ watchTime });
  }

  if (repeat.every_hours === DAY_HOURS) {
    return t.reward_watch_daily({ watchTime, days: strong(repeat.times) });
  }

  return repeat.every_hours
    ? t.reward_watch_every({
        watchTime,
        hours: strong(repeat.every_hours),
        timesCount: strong(repeat.times),
      })
    : t.reward_watch_times({ watchTime, timesCount: strong(repeat.times) });
};

export const periodsText = (done: number, repeat: RepeatType): string => {
  const values = { done: Math.min(done, repeat.times), total: repeat.times };

  return repeat.every_hours === DAY_HOURS
    ? t.reward_days_done(values)
    : t.reward_times_done(values);
};

export type PeriodsType = {
  current: number;
  required: number;
  done: number;
  closed: boolean;
  repeat: RepeatType | undefined;
};

export const periodTotals = (item: PeriodsType): { current: number; required: number } => {
  const watched = Math.min(item.current, item.required);

  if (!item.repeat) {
    return { current: watched, required: item.required };
  }

  const times = item.repeat.times;
  const done = Math.min(item.done, times);
  const open = item.closed || done >= times ? 0 : watched;

  return { current: done * item.required + open, required: item.required * times };
};

export const dropTotals = (drop: TimedDropType): { current: number; required: number } =>
  periodTotals({
    current: drop.current_minutes,
    required: drop.required_minutes,
    done: drop.periods_done,
    closed: drop.claimed,
    repeat: drop.repeat,
  });

export const campaignEnd = (endsAt: string, now: number): string => {
  const time = strong(relativeTime(endsAt, now));

  return new Date(endsAt).getTime() > now
    ? t.dashboard_ends_in({ endsTime: time })
    : t.dashboard_ended_ago({ endedTime: time });
};

// ! Same rule as `util::game_slug` in the core.
const gameSlug = (name: string): string =>
  name
    .replace(/['\u2019]/g, "")
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");

export const categoryUrl = (name: string, slug: string | undefined): string => {
  const path = slug || gameSlug(name);

  return path
    ? `${TWITCH_CATEGORY_URL}/${path}`
    : `${TWITCH_SEARCH_URL}?term=${encodeURIComponent(name)}`;
};

// ! A sitewide quest lists many categories in no meaningful order: its first one is no game to show.
export const sitewideQuest = (campaign: CampaignType): boolean =>
  campaign.kind === CampaignKind.QUEST && campaign.categories.length > 1;

export const campaignHeading = (campaign: CampaignType): string =>
  sitewideQuest(campaign) ? campaign.name : campaign.game.name;

export const campaignArtUrl = (campaign: CampaignType): string | undefined =>
  sitewideQuest(campaign) ? campaign.image_url : campaign.game.box_art_url;

export const campaignSubtitle = (campaign: CampaignType): string =>
  sitewideQuest(campaign) ? "" : campaign.name;

// ! Twitch's Drops page lists no badge-only campaign and no quest: a link to it would land on nothing.
export const onDropsPage = (campaign: CampaignType): boolean =>
  campaign.kind !== CampaignKind.QUEST &&
  campaign.drops.some((drop) =>
    drop.benefits.some((benefit) => benefit.kind !== BenefitKind.BADGE),
  );

// ! Same rule as `Campaign::needs_link` in the core: Twitch hands chat badges out itself.
export const needsLink = (campaign: CampaignType): boolean => {
  const benefits = campaign.drops.flatMap((drop) => drop.benefits);

  return benefits.length === 0 || benefits.some((benefit) => benefit.kind !== BenefitKind.BADGE);
};

export const dropGivesBadge = (drop: TimedDropType): boolean =>
  drop.benefits.some((benefit) => benefit.kind === BenefitKind.BADGE);

export const campaignGivesBadge = (campaign: CampaignType): boolean =>
  campaign.kind === CampaignKind.QUEST || campaign.drops.some(dropGivesBadge);

export const campaignMatches = (campaign: CampaignType, needle: string): boolean =>
  !needle || matchesSearch(campaign.game.name, needle) || matchesSearch(campaign.name, needle);
