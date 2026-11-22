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

<li class="row group border-b last:border-b-0">
  <!-- ! Mouse targets only: the bars stay the keyboard and screen reader controls. -->
  <button
    class="head flex min-w-0 items-center gap-2.5 border-r bg-card px-3 py-1.5 text-left transition-colors group-hover:bg-accent"
    type="button"
    tabindex="-1"
    onclick={() => onopen(first)}
  >
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
  </button>
  <div class="lane transition-colors group-hover:bg-accent">
    <div class="track" data-track style:padding="{layout.padding}px 0" style:gap="{layout.gap}px">
      <button
        class="lane-hit"
        type="button"
        tabindex="-1"
        aria-hidden="true"
        onclick={() => onopen(first)}
      ></button>
      {#each row.items as item (item.key)}
        <TimelineBar
          {item}
          rowName={row.name}
          {range}
          height={layout.laneHeight}
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
    display: grid;
    grid-template-columns: var(--head-width) minmax(0, 1fr);
  }
  .head {
    position: relative;
    z-index: 2;
    width: 100%;
    font: inherit;
    cursor: pointer;
  }
  .lane {
    position: relative;
    display: flex;
    align-items: center;
  }
  .track {
    position: relative;
    display: flex;
    flex-direction: column;
    width: 100%;
  }
  .lane-hit {
    position: absolute;
    inset: 0;
    padding: 0;
    border: 0;
    background: transparent;
    cursor: pointer;
  }
</style>
