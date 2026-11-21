<script lang="ts">
  import type { Snippet } from "svelte";

  import { t } from "../../lib/i18n/i18n.svelte";
  import { followLink } from "../../lib/links/links.utils";
  import type { LinkTargetType } from "../../lib/text.utils";
  import { linkPair } from "./link-pair.svelte";

  type LinkTwinPropsType = { target: LinkTargetType; live: boolean; children: Snippet };

  let { target, live, children }: LinkTwinPropsType = $props();

  const pair = linkPair();

  const hover = (hovered: boolean) => {
    if (pair) {
      pair.hovered = hovered;
    }
  };
</script>

<!-- ! The name's own link is the one tabbed to and read: this one only widens the picture into the same target. -->
{#if "url" in target}
  {@const url = target.url}
  <a
    class="inline-flex shrink-0"
    href={url}
    rel="noreferrer"
    tabindex="-1"
    aria-hidden="true"
    onclick={(event) => followLink(event, url)}
    onpointerenter={() => hover(true)}
    onpointerleave={() => hover(false)}
  >
    {@render children()}
  </a>
{:else}
  <button
    class="inline-flex shrink-0 rounded-md"
    type="button"
    tabindex="-1"
    aria-hidden="true"
    onclick={target.open}
    onpointerenter={() => hover(true)}
    onpointerleave={() => hover(false)}
  >
    {@render children()}
  </button>
{/if}{#if live}<span class="sr-only">{t.common_live()}</span>{/if}
