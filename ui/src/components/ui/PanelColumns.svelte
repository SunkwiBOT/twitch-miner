<script lang="ts">
  import type { Snippet } from "svelte";

  import type { PanelWidth } from "./ui.constants";

  type PanelColumnsPropsType = { width: PanelWidth; children: Snippet };

  let { width, children }: PanelColumnsPropsType = $props();
</script>

<!-- ! Columns, not a grid: a short panel is not stretched to its neighbour's height, the next one fills under it.
     Inline-level panels: WebKit paints a block panel's edge from the previous column at the top of the next one. -->
<div
  class="gap-5 [columns:var(--panel-width)_auto] *:mb-5 *:w-full *:[display:inline-flex] *:break-inside-avoid *:last:mb-0 has-[>:only-child]:[columns:auto]"
  style:--panel-width={width}
>
  {@render children()}
</div>
