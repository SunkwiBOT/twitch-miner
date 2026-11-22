<script lang="ts" generics="T">
  import {
    getBarGeometry,
    getLabelPlacement,
    getNextBarIndex,
  } from "../../lib/catalogue/timeline.utils";
  import type { TimelineItemType, TimelineRangeType } from "../../lib/catalogue/timeline.utils";
  import { shortDuration } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import { formatClock, formatDayMonth, formatWeekdayDayMonth } from "../../lib/time/time.utils";
  import {
    BAR_PADDING_PX,
    BarLabelPlacement,
    TIMELINE_PILL_MS,
  } from "../../lib/catalogue/timeline.constants";
  import Badge from "../ui/Badge.svelte";
  import RichText from "../ui/RichText.svelte";
  import TimeLeftTag from "./TimeLeftTag.svelte";

  type TimelineBarPropsType = {
    item: TimelineItemType<T>;
    rowName: string;
    range: TimelineRangeType;
    height: number;
    laneWidth: number;
    now: number;
    todayEnd: number;
    onopen: (value: T) => void;
  };

  let { item, rowName, range, height, laneWidth, now, todayEnd, onopen }: TimelineBarPropsType =
    $props();

  let brokenImage = $state<string | null>(null);

  const image = $derived(item.image && item.image !== brokenImage ? item.image : null);

  const geometry = $derived(getBarGeometry(range, item.start, item.end, now));

  const endLabel = $derived.by(() => {
    if (geometry?.isCutRight) {
      return t.timeline_until({ date: strong(formatDayMonth(item.end)) });
    }

    return item.end < todayEnd
      ? t.timeline_ends_today({ endTime: strong(formatClock(item.end)) })
      : t.timeline_ends_on({
          date: strong(formatWeekdayDayMonth(item.end)),
          endTime: strong(formatClock(item.end)),
        });
  });

  const left = $derived(item.end - now);

  const pill = $derived(
    left > 0 && left < TIMELINE_PILL_MS
      ? t.timeline_left({ remainingTime: shortDuration(left) })
      : "",
  );

  let measured = $state(0);

  // ! Measured, never guessed: a label wider than its bar is drawn outside, so no bar cuts its own label.
  const labelWidth = $derived(measured + 2 * BAR_PADDING_PX);

  const placement = $derived(
    geometry ? getLabelPlacement(geometry, laneWidth, labelWidth) : BarLabelPlacement.INSIDE,
  );

  const onkeydown = (event: KeyboardEvent): void => {
    const track =
      event.currentTarget instanceof HTMLElement
        ? event.currentTarget.closest("[data-track]")
        : null;

    if (!track) {
      return;
    }

    const bars = Array.from(track.querySelectorAll<HTMLButtonElement>("[data-bar]"));
    const next = getNextBarIndex(
      bars.findIndex((bar) => bar === event.currentTarget),
      event.key,
      bars.length,
    );
    const target = next === null ? undefined : bars.at(next);

    if (target) {
      event.preventDefault();
      target.focus();
    }
  };
</script>

