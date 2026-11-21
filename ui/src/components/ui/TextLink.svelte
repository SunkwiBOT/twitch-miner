<script lang="ts">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import { followLink } from "../../lib/links/links.utils";
  import { linkPair } from "./link-pair.svelte";
  import LinkText from "./LinkText.svelte";
  import { LINK_SIZES, LinkMark, LinkSize } from "./link.constants";

  type TextLinkPropsType = {
    text: string;
    target: { url: string; mark: LinkMark } | { open: () => void };
    size: LinkSize;
    lead?: Snippet;
  };

  let { text, target, size, lead }: TextLinkPropsType = $props();

  const pair = linkPair();
  const mark = $derived("url" in target ? target.mark : LinkMark.NONE);
  const fit = $derived(lead !== undefined || mark === LinkMark.START || mark === LinkMark.END);
  const classes = $derived(
    cn(
      "group/link min-w-0 text-left",
      LINK_SIZES[size].text,
      fit ? "inline-flex max-w-full items-center gap-1.5 align-bottom" : "inline",
    ),
  );
</script>

{#snippet content()}
  {#if lead}{@render lead()}{/if}<LinkText {text} {mark} {size} {fit} />
{/snippet}

{#if "url" in target}
  {@const url = target.url}
  <a
    class={classes}
    href={url}
    rel="noreferrer"
    data-twin-hover={pair?.hovered || undefined}
    onclick={(event) => followLink(event, url)}
  >
    {@render content()}
  </a>
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
