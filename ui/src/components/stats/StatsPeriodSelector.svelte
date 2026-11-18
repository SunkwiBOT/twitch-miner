<script lang="ts">
  import { HistoryRetention } from "../../lib/bindings/HistoryRetention";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { RETENTION_LABELS } from "../settings/settings.constants";
  import Segmented from "../ui/Segmented.svelte";
  import { STATS_PERIODS } from "./stats.constants";
  import type { StatsPeriodDaysType } from "./stats.types";

  type StatsPeriodSelectorPropsType = {
    value: StatsPeriodDaysType;
    retention: HistoryRetention;
    retentionDays: number | null;
    onchange: (value: StatsPeriodDaysType) => void;
  };

  let { value, retention, retentionDays, onchange }: StatsPeriodSelectorPropsType = $props();

  const PERIOD_LABELS: Record<StatsPeriodDaysType, () => string> = {
    7: () => t.stats_period_7(),
    30: () => t.stats_period_30(),
    90: () => t.stats_period_90(),
  };

  // ! A period beyond retention stays selectable while it is already the
  // active one, so shrinking the retention never leaves the tab bar with no
  // active tab.
  const options = $derived(
    STATS_PERIODS.map((period) => ({
      value: String(period),
      label: PERIOD_LABELS[period](),
      disabled: retentionDays !== null && period > retentionDays && period !== value,
      tip: t.stats_period_disabled_retention({ retention: RETENTION_LABELS[retention]() }),
    })),
  );
</script>

<Segmented
  value={String(value)}
  {options}
  label={t.stats_period_label()}
  onchange={(next) => {
    const period = STATS_PERIODS.find((candidate) => String(candidate) === next);

    if (period !== undefined) {
      onchange(period);
    }
  }}
/>
