<script lang="ts">
  import type { MinerKind } from "../../lib/bindings/MinerKind";
  import { MINER_ICONS, MINER_TONES } from "../../lib/events/events.constants";
  import { MINING_BADGE_HINTS, MINING_BADGE_TEXTS } from "../../lib/events/labels.utils";
  import Badge from "./Badge.svelte";

  type MiningBadgePropsType = { kind: MinerKind };

  let { kind }: MiningBadgePropsType = $props();
</script>

<Badge
  text={MINING_BADGE_TEXTS[kind]()}
  tone={MINER_TONES[kind]}
  icon={MINER_ICONS[kind]}
  hint={MINING_BADGE_HINTS[kind]()}
/>
