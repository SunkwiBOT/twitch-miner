<script lang="ts">
  import {
    campaignArtUrl,
    campaignHeading,
    campaignSubtitle,
    periodsText,
  } from "../lib/catalogue/campaign.utils";
  import { ACCOUNT_STATE_ICONS, AccountState } from "../lib/accounts/account-state.constants";
  import { userPaused } from "../lib/accounts/account-state.utils";
  import { maxWatchedChannelsParams } from "../lib/accounts/slots.utils";
  import { coreLimits } from "../lib/state/limits";
  import type { PointsReason } from "../lib/bindings/PointsReason";
  import ChannelAvatar from "./ui/ChannelAvatar.svelte";
  import { AccountStatus } from "../lib/bindings/AccountStatus";
  import type { AccountSnapshotType } from "../lib/bindings/AccountSnapshotType";
  import { campaignOf } from "../lib/catalogue/catalogue.svelte";
  import { gameArt, gameUrl } from "../lib/catalogue/games.utils";
  import { Mode } from "../lib/bindings/Mode";
  import { MinerKind } from "../lib/bindings/MinerKind";
  import { SlotPurposeKind } from "../lib/bindings/SlotPurposeKind";
  import { channelUrl } from "../lib/links/twitch.utils";
  import { BoxArtSize } from "../lib/format.constants";
  import { boxArt, number } from "../lib/format.utils";
  import { t } from "../lib/i18n/i18n.svelte";
  import { strong } from "../lib/text.utils";
  import { PAGE_ICONS, Page } from "../lib/state/state.constants";
  import { navigate } from "../lib/state/router";
  import { openCampaignSheet } from "../lib/state/sheets";
  import { ticker } from "../lib/state/ticker.svelte";
  import { formatClock, relativeTime } from "../lib/time/time.utils";
  import TextButton from "./ui/TextButton.svelte";
  import RichText from "./ui/RichText.svelte";
  import TextLink from "./ui/TextLink.svelte";
  import GameName from "./ui/GameName.svelte";
  import LinkPair from "./ui/LinkPair.svelte";
  import LinkTwin from "./ui/LinkTwin.svelte";
  import Icon from "./ui/Icon.svelte";
  import PointsBalance from "./ui/PointsBalance.svelte";
  import MiningBadge from "./ui/MiningBadge.svelte";
  import PointsReasonBadge from "./ui/PointsReasonBadge.svelte";
  import ProgressLine from "./ui/ProgressLine.svelte";
  import Thumb from "./ui/Thumb.svelte";
  import ViewerCount from "./ui/ViewerCount.svelte";
  import NameRun from "./ui/NameRun.svelte";
  import { AvatarSize, ThumbShape, ThumbSize } from "./ui/picture.constants";
  import { BalanceAlign } from "./ui/ui.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "./ui/button.constants";
  import { IconName } from "../lib/icons.constants";
  import { LinkMark, LinkSize } from "./ui/link.constants";
  import type { SlotViewType } from "../lib/bindings/SlotViewType";

  type AccountActivityPropsType = { account: AccountSnapshotType };

  let { account }: AccountActivityPropsType = $props();

  const capacity = $derived(
    Math.max(1, Math.min(coreLimits().max_watched_channels, account.settings.slots)),
  );

  const free = $derived(
    account.status === AccountStatus.PAUSED ? 0 : Math.max(0, capacity - account.slots.length),
  );
</script>

