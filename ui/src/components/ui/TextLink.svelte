<script lang="ts">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import type { LinkTargetType } from "../../lib/text.utils";
  import ExternalAnchor from "./ExternalAnchor.svelte";
  import { linkPair } from "./link-pair.svelte";
  import { nameListKind } from "./name-list.svelte";
  import LinkText from "./LinkText.svelte";
  import { INLINE_PICTURE_CLASSES } from "./ui.constants";
  import { inRunningText } from "./running-text.svelte";
  import {
    LINK_SIZES,
    LinkMark,
    LinkRest,
    LinkSize,
    LinkTone,
    NameListKind,
  } from "./link.constants";

  type TextLinkPropsType = {
    text: string;
    target:
      | (Extract<LinkTargetType, { url: string }> & { mark: LinkMark })
      | Extract<LinkTargetType, { open: () => void }>;
    size: LinkSize;
    tone?: LinkTone;
    rest?: LinkRest;
    lead?: Snippet;
  };

  let {
    text,
    target,
    size,
    tone = LinkTone.STRONG,
    rest = LinkRest.UNDERLINED,
    lead,
  }: TextLinkPropsType = $props();

  const pair = linkPair();
  const listKind = nameListKind();
  const running = inRunningText();
  const mark = $derived.by(() => {
    if (!("url" in target)) {
      return LinkMark.NONE;
    }

    if (listKind === NameListKind.WRAP) {
      return LinkMark.OVERLAY;
    }

    return listKind === NameListKind.RUN && target.mark === LinkMark.END
      ? LinkMark.PUSH
      : target.mark;
  });
  const fit = $derived(
    lead !== undefined ||
      mark === LinkMark.START ||
      mark === LinkMark.END ||
      mark === LinkMark.PUSH,
  );
  const classes = $derived(
    cn(
      "group/link min-w-0 text-left",
      LINK_SIZES[size].text,
      fit &&
        (running
          ? "inline-flex max-w-full items-baseline gap-1.5 align-baseline"
          : "inline-flex max-w-full items-center gap-1.5 align-bottom"),
      !fit && "inline",
    ),
  );
</script>

{#snippet content()}
  {#if lead}<span class={running ? INLINE_PICTURE_CLASSES : "inline-flex shrink-0"}>
      {@render lead()}
    </span>{/if}<LinkText {text} {mark} {size} {tone} {rest} {fit} />
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
