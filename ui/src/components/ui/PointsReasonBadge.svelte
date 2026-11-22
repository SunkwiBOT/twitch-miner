<script lang="ts">
  import {
    POINTS_REASON_HINTS,
    POINTS_REASON_ICONS,
    POINTS_REASON_LABELS,
    POINTS_REASON_TONES,
  } from "../../lib/accounts/accounts.utils";
  import type { PointsReason } from "../../lib/bindings/PointsReason";
  import Badge from "./Badge.svelte";

  type PointsReasonBadgePropsType = { reason: PointsReason };

  let { reason }: PointsReasonBadgePropsType = $props();
</script>

<Badge
  text={POINTS_REASON_LABELS[reason]()}
  tone={POINTS_REASON_TONES[reason]}
  icon={POINTS_REASON_ICONS[reason]}
  hint={POINTS_REASON_HINTS[reason]()}
/>
