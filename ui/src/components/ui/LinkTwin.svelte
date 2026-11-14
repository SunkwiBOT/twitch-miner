<script lang="ts">
  import type { Snippet } from "svelte";

  import { t } from "../../lib/i18n/i18n.svelte";
  import { followLink } from "../../lib/links/links.utils";

  type LinkTwinPropsType = { url: string; live: boolean; children: Snippet };

  let { url, live, children }: LinkTwinPropsType = $props();
</script>

<!-- ! The name's own link is the one tabbed to and read: this one only widens the target and lights the name. -->
<a
  class="inline-flex shrink-0"
  href={url}
  rel="noreferrer"
  tabindex="-1"
  aria-hidden="true"
  data-link-twin
  onclick={(event) => followLink(event, url)}>{@render children()}</a
>{#if live}<span class="sr-only">{t.common_live()}</span>{/if}
