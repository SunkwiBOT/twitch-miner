<script lang="ts">
  import { PanelWidth } from "../../ui/ui.constants";
  import PanelColumns from "../../ui/PanelColumns.svelte";
  import { untrack } from "svelte";

  import { editsOf } from "../../../lib/state/edits.svelte";
  import { AccountField } from "../../../lib/state/state.constants";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import Panel from "../../ui/Panel.svelte";
  import Setting from "../../ui/Setting.svelte";
  import CampaignScopeFields from "./CampaignScopeFields.svelte";
  import MiningOrderList from "./MiningOrderList.svelte";

  type DropsSettingsTabPropsType = { account: AccountSnapshotType };

  let { account }: DropsSettingsTabPropsType = $props();

  const edits = editsOf(untrack(() => account.id));
  const drops = $derived(edits.applied(account.settings).drops);
</script>

<PanelColumns width={PanelWidth.LG}>
  <Panel title={t.mining_order_title()} description={t.mining_order_hint()}>
    <MiningOrderList
      order={drops.mining_order}
      onchange={(order) =>
        edits.set(AccountField.MINING_ORDER, (s) => (s.drops.mining_order = order))}
    />
  </Panel>
  <Panel title={t.drops_claim_title()}>
    <div>
      <Setting
        title={t.account_drops_auto_claim()}
        description={t.drops_auto_claim_hint()}
        checked={drops.auto_claim}
        disabled={false}
        onchange={(value) =>
          edits.set(AccountField.AUTO_CLAIM, (s) => (s.drops.auto_claim = value))}
      />
      <Setting
        title={t.account_drops_skip_unfinishable()}
        description={t.drops_skip_hint()}
        checked={drops.skip_unfinishable}
        disabled={false}
        onchange={(value) =>
          edits.set(AccountField.SKIP_UNFINISHABLE, (s) => (s.drops.skip_unfinishable = value))}
      />
      <Setting
        title={t.drops_mine_points()}
        description={t.drops_mine_points_hint()}
        checked={drops.mine_points}
        disabled={false}
        onchange={(value) =>
          edits.set(AccountField.MINE_POINTS, (s) => (s.drops.mine_points = value))}
      />
      <Setting
        title={t.drops_points_followed_only()}
        description={t.drops_points_followed_only_hint()}
        checked={drops.points_followed_only}
        disabled={!drops.mine_points}
        onchange={(value) =>
          edits.set(
            AccountField.POINTS_FOLLOWED_ONLY,
            (s) => (s.drops.points_followed_only = value),
          )}
      />
      <Setting
        title={t.drops_streamers_first()}
        description={t.drops_streamers_first_hint()}
        checked={drops.streamers_first}
        disabled={false}
        onchange={(value) =>
          edits.set(AccountField.STREAMERS_FIRST, (s) => (s.drops.streamers_first = value))}
      />
    </div>
  </Panel>
  <Panel title={t.drops_scope()}>
    <CampaignScopeFields
      id="drops-order"
      scope={drops.scope}
      linkedFirst={drops.linked_first}
      order={drops.order}
      selectionOnly={drops.selection_only}
      selectionHint={t.drops_selection_only_hint()}
      onselectiononly={(value) =>
        edits.set(AccountField.SELECTION_ONLY, (s) => (s.drops.selection_only = value))}
      onscope={(scope) => edits.set(AccountField.SCOPE, (s) => (s.drops.scope = scope))}
      onlinkedfirst={(value) =>
        edits.set(AccountField.LINKED_FIRST, (s) => (s.drops.linked_first = value))}
      onorder={(order) => edits.set(AccountField.ORDER, (s) => (s.drops.order = order))}
    />
  </Panel>
</PanelColumns>
