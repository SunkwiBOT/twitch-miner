import { BetResult } from "../bindings/BetResult";
import { EventKind } from "../bindings/EventKind";
import type { MinerEventType } from "../bindings/MinerEventType";
import { PointsReason } from "../bindings/PointsReason";
import { PauseReason } from "../bindings/PauseReason";
import { QueueBlock } from "../bindings/QueueBlock";
import { badgeList } from "../badges/badges.svelte";
import { badgeKey } from "../badges/badges.utils";
import { campaignOf, catalogueCampaign } from "../catalogue/catalogue.svelte";
import { POINTS_EARNED_REASON_TEXT, reasonLabel } from "./event-reasons.utils";
import { number } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { channelUrl } from "../links/twitch.utils";
import { account } from "../state/app.svelte";
import { openBadgeSheet, openCampaignSheet } from "../state/sheets";
import { relativeTime } from "../time/time.utils";
import {
  escapeEmphasis,
  linkMarker,
  strong,
  type LinkTargetType,
  type TextLinkType,
} from "../text.utils";

type ShortLongTextType = [short: () => string, long: (args: { channel: string }) => string];

const WATCH_POINTS_TEXTS: Record<PointsReason, ShortLongTextType> = {
  [PointsReason.STREAK_BONUS]: [
    t.ev_watch_points_streak_bonus_short,
    t.ev_watch_points_streak_bonus,
  ],
  [PointsReason.STREAK_AT_RISK]: [
    t.ev_watch_points_streak_at_risk_short,
    t.ev_watch_points_streak_at_risk,
  ],
  [PointsReason.SUBSCRIBED]: [t.ev_watch_points_subscribed_short, t.ev_watch_points_subscribed],
  [PointsReason.LIST_ORDER]: [t.ev_watch_points_list_order_short, t.ev_watch_points_list_order],
};

type EventLinkTargetType = LinkTargetType | null;

const campaignSheet = (event: MinerEventType): EventLinkTargetType => {
  const id = event.context.campaign_id;
  const owner = account(event.account_id);
  const campaign =
    id === undefined ? undefined : owner ? campaignOf(owner, id) : catalogueCampaign(id);

  return campaign ? { open: () => openCampaignSheet(campaign, owner?.id ?? null) } : null;
};

const badgeSheet = (event: MinerEventType): EventLinkTargetType => {
  const key = event.context.badge_key;
  const badge = badgeList.badges.find((item) => badgeKey(item.set_id, item.version_id) === key);
  const accountId = account(event.account_id)?.id ?? null;

  return badge ? { open: () => openBadgeSheet(badge, accountId) } : null;
};

export type EventTextType = { text: string; links: TextLinkType[] };

export type EventTextOptionsType = {
  channelShown: boolean;
  linked: boolean;
};

