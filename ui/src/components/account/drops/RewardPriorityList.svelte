<script lang="ts">
  import {
    REWARD_KIND_HINTS,
    REWARD_KIND_LABELS,
  } from "../../../lib/accounts/reward-priority.utils";
  import { BenefitKind } from "../../../lib/bindings/BenefitKind";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import Icon from "../../ui/Icon.svelte";
  import SortableList from "../../ui/SortableList.svelte";
  import { IconName } from "../../../lib/icons.constants";

  type RewardPriorityListPropsType = {
    order: BenefitKind[];
    onchange: (order: BenefitKind[]) => void;
  };

  let { order, onchange }: RewardPriorityListPropsType = $props();

  const KIND_ICONS: Record<BenefitKind, IconName> = {
    [BenefitKind.BADGE]: IconName.AWARD,
    [BenefitKind.EMOTE]: IconName.STAR_FILLED,
    [BenefitKind.IN_GAME_ITEM]: IconName.PACKAGE,
    [BenefitKind.OTHER]: IconName.GIFT,
  };
</script>

<SortableList
  items={order}
  key={(kind) => kind}
  empty=""
  upLabel={t.common_up()}
  downLabel={t.common_down()}
  onreorder={(list) => onchange(list)}
>
  {#snippet row(kind)}
    <Icon name={KIND_ICONS[kind]} size={18} class="shrink-0 text-link" />
    <span class="flex min-w-0 flex-col gap-0.5">
      <span class="text-sm font-semibold">{REWARD_KIND_LABELS[kind]()}</span>
      <span class="text-xs text-muted-foreground">{REWARD_KIND_HINTS[kind]()}</span>
    </span>
  {/snippet}
</SortableList>