{#snippet label()}
  <span class="sr-only">{`${rowName}: `}</span>
  {#if image}<img
      class="size-4.5 shrink-0 object-contain"
      src={image}
      alt=""
      loading="lazy"
      decoding="async"
      onerror={() => (brokenImage = image)}
    />{/if}
  <span class="name min-w-0 truncate">{item.name}</span>
  <span class="shrink-0 font-medium text-muted-foreground"><RichText text={endLabel} /></span>
  {#if pill}<TimeLeftTag text={pill} urgent={true} />{/if}
  {#if item.tag}<Badge
      text={item.tag.text}
      tone={item.tag.tone}
      icon={item.tag.icon}
      hint={item.tag.hint}
    />{/if}
{/snippet}

{#if geometry}
  <div class="item" style:--bar-pad="{BAR_PADDING_PX}px">
    <button
      class="lane-hit"
      type="button"
      tabindex="-1"
      aria-hidden="true"
      onclick={() => onopen(item.value)}
    ></button>
    <span
      class="measure text-hint font-semibold"
      aria-hidden="true"
      inert
      bind:offsetWidth={measured}
    >
      {@render label()}
    </span>
    <div class="line" style:height="{height}px">
      <button
        class="bar text-hint font-semibold text-foreground"
        class:cut-left={geometry.isCutLeft}
        class:cut-right={geometry.isCutRight}
        class:outside={placement !== BarLabelPlacement.INSIDE}
        class:muted={item.tag?.muted}
        data-bar
        type="button"
        title={`${rowName} · ${item.name}`}
        style:left="{geometry.left}%"
        style:width="{geometry.width}%"
        style:--p="{geometry.progress}%"
        onclick={() => onopen(item.value)}
        {onkeydown}
      >
        {#if placement === BarLabelPlacement.INSIDE}
          {@render label()}
        {:else if placement === BarLabelPlacement.BELOW}
          <span class="sr-only">{@render label()}</span>
        {:else}
          <span class="beside" class:left={placement === BarLabelPlacement.LEFT}>
            {@render label()}
          </span>
        {/if}
      </button>
    </div>
    {#if placement === BarLabelPlacement.BELOW}
      <!-- ! A copy for the eye: the bar carries the label for the keyboard and screen readers. -->
      <div
        class="below text-hint font-semibold text-foreground"
        inert
        style:padding-left="min({geometry.left}%, max(0px, calc(100% - {Math.min(
          labelWidth,
          laneWidth / 2,
        )}px)))"
      >
        {@render label()}
      </div>
    {/if}
  </div>
{/if}

<style>
  .item {
    position: relative;
  }
  .line {
    position: relative;
  }
  .measure {
    position: absolute;
    top: 0;
    left: 0;
    display: flex;
    align-items: center;
    gap: 6px;
    line-height: 1.2;
    white-space: nowrap;
    visibility: hidden;
    pointer-events: none;
  }
  .lane-hit {
    position: absolute;
    inset: 0;
    padding: 0;
    border: 0;
    background: transparent;
    cursor: pointer;
  }
  .bar {
    position: absolute;
    top: 0;
    bottom: 0;
    z-index: 3;
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 6px;
    padding: 0 var(--bar-pad);
    border: 0;
    border-radius: var(--radius);
    background: linear-gradient(to right, var(--muted) 0 var(--p), var(--input) var(--p));
    line-height: 1.2;
    text-align: left;
    white-space: nowrap;
    cursor: pointer;
  }
  /* ! Hatched, not plain grey: a plain grey bar reads as one watched to the end. */
  .bar.muted {
    background: repeating-linear-gradient(
      135deg,
      var(--muted) 0 6px,
      color-mix(in oklab, var(--muted), var(--muted-foreground) 18%) 6px 12px
    );
    color: var(--muted-foreground);
  }
  .bar::after {
    content: "";
    position: absolute;
    inset: 0;
    z-index: -1;
    border-radius: inherit;
    background: var(--foreground);
    opacity: 0;
    transition: opacity 0.12s ease;
    pointer-events: none;
  }
  .bar:hover::after,
  .item:has(.lane-hit:hover) .bar::after {
    opacity: 0.1;
  }
  .bar:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: -2px;
  }
  .bar.cut-left {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }
  .bar.cut-right {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }
  .bar:not(.outside) {
    overflow: hidden;
  }
  .bar.outside {
    padding: 0;
  }
  .beside {
    position: absolute;
    top: 50%;
    left: calc(100% + var(--bar-pad));
    display: flex;
    align-items: center;
    gap: 6px;
    transform: translateY(-50%);
  }
  .beside.left {
    left: auto;
    right: calc(100% + var(--bar-pad));
  }
  /* ! Clicks fall through to the lane's own target. */
  .below {
    position: relative;
    z-index: 3;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 4px 6px;
    padding-top: 4px;
    line-height: 1.2;
    pointer-events: none;
  }
  .below .name {
    max-width: 100%;
  }
</style>
