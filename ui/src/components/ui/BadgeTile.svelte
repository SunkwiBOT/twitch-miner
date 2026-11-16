<script lang="ts">
  import { cn } from "$lib/utils";
  import {
    BADGE_TILE_CLASSES,
    BADGE_TILE_INTERACTIVE_CLASSES,
  } from "../../lib/badges/badges.constants";
  import { IconName } from "../../lib/icons.constants";
  import Thumb from "./Thumb.svelte";
  import { ThumbShape, ThumbSize } from "./picture.constants";

  type BadgeTilePropsType = {
    image: string | null | undefined;
    title: string;
    small?: boolean;
    link?: boolean;
    tooltip?: boolean;
    onclick?: () => void;
  };

  let {
    image,
    title,
    small = false,
    link = false,
    tooltip = false,
    onclick,
  }: BadgeTilePropsType = $props();

  const label = $derived(
    cn("min-w-0 truncate font-semibold", small ? "text-xs" : "text-sm", link && "text-link"),
  );
</script>

{#snippet content()}
  <Thumb
    src={image}
    round={false}
    contain={true}
    size={ThumbSize.MD}
    shape={ThumbShape.SQUARE}
    fallback={IconName.AWARD}
  />
  <span class={label}>{title}</span>
{/snippet}

{#if onclick}
  <button
    class={cn(BADGE_TILE_CLASSES, BADGE_TILE_INTERACTIVE_CLASSES)}
    type="button"
    title={tooltip ? title : undefined}
    {onclick}
  >
    {@render content()}
  </button>
{:else}
  <div class={BADGE_TILE_CLASSES} title={tooltip ? title : undefined}>
    {@render content()}
  </div>
{/if}
