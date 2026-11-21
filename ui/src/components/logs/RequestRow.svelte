<script lang="ts">
  import { cn } from "$lib/utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { RequestEntryType } from "../../lib/bindings/RequestEntryType";
  import { RequestOutcome } from "../../lib/bindings/RequestOutcome";
  import { elapsed, number } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import {
    GROUP_COUNT_PREFIX,
    NO_TARGET,
    PART_SEPARATOR,
    REQUEST_LINE_CLASSES,
    TIME_RANGE_SEPARATOR,
  } from "../../lib/requests/requests.constants";
  import {
    REQUEST_KIND_LABELS,
    resultHint,
    resultText,
    SOCKET_EVENT_LABELS,
    targetParts,
  } from "../../lib/requests/requests.utils";
  import { formatClock, formatClockWithSeconds } from "../../lib/time/time.utils";
  import Avatar from "../ui/Avatar.svelte";
  import Badge from "../ui/Badge.svelte";
  import RichText from "../ui/RichText.svelte";
  import { AvatarSize } from "../ui/picture.constants";
  import Tip from "../ui/Tip.svelte";
  import { BadgeTone } from "../../lib/tones.constants";

  type RequestRowPropsType = {
    entry: RequestEntryType;
    account: AccountSnapshotType | undefined;
  };

  let { entry, account }: RequestRowPropsType = $props();

  const failed = $derived(entry.outcome !== RequestOutcome.OK);
  const kind = $derived(REQUEST_KIND_LABELS[entry.kind]());
  const notModifiedHint = $derived(resultHint(entry));
</script>

<div class={REQUEST_LINE_CLASSES} role="listitem">
  <span class="whitespace-nowrap text-muted-foreground">
    {#if entry.group}
      <time datetime={entry.at}>{formatClock(entry.at)}</time>{TIME_RANGE_SEPARATOR}<time
        datetime={entry.group.until}
      >
        {formatClock(entry.group.until)}
      </time>
    {:else}
      <time datetime={entry.at}>{formatClockWithSeconds(entry.at)}</time>
    {/if}
  </span>

  <span class="min-w-0 items-center gap-1.5 self-start @max-2xl:sr-only @2xl:flex">
    {#if entry.account_id}
      <Avatar
        url={account?.avatar_url}
        name={account?.display_name ?? entry.account_id}
        size={AvatarSize.XS}
      />
      <span class="truncate text-link">{account?.display_name ?? entry.account_id}</span>
    {/if}
  </span>

  <span class="hidden text-muted-foreground @4xl:block">{kind}</span>

  <span class="min-w-0">
    <span class="text-muted-foreground @4xl:hidden">{kind}{PART_SEPARATOR}</span>
    <span>
      {#if entry.target}{#each targetParts(entry.target) as part, index (index)}{#if index > 0}<wbr
            />{/if}{part}{/each}{:else}{NO_TARGET}{/if}
    </span>
    {#if entry.socket}
      <span class="text-muted-foreground">
        {PART_SEPARATOR}{SOCKET_EVENT_LABELS[entry.socket]()}
      </span>
    {/if}
    {#if entry.group}
      {@const hint = t.requests_group_hint({
        count: entry.group.count,
        formattedCount: strong(number(entry.group.count)),
        from: strong(formatClock(entry.at)),
        to: strong(formatClock(entry.group.until)),
      })}
      <Badge
        text={`${GROUP_COUNT_PREFIX}${number(entry.group.count)}`}
        tone={BadgeTone.NEUTRAL}
        dot={false}
        {hint}
      />
      <span class="sr-only"><RichText text={hint} /></span>
    {/if}
    {#if entry.detail}
      <span class="block text-muted-foreground">{entry.detail}</span>
    {/if}
  </span>

  <span class={cn("whitespace-nowrap", failed ? "text-destructive" : "text-success-foreground")}>
    {#if notModifiedHint}
      <Tip text={notModifiedHint}>
        {#snippet children(props)}
          <span {...props} class="cursor-help underline decoration-dotted underline-offset-2">
            {resultText(entry)}
          </span>
        {/snippet}
      </Tip>
      <span class="sr-only">{notModifiedHint}</span>
    {:else}
      {resultText(entry)}
    {/if}
  </span>

  <span class="text-right whitespace-nowrap text-muted-foreground @max-2xl:sr-only @2xl:block">
    {elapsed(entry.duration_ms)}
    {#if entry.group}
      {PART_SEPARATOR}{t.requests_max({ duration: elapsed(entry.group.max_ms) })}
    {/if}
  </span>
</div>
