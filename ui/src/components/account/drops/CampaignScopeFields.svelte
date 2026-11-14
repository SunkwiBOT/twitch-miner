<script lang="ts">
  import { dropsScopeChoices } from "../../../lib/accounts/scope.utils";
  import { DropsOrder } from "../../../lib/bindings/DropsOrder";
  import type { DropsScope } from "../../../lib/bindings/DropsScope";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import ChoiceCards from "../../ui/ChoiceCards.svelte";
  import FormField from "../../ui/FormField.svelte";
  import Select from "../../ui/Select.svelte";
  import Setting from "../../ui/Setting.svelte";
  import { MenuAlign } from "../../ui/ui.constants";

  type CampaignScopeFieldsPropsType = {
    id: string;
    scope: DropsScope;
    linkedFirst: boolean;
    order: DropsOrder;
    selectionOnly: boolean;
    selectionHint: string;
    onselectiononly: (value: boolean) => void;
    onscope: (scope: DropsScope) => void;
    onlinkedfirst: (value: boolean) => void;
    onorder: (order: DropsOrder) => void;
  };

  let {
    id,
    scope,
    linkedFirst,
    order,
    selectionOnly,
    selectionHint,
    onselectiononly,
    onscope,
    onlinkedfirst,
    onorder,
  }: CampaignScopeFieldsPropsType = $props();
</script>

<ChoiceCards
  vertical={true}
  value={scope}
  label={t.drops_scope()}
  onchange={onscope}
  choices={dropsScopeChoices()}
/>
<Setting
  title={t.selection_only_title()}
  description={selectionHint}
  checked={selectionOnly}
  disabled={false}
  onchange={onselectiononly}
/>
<Setting
  title={t.drops_linked_first()}
  description={t.drops_linked_first_hint()}
  checked={linkedFirst}
  disabled={false}
  onchange={onlinkedfirst}
/>
<FormField {id} label={t.drops_order()}>
  <Select
    {id}
    value={order}
    label={t.drops_order()}
    align={MenuAlign.LEFT}
    onchange={onorder}
    options={[
      {
        value: DropsOrder.ENDING_SOONEST,
        label: t.drops_order_ending_soonest(),
        description: t.drops_order_ending_hint(),
      },
      {
        value: DropsOrder.LOWEST_REMAINING,
        label: t.drops_order_lowest_remaining(),
        description: t.drops_order_lowest_hint(),
      },
    ]}
  />
</FormField>
