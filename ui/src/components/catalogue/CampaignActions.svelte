<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { IconName } from "../../lib/icons.constants";
  import {
    prioritizeCampaign,
    setCampaignIgnored,
    setCampaignSelected,
  } from "../../lib/state/account-settings.svelte";
  import { app } from "../../lib/state/app.svelte";
  import ActionButton from "../ui/ActionButton.svelte";
  import MultiSelect from "../ui/MultiSelect.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { MenuAlign } from "../ui/menu.constants";
  import { CampaignAction } from "./catalogue.constants";

  type CampaignActionsPropsType = {
    campaignId: string;
    selected: string[];
    minable: boolean;
    busy: CampaignAction | null;
    onselect: (ids: string[]) => void;
  };

  let {
    campaignId,
    selected,
    minable,
    busy = $bindable(),
    onselect,
  }: CampaignActionsPropsType = $props();

  const accounts = $derived(app.snapshot?.accounts ?? []);
  const chosen = $derived(accounts.filter((a) => selected.includes(a.id)));

  const allIgnore = $derived(
    chosen.length > 0 &&
      chosen.every((a) => a.settings.drops.exclude_campaign_ids.includes(campaignId)),
  );

  const allSelected = $derived(
    chosen.length > 0 &&
      chosen.every((a) => a.settings.drops.selected_campaign_ids.includes(campaignId)),
  );

  const act = async (key: CampaignAction, action: () => Promise<unknown>): Promise<void> => {
    busy = key;
    await action();
    busy = null;
  };
</script>

<div class="w-56 sm:mr-auto">
  <MultiSelect
    values={selected}
    options={accounts.map((a) => ({ value: a.id, label: a.display_name, image: a.avatar_url }))}
    label={t.common_account()}
    allLabel={t.accounts_all()}
    noneLabel={t.accounts_none()}
    countLabel={(count) => t.accounts_count({ count })}
    align={MenuAlign.LEFT}
    onchange={onselect}
  />
</div>
<ActionButton
  icon={allIgnore ? IconName.EYE : IconName.EYE_OFF}
  label={allIgnore ? t.ignored_restore() : t.campaign_ignore()}
  busyLabel={t.common_saving()}
  busy={busy === CampaignAction.IGNORE}
  disabled={busy !== null || selected.length === 0}
  variant={ButtonVariant.DEFAULT}
  size={ButtonSize.MD}
  type={ButtonType.BUTTON}
  onclick={() =>
    act(CampaignAction.IGNORE, () => setCampaignIgnored(selected, campaignId, !allIgnore))}
/>
{#if minable}
  <ActionButton
    icon={allSelected ? IconName.STAR_FILLED : IconName.STAR}
    label={allSelected ? t.campaign_unselect() : t.campaign_select()}
    busyLabel={t.common_saving()}
    busy={busy === CampaignAction.SELECT}
    disabled={busy !== null || selected.length === 0}
    variant={ButtonVariant.DEFAULT}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    onclick={() =>
      act(CampaignAction.SELECT, () => setCampaignSelected(selected, campaignId, !allSelected))}
  />
  <ActionButton
    icon={IconName.ARROW_UP}
    label={t.campaign_prioritize()}
    busyLabel={t.common_saving()}
    busy={busy === CampaignAction.FIRST}
    disabled={busy !== null || selected.length === 0}
    variant={ButtonVariant.PRIMARY}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    onclick={() => act(CampaignAction.FIRST, () => prioritizeCampaign(selected, campaignId))}
  />
{/if}
