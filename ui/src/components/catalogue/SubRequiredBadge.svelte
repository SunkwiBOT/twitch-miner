<script lang="ts">
  import { SUB_REQUIRED_VIEW } from "../../lib/badge-views.constants";
  import { badgeProps } from "../../lib/badge-views.utils";
  import Badge from "../ui/Badge.svelte";

  type SubRequiredBadgePropsType = { class?: string };

  let { class: className }: SubRequiredBadgePropsType = $props();
</script>

<Badge class={className} {...badgeProps(SUB_REQUIRED_VIEW)} />
