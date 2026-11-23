<script lang="ts">
  import { cn } from "$lib/utils";
  import { MarkdownBlockKind } from "../../lib/markdown/markdown.constants";
  import { parseMarkdown } from "../../lib/markdown/markdown.utils";
  import RichText from "./RichText.svelte";

  type MarkdownPropsType = { markdown: string };

  let { markdown }: MarkdownPropsType = $props();

  const blocks = $derived(parseMarkdown(markdown));
</script>

<div class="flex flex-col gap-2 text-hint">
  {#each blocks as block, index (index)}
    {#if block.kind === MarkdownBlockKind.HEADING}
      <h4 class="pt-1 font-semibold text-foreground first:pt-0">
        <RichText text={block.text.text} links={block.text.links} />
      </h4>
    {:else if block.kind === MarkdownBlockKind.PARAGRAPH}
      <p><RichText text={block.text.text} links={block.text.links} /></p>
    {:else}
      <svelte:element
        this={block.kind === MarkdownBlockKind.NUMBERS ? "ol" : "ul"}
        class={cn(
          "m-0 flex flex-col gap-1 ps-5 marker:text-muted-foreground",
          block.kind === MarkdownBlockKind.NUMBERS ? "list-decimal" : "list-disc",
        )}
      >
        {#each block.items as item, itemIndex (itemIndex)}
          <li><RichText text={item.text} links={item.links} /></li>
        {/each}
      </svelte:element>
    {/if}
  {/each}
</div>
