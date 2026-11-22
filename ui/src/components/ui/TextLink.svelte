<script lang="ts">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import type { LinkTargetType } from "../../lib/text.utils";
  import ExternalAnchor from "./ExternalAnchor.svelte";
  import { linkPair } from "./link-pair.svelte";
  import LinkText from "./LinkText.svelte";
  import { LINK_SIZES, LinkMark, LinkSize, LinkTone } from "./link.constants";

  type TextLinkPropsType = {
    text: string;
    target:
      | (Extract<LinkTargetType, { url: string }> & { mark: LinkMark })
      | Extract<LinkTargetType, { open: () => void }>;
    size: LinkSize;
    tone?: LinkTone;
    lead?: Snippet;
  };

  let { text, target, size, tone = LinkTone.STRONG, lead }: TextLinkPropsType = $props();

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
  {#if lead}{@render lead()}{/if}<LinkText {text} {mark} {size} {tone} {fit} />
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
