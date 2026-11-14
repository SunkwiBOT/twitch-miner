<script lang="ts" module>
  import type { Snippet } from "svelte";

  export type PanelGridCellType = { key: string; wide: boolean; panel: Snippet };
</script>

<script lang="ts">
  import { cn } from "$lib/utils";

  type PanelGridPropsType = { cells: PanelGridCellType[] };

  let { cells }: PanelGridPropsType = $props();
</script>

<div class="@container">
  <div class="grid gap-5 max-md:gap-4 @3xl:grid-cols-2">
    {#each cells as cell (cell.key)}
      <div class={cn("@container flex min-w-0 flex-col *:flex-1", cell.wide && "@3xl:col-span-2")}>
        {@render cell.panel()}
      </div>
    {/each}
  </div>
</div>
