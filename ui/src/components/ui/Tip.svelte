<script lang="ts">
  import type { Snippet } from "svelte";

  import { Tooltip, TooltipContent, TooltipTrigger } from "$lib/components/ui/tooltip";
  import RichText from "./RichText.svelte";

  type TipPropsType = {
    text: string;
    children: Snippet<[Record<string, unknown>]>;
  };

  let { text, children }: TipPropsType = $props();
</script>

<Tooltip ignoreNonKeyboardFocus={true}>
  <TooltipTrigger>
    {#snippet child({ props })}{@render children(props)}{/snippet}
  </TooltipTrigger>
  <TooltipContent><RichText {text} /></TooltipContent>
</Tooltip>
