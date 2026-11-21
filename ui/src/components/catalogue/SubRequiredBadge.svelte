<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import Badge from "../ui/Badge.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { IconName } from "../../lib/icons.constants";

  type SubRequiredBadgePropsType = { class?: string };

  let { class: className }: SubRequiredBadgePropsType = $props();
</script>

<Badge
  class={className}
  text={t.badge_progress_sub_required()}
  tone={BadgeTone.WARNING}
  dot={false}
  icon={IconName.ALERT}
  hint={t.badge_hint_sub_only()}
/>
