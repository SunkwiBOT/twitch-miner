<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import { followBadgeLink, followCampaignLink, followLink } from "../../lib/links/links.utils";
  import Icon from "./Icon.svelte";
  import {
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

  const iconSize = $derived(size === ButtonSize.SM ? 13 : 14);

  const follow = (event: MouseEvent): void => {
    if (resolve?.kind === ResolveKind.CAMPAIGN) {
      followCampaignLink(event, resolve.accountId, resolve.campaignId, url);
    } else if (resolve?.kind === ResolveKind.BADGE) {
      followBadgeLink(event, resolve.badgeKey, url);
    } else {
      followLink(event, url);
    }
  };
</script>

<Button
  href={url}
  rel="noreferrer"
  variant={BUTTON_VARIANTS[variant]}
  size={BUTTON_SIZES[size]}
  class={cn("no-underline hover:no-underline", BUTTON_VARIANT_CLASSES[variant], className)}
  aria-label={ariaLabel}
  onclick={(event: MouseEvent) => {
    follow(event);
    onfollow?.();
  }}
>
  {#if icon}<Icon name={icon} size={iconSize} />{/if}
  <span class="truncate">{label}</span>
  <span class="shrink-0 opacity-70"><Icon name={IconName.EXTERNAL} size={iconSize - 1} /></span>
</Button>
