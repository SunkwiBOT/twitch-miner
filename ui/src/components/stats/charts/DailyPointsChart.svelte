<script lang="ts">
  import { number } from "../../../lib/format.utils";
  import {
    axisLabelStep,
    niceTicks,
    xTickAnchor,
    Y_AXIS_LABEL_GAP_PX,
    yAxisLeftPad,
  } from "./chart-scale.utils";
  import { reasonLabel } from "../../../lib/events/event-reasons.utils";
  import { reasonTone } from "../stats.utils";
  import type { StatsDayBucketType } from "../stats.types";

  type DailyPointsChartPropsType = {
    days: StatsDayBucketType[];
    reasons: string[];
    ariaLabel: string;
  };

  let { days, reasons, ariaLabel }: DailyPointsChartPropsType = $props();

  // ! Drawn at the container's own measured width (no viewBox up/downscale)
  // so the axis text stays `text-tiny` size at every width; 760 is only
  // the pre-mount guess, replaced once `bind:clientWidth` reports the real one.
  let measuredWidth = $state(0);
  const WIDTH = $derived(measuredWidth > 0 ? Math.round(measuredWidth) : 760);
  const HEIGHT = 200;
  const PAD_RIGHT = 8;
  const PAD_BOTTOM = 18;
  const PAD_TOP = 8;

  const maxRaw = $derived(Math.max(1, ...days.map((day) => day.total)));
  const gridValues = $derived(niceTicks(maxRaw));
  const maxTotal = $derived(Math.max(...gridValues, 1));

  const PAD_LEFT = $derived(yAxisLeftPad(gridValues.map(number)));
  const plotWidth = $derived(WIDTH - PAD_LEFT - PAD_RIGHT);
  const plotHeight = $derived(HEIGHT - PAD_TOP - PAD_BOTTOM);

  const barGap = $derived(plotWidth / Math.max(1, days.length));
  const barWidth = $derived(Math.max(2, barGap * 0.64));

  const longestDayLabel = $derived(Math.max(0, ...days.map((day) => day.dateLabel.length)));
  const tickEvery = $derived(axisLabelStep(longestDayLabel, barGap));
  const lastTickIndex = $derived(
    Math.floor((days.length - 1) / Math.max(1, tickEvery)) * tickEvery,
  );

  const yOf = (value: number): number => PAD_TOP + plotHeight - (value / maxTotal) * plotHeight;

  const barsOf = (day: StatsDayBucketType) => {
    let cursor = 0;

    return reasons.map((reason) => {
      const value = day.byReason[reason] ?? 0;
      const y = yOf(cursor + value);
      const height = yOf(cursor) - y;

      cursor += value;

      return { reason, value, y, height };
    });
  };
</script>

<figure class="flex flex-col gap-2" bind:clientWidth={measuredWidth}>
  <svg
    width={WIDTH}
    height={HEIGHT}
    viewBox={`0 0 ${WIDTH} ${HEIGHT}`}
    role="img"
    aria-label={ariaLabel}
    class="block text-muted-foreground"
  >
    <title>{ariaLabel}</title>
    {#each gridValues as grid (grid)}
      <line
        x1={PAD_LEFT}
        x2={WIDTH - PAD_RIGHT}
        y1={yOf(grid)}
        y2={yOf(grid)}
        class="stroke-border"
        stroke-width="1"
      />
      <text
        x={PAD_LEFT - Y_AXIS_LABEL_GAP_PX}
        y={yOf(grid) + 3}
        text-anchor="end"
        class="fill-current text-tiny"
      >
        {number(grid)}
      </text>
    {/each}
    {#each days as day, index (day.ms)}
      {@const x = PAD_LEFT + index * barGap + (barGap - barWidth) / 2}
      <g>
        <title>{day.dateLabel} — {number(day.total)}</title>
        {#each barsOf(day) as segment (segment.reason)}
          {#if segment.value > 0}
            <rect
              {x}
              y={segment.y}
              width={barWidth}
              height={Math.max(segment.height, 1)}
              class={reasonTone(segment.reason).fill}
              rx="1"
            />
          {/if}
        {/each}
      </g>
      {#if index % tickEvery === 0}
        <text
          x={x + barWidth / 2}
          y={HEIGHT - 4}
          text-anchor={xTickAnchor(index === 0, index === lastTickIndex)}
          class="fill-current text-tiny"
        >
          {day.dateLabel}
        </text>
      {/if}
    {/each}
  </svg>
  <figcaption class="flex flex-wrap items-center gap-x-3 gap-y-1.5 text-tiny text-muted-foreground">
    {#each reasons as reason (reason)}
      <span class="flex items-center gap-1.5">
        <span class={`size-2.5 shrink-0 rounded-sm ${reasonTone(reason).bg}`} aria-hidden="true"
        ></span>
        {reasonLabel(reason)}
      </span>
    {/each}
  </figcaption>
</figure>
