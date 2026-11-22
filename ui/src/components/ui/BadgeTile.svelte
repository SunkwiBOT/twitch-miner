<script lang="ts">
  import { cn } from "$lib/utils";
  import {
    BADGE_TILE_CLASSES,
    BADGE_TILE_INTERACTIVE_CLASSES,
  } from "../../lib/badges/badges.constants";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { IconName } from "../../lib/icons.constants";
  import Icon from "./Icon.svelte";
  import LinkText from "./LinkText.svelte";
  import { LinkMark, LinkSize } from "./link.constants";
  import RichText from "./RichText.svelte";
  import Thumb from "./Thumb.svelte";
  import { ThumbShape, ThumbSize } from "./picture.constants";

  type BadgeTilePropsType = {
    image: string | null | undefined;
    title: string;
    details?: string[];
    fallback?: IconName;
    small?: boolean;
    tooltip?: boolean;
    onclick?: () => void;
  };

  let {
    image,
    title,
    details = [],
    fallback = IconName.AWARD,
    small = false,
    tooltip = false,
    onclick,
  }: BadgeTilePropsType = $props();
</script>

{#snippet content(interactive: boolean)}
  <Thumb
    src={image}
    round={false}
    contain={true}
    size={ThumbSize.MD}
    shape={ThumbShape.SQUARE}
    {fallback}
  />
  <span class="flex min-w-0 flex-col">
    {#if interactive}
      <LinkText
        text={title}
        mark={LinkMark.NONE}
        size={small ? LinkSize.SM : LinkSize.MD}
        fit={true}
      />
    {:else}
      <span class={cn("min-w-0 truncate font-semibold", small ? "text-xs" : "text-sm")}>
        {title}
      </span>
    {/if}
    {#each details as detail, index (index)}<span
        class="min-w-0 truncate text-xs text-muted-foreground"
      >
        <RichText text={detail} />
      </span>{/each}
  </span>
{/snippet}

{#if onclick}
  <button
    class={cn(BADGE_TILE_CLASSES, BADGE_TILE_INTERACTIVE_CLASSES, "group/link")}
    type="button"
    title={tooltip ? title : undefined}
    {onclick}
  >
    {@render content(true)}
    <span
      class="absolute inset-y-0 right-0 flex items-center gap-1 rounded-r-lg bg-linear-to-l from-accent from-75% to-transparent pr-2.5 pl-6 text-xs font-semibold text-link opacity-0 transition-opacity group-hover/tile:opacity-100 group-focus-visible/tile:opacity-100"
      aria-hidden="true"
    >
      <Icon name={IconName.EYE} size={13} />{t.campaign_details()}
    </span>
  </button>
{:else}
  <div class={BADGE_TILE_CLASSES} title={tooltip ? title : undefined}>
    {@render content(false)}
  </div>
{/if}
