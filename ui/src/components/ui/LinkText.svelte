<script lang="ts">
  import { cn } from "$lib/utils";
  import { IconName } from "../../lib/icons.constants";
  import Icon from "./Icon.svelte";
  import { LINK_SIZES, LinkMark, LinkSize } from "./link.constants";

  type LinkTextPropsType = { text: string; mark: LinkMark; size: LinkSize; fit: boolean };

  let { text, mark, size, fit }: LinkTextPropsType = $props();
</script>

<!-- ! Styled from the closest `group/link`: the link itself, or a wider block (a card) that is the link. -->
{#snippet icon()}
  <span
    class={cn(
      "shrink-0 text-link",
      mark === LinkMark.OVERLAY
        ? "absolute bottom-0 left-full ml-1.5 hidden link-active:inline-block"
        : "opacity-0 transition-opacity link-active:opacity-100",
    )}
  >
    <Icon name={IconName.EXTERNAL} size={LINK_SIZES[size].icon} />
  </span>
{/snippet}

{#if mark === LinkMark.START}{@render icon()}{/if}<span
  class={cn(
    "underline decoration-link decoration-1 underline-offset-3 transition-colors link-active:text-link link-active:decoration-2",
    LINK_SIZES[size].label,
    mark === LinkMark.OVERLAY && "relative",
    fit && LINK_SIZES[size].fit,
  )}
>
  {text}{#if mark === LinkMark.OVERLAY}{@render icon()}{/if}
</span>{#if mark === LinkMark.END}{@render icon()}{/if}
