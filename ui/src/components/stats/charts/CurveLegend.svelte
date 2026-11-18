<script lang="ts">
  import { BetResult } from "../../../lib/bindings/BetResult";
  import { EventKind } from "../../../lib/bindings/EventKind";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import Icon from "../../ui/Icon.svelte";
  import { STATS_ANNOTATION_STYLES } from "../stats-annotations.constants";

  const ICON_SIZE = 12;

  const entries = $derived([
    { icon: null, label: t.stats_curve_legend_balance() },
    {
      ...STATS_ANNOTATION_STYLES[EventKind.STREAK_RECOVERED],
      label: t.stats_curve_legend_streak(),
    },
    {
      ...STATS_ANNOTATION_STYLES[EventKind.RAID_JOINED],
      label: t.stats_curve_legend_raid(),
    },
    {
      ...STATS_ANNOTATION_STYLES[EventKind.MOMENT_CLAIMED],
      label: t.stats_curve_legend_moment(),
    },
    {
      ...STATS_ANNOTATION_STYLES[EventKind.PREDICTION_PLACED],
      label: t.stats_curve_legend_prediction_placed(),
    },
    {
      ...STATS_ANNOTATION_STYLES[`${EventKind.PREDICTION_RESULT}_${BetResult.WIN}`],
      label: t.stats_curve_legend_prediction_won(),
    },
    {
      ...STATS_ANNOTATION_STYLES[`${EventKind.PREDICTION_RESULT}_${BetResult.LOSE}`],
      label: t.stats_curve_legend_prediction_lost(),
    },
    {
      ...STATS_ANNOTATION_STYLES[`${EventKind.PREDICTION_RESULT}_${BetResult.REFUND}`],
      label: t.stats_curve_legend_prediction_refund(),
    },
  ]);
</script>

<ul class="flex flex-wrap items-center gap-x-3 gap-y-1.5 text-tiny text-muted-foreground">
  {#each entries as entry (entry.label)}
    <li class="flex items-center gap-1.5">
      {#if entry.icon === null}
        <span class="h-0.5 w-3.5 rounded-full bg-primary" aria-hidden="true"></span>
      {:else}
        <span class={entry.text}><Icon name={entry.icon} size={ICON_SIZE} /></span>
      {/if}
      {entry.label}
    </li>
  {/each}
</ul>