{#snippet watched(slot: SlotViewType)}
  <span class="text-xs text-muted-foreground">
    <span class="inline-block max-w-full">
      <RichText text={t.dashboard_watching_since({ sinceTime: strong(formatClock(slot.since)) })} />
    </span>{#if slot.last_beacon_at}{" · "}<span class="inline-block max-w-full">
        <RichText
          text={t.slot_last_minute({
            lastMinuteTime: strong(relativeTime(slot.last_beacon_at, ticker.now)),
          })}
        />
      </span>{/if}{" · "}<span class="inline-block max-w-full">
      <RichText text={t.slot_segments({ segmentCount: strong(number(slot.segments_requested)) })} />
    </span>
  </span>
{/snippet}

{#snippet points(reason: PointsReason | null)}
  <span class="flex flex-wrap gap-1.5">
    <MiningBadge kind={MinerKind.POINTS} />{#if reason}<PointsReasonBadge {reason} />{/if}
  </span>
{/snippet}

{#snippet streamer(slot: SlotViewType)}
  <span class="flex max-w-[45cqi] shrink-0 flex-col items-end gap-1.5 text-right">
    <LinkPair>
      <LinkTwin target={{ url: channelUrl(slot.channel.login) }} live={slot.channel.live}>
        <ChannelAvatar
          url={slot.channel.avatar_url}
          name={slot.channel.display_name}
          size={AvatarSize.LG}
          live={slot.channel.live}
        />
      </LinkTwin>
      <span class="flex max-w-full flex-col items-end gap-0.5">
        <TextLink
          text={slot.channel.display_name}
          target={{ url: channelUrl(slot.channel.login), mark: LinkMark.START }}
          size={LinkSize.MD}
        />
        {#if slot.channel.viewers !== undefined}<ViewerCount count={slot.channel.viewers} />{/if}
      </span>
    </LinkPair>
  </span>
{/snippet}

{#snippet idle(icon: IconName, text: string)}
  <span class="shrink-0"><Icon name={icon} size={16} /></span>
  <span class="flex-1"><RichText {text} /></span>
{/snippet}

{#each account.slots as slot (slot.channel.id + slot.purpose.kind)}
  {#if slot.drop}
    {@const progress = slot.drop}
    {@const c = campaignOf(account, progress.campaign_id)}
    {@const d = c?.drops.find((x) => x.id === progress.drop_id)}
    {#snippet art()}
      <Thumb
        src={boxArt(c && campaignArtUrl(c), BoxArtSize.ACTIVITY)}
        round={false}
        contain={false}
        size={ThumbSize.XL}
        shape={ThumbShape.BOX_ART}
        fallback={IconName.GIFT}
      />
    {/snippet}
    {#snippet subtitle()}
      {#if !c || campaignSubtitle(c)}<span class="truncate text-hint text-muted-foreground">
          {progress.campaign_name}
        </span>{/if}
    {/snippet}
    {#snippet reward()}
      <Thumb
        src={d?.benefits.at(0)?.image_url ?? progress.image_url}
        round={false}
        contain={true}
        size={ThumbSize.MD}
        shape={ThumbShape.SQUARE}
        fallback={IconName.GIFT}
      />
      <div class="flex min-w-0 flex-1 flex-col gap-1">
        <span class="flex min-w-0 items-baseline gap-2">
          <span class="truncate text-xs font-semibold">{progress.drop_name}</span>
          {#if d?.repeat}<span class="shrink-0 text-hint text-muted-foreground">
              {periodsText(d.periods_done, d.repeat)}
            </span>{/if}
        </span>
        <ProgressLine
          current={progress.current_minutes}
          required={progress.required_minutes}
          stacked={false}
        />
      </div>
    {/snippet}
    <div
      class="@container grid grid-cols-[auto_minmax(0,1fr)_auto] items-start gap-x-3.5 gap-y-1.5 border-b px-5 py-4 last:border-b-0"
    >
      {#if c}
        {@const known = c}
        {@const openSheet = () => openCampaignSheet(known, account.id)}
        <LinkPair>
          <LinkTwin target={{ open: openSheet }} live={false}>{@render art()}</LinkTwin>
          <div class="flex min-w-0 flex-col gap-1.5">
            <span class="flex"><MiningBadge kind={slot.miner} /></span>
            <span class="flex min-w-0 flex-col">
              <TextLink
                text={campaignHeading(known)}
                target={{ open: openSheet }}
                size={LinkSize.MD}
              />
              {@render subtitle()}
            </span>
          </div>
        </LinkPair>
      {:else}
        <span class="shrink-0">{@render art()}</span>
        <div class="flex min-w-0 flex-col gap-1.5">
          <span class="flex"><MiningBadge kind={slot.miner} /></span>
          <span class="flex min-w-0 flex-col">
            <GameName
              name={progress.game}
              url={gameUrl(progress.game, undefined)}
              art={gameArt(progress.game, undefined)}
              size={ThumbSize.XS}
            />
            {@render subtitle()}
          </span>
        </div>
      {/if}
      {@render streamer(slot)}
      <div class="col-span-3 flex min-w-0 flex-col gap-1.5">
        {@render watched(slot)}
        {#if c}
          {@const known = c}
          <button
            class="flex items-center gap-2.5 rounded-lg border bg-muted/50 p-2 text-left transition-colors hover:bg-accent"
            type="button"
            onclick={() => openCampaignSheet(known, account.id)}
          >
            {@render reward()}
          </button>
        {:else}
          <div class="flex items-center gap-2.5 rounded-lg border bg-muted/50 p-2">
            {@render reward()}
          </div>
        {/if}
      </div>
    </div>
  {:else}
    <div
      class="grid grid-cols-[auto_minmax(0,1fr)_auto] items-start gap-x-3.5 gap-y-1.5 border-b px-5 py-4 last:border-b-0"
    >
      <LinkPair>
        <LinkTwin target={{ url: channelUrl(slot.channel.login) }} live={slot.channel.live}>
          <ChannelAvatar
            url={slot.channel.avatar_url}
            name={slot.channel.display_name}
            size={AvatarSize.LG}
            live={slot.channel.live}
          />
        </LinkTwin>
        <div class="flex min-w-0 flex-col gap-1">
          {@render points(
            slot.purpose.kind === SlotPurposeKind.POINTS ? slot.purpose.reason : null,
          )}
          <!-- ! One line, name · category · viewers, wrapping between the parts only when the card has no room. -->
          <NameRun>
            <span
              class="flex min-w-0 flex-wrap items-center gap-x-1.5 gap-y-0.5 text-hint text-muted-foreground"
              data-slot="streamer-meta"
            >
              <TextLink
                text={slot.channel.display_name}
                target={{ url: channelUrl(slot.channel.login), mark: LinkMark.END }}
                size={LinkSize.MD}
              />
              {#if slot.channel.game}<span class="flex max-w-full min-w-0 items-center gap-1.5">
                  <span aria-hidden="true">·</span><GameName
                    name={slot.channel.game.name}
                    url={gameUrl(slot.channel.game.name, slot.channel.game.slug)}
                    art={gameArt(slot.channel.game.name, slot.channel.game.box_art_url)}
                    size={ThumbSize.XS}
                  />
                </span>{/if}
              {#if slot.channel.viewers !== undefined}<span class="flex items-center gap-1.5">
                  <span aria-hidden="true">·</span><ViewerCount count={slot.channel.viewers} />
                </span>{/if}
            </span>
          </NameRun>
        </div>
      </LinkPair>
      {#if slot.points}
        <PointsBalance
          balance={slot.points.balance}
          earned={slot.points.earned_session}
          align={BalanceAlign.END}
        />
      {:else}<span></span>{/if}
      <div class="col-span-3 min-w-0">{@render watched(slot)}</div>
    </div>
  {/if}
{/each}

<div
  class="flex flex-wrap items-center gap-2.5 px-5 py-4 text-hint text-muted-foreground empty:hidden"
>
  {#if userPaused(account.settings, ticker.now)}
    {@render idle(ACCOUNT_STATE_ICONS[AccountState.PAUSED], t.activity_paused())}
  {:else if account.status === AccountStatus.LOGIN_REQUIRED}
    {@render idle(ACCOUNT_STATE_ICONS[AccountState.PROBLEM], t.status_login_required())}
  {:else if account.slots.length === 0}
    {@render idle(IconName.CLOCK, t.activity_nothing())}
    <TextButton
      label={t.nav_account_drops()}
      icon={PAGE_ICONS[Page.ACCOUNT_DROPS]}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => navigate(Page.ACCOUNT_DROPS, account.id)}
    />
    <TextButton
      label={t.nav_account_points()}
      icon={PAGE_ICONS[Page.ACCOUNT_POINTS]}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => navigate(Page.ACCOUNT_POINTS, account.id)}
    />
  {:else if free > 0 && account.settings.mode !== Mode.DROPS}
    {@render idle(
      IconName.PLUS,
      account.settings.points.streamers.length
        ? t.activity_free_slot_offline(maxWatchedChannelsParams())
        : t.activity_free_slot(maxWatchedChannelsParams()),
    )}
    <TextButton
      label={t.nav_account_points()}
      icon={PAGE_ICONS[Page.ACCOUNT_POINTS]}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => navigate(Page.ACCOUNT_POINTS, account.id)}
    />
  {/if}
</div>
