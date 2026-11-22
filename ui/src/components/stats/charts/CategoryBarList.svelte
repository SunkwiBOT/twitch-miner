<script lang="ts">
  import { number } from "../../../lib/format.utils";
  import GameName from "../../ui/GameName.svelte";
  import { ThumbSize } from "../../ui/picture.constants";
  import { categoryTone } from "../stats.utils";
  import type { StatsBarRowType } from "../stats.types";

  type CategoryBarListPropsType = { rows: StatsBarRowType[]; ariaLabel: string; emptyText: string };

  let { rows, ariaLabel, emptyText }: CategoryBarListPropsType = $props();

  const max = $derived(Math.max(1, ...rows.map((row) => row.value)));
</script>

{#if rows.length === 0}
  <p class="text-hint text-muted-foreground">{emptyText}</p>
{:else}
  <!-- ! The name sits above its bar, so the bar's width never takes the room of a long category name. -->
  <ul class="flex flex-col gap-3" aria-label={ariaLabel}>
    {#each rows as row, index (row.key)}
      <li class="flex flex-col gap-1.5">
        <span class="flex items-center justify-between gap-3 text-hint">
          {#if row.url}
            <GameName name={row.label} url={row.url} art={row.art} size={ThumbSize.XS} />
          {:else}
            <span class="min-w-0 text-muted-foreground">{row.label}</span>
          {/if}
          <span class="shrink-0 font-semibold tabular-nums">{number(row.value)}</span>
        </span>
        <span class="block h-2 overflow-hidden rounded-full bg-muted">
          <span
            class={`block h-full rounded-full ${categoryTone(index)}`}
            style={`width: ${Math.max(2, (row.value / max) * 100)}%`}
          ></span>
        </span>
      </li>
    {/each}
  </ul>
{/if}
