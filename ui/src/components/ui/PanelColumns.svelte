<script lang="ts">
  import type { Snippet } from "svelte";

  import type { PanelWidth } from "./ui.constants";

  type PanelColumnsPropsType = { width: PanelWidth; children: Snippet };

  let { width, children }: PanelColumnsPropsType = $props();
</script>

<!-- ! Columns, not a grid: a short panel is not stretched to the height of its
neighbour's row, the next panel fills the space under it. A lone panel keeps
the whole width. -->
<div
  class="gap-5 [columns:var(--panel-width)_auto] *:mb-5 *:break-inside-avoid *:last:mb-0 has-[>:only-child]:[columns:auto]"
  style:--panel-width={width}
>
  {@render children()}
</div>
