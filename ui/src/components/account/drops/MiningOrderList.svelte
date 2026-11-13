<script lang="ts">
  import { MINING_LANE_HINTS, MINING_LANE_LABELS } from "../../../lib/accounts/mining-order.utils";
  import { MiningLaneKind } from "../../../lib/bindings/MiningLaneKind";
  import type { MiningLaneType } from "../../../lib/bindings/MiningLaneType";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import Badge from "../../ui/Badge.svelte";
  import Icon from "../../ui/Icon.svelte";
  import SortableList from "../../ui/SortableList.svelte";
  import Toggle from "../../ui/Toggle.svelte";
  import { BadgeTone, IconName } from "../../ui/ui.constants";

  type MiningOrderListPropsType = {
    order: MiningLaneType[];
    onchange: (order: MiningLaneType[]) => void;
  };

  let { order, onchange }: MiningOrderListPropsType = $props();

  const LANE_ICONS: Record<MiningLaneKind, IconName> = {
    [MiningLaneKind.PRIORITY_GAMES]: IconName.STAR,
    [MiningLaneKind.BADGES]: IconName.AWARD,
    [MiningLaneKind.OTHER_CAMPAIGNS]: IconName.GIFT,
  };

  const toggle = (kind: MiningLaneKind, enabled: boolean): void => {
    onchange(order.map((lane) => (lane.kind === kind ? { ...lane, enabled } : lane)));
  };
</script>

<SortableList
  items={order}
  key={(lane) => lane.kind}
  empty=""
  upLabel={t.common_up()}
  downLabel={t.common_down()}
  onreorder={(list) => onchange(list)}
>
  {#snippet row(lane)}
    <Icon
      name={LANE_ICONS[lane.kind]}
      size={18}
      class={lane.enabled ? "shrink-0 text-link" : "shrink-0 text-muted-foreground"}
    />
    <span class="flex min-w-0 flex-col gap-0.5">
      <span
        class={lane.enabled
          ? "text-sm font-semibold"
          : "text-sm font-semibold text-muted-foreground"}
        >{MINING_LANE_LABELS[lane.kind]()}{#if lane.kind === MiningLaneKind.BADGES}<span
            class="ms-1.5 align-middle"
            ><Badge text={t.common_beta()} tone={BadgeTone.ACCENT} dot={false} /></span
          >{/if}</span
      >
      <span class="text-xs text-muted-foreground">{MINING_LANE_HINTS[lane.kind]()}</span>
    </span>
  {/snippet}
  {#snippet actions(lane)}
    <span class="mr-1.5 flex items-center">
      <Toggle
        checked={lane.enabled}
        disabled={false}
        label={t.mining_lane_switch({ lane: MINING_LANE_LABELS[lane.kind]() })}
        onchange={(value) => toggle(lane.kind, value)}
      />
    </span>
  {/snippet}
</SortableList>
