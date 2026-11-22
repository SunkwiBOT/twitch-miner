<script lang="ts">
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
  import PointsReasonBadge from "../../ui/PointsReasonBadge.svelte";
  import RichText from "../../ui/RichText.svelte";
  import TextLink from "../../ui/TextLink.svelte";
  import LinkText from "../../ui/LinkText.svelte";
  import Tip from "../../ui/Tip.svelte";
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
    beside: boolean;
    onmeasure: (login: string, fits: boolean | null) => void;
  };

  let { streamer, channel, view, beside, onmeasure }: StreamerRowPropsType = $props();

  let rowWidth = $state(0);
  let balanceWidth = $state(0);
  let nameWidth = $state(0);
  let nameRoom = $state(0);
  let content = $state<HTMLElement>();
  let column = $state<HTMLElement>();
  let nameLine = $state<HTMLElement>();
  let metaLine = $state<HTMLElement>();

  const name = $derived(channel?.display_name ?? streamer.login);
  const cut = $derived(nameWidth > nameRoom);

  // ! Synchronous: the line goes back to its width before the browser paints.
  const naturalWidth = (line: HTMLElement): number => {
    const width = line.style.width;

    line.style.width = "max-content";

    const natural = line.offsetWidth;

    line.style.width = width;

    return natural;
  };

  // ! The list picks one placement for every row: each row reports whether its lines at their natural width and its balance fit side by side.
  $effect(() => {
    const login = streamer.login;

    void [view, channel, name];

    if (!content || !column || !nameLine || !metaLine) {
      return;
    }

    const lines = Math.max(naturalWidth(nameLine), naturalWidth(metaLine));
    const gap = parseFloat(getComputedStyle(content).columnGap);
    const need = column.offsetLeft + lines + (balanceWidth > 0 ? gap + balanceWidth : 0);

    onmeasure(login, need <= rowWidth);

    return () => onmeasure(login, null);
  });
</script>

{#snippet nameLink()}
  <TextLink
    text={name}
    target={{ url: channelUrl(streamer.login), mark: LinkMark.END }}
    size={LinkSize.MD}
  />
{/snippet}

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

<div class="relative flex min-w-0 flex-1" bind:clientWidth={rowWidth}>
  <div class="invisible absolute flex w-max flex-col items-start" aria-hidden="true" inert>
    <span class="inline-flex items-center gap-1.5" bind:offsetWidth={nameWidth}>
      <LinkText text={name} mark={LinkMark.END} size={LinkSize.MD} fit={false} />
    </span>
    <div bind:offsetWidth={balanceWidth}>{@render balance(BalanceAlign.END)}</div>
  </div>
  <div
    class={["flex min-w-0 flex-1 items-center gap-3 transition", !streamer.enabled && "grayscale"]}
    bind:this={content}
  >
    <LinkPair>
      <span class={["inline-flex transition", !streamer.enabled && "opacity-50"]}>
        <LinkTwin target={{ url: channelUrl(streamer.login) }} live={isLive(streamer.login)}>
          <ChannelAvatar
            url={channel?.avatar_url}
            {name}
            size={AvatarSize.MD}
            live={isLive(streamer.login)}
          />
        </LinkTwin>
      </span>
      <div class="flex min-w-0 flex-1 flex-col gap-0.5" bind:this={column}>
        <span
          class="flex min-w-0 flex-wrap items-center gap-1.5"
          bind:clientWidth={nameRoom}
          bind:this={nameLine}
        >
          {#if cut}
            <Tip text={name}>
              {#snippet children(props)}
                <span {...props} tabindex="-1" class="inline-flex max-w-full min-w-0">
                  {@render nameLink()}
                </span>
              {/snippet}
            </Tip>
          {:else}
            {@render nameLink()}
          {/if}
          {#if view?.watching}<Badge
              text={t.streamer_watching()}
              tone={BadgeTone.SUCCESS}
              icon={IconName.PLAY}
              hint={t.badge_hint_watching()}
            />{/if}
          {#if view?.streak_at_risk}<PointsReasonBadge reason={PointsReason.STREAK_AT_RISK} />{/if}
        </span>
        <span
          class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-0.5 text-xs text-muted-foreground"
          bind:this={metaLine}
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
        {#if !beside}{@render balance(BalanceAlign.START)}{/if}
      </div>
      {#if beside}{@render balance(BalanceAlign.END)}{/if}
    </LinkPair>
  </div>
</div>