export const eventText = (
  event: MinerEventType,
  { channelShown, linked }: EventTextOptionsType,
): EventTextType => {
  const context = event.context;
  const links: TextLinkType[] = [];

  const named = (label: string | undefined, target: EventLinkTargetType): string | undefined => {
    if (label === undefined) {
      return undefined;
    }

    if (linked && target) {
      links.push({ label, ...target });

      return linkMarker(links.length - 1);
    }

    return strong(label);
  };

  const bold = (value: string | number | undefined): string | undefined =>
    value === undefined ? undefined : strong(typeof value === "number" ? number(value) : value);

  const campaignTarget = linked ? campaignSheet(event) : null;
  const channel = named(
    context.channel_name ?? context.channel,
    context.channel ? { url: channelUrl(context.channel) } : null,
  );
  const drop = named(context.drop_name, campaignTarget);
  const campaign = drop
    ? bold(context.campaign_name)
    : named(context.campaign_name, campaignTarget);
  const badge = named(context.badge, linked ? badgeSheet(event) : null);
  const fallback = escapeEmphasis(event.title);

  const channelOnly = ([short, long]: ShortLongTextType): string =>
    channel ? (channelShown ? short() : long({ channel })) : fallback;

  const pick = <A extends object>(
    [short, long]: [(args: A) => string, (args: A & { channel: string }) => string],
    args: A,
  ): string => (channel ? (channelShown ? short(args) : long({ ...args, channel })) : fallback);

  const text = ((): string => {
    switch (event.kind) {
      case EventKind.WATCH_STARTED:
        if (campaign) {
          return pick([t.ev_watch_drop_short, t.ev_watch_drop], { campaign });
        }

        if (!context.watch_reason) {
          return fallback;
        }

        return channelOnly(WATCH_POINTS_TEXTS[context.watch_reason]);
      case EventKind.WATCH_STOPPED:
        return channelOnly([t.ev_watch_stopped_short, t.ev_watch_stopped]);
      case EventKind.DROP_CLAIMED:
        return drop && campaign ? t.ev_drop_claimed({ drop, campaign }) : fallback;
      case EventKind.CAMPAIGN_FINISHED:
        return campaign ? t.ev_campaign_finished({ campaign }) : fallback;
      case EventKind.CAMPAIGN_SET_ASIDE:
        return campaign && context.set_aside_until && context.required !== undefined
          ? t.ev_campaign_set_aside({
              campaign,
              minutes: bold(context.required) ?? "",
              retryTime: strong(
                relativeTime(context.set_aside_until, new Date(event.at).getTime()),
              ),
            })
          : fallback;
      case EventKind.CAMPAIGN_RETRIED:
        return campaign ? t.ev_campaign_retried({ campaign }) : fallback;
      case EventKind.DROP_PROGRESS:
        if (context.current === undefined || context.required === undefined) {
          return fallback;
        }

        return drop
          ? t.ev_drop_progress({
              drop,
              current: bold(context.current) ?? "",
              required: bold(context.required) ?? "",
            })
          : t.ev_drop_progress_unknown({
              current: bold(context.current) ?? "",
              required: bold(context.required) ?? "",
            });
      case EventKind.ERROR:
        if (context.lane && drop) {
          return t.ev_lane_equip_failed({ drop });
        }

        if (badge && drop) {
          return t.ev_badge_equip_failed({ badge, drop });
        }

        return drop && context.drop_id && context.status
          ? t.ev_claim_refused({ drop, status: strong(context.status) })
          : fallback;
      case EventKind.BONUS_CLAIMED:
        return channelOnly([t.ev_bonus_short, t.ev_bonus]);
      case EventKind.POINTS_EARNED: {
        if (context.earned === undefined) {
          return fallback;
        }

        const reason = context.reason;

        if (reason !== undefined && POINTS_EARNED_REASON_TEXT.has(reason)) {
          return pick([t.ev_points_reason_short, t.ev_points_reason], {
            count: context.earned,
            points: bold(context.earned) ?? "",
            reason: reasonLabel(reason),
          });
        }

        return pick([t.ev_points_short, t.ev_points], {
          count: context.earned,
          points: bold(context.earned) ?? "",
        });
      }
      case EventKind.POINTS_SPENT: {
        if (context.spent !== undefined) {
          return pick([t.ev_points_spent_short, t.ev_points_spent], {
            count: context.spent,
            points: bold(context.spent) ?? "",
          });
        }

        if (context.balance === undefined) {
          return fallback;
        }

        return pick([t.ev_points_spent_balance_short, t.ev_points_spent_balance], {
          balance: bold(context.balance) ?? "",
        });
      }
      case EventKind.REWARD_REDEEMED: {
        if (!context.reward_title || context.count === undefined) {
          return fallback;
        }

        const sent = context.sent ?? 0;
        const parts = {
          rewardCount: bold(context.count) ?? "",
          reward: strong(context.reward_title),
          points: bold(context.spent ?? 0) ?? "",
        };

        return sent === context.count
          ? pick([t.ev_rewards_redeemed_short, t.ev_rewards_redeemed], {
              ...parts,
              sent,
              formattedSent: bold(sent) ?? "",
            })
          : pick([t.ev_rewards_redeemed_partial_short, t.ev_rewards_redeemed_partial], {
              ...parts,
              sent: bold(sent) ?? "",
              total: bold(context.count) ?? "",
            });
      }
      case EventKind.STREAM_ONLINE:
        return channelOnly([t.ev_online_short, t.ev_online]);
      case EventKind.STREAM_OFFLINE:
        return channelOnly([t.ev_offline_short, t.ev_offline]);
      case EventKind.RAID_JOINED:
        return channelOnly([t.ev_raid_short, t.ev_raid]);
      case EventKind.MOMENT_CLAIMED:
        return channelOnly([t.ev_moment_short, t.ev_moment]);
      case EventKind.PREDICTION_PLACED: {
        if (context.points === undefined) {
          return fallback;
        }

        return pick([t.ev_bet_placed_short, t.ev_bet_placed], {
          count: context.points,
          points: bold(context.points) ?? "",
          outcome: bold(context.outcome) ?? "",
        });
      }
      case EventKind.PREDICTION_RESULT: {
        if (context.bet_result === BetResult.WIN) {
          const won = context.points_won ?? 0;

          return t.ev_bet_won({ count: won, points: bold(won) ?? "" });
        }

        if (context.bet_result === BetResult.LOSE) {
          const lost = context.staked ?? 0;

          return t.ev_bet_lost({ count: lost, points: bold(lost) ?? "" });
        }

        return context.bet_result === BetResult.REFUND ? t.ev_bet_refund() : fallback;
      }
      case EventKind.LOGIN_COMPLETED:
        return t.ev_login({ login: strong(context.account_name ?? event.account_login) });
      case EventKind.SESSION_EXPIRED:
        return t.ev_session_expired();
      case EventKind.UPDATE_AVAILABLE:
        return context.version
          ? t.ev_update_available({ version: strong(context.version) })
          : fallback;
      case EventKind.VERSION_UPDATED:
        return context.version && context.previous_version
          ? t.ev_version_updated({
              version: strong(context.version),
              previous: strong(context.previous_version),
            })
          : fallback;
      case EventKind.STREAK_RECOVERED:
        return channelOnly([t.ev_streak_recovered_short, t.ev_streak_recovered]);
      case EventKind.GOAL_CONTRIBUTED: {
        if (context.points === undefined) {
          return fallback;
        }

        return pick([t.ev_goal_contributed_short, t.ev_goal_contributed], {
          count: context.points,
          points: bold(context.points) ?? "",
          goal: bold(context.goal) ?? "",
        });
      }
      case EventKind.CHAT_MENTION:
        if (!context.chatter) {
          return fallback;
        }

        return pick([t.ev_chat_mention_short, t.ev_chat_mention], {
          chatter: strong(context.chatter),
        });
      case EventKind.INFO:
        if (context.lane && drop) {
          return t.ev_lane_equipped({ drop });
        }

        if (badge && drop) {
          return t.ev_badge_equipped({ badge, drop });
        }

        if (campaign && context.queue_block === QueueBlock.NO_LIVE_CHANNEL) {
          return t.ev_campaign_no_live_channel({ campaign });
        }

        return campaign && context.queue_block === QueueBlock.ONE_CAMPAIGN_AT_A_TIME
          ? t.ev_campaign_waiting({ campaign })
          : fallback;
      case EventKind.PAUSED:
        if (context.pause_reason === PauseReason.QUIET_WINDOW) {
          return t.ev_paused_quiet_window();
        }

        if (context.pause_until) {
          return t.ev_paused_scheduled({
            resumeTime: strong(relativeTime(context.pause_until, new Date(event.at).getTime())),
          });
        }

        return t.status_paused();
      case EventKind.RESUMED:
        return t.ev_resumed();
    }
  })();

  return { text, links };
};
