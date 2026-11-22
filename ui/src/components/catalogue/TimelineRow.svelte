<script lang="ts" generics="T">
  import { getLaneLayout } from "../../lib/catalogue/timeline.utils";
  import type { TimelineRangeType, TimelineRowType } from "../../lib/catalogue/timeline.utils";
  import RichText from "../ui/RichText.svelte";
  import TimelineBar from "./TimelineBar.svelte";

  type TimelineRowPropsType = {
    row: TimelineRowType<T>;
    range: TimelineRangeType;
    laneWidth: number;
    now: number;
    todayEnd: number;
    onopen: (value: T) => void;
  };

  let { row, range, laneWidth, now, todayEnd, onopen }: TimelineRowPropsType = $props();

  const layout = $derived(getLaneLayout(row.items.length));
  const first = $derived(row.items[0].value);

  let brokenArt = $state<string | null>(null);
</script>

{#snippet head()}
  {#if row.art && row.art !== brokenArt}
    {@const art = row.art}
    <img
      class={[
        "block h-10 w-7.5 shrink-0 rounded-md bg-muted max-md:hidden",
        row.contain ? "object-contain p-0.5" : "object-cover",
      ]}
      src={art}
      alt=""
      loading="lazy"
      decoding="async"
      onerror={() => (brokenArt = art)}
    />
  {:else}
    <span class="block h-10 w-7.5 shrink-0 rounded-md bg-muted max-md:hidden"></span>
  {/if}
  <span class="flex min-w-0 flex-col">
    <span class="line-clamp-2 font-semibold break-words">{row.name}</span>
    <span class="truncate text-xs text-muted-foreground max-[860px]:hidden">
      <RichText text={row.detail} />
    </span>
  </span>
{/snippet}

<!-- ! One bar: its button covers the whole row. Stacked bars: each covers its own band of the lane. -->
<li class={["row border-b last:border-b-0", !layout.isStacked && "group"]}>
  {#if layout.isStacked}
    <!-- ! Mouse target only for the soonest-ending bar: the bars stay the keyboard and screen reader controls. -->
    <button
      class="head flex min-w-0 cursor-pointer items-center gap-2.5 border-r bg-card px-3 py-1.5 text-left transition-colors hover:bg-accent"
      type="button"
      tabindex="-1"
      onclick={() => onopen(first)}
    >
      {@render head()}
    </button>
  {:else}
    <div
      class="head flex min-w-0 items-center gap-2.5 border-r bg-card px-3 py-1.5 transition-colors group-hover:bg-accent"
    >
      {@render head()}
    </div>
  {/if}
  <div class={["lane transition-colors", !layout.isStacked && "group-hover:bg-accent"]}>
    <div
      class="track"
      data-track
      style:--lane-pad="{layout.padding}px"
      style:--lane-gap="{layout.gap}px"
    >
      {#each row.items as item (item.key)}
        <TimelineBar
          {item}
          rowName={row.name}
          {range}
          height={layout.laneHeight}
          stacked={layout.isStacked}
          {laneWidth}
          {now}
          {todayEnd}
          {onopen}
        />
      {/each}
    </div>
  </div>
</li>

<style>
  .row {
    position: relative;
    display: grid;
    grid-template-columns: var(--head-width) minmax(0, 1fr);
  }
  .head {
    width: 100%;
    font: inherit;
  }
  .lane {
    display: flex;
  }
  .track {
    display: flex;
    flex-direction: column;
    width: 100%;
  }
  .row:has(> .head:hover) .track > :global(:first-child) {
    background-color: var(--accent);
  }
</style>
