<script lang="ts">
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { strong } from "../../lib/text.utils";
  import { formatClock, formatDate, formatShortDateTime } from "../../lib/time/time.utils";
  import Badge from "./Badge.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { IconName } from "../../lib/icons.constants";

  type PauseBadgePropsType = { account: AccountSnapshotType; class?: string };

  let { account, class: className }: PauseBadgePropsType = $props();

  const until = $derived(account.settings.paused_until);

  const active = $derived.by(() => {
    void ticker.now;

    return until !== undefined && new Date(until) > new Date();
  });

  const sameDay = $derived.by(() => {
    void ticker.now;

    return until !== undefined && formatDate(until, true) === formatDate(Date.now(), true);
  });
</script>

{#if active && until}
  <Badge
    class={className}
    text={t.pause_badge({ time: sameDay ? formatClock(until) : formatShortDateTime(until) })}
    tone={BadgeTone.WARNING}
    dot={true}
    icon={IconName.CLOCK}
    hint={t.pause_badge_hint({ time: strong(formatShortDateTime(until)) })}
  />
{/if}
