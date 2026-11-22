<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import {
    EXTERNAL_LINK_REL,
    externalLinkOnClick,
    followBadgeLink,
    followCampaignLink,
    followLink,
  } from "../../lib/links/links.utils";
  import Icon from "./Icon.svelte";
  import {
    BUTTON_ICON_SIZES,
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    BUTTON_VARIANT_CLASSES,
    ButtonSize,
    ButtonVariant,
  } from "./button.constants";
  import { ResolveKind } from "./external-button.constants";
  import { IconName } from "../../lib/icons.constants";

  // ! A campaign or badge link goes through the catalogue by id, never through
  // the raw `url`: the backend resolves it from its own data, so a caller can
  // never point the button anywhere else.
  type ResolveType =
    | { kind: ResolveKind.CAMPAIGN; accountId: string; campaignId: string }
    | { kind: ResolveKind.BADGE; badgeKey: string };

  type ExternalButtonPropsType = {
    url: string;
    label: string;
    variant: ButtonVariant;
    size: ButtonSize;
    icon?: IconName;
    ariaLabel?: string;
    class?: string;
    resolve?: ResolveType;
    onfollow?: () => void;
  };

  let {
    url,
    label,
    variant,
    size,
    icon,
    ariaLabel,
    class: className,
    resolve,
    onfollow,
  }: ExternalButtonPropsType = $props();

  const iconSize = $derived(BUTTON_ICON_SIZES[size]);

  const follow = (event: MouseEvent, linkUrl: string): void => {
    if (resolve?.kind === ResolveKind.CAMPAIGN) {
      followCampaignLink(event, resolve.accountId, resolve.campaignId, linkUrl);
    } else if (resolve?.kind === ResolveKind.BADGE) {
      followBadgeLink(event, resolve.badgeKey, linkUrl);
    } else {
      followLink(event, linkUrl);
    }
  };
</script>

<Button
  href={url}
  rel={EXTERNAL_LINK_REL}
  variant={BUTTON_VARIANTS[variant]}
  size={BUTTON_SIZES[size]}
  class={cn(
    "group/external no-underline hover:no-underline",
    BUTTON_VARIANT_CLASSES[variant],
    className,
  )}
  aria-label={ariaLabel}
  onclick={externalLinkOnClick(url, follow, onfollow)}
>
  {#if icon}<Icon name={icon} size={iconSize} />{/if}
  <span class="truncate">{label}</span>
  <!-- ! Like a text link: the external mark shows on hover or focus only, its room kept so the label never moves. -->
  <span
    class="shrink-0 opacity-0 transition-opacity group-hover/external:opacity-70 group-focus-visible/external:opacity-70"
  >
    <Icon name={IconName.EXTERNAL} size={iconSize - 1} />
  </span>
</Button>
