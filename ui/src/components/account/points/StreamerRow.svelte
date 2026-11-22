<script lang="ts">
  import { POINTS_REASON_ICONS, POINTS_REASON_TONES } from "../../../lib/accounts/accounts.utils";
  import type { ChannelType } from "../../../lib/bindings/ChannelType";
  import { PointsReason } from "../../../lib/bindings/PointsReason";
  import type { StreamerSettingsType } from "../../../lib/bindings/StreamerSettingsType";
  import type { StreamerViewType } from "../../../lib/bindings/StreamerViewType";
  import { gameArt, gameUrl } from "../../../lib/catalogue/games.utils";
  import { isLive } from "../../../lib/channels/channels.svelte";
  import { channelUrl } from "../../../lib/links/twitch.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { ticker } from "../../../lib/state/ticker.svelte";
  import { strong } from "../../../lib/text.utils";
  import { relativeTime } from "../../../lib/time/time.utils";
  import Badge from "../../ui/Badge.svelte";
  import ChannelAvatar from "../../ui/ChannelAvatar.svelte";
  import LinkPair from "../../ui/LinkPair.svelte";
  import LinkTwin from "../../ui/LinkTwin.svelte";
  import PointsBalance from "../../ui/PointsBalance.svelte";
  import RichText from "../../ui/RichText.svelte";
  import TextLink from "../../ui/TextLink.svelte";
  import GameName from "../../ui/GameName.svelte";
  import ViewerCount from "../../ui/ViewerCount.svelte";
  import { AvatarSize, ThumbSize } from "../../ui/picture.constants";
  import { BadgeTone } from "../../../lib/tones.constants";
  import { BalanceAlign } from "../../ui/ui.constants";
  import { IconName } from "../../../lib/icons.constants";
  import { LinkMark, LinkSize } from "../../ui/link.constants";

  type StreamerRowPropsType = {
    streamer: StreamerSettingsType;
    channel: ChannelType | undefined;
    view: StreamerViewType | undefined;
  };

  let { streamer, channel, view }: StreamerRowPropsType = $props();
</script>

{#snippet balance(align: BalanceAlign)}
  {#if view && view.points.balance > 0}
    <PointsBalance
      balance={view.points.balance}
      earned={view.points.earned_session}
      streak={view.points.streak?.count}
      {align}
    >
      {#if view.points.last_bonus_at}<RichText
          text={t.streamer_last_bonus({
            lastBonusTime: strong(relativeTime(view.points.last_bonus_at, ticker.now)),
          })}
        />{/if}
    </PointsBalance>
  {/if}
{/snippet}

<div class="@container/streamer flex min-w-0 flex-1">
  <div
    class={["flex min-w-0 flex-1 items-center gap-3 transition", !streamer.enabled && "grayscale"]}
  >
    <LinkPair>
      <span class={["inline-flex transition", !streamer.enabled && "opacity-50"]}>
        <LinkTwin target={{ url: channelUrl(streamer.login) }} live={isLive(streamer.login)}>
          <ChannelAvatar
            url={channel?.avatar_url}
            name={channel?.display_name ?? streamer.login}
            size={AvatarSize.MD}
            live={isLive(streamer.login)}
          />
        </LinkTwin>
      </span>
      <div class="flex min-w-0 flex-1 flex-col gap-0.5">
        <span class="flex min-w-0 flex-wrap items-center gap-1.5">
          <TextLink
            text={channel?.display_name ?? streamer.login}
            target={{ url: channelUrl(streamer.login), mark: LinkMark.END }}
            size={LinkSize.MD}
          />
          {#if view?.watching}<Badge
              text={t.streamer_watching()}
              tone={BadgeTone.SUCCESS}
              icon={IconName.PLAY}
              hint={t.badge_hint_watching()}
            />{/if}
          {#if view?.streak_at_risk}<Badge
              text={t.points_reason_streak_at_risk()}
              tone={POINTS_REASON_TONES[PointsReason.STREAK_AT_RISK]}
              hint={t.badge_hint_streak_risk()}
              icon={POINTS_REASON_ICONS[PointsReason.STREAK_AT_RISK]}
            />{/if}
        </span>
        <span
          class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-0.5 text-xs text-muted-foreground"
        >
          {#if channel?.live}
            {#if channel.game}<GameName
                name={channel.game.name}
                url={gameUrl(channel.game.name, channel.game.slug)}
                art={gameArt(channel.game.name, channel.game.box_art_url)}
                size={ThumbSize.XS}
              />{/if}
            {#if channel.viewers !== undefined}<ViewerCount count={channel.viewers} />{/if}
          {:else if !channel}
            <span class="truncate">{t.points_offline_hint()}</span>
          {/if}
        </span>
        <!-- ! In a narrow row the balance goes under the name, which keeps the whole width. -->
        <div class="@min-[36rem]/streamer:hidden">{@render balance(BalanceAlign.START)}</div>
      </div>
      <div class="@max-[36rem]/streamer:hidden">{@render balance(BalanceAlign.END)}</div>
    </LinkPair>
  </div>
</div>
