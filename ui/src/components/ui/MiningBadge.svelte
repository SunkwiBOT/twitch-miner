<script lang="ts">
  import { MinerKind } from "../../lib/bindings/MinerKind";
  import { MINER_ICONS, MINER_TONES } from "../../lib/events/events.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Badge from "./Badge.svelte";

  type MiningBadgePropsType = { kind: MinerKind };

  let { kind }: MiningBadgePropsType = $props();

  const TEXTS: Record<MinerKind, { text: () => string; hint: () => string }> = {
    [MinerKind.DROPS]: {
      text: () => t.badge_mining_drops(),
      hint: () => t.badge_hint_mining_drops(),
    },
    [MinerKind.BADGES]: {
      text: () => t.badge_mining_badges(),
      hint: () => t.badge_hint_mining_badges(),
    },
    [MinerKind.POINTS]: {
      text: () => t.badge_mining_points(),
      hint: () => t.badge_hint_mining_points(),
    },
  };
</script>

<Badge
  text={TEXTS[kind].text()}
  tone={MINER_TONES[kind]}
  dot={false}
  icon={MINER_ICONS[kind]}
  hint={TEXTS[kind].hint()}
/>
