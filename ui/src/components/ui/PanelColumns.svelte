<script lang="ts">
  import type { Snippet } from "svelte";

  import type { PanelWidth } from "./ui.constants";

  type PanelColumnsPropsType = { width: PanelWidth; children: Snippet };

  let { width, children }: PanelColumnsPropsType = $props();
</script>

<!-- ! Columns, not a grid: a short panel is not stretched to its neighbour's height, the next one fills under it.
     Inline-level panels: WebKit paints a block panel's edge from the previous column at the top of the next one.
     Each panel is then one line box: `orphans`/`widows` at 1 let the columns balance two or three of them.
     No more columns than panels, so a few panels share the whole width instead of leaving empty columns. -->
<div
  class="gap-5 [columns:var(--panel-width)_var(--panel-count,auto)] [orphans:1] [widows:1] *:mb-5 *:w-full *:[display:inline-flex] *:break-inside-avoid *:last:mb-0 has-[>:only-child]:[--panel-count:1] has-[>:nth-child(2):last-child]:[--panel-count:2] has-[>:nth-child(3):last-child]:[--panel-count:3]"
  style:--panel-width={width}
>
  {@render children()}
</div>
