<script lang="ts">
  import type { Snippet } from "svelte";

  import { EXTERNAL_LINK_REL, externalLinkOnClick, followLink } from "../../lib/links/links.utils";

  type ExternalAnchorPropsType = {
    url: string;
    class?: string;
    tabindex?: number;
    ariaHidden?: boolean;
    dataTwinHover?: boolean;
    follow?: (event: MouseEvent, linkUrl: string) => void;
    onfollow?: () => void;
    onpointerenter?: () => void;
    onpointerleave?: () => void;
    onfocus?: () => void;
    onblur?: () => void;
    children: Snippet;
  };

  let {
    url,
    class: className,
    tabindex,
    ariaHidden,
    dataTwinHover,
    follow = followLink,
    onfollow,
    onpointerenter,
    onpointerleave,
    onfocus,
    onblur,
    children,
  }: ExternalAnchorPropsType = $props();
</script>

<a
  class={className}
  href={url}
  rel={EXTERNAL_LINK_REL}
  {tabindex}
  aria-hidden={ariaHidden || undefined}
  data-twin-hover={dataTwinHover || undefined}
  onclick={externalLinkOnClick(url, follow, onfollow)}
  {onpointerenter}
  {onpointerleave}
  {onfocus}
  {onblur}
>
  {@render children()}
</a>
