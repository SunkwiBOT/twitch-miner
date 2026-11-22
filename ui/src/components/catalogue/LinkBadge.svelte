<script lang="ts">
  import { NOT_LINKED_VIEW } from "../../lib/badge-views.constants";
  import { badgeProps } from "../../lib/badge-views.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Badge from "../ui/Badge.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { IconName } from "../../lib/icons.constants";

  // ! `needed` with no known `linked`: the campaign asks for a link, no account says whether it has one.
  type LinkBadgePropsType = { linked: boolean | null | undefined; needed?: boolean };

  let { linked, needed = false }: LinkBadgePropsType = $props();
</script>

{#if linked === true}
  <Badge
    text={t.dashboard_linked()}
    tone={BadgeTone.SUCCESS}
    icon={IconName.LINK}
    hint={t.badge_hint_linked()}
  />
{:else if linked === false}
  <Badge {...badgeProps(NOT_LINKED_VIEW)} />
{:else if needed}
  <Badge
    text={t.catalogue_link_required()}
    tone={BadgeTone.MUTED}
    icon={IconName.LINK}
    hint={t.badge_hint_link_required()}
  />
{/if}
