<script lang="ts">
  import { cn } from "$lib/utils";
  import { number } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import {
    BALANCE_ICON_CLASSES,
    BALANCE_ICONS,
    BalanceTracking,
  } from "../../../lib/spam/spam.constants";
  import { BALANCE_HINTS, changedAgo } from "../../../lib/spam/spam.utils";
  import Icon from "../../ui/Icon.svelte";
  import Tip from "../../ui/Tip.svelte";
  import { IconName } from "../../../lib/icons.constants";

  type SpamBalancePropsType = {
    balance: number;
    changedAt: string;
    tracking: BalanceTracking;
    free: boolean;
    now: number;
  };

  let { balance, changedAt, tracking, free, now }: SpamBalancePropsType = $props();

  const status = $derived.by(() => {
    switch (tracking) {
      case BalanceTracking.LIVE:
        return t.spam_balance_changed({ time: changedAgo(changedAt, now) });
      case BalanceTracking.PAUSED:
        return t.spam_balance_paused();
      case BalanceTracking.FAILING:
        return t.spam_balance_failing();
    }
  });
</script>

<span class="flex shrink-0 flex-col items-end gap-0.5 text-right">
  <span
    class="inline-flex items-center gap-1.5 text-xl leading-tight font-semibold tabular-nums"
    role="img"
    aria-label={free ? t.spam_balance_unlimited() : t.spam_balance({ points: number(balance) })}
  >
    <span class="text-link"><Icon name={IconName.CHANNEL_POINTS} size={18} /></span>{#if free}<Icon
        name={IconName.INFINITY}
        size={22}
      />{:else}{number(balance)}{/if}
  </span>
  <Tip text={BALANCE_HINTS[tracking]()}>
    {#snippet children(props)}
      <button
        {...props}
        type="button"
        class="inline-flex cursor-help items-center gap-1.5 text-xs text-muted-foreground"
      >
        <Icon
          name={BALANCE_ICONS[tracking]}
          size={12}
          class={cn("shrink-0", BALANCE_ICON_CLASSES[tracking])}
        />
        {status}
      </button>
    {/snippet}
  </Tip>
</span>
