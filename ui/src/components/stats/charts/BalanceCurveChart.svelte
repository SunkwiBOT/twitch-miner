<script lang="ts">
  import type { MinerEventType } from "../../../lib/bindings/MinerEventType";
  import { eventText } from "../../../lib/events/event-text.utils";
  import { number } from "../../../lib/format.utils";
  import { formatDayMonth, formatShortDateTime } from "../../../lib/time/time.utils";
  import Icon from "../../ui/Icon.svelte";
  import { annotationStyle } from "../stats-annotations.utils";
  import {
    axisLabelCount,
    curveYTicks,
    xTickAnchor,
    Y_AXIS_LABEL_GAP_PX,
    yAxisLeftPad,
  } from "./chart-scale.utils";

  type BalanceCurveChartPropsType = {
    events: MinerEventType[];
    domainStart: number;
    domainEnd: number;
    ariaLabel: string;
  };

  let { events, domainStart, domainEnd, ariaLabel }: BalanceCurveChartPropsType = $props();

  // ! Drawn at the container's own measured width (no viewBox up/downscale)
  // so the axis text stays `text-tiny` size at every width; 760 is only
  // the pre-mount guess, replaced once `bind:clientWidth` reports the real one.
  let measuredWidth = $state(0);
  const WIDTH = $derived(measuredWidth > 0 ? Math.round(measuredWidth) : 760);
  const HEIGHT = 220;
  const PAD_RIGHT = 10;
  const PAD_TOP = 14;
  const PAD_BOTTOM = 22;

  const balancePoints = $derived(
    events.flatMap((event) => {
      const balance = event.context.balance;

      return balance === undefined ? [] : [{ ms: Date.parse(event.at), balance }];
    }),
  );

  const annotationPoints = $derived(
    events
      .map((event) => ({ event, style: annotationStyle(event) }))
      .filter(
        (entry): entry is { event: MinerEventType; style: NonNullable<typeof entry.style> } =>
          entry.style !== null,
      ),
  );

  // ! Every balance-affecting event gets a hover target, not only the
  // annotated ones: events already covered by an icon marker above are
  // skipped so the same point is not described twice.
  const plainBalancePoints = $derived(
    events.filter(
      (event) => event.context.balance !== undefined && annotationStyle(event) === null,
    ),
  );

  const gridValues = $derived(curveYTicks(balancePoints.map((p) => p.balance)));
  const gridLabels = $derived(gridValues.map(number));
  const domainMin = $derived(gridValues[0] ?? 0);
  const domainMax = $derived(gridValues[gridValues.length - 1] ?? 1);

  const PAD_LEFT = $derived(yAxisLeftPad(gridLabels));
  const plotWidth = $derived(WIDTH - PAD_LEFT - PAD_RIGHT);
  const plotHeight = $derived(HEIGHT - PAD_TOP - PAD_BOTTOM);

  const xOf = (ms: number): number =>
    PAD_LEFT + ((ms - domainStart) / (domainEnd - domainStart || 1)) * plotWidth;

  const yOf = (balance: number): number =>
    PAD_TOP + plotHeight - ((balance - domainMin) / (domainMax - domainMin || 1)) * plotHeight;

  const balanceAt = (ms: number): number => {
    let last = balancePoints[0]?.balance ?? 0;

    for (const point of balancePoints) {
      if (point.ms > ms) {
        break;
      }

      last = point.balance;
    }

    return last;
  };

  const linePath = $derived(
    balancePoints.length === 0
      ? ""
      : balancePoints
          .map((p, i) => `${i === 0 ? "M" : "L"}${xOf(p.ms)},${yOf(p.balance)}`)
          .join(" "),
  );

  const areaPath = $derived(
    balancePoints.length === 0
      ? ""
      : `${linePath} L${xOf(balancePoints[balancePoints.length - 1].ms)},${yOf(domainMin)} L${xOf(balancePoints[0].ms)},${yOf(domainMin)} Z`,
  );

  // ! `formatDayMonth(domainEnd)` stands in for every tick's label length:
  // dates of one period share the same digit/letter count give or take one,
  // close enough to decide how many of the up-to-6 ticks fit without
  // overlapping.
  const TICK_COUNT = $derived(axisLabelCount(formatDayMonth(domainEnd).length, plotWidth, 6));

  const xTicks = $derived(
    Array.from(
      { length: TICK_COUNT },
      (_, i) => domainStart + ((domainEnd - domainStart) * i) / (TICK_COUNT - 1),
    ),
  );

  const MARKER_RADIUS = 8;
  const ICON_SIZE = 11;
  const PLAIN_MARKER_RADIUS = 3;

  // ! The tooltip's own sentence: built with the Journal's `eventText` like
  // every other event, not the raw title. `**bold**` markers are stripped
  // for the plain-text SVG `<title>` the same way TimelineBar does for
  // sizing; `channelShown: true` skips repeating the streamer this curve
  // already names.
  const tooltipText = (event: MinerEventType): string =>
    eventText(event, { channelShown: true, linked: false }).text.replaceAll("**", "");
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
    {#if balancePoints.length > 0}
      <path d={areaPath} class="fill-primary/15" />
      <path d={linePath} class="stroke-primary" fill="none" stroke-width="2" />
    {/if}
    {#each plainBalancePoints as event (event.id)}
      {@const x = xOf(Date.parse(event.at))}
      {@const y = yOf(event.context.balance ?? 0)}
      <circle cx={x} cy={y} r={PLAIN_MARKER_RADIUS} class="fill-primary" stroke="none">
        <title>{formatShortDateTime(event.at)} — {tooltipText(event)}</title>
      </circle>
    {/each}
    {#each annotationPoints as { event, style } (event.id)}
      {@const x = xOf(Date.parse(event.at))}
      {@const y = yOf(balanceAt(Date.parse(event.at)))}
      <g>
        <title>{formatShortDateTime(event.at)} — {tooltipText(event)}</title>
        <line
          x1={x}
          x2={x}
          y1={PAD_TOP}
          y2={HEIGHT - PAD_BOTTOM}
          stroke-dasharray="2 3"
          opacity="0.3"
          class="stroke-current"
        />
        <circle cx={x} cy={y} r={MARKER_RADIUS} class={style.bg} stroke="none" />
        <foreignObject
          x={x - ICON_SIZE / 2}
          y={y - ICON_SIZE / 2}
          width={ICON_SIZE}
          height={ICON_SIZE}
        >
          <div class="flex size-full items-center justify-center text-white">
            <Icon name={style.icon} size={ICON_SIZE - 2} />
          </div>
        </foreignObject>
      </g>
    {/each}
    {#each xTicks as tick, i (tick)}
      <text
        x={xOf(tick)}
        y={HEIGHT - 4}
        text-anchor={xTickAnchor(i === 0, i === xTicks.length - 1)}
        class="fill-current text-tiny"
      >
        {formatDayMonth(tick)}
      </text>
    {/each}
  </svg>
</figure>
