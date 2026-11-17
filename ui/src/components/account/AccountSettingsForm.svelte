<script lang="ts">
  import { SLOTS_MAX } from "../../lib/accounts/slots.constants";
  import { testProxy } from "../../lib/api/client";
  import type { MiningLaneType } from "../../lib/bindings/MiningLaneType";
  import { Mode } from "../../lib/bindings/Mode";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { reportFailure } from "../../lib/state/session";
  import { toast } from "../../lib/state/toasts";
  import { strong } from "../../lib/text.utils";
  import ActionButton from "../ui/ActionButton.svelte";
  import ChoiceCards from "../ui/ChoiceCards.svelte";
  import FormField from "../ui/FormField.svelte";
  import Panel from "../ui/Panel.svelte";
  import SecretInput from "../ui/SecretInput.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
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

  let testing = $state(false);

  const checkProxy = async (): Promise<void> => {
    testing = true;

    try {
      toast(ToastKind.SUCCESS, t.settings_proxy_ok_title(), {
        description: t.settings_proxy_ok({ ms: strong(await testProxy(proxy)) }),
      });
    } catch (e: unknown) {
      reportFailure(e);
    } finally {
      testing = false;
    }
  };
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
<Panel title={t.account_slots_title()} description={t.account_slots_hint()}>
  <ChoiceCards
    vertical={false}
    value={slots}
    label={t.account_slots_title()}
    onchange={onSlotsChange}
    choices={[
      { value: SLOTS_MAX, title: t.slots_two_title(), description: t.slots_two_hint() },
      { value: 1, title: t.slots_one_title(), description: t.slots_one_hint() },
    ]}
  />
</Panel>

<Panel title={t.settings_proxy()} description={t.account_proxy_hint({ count: accountCount })}>
  <FormField
    id="{idPrefix}-proxy"
    label={t.account_proxy({ count: accountCount })}
    hint={t.settings_proxy_hint()}
  >
    <SecretInput
      id="{idPrefix}-proxy"
      class="font-mono"
      autocomplete="off"
      value={proxy}
      oninput={(event) => onProxyChange(event.currentTarget.value.trim())}
      placeholder={proxyPlaceholder}
    />
  </FormField>
  <div class="flex flex-wrap justify-end gap-2">
    <ActionButton
      icon={IconName.GLOBE}
      label={t.settings_proxy_test()}
      busyLabel={t.common_testing()}
      busy={testing}
      disabled={!proxy}
      variant={ButtonVariant.DEFAULT}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={checkProxy}
    />
  </div>
</Panel>
