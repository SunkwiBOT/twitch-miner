import { EventCategory } from "../bindings/EventCategory";
import { EventKind } from "../bindings/EventKind";
import { MinerKind } from "../bindings/MinerKind";
import { IconName } from "../icons.constants";
import { BadgeTone } from "../tones.constants";

export const TWITCH_WATCH_STREAK_REASON = "WATCH_STREAK";

export const EVENT_TONES: Record<EventKind, BadgeTone> = {
  [EventKind.INFO]: BadgeTone.NEUTRAL,
  [EventKind.ERROR]: BadgeTone.DANGER,
  [EventKind.WATCH_STARTED]: BadgeTone.NEUTRAL,
  [EventKind.WATCH_STOPPED]: BadgeTone.NEUTRAL,
  [EventKind.DROP_PROGRESS]: BadgeTone.NEUTRAL,
  [EventKind.DROP_CLAIMED]: BadgeTone.ACCENT,
  [EventKind.CAMPAIGN_FINISHED]: BadgeTone.ACCENT,
  [EventKind.POINTS_EARNED]: BadgeTone.SUCCESS,
  [EventKind.POINTS_SPENT]: BadgeTone.NEUTRAL,
  [EventKind.REWARD_REDEEMED]: BadgeTone.NEUTRAL,
  [EventKind.BONUS_CLAIMED]: BadgeTone.SUCCESS,
  [EventKind.PREDICTION_PLACED]: BadgeTone.NEUTRAL,
  [EventKind.PREDICTION_RESULT]: BadgeTone.WARNING,
  [EventKind.STREAM_ONLINE]: BadgeTone.NEUTRAL,
  [EventKind.STREAM_OFFLINE]: BadgeTone.NEUTRAL,
  [EventKind.RAID_JOINED]: BadgeTone.NEUTRAL,
  [EventKind.MOMENT_CLAIMED]: BadgeTone.NEUTRAL,
  [EventKind.STREAK_RECOVERED]: BadgeTone.NEUTRAL,
  [EventKind.GOAL_CONTRIBUTED]: BadgeTone.NEUTRAL,
  [EventKind.CHAT_MENTION]: BadgeTone.INFO,
  [EventKind.SESSION_EXPIRED]: BadgeTone.DANGER,
  [EventKind.LOGIN_COMPLETED]: BadgeTone.NEUTRAL,
  [EventKind.UPDATE_AVAILABLE]: BadgeTone.INFO,
};

export const EVENT_ICONS: Record<EventKind, IconName> = {
  [EventKind.INFO]: IconName.BELL,
  [EventKind.ERROR]: IconName.ALERT,
  [EventKind.WATCH_STARTED]: IconName.PLAY,
  [EventKind.WATCH_STOPPED]: IconName.PAUSE,
  [EventKind.DROP_PROGRESS]: IconName.CLOCK,
  [EventKind.DROP_CLAIMED]: IconName.GIFT,
  [EventKind.CAMPAIGN_FINISHED]: IconName.CHECK,
  [EventKind.POINTS_EARNED]: IconName.CHANNEL_POINTS,
  [EventKind.POINTS_SPENT]: IconName.CHANNEL_POINTS,
  [EventKind.REWARD_REDEEMED]: IconName.ZAP,
  [EventKind.BONUS_CLAIMED]: IconName.CHANNEL_POINTS,
  [EventKind.PREDICTION_PLACED]: IconName.TARGET,
  [EventKind.PREDICTION_RESULT]: IconName.TROPHY,
  [EventKind.STREAM_ONLINE]: IconName.RADIO,
  [EventKind.STREAM_OFFLINE]: IconName.TV,
  [EventKind.RAID_JOINED]: IconName.ZAP,
  [EventKind.MOMENT_CLAIMED]: IconName.STAR,
  [EventKind.STREAK_RECOVERED]: IconName.FLAME,
  [EventKind.GOAL_CONTRIBUTED]: IconName.TARGET,
  [EventKind.CHAT_MENTION]: IconName.AT,
  [EventKind.SESSION_EXPIRED]: IconName.LOGOUT,
  [EventKind.LOGIN_COMPLETED]: IconName.LOG_IN,
  [EventKind.UPDATE_AVAILABLE]: IconName.IMPORT,
};

export const MINER_TONES: Record<MinerKind, BadgeTone> = {
  [MinerKind.DROPS]: BadgeTone.ACCENT,
  [MinerKind.BADGES]: BadgeTone.INFO,
  [MinerKind.POINTS]: BadgeTone.SUCCESS,
};

export const MINER_ICONS: Record<MinerKind, IconName> = {
  [MinerKind.DROPS]: IconName.GIFT,
  [MinerKind.BADGES]: IconName.AWARD,
  [MinerKind.POINTS]: IconName.CHANNEL_POINTS,
};

export const JOURNAL_ALL = "ALL";

export type JournalFilterType = EventCategory | typeof JOURNAL_ALL;

export const JOURNAL_FILTER_SLUGS: Record<JournalFilterType, string> = {
  [JOURNAL_ALL]: "all",
  [EventCategory.DROPS]: "drops",
  [EventCategory.BADGES]: "badges",
  [EventCategory.POINTS]: "points",
  [EventCategory.OTHER]: "other",
};

export enum KindGroup {
  DROPS = "DROPS",
  POINTS = "POINTS",
  WATCH = "WATCH",
  OTHER = "OTHER",
}

export const KIND_GROUPS: { group: KindGroup; kinds: EventKind[]; categories: EventCategory[] }[] =
  [
    {
      group: KindGroup.DROPS,
      kinds: [EventKind.DROP_PROGRESS, EventKind.DROP_CLAIMED, EventKind.CAMPAIGN_FINISHED],
      categories: [EventCategory.DROPS, EventCategory.BADGES],
    },
    {
      group: KindGroup.POINTS,
      kinds: [
        EventKind.POINTS_EARNED,
        EventKind.POINTS_SPENT,
        EventKind.REWARD_REDEEMED,
        EventKind.BONUS_CLAIMED,
        EventKind.PREDICTION_PLACED,
        EventKind.PREDICTION_RESULT,
        EventKind.RAID_JOINED,
        EventKind.MOMENT_CLAIMED,
        EventKind.STREAK_RECOVERED,
        EventKind.GOAL_CONTRIBUTED,
        EventKind.STREAM_ONLINE,
        EventKind.STREAM_OFFLINE,
        EventKind.CHAT_MENTION,
      ],
      categories: [EventCategory.POINTS],
    },
    {
      group: KindGroup.WATCH,
      kinds: [EventKind.WATCH_STARTED, EventKind.WATCH_STOPPED],
      categories: [EventCategory.DROPS, EventCategory.BADGES, EventCategory.POINTS],
    },
    {
      group: KindGroup.OTHER,
      kinds: [
        EventKind.INFO,
        EventKind.ERROR,
        EventKind.SESSION_EXPIRED,
        EventKind.LOGIN_COMPLETED,
        EventKind.UPDATE_AVAILABLE,
      ],
      categories: [EventCategory.OTHER],
    },
  ];

export const POINTS_ROUTINE: ReadonlySet<EventKind> = new Set([
  EventKind.POINTS_EARNED,
  EventKind.POINTS_SPENT,
]);
