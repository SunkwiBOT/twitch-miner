<script lang="ts">
  import { TableCell, TableRow } from "$lib/components/ui/table";
  import type { MinerEventType } from "../lib/bindings/MinerEventType";
  import { historyView } from "../lib/events/detail.utils";
  import {
    EVENT_ICONS,
    EVENT_TONES,
    MINER_ICONS,
    MINER_TONES,
  } from "../lib/events/events.constants";
  import { eventIcon } from "../lib/events/events.utils";
  import { EVENT_KIND_LABELS, MINER_LABELS } from "../lib/events/labels.utils";
  import ChannelLink from "./ui/ChannelLink.svelte";
  import { app } from "../lib/state/app.svelte";
  import { formatShortDateTime } from "../lib/time/time.utils";
  import Avatar from "./ui/Avatar.svelte";
  import Badge from "./ui/Badge.svelte";
  import RewardImage from "./ui/RewardImage.svelte";
  import RichText from "./ui/RichText.svelte";
  import Thumb from "./ui/Thumb.svelte";
  import { AvatarSize, THUMB_PIXELS, ThumbShape, ThumbSize } from "./ui/picture.constants";

  type HistoryEventPropsType = {
    event: MinerEventType;
    showAccount: boolean;
    wide: boolean;
  };

  let { event, showAccount, wide }: HistoryEventPropsType = $props();

  const { owner, login, accountName, miner, sentence, reward, detail, picture } = $derived(
    historyView(event, app.snapshot?.accounts ?? []),
  );
</script>

{#snippet eventPicture()}
  {#if reward}
    <RewardImage url={reward.url} color={reward.color} size={THUMB_PIXELS[ThumbSize.SM]} />
  {:else}<Thumb
      src={picture}
      round={false}
      contain={true}
      size={ThumbSize.SM}
      shape={ThumbShape.SQUARE}
      fallback={eventIcon(event)}
    />{/if}
{/snippet}

{#snippet sentenceText()}
  <span class="leading-snug break-words">
    <RichText text={sentence.text} links={sentence.links} />
  </span>
{/snippet}

{#snippet channelLink(channel: string, size: AvatarSize)}
  {@const channelName = event.context.channel_name ?? channel}
  <ChannelLink login={channel} name={channelName} avatar={event.context.avatar} {size} />
{/snippet}

{#snippet kindBadge()}
  <Badge
    text={EVENT_KIND_LABELS[event.kind]()}
    tone={EVENT_TONES[event.kind]}
    dot={false}
    icon={EVENT_ICONS[event.kind]}
  />
{/snippet}

{#snippet minerBadge()}
  {#if miner}<Badge
      text={MINER_LABELS[miner]()}
      tone={MINER_TONES[miner]}
      dot={false}
      icon={MINER_ICONS[miner]}
    />{/if}
{/snippet}

{#if wide}
  <TableRow>
    <TableCell class="pl-0 text-xs text-muted-foreground tabular-nums">
      {formatShortDateTime(event.at)}
    </TableCell>
    {#if showAccount}
      <TableCell>
        {#if event.account_id}
          <span
            class="inline-flex max-w-40 min-w-32 items-center gap-1.5 whitespace-nowrap"
            title={accountName}
          >
            <Avatar url={owner?.avatar_url} name={accountName} size={AvatarSize.XS} />
            <span class="truncate font-semibold">{accountName}</span>
          </span>
        {:else}
          <span class="text-muted-foreground">—</span>
        {/if}
      </TableCell>
    {/if}
    <TableCell class="max-w-0 min-w-72 whitespace-normal">
      <span class="flex items-center gap-2.5">
        {@render eventPicture()}
        <span class="flex min-w-0 flex-col">
          {@render sentenceText()}
          {#if detail}<span class="truncate text-xs text-muted-foreground">{detail}</span>{/if}
        </span>
      </span>
    </TableCell>
    <TableCell>
      {#if login}
        <span
          class="inline-flex max-w-48 min-w-40 whitespace-nowrap"
          title={event.context.channel_name ?? login}
        >
          {@render channelLink(login, AvatarSize.MD)}
        </span>
      {:else}
        <span class="text-muted-foreground">—</span>
      {/if}
    </TableCell>
    <TableCell>{@render kindBadge()}</TableCell>
    <TableCell>
      {#if miner}{@render minerBadge()}{:else}<span class="text-muted-foreground">—</span>{/if}
    </TableCell>
  </TableRow>
{:else}
  <li class="flex gap-2.5 py-3">
    {@render eventPicture()}
    <div class="flex min-w-0 flex-1 flex-col gap-1">
      {@render sentenceText()}
      {#if detail}<span class="text-xs break-words text-muted-foreground">{detail}</span>{/if}
      <span class="flex flex-wrap items-center gap-x-2.5 gap-y-1 text-xs text-muted-foreground">
        <span class="tabular-nums">{formatShortDateTime(event.at)}</span>
        {#if showAccount && event.account_id}
          <span class="inline-flex min-w-0 items-center gap-1.5">
            <Avatar url={owner?.avatar_url} name={accountName} size={AvatarSize.XS} />
            <span class="font-semibold text-foreground">{accountName}</span>
          </span>
        {/if}
        {#if login}
          <span class="inline-flex min-w-0">{@render channelLink(login, AvatarSize.XS)}</span>
        {/if}
      </span>
      <span class="flex flex-wrap items-center gap-1.5">
        {@render kindBadge()}
        {@render minerBadge()}
      </span>
    </div>
  </li>
{/if}
