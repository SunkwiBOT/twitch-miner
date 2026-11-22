<script lang="ts">
  import { LinkFacet } from "../../lib/accounts/campaign-groups.constants";
  import { LINK_UNKNOWN_VIEW, NOT_LINKED_VIEW } from "../../lib/badge-views.constants";
  import { badgeProps } from "../../lib/badge-views.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Badge from "../ui/Badge.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { IconName } from "../../lib/icons.constants";

  // ! `linkFacet` gives the state: the same rule the core ranks "linked first" by.
  type LinkBadgePropsType = { link: LinkFacet };

  let { link }: LinkBadgePropsType = $props();
</script>

{#if link === LinkFacet.LINKED}
  <Badge
    text={t.dashboard_linked()}
    tone={BadgeTone.SUCCESS}
    icon={IconName.LINK}
    hint={t.badge_hint_linked()}
  />
{:else if link === LinkFacet.NOT_LINKED}
  <Badge {...badgeProps(NOT_LINKED_VIEW)} />
{:else if link === LinkFacet.UNKNOWN}
  <Badge {...badgeProps(LINK_UNKNOWN_VIEW)} />
{/if}
