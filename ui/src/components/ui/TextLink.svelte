<script lang="ts">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import type { LinkTargetType } from "../../lib/text.utils";
  import ExternalAnchor from "./ExternalAnchor.svelte";
  import { linkPair } from "./link-pair.svelte";
  import LinkText from "./LinkText.svelte";
  import { LINK_SIZES, LinkMark, LinkSize } from "./link.constants";

  type TextLinkPropsType = {
    text: string;
    target:
      | (Extract<LinkTargetType, { url: string }> & { mark: LinkMark })
      | Extract<LinkTargetType, { open: () => void }>;
    size: LinkSize;
    lead?: Snippet;
    class?: string;
  };

  let { text, target, size, lead, class: className }: TextLinkPropsType = $props();

  const pair = linkPair();
  const mark = $derived("url" in target ? target.mark : LinkMark.NONE);
  const fit = $derived(lead !== undefined || mark === LinkMark.START || mark === LinkMark.END);
  const classes = $derived(
    cn(
      "group/link min-w-0 text-left",
      LINK_SIZES[size].text,
      fit ? "inline-flex max-w-full items-center gap-1.5 align-bottom" : "inline",
      className,
    ),
  );
</script>

{#snippet content()}
  {#if lead}{@render lead()}{/if}<LinkText {text} {mark} {size} {fit} />
{/snippet}

{#if "url" in target}
  <ExternalAnchor url={target.url} class={classes} dataTwinHover={pair?.hovered}>
    {@render content()}
  </ExternalAnchor>
{:else}
  <button
    class={classes}
    type="button"
    data-twin-hover={pair?.hovered || undefined}
    onclick={target.open}
  >
    {@render content()}
  </button>
{/if}
