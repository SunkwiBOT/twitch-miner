<script lang="ts">
  import type { Snippet } from "svelte";

  import { t } from "../../lib/i18n/i18n.svelte";
  import type { LinkTargetType } from "../../lib/text.utils";
  import ExternalAnchor from "./ExternalAnchor.svelte";
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
  <ExternalAnchor
    url={target.url}
    class="inline-flex shrink-0"
    tabindex={-1}
    ariaHidden={true}
    onpointerenter={() => hover(true)}
    onpointerleave={() => hover(false)}
  >
    {@render children()}
  </ExternalAnchor>
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
