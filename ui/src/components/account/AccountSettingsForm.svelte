<script lang="ts">
  import { maxWatchedChannelsParams } from "../../lib/accounts/slots.utils";
  import type { MiningLaneType } from "../../lib/bindings/MiningLaneType";
  import { Mode } from "../../lib/bindings/Mode";
  import { t } from "../../lib/i18n/i18n.svelte";
  import ChoiceCards from "../ui/ChoiceCards.svelte";
  import Panel from "../ui/Panel.svelte";
  import ProxyField from "../ui/ProxyField.svelte";
  import { proxyHint } from "../settings/settings.utils";
  import MiningOrderList from "./drops/MiningOrderList.svelte";

  type AccountSettingsFormPropsType = {
    mode: Mode;
    onModeChange: (mode: Mode) => void;
    miningOrder: MiningLaneType[];
    onMiningOrderChange: (order: MiningLaneType[]) => void;
    slots: number;
    onSlotsChange: (slots: number) => void;
    proxy: string;
    onProxyChange: (proxy: string) => void;
    proxyPlaceholder: string;
    // ! Singular by default (one account's own settings page); the bulk
    // import step passes how many accounts these settings will apply to.
    accountCount?: number;
    // ! The dialog can open over the account settings page: a shared id
    // would make the label jump to the page's own field underneath.
    idPrefix?: string;
  };

  let {
    mode,
    onModeChange,
    miningOrder,
    onMiningOrderChange,
    slots,
    onSlotsChange,
    proxy,
    onProxyChange,
    proxyPlaceholder,
    accountCount = 1,
    idPrefix = "account",
  }: AccountSettingsFormPropsType = $props();

  const watchedChannels = maxWatchedChannelsParams();
</script>

<Panel title={t.account_mode_title()} description={t.account_mode_desc({ count: accountCount })}>
  <ChoiceCards
    vertical={true}
    value={mode}
    label={t.account_mode_title()}
    onchange={onModeChange}
    choices={[
      { value: Mode.AUTO, title: t.mode_auto_title(), description: t.mode_auto_hint() },
      { value: Mode.DROPS, title: t.mode_drops_title(), description: t.mode_drops_hint() },
      { value: Mode.POINTS, title: t.mode_points_title(), description: t.mode_points_hint() },
    ]}
  />
</Panel>
<Panel title={t.mining_order_title()} description={t.mining_order_hint()}>
  <MiningOrderList order={miningOrder} onchange={onMiningOrderChange} />
</Panel>
<Panel title={t.account_slots_title()} description={t.account_slots_hint(watchedChannels)}>
  <ChoiceCards
    vertical={false}
    value={slots}
    label={t.account_slots_title()}
    onchange={onSlotsChange}
    choices={[
      {
        value: watchedChannels.maxWatchedChannels,
        title: t.slots_max_title(),
        description: t.slots_max_hint(watchedChannels),
      },
      ...(watchedChannels.maxWatchedChannels > 1
        ? [{ value: 1, title: t.slots_one_title(), description: t.slots_one_hint() }]
        : []),
    ]}
  />
</Panel>

<Panel title={t.settings_proxy()} description={t.account_proxy_hint({ count: accountCount })}>
  <ProxyField
    id="{idPrefix}-proxy"
    label={t.account_proxy({ count: accountCount })}
    hint={proxyHint()}
    {proxy}
    {onProxyChange}
    {proxyPlaceholder}
  />
</Panel>
