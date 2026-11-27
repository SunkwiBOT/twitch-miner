<script lang="ts">
  import type { TextCategoryType } from "../../lib/text.utils";
  import NameRun from "./NameRun.svelte";
  import RichText from "./RichText.svelte";

  type DetailLinePropsType = {
    parts: string[];
    categories: TextCategoryType[];
  };

  let { parts, categories }: DetailLinePropsType = $props();

  const shown = $derived(parts.filter(Boolean));
</script>

<NameRun>
  <span class="text-xs text-muted-foreground">
    {#each shown as part, index (index)}{#if index > 0}{" · "}{/if}<span
        class="inline-block max-w-full"
      >
        <RichText text={part} {categories} />
      </span>{/each}
  </span>
</NameRun>
