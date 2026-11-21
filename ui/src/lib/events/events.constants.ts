import { EventCategory } from "../bindings/EventCategory";
import { EventKind } from "../bindings/EventKind";
import { MinerKind } from "../bindings/MinerKind";
import { IconName } from "../icons.constants";
import { BadgeTone } from "../tones.constants";

// ! The Twitch `points-earned` reason codes the app treats specially
// (a dedicated label, icon, tone, or bucket): kept to `string` on the wire
// since `context.reason` is an open external set, but each of these codes
// is written once here and read through `isTwitchPointsReason`.
export enum TwitchPointsReason {
  WATCH = "WATCH",
  WATCH_STREAK = "WATCH_STREAK",
  CLAIM = "CLAIM",
  RAID = "RAID",
  COMMUNITY_MOMENT_COMPLETE = "COMMUNITY_MOMENT_COMPLETE",
  PREDICTION = "PREDICTION",
  REFUND = "REFUND",
}

const TWITCH_POINTS_REASONS: ReadonlySet<string> = new Set(Object.values(TwitchPointsReason));

export const isTwitchPointsReason = (value: string): value is TwitchPointsReason =>
  TWITCH_POINTS_REASONS.has(value);

export const EVENT_TONES: Record<EventKind, BadgeTone> = {
  [EventKind.INFO]: BadgeTone.NEUTRAL,
  [EventKind.ERROR]: BadgeTone.DANGER,
  [EventKind.WATCH_STARTED]: BadgeTone.NEUTRAL,
  [EventKind.WATCH_STOPPED]: BadgeTone.NEUTRAL,
  [EventKind.DROP_PROGRESS]: BadgeTone.NEUTRAL,
  [EventKind.DROP_CLAIMED]: BadgeTone.ACCENT,
  [EventKind.CAMPAIGN_FINISHED]: BadgeTone.ACCENT,
  [EventKind.CAMPAIGN_SET_ASIDE]: BadgeTone.WARNING,
  [EventKind.CAMPAIGN_RETRIED]: BadgeTone.NEUTRAL,
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
  [EventKind.VERSION_UPDATED]: BadgeTone.INFO,
  [EventKind.PAUSED]: BadgeTone.MUTED,
  [EventKind.RESUMED]: BadgeTone.SUCCESS,
};

export const EVENT_ICONS: Record<EventKind, IconName> = {
  [EventKind.INFO]: IconName.BELL,
  [EventKind.ERROR]: IconName.ALERT,
  [EventKind.WATCH_STARTED]: IconName.PLAY,
  [EventKind.WATCH_STOPPED]: IconName.PAUSE,
  [EventKind.DROP_PROGRESS]: IconName.CLOCK,
  [EventKind.DROP_CLAIMED]: IconName.GIFT,
  [EventKind.CAMPAIGN_FINISHED]: IconName.CHECK,
  [EventKind.CAMPAIGN_SET_ASIDE]: IconName.CLOCK,
  [EventKind.CAMPAIGN_RETRIED]: IconName.REFRESH,
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
  [EventKind.VERSION_UPDATED]: IconName.CHECK,
  [EventKind.PAUSED]: IconName.PAUSE,
  [EventKind.RESUMED]: IconName.PLAY,
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
  GENERIC = "GENERIC",
  OTHER = "OTHER",
}

export const KIND_GROUPS: { group: KindGroup; kinds: EventKind[]; categories: EventCategory[] }[] =
  [
    {
      group: KindGroup.DROPS,
      kinds: [
        EventKind.DROP_PROGRESS,
        EventKind.DROP_CLAIMED,
        EventKind.CAMPAIGN_FINISHED,
        EventKind.CAMPAIGN_SET_ASIDE,
        EventKind.CAMPAIGN_RETRIED,
      ],
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
      group: KindGroup.GENERIC,
      kinds: [EventKind.INFO, EventKind.ERROR],
      categories: [
        EventCategory.DROPS,
        EventCategory.BADGES,
        EventCategory.POINTS,
        EventCategory.OTHER,
      ],
    },
    {
      group: KindGroup.OTHER,
      kinds: [
        EventKind.SESSION_EXPIRED,
        EventKind.LOGIN_COMPLETED,
        EventKind.UPDATE_AVAILABLE,
        EventKind.VERSION_UPDATED,
        EventKind.PAUSED,
        EventKind.RESUMED,
      ],
      categories: [EventCategory.OTHER],
    },
  ];

export const POINTS_ROUTINE: ReadonlySet<EventKind> = new Set([
  EventKind.POINTS_EARNED,
  EventKind.POINTS_SPENT,
]);
