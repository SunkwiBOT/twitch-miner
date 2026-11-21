<script lang="ts">
  import { number } from "../../../lib/format.utils";
  import TextLink from "../../ui/TextLink.svelte";
  import { LinkMark, LinkSize } from "../../ui/link.constants";
  import { categoryTone } from "../stats.utils";
  import type { StatsBarRowType } from "../stats.types";

  type CategoryBarListPropsType = { rows: StatsBarRowType[]; ariaLabel: string; emptyText: string };

  let { rows, ariaLabel, emptyText }: CategoryBarListPropsType = $props();

  const max = $derived(Math.max(1, ...rows.map((row) => row.value)));
</script>

{#if rows.length === 0}
  <p class="text-hint text-muted-foreground">{emptyText}</p>
{:else}
  <ul class="flex flex-col gap-2.5" aria-label={ariaLabel}>
    {#each rows as row, index (row.key)}
      <li class="flex items-center gap-2.5">
        <span class="flex w-32 shrink-0 text-hint" title={row.label}>
          {#if row.url}
            <TextLink
              text={row.label}
              target={{ url: row.url, mark: LinkMark.END }}
              size={LinkSize.MD}
            />
          {:else}
            <span class="truncate">{row.label}</span>
          {/if}
        </span>
        <span class="h-2 flex-1 overflow-hidden rounded-full bg-muted">
          <span
            class={`block h-full rounded-full ${categoryTone(index)}`}
            style={`width: ${Math.max(2, (row.value / max) * 100)}%`}
          ></span>
        </span>
        <span class="w-14 shrink-0 text-right text-hint font-semibold tabular-nums">
          {number(row.value)}
        </span>
      </li>
    {/each}
  </ul>
{/if}
