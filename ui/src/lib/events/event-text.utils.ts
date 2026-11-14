import { BetResult } from "../bindings/BetResult";
import { EventKind } from "../bindings/EventKind";
import type { MinerEventType } from "../bindings/MinerEventType";
import { PointsReason } from "../bindings/PointsReason";
import { onDropsPage } from "../catalogue/campaign.utils";
import { catalogueCampaign } from "../catalogue/catalogue.svelte";
import { number } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { campaignUrl, channelUrl } from "../links/twitch.utils";
import { escapeEmphasis, linkMarker, strong, type TextLinkType } from "../text.utils";

const WATCH_POINTS_TEXTS: Record<PointsReason, (args: { channel: string }) => string> = {
  [PointsReason.STREAK_BONUS]: (args) => t.ev_watch_points_streak_bonus(args),
  [PointsReason.STREAK_AT_RISK]: (args) => t.ev_watch_points_streak_at_risk(args),
  [PointsReason.SUBSCRIBED]: (args) => t.ev_watch_points_subscribed(args),
  [PointsReason.LIST_ORDER]: (args) => t.ev_watch_points_list_order(args),
};

const dropsPageUrl = (campaignId: string | undefined): string | null => {
  const known = campaignId === undefined ? undefined : catalogueCampaign(campaignId);

  return known && onDropsPage(known) ? campaignUrl(known.id) : null;
};

export type EventTextType = { text: string; links: TextLinkType[] };

export const eventText = (event: MinerEventType, linked: boolean): EventTextType => {
  const context = event.context;
  const links: TextLinkType[] = [];

  const named = (label: string | undefined, url: string | null): string | undefined => {
    if (label === undefined) {
      return undefined;
    }

    if (linked && url) {
      links.push({ label, url });

      return linkMarker(links.length - 1);
    }

    return strong(label);
  };

  const bold = (value: string | number | undefined): string | undefined =>
    value === undefined ? undefined : strong(typeof value === "number" ? number(value) : value);

  const campaignLink = dropsPageUrl(context.campaign_id);
  const channel = named(
    context.channel_name ?? context.channel,
    context.channel ? channelUrl(context.channel) : null,
  );
  const drop = named(context.drop_name, campaignLink);
  const campaign = drop ? bold(context.campaign_name) : named(context.campaign_name, campaignLink);
  const fallback = escapeEmphasis(event.title);

  const text = ((): string => {
    switch (event.kind) {
      case EventKind.WATCH_STARTED:
        if (channel && campaign) {
          return t.ev_watch_drop({ channel, campaign });
        }

        return channel && context.watch_reason
          ? WATCH_POINTS_TEXTS[context.watch_reason]({ channel })
          : fallback;
      case EventKind.WATCH_STOPPED:
        return channel ? t.ev_watch_stopped({ channel }) : fallback;
      case EventKind.DROP_CLAIMED:
        return drop && campaign ? t.ev_drop_claimed({ drop, campaign }) : fallback;
      case EventKind.CAMPAIGN_FINISHED:
        return campaign ? t.ev_campaign_finished({ campaign }) : fallback;
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
        if (context.badge && drop) {
          return t.ev_badge_equip_failed({ badge: strong(context.badge), drop });
        }

        return drop && context.drop_id && context.status
          ? t.ev_claim_refused({ drop, status: strong(context.status) })
          : fallback;
      case EventKind.BONUS_CLAIMED:
        return channel ? t.ev_bonus({ channel }) : fallback;
      case EventKind.POINTS_EARNED:
        return channel && context.earned !== undefined
          ? t.ev_points({ channel, count: context.earned, points: bold(context.earned) ?? "" })
          : fallback;
      case EventKind.POINTS_SPENT:
        if (channel && context.spent !== undefined) {
          return t.ev_points_spent({
            channel,
            count: context.spent,
            points: bold(context.spent) ?? "",
          });
        }

        return channel && context.balance !== undefined
          ? t.ev_points_spent_balance({ channel, balance: bold(context.balance) ?? "" })
          : fallback;
      case EventKind.REWARD_REDEEMED: {
        if (!channel || !context.reward_title || context.count === undefined) {
          return fallback;
        }

        const sent = context.sent ?? 0;
        const parts = {
          count: bold(context.count) ?? "",
          reward: strong(context.reward_title),
          channel,
          points: bold(context.spent ?? 0) ?? "",
        };

        return sent === context.count
          ? t.ev_rewards_redeemed({ ...parts, sent, shown: number(sent) })
          : t.ev_rewards_redeemed_partial({
              ...parts,
              sent: number(sent),
              total: number(context.count),
            });
      }
      case EventKind.STREAM_ONLINE:
        return channel ? t.ev_online({ channel }) : fallback;
      case EventKind.STREAM_OFFLINE:
        return channel ? t.ev_offline({ channel }) : fallback;
      case EventKind.RAID_JOINED:
        return channel ? t.ev_raid({ channel }) : fallback;
      case EventKind.MOMENT_CLAIMED:
        return channel ? t.ev_moment({ channel }) : fallback;
      case EventKind.PREDICTION_PLACED:
        return channel && context.points !== undefined
          ? t.ev_bet_placed({
              channel,
              count: context.points,
              points: bold(context.points) ?? "",
              outcome: bold(context.outcome) ?? "",
            })
          : fallback;
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
      case EventKind.STREAK_RECOVERED:
        return channel ? t.ev_streak_recovered({ channel }) : fallback;
      case EventKind.GOAL_CONTRIBUTED:
        return channel && context.points !== undefined
          ? t.ev_goal_contributed({
              count: context.points,
              points: bold(context.points) ?? "",
              goal: bold(context.goal) ?? "",
              channel,
            })
          : fallback;
      case EventKind.CHAT_MENTION:
        return channel && context.chatter
          ? t.ev_chat_mention({ chatter: strong(context.chatter), channel })
          : fallback;
      case EventKind.INFO:
        return context.badge && drop
          ? t.ev_badge_equipped({ badge: strong(context.badge), drop })
          : fallback;
    }
  })();

  return { text, links };
};
