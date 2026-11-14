import { EventCategory } from "../bindings/EventCategory";
import { EventKind } from "../bindings/EventKind";
import { MinerKind } from "../bindings/MinerKind";
import { t } from "../i18n/i18n.svelte";
import { IconName } from "../icons.constants";
import { JOURNAL_ALL, KindGroup, type JournalFilterType } from "./events.constants";

export const EVENT_KIND_LABELS: Record<EventKind, () => string> = {
  [EventKind.INFO]: () => t.event_kind_info(),
  [EventKind.ERROR]: () => t.event_kind_error(),
  [EventKind.WATCH_STARTED]: () => t.event_kind_watch_started(),
  [EventKind.WATCH_STOPPED]: () => t.event_kind_watch_stopped(),
  [EventKind.DROP_PROGRESS]: () => t.event_kind_drop_progress(),
  [EventKind.DROP_CLAIMED]: () => t.event_kind_drop_claimed(),
  [EventKind.CAMPAIGN_FINISHED]: () => t.event_kind_campaign_finished(),
  [EventKind.POINTS_EARNED]: () => t.event_kind_points_earned(),
  [EventKind.POINTS_SPENT]: () => t.event_kind_points_spent(),
  [EventKind.REWARD_REDEEMED]: () => t.event_kind_reward_redeemed(),
  [EventKind.BONUS_CLAIMED]: () => t.event_kind_bonus_claimed(),
  [EventKind.PREDICTION_PLACED]: () => t.event_kind_prediction_placed(),
  [EventKind.PREDICTION_RESULT]: () => t.event_kind_prediction_result(),
  [EventKind.STREAM_ONLINE]: () => t.event_kind_stream_online(),
  [EventKind.STREAM_OFFLINE]: () => t.event_kind_stream_offline(),
  [EventKind.RAID_JOINED]: () => t.event_kind_raid_joined(),
  [EventKind.MOMENT_CLAIMED]: () => t.event_kind_moment_claimed(),
  [EventKind.STREAK_RECOVERED]: () => t.event_kind_streak_recovered(),
  [EventKind.GOAL_CONTRIBUTED]: () => t.event_kind_goal_contributed(),
  [EventKind.CHAT_MENTION]: () => t.event_kind_chat_mention(),
  [EventKind.SESSION_EXPIRED]: () => t.event_kind_session_expired(),
  [EventKind.LOGIN_COMPLETED]: () => t.event_kind_login_completed(),
  [EventKind.UPDATE_AVAILABLE]: () => t.event_kind_update_available(),
};

export const MINER_LABELS: Record<MinerKind, () => string> = {
  [MinerKind.DROPS]: () => t.miner_drops(),
  [MinerKind.BADGES]: () => t.miner_badges(),
  [MinerKind.POINTS]: () => t.miner_points(),
};

export const MINING_BADGE_TEXTS: Record<MinerKind, () => string> = {
  [MinerKind.DROPS]: () => t.badge_mining_drops(),
  [MinerKind.BADGES]: () => t.badge_mining_badges(),
  [MinerKind.POINTS]: () => t.badge_mining_points(),
};

export const MINING_BADGE_HINTS: Record<MinerKind, () => string> = {
  [MinerKind.DROPS]: () => t.badge_hint_mining_drops(),
  [MinerKind.BADGES]: () => t.badge_hint_mining_badges(),
  [MinerKind.POINTS]: () => t.badge_hint_mining_points(),
};

export const JOURNAL_FILTERS: {
  value: JournalFilterType;
  label: () => string;
  icon?: IconName;
}[] = [
  { value: JOURNAL_ALL, label: () => t.journal_filter_all() },
  { value: EventCategory.DROPS, label: () => t.journal_filter_drops(), icon: IconName.GIFT },
  { value: EventCategory.BADGES, label: () => t.journal_filter_badges(), icon: IconName.AWARD },
  {
    value: EventCategory.POINTS,
    label: () => t.journal_filter_points(),
    icon: IconName.CHANNEL_POINTS,
  },
  { value: EventCategory.OTHER, label: () => t.journal_filter_other() },
];

export const KIND_GROUP_LABELS: Record<KindGroup, () => string> = {
  [KindGroup.DROPS]: () => t.journal_filter_drops(),
  [KindGroup.POINTS]: () => t.journal_filter_points(),
  [KindGroup.WATCH]: () => t.journal_group_watch(),
  [KindGroup.OTHER]: () => t.journal_filter_other(),
};
