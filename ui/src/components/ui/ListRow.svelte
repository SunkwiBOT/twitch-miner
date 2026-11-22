<script lang="ts">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import DetailLine from "./DetailLine.svelte";
  import LinkText from "./LinkText.svelte";
  import { LinkMark, LinkSize } from "./link.constants";
  import RichText from "./RichText.svelte";
  import { RAISED_CLASSES, STRETCHED_BUTTON_CLASSES } from "./ui.constants";

  type ListRowPropsType = {
    title: string;
    subtitle?: string;
    details?: string[];
    lead?: Snippet;
    badges?: Snippet;
    below?: Snippet;
    trailing?: Snippet;
    onopen?: () => void;
    muted: boolean;
  };

  let { title, subtitle, details, lead, badges, below, trailing, onopen, muted }: ListRowPropsType =
    $props();
</script>

<div
  class={cn(
    "relative flex min-h-13 gap-3 border-b px-3 py-2.5 transition-colors last:border-b-0",
    below ? "items-start" : "items-center",
    onopen && "hover:bg-accent",
  )}
  role="listitem"
>
  <span
    class={cn(
      "flex min-w-0 flex-1 gap-3",
      below ? "items-start" : "items-center",
      muted && "opacity-55 grayscale",
    )}
  >
    {#if lead}{@render lead()}{/if}
    <span class="flex min-w-0 flex-1 flex-col gap-0.5">
      <span class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1">
        {#if onopen}
          <button
            class={cn(STRETCHED_BUTTON_CLASSES, "group/link block min-w-0 truncate")}
            type="button"
            onclick={onopen}
          >
            <LinkText text={title} mark={LinkMark.NONE} size={LinkSize.MD} fit={true} />
          </button>
        {:else}
          <span class="truncate font-semibold">{title}</span>
        {/if}
        {#if badges}<span class={cn(RAISED_CLASSES, "inline-flex max-w-full flex-wrap gap-1.5")}>
            {@render badges()}
          </span>{/if}
      </span>
      {#if subtitle}<span class="truncate text-xs text-muted-foreground">
          <RichText text={subtitle} />
        </span>{/if}
      {#if details}<DetailLine parts={details} />{/if}
      {#if below}<span class="mt-1 block">{@render below()}</span>{/if}
    </span>
  </span>
  {#if trailing}<span class={cn(RAISED_CLASSES, "flex shrink-0 items-center gap-1.5")}>
      {@render trailing()}
    </span>{/if}
</div>
