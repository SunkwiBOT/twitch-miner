<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { accountOptions } from "../../lib/accounts/account-filter.utils";
  import {
    prioritizeIn,
    setCampaignIgnoredIn,
    setCampaignSelectedIn,
  } from "../../lib/accounts/changes.utils";
  import type { AccountSettingsType } from "../../lib/bindings/AccountSettingsType";
  import { app } from "../../lib/state/app.svelte";
  import { changeAccounts } from "../../lib/state/bulk.svelte";
  import { strong } from "../../lib/text.utils";
  import ActionButton from "../ui/ActionButton.svelte";
  import MultiSelect from "../ui/MultiSelect.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { MenuAlign } from "../ui/menu.constants";
  import { CampaignAction } from "./catalogue.constants";
  import { busyLabel } from "./catalogue.utils";

  type CampaignActionsPropsType = {
    campaignId: string;
    name: string;
    selected: string[];
    minable: boolean;
    busy: CampaignAction | null;
    onselect: (ids: string[]) => void;
  };

  let {
    campaignId,
    name,
    selected,
    minable,
    busy = $bindable(),
    onselect,
  }: CampaignActionsPropsType = $props();

  const accounts = $derived(app.snapshot?.accounts ?? []);
  const chosen = $derived(accounts.filter((a) => selected.includes(a.id)));
  const item = $derived(strong(name));
  const unminable = $derived(minable ? undefined : t.campaign_nothing_to_mine());

  const allIgnore = $derived(
    chosen.length > 0 &&
      chosen.every((a) => a.settings.drops.exclude_campaign_ids.includes(campaignId)),
  );

  const allSelected = $derived(
    chosen.length > 0 &&
      chosen.every((a) => a.settings.drops.selected_campaign_ids.includes(campaignId)),
  );

  const act = async (
    key: CampaignAction,
    text: string,
    change: (settings: AccountSettingsType) => void,
  ): Promise<void> => {
    busy = key;
    await changeAccounts(selected, text, change);
    busy = null;
  };
</script>

<div class="w-full sm:mr-auto sm:w-56">
  <MultiSelect
    values={selected}
    options={accountOptions(accounts)}
    label={t.common_account()}
    allLabel={t.common_accounts_all()}
    noneLabel={t.common_accounts_none()}
    countLabel={(count) => t.common_accounts_count({ count })}
    align={MenuAlign.LEFT}
    onchange={onselect}
  />
</div>
<ActionButton
  icon={allIgnore ? IconName.EYE : IconName.EYE_OFF}
  label={allIgnore ? t.ignored_restore() : t.campaign_ignore()}
  busyLabel={busy ? busyLabel(busy) : ""}
  busy={busy === CampaignAction.IGNORE || busy === CampaignAction.RESTORE}
  disabled={busy !== null || selected.length === 0}
  variant={ButtonVariant.DEFAULT}
  size={ButtonSize.MD}
  type={ButtonType.BUTTON}
  onclick={() =>
    allIgnore
      ? act(CampaignAction.RESTORE, t.change_unignored({ item }), (settings) =>
          setCampaignIgnoredIn(settings, campaignId, false),
        )
      : act(CampaignAction.IGNORE, t.change_ignored({ item }), (settings) =>
          setCampaignIgnoredIn(settings, campaignId, true),
        )}
/>
<ActionButton
  icon={allSelected ? IconName.STAR_FILLED : IconName.STAR}
  label={allSelected ? t.campaign_unselect() : t.campaign_select()}
  busyLabel={busy ? busyLabel(busy) : ""}
  busy={busy === CampaignAction.SELECT || busy === CampaignAction.UNSELECT}
  disabled={busy !== null || selected.length === 0}
  reason={unminable}
  variant={ButtonVariant.DEFAULT}
  size={ButtonSize.MD}
  type={ButtonType.BUTTON}
  onclick={() =>
    allSelected
      ? act(CampaignAction.UNSELECT, t.change_unselected({ item }), (settings) =>
          setCampaignSelectedIn(settings, campaignId, false),
        )
      : act(CampaignAction.SELECT, t.change_selected({ item }), (settings) =>
          setCampaignSelectedIn(settings, campaignId, true),
        )}
/>
<ActionButton
  icon={IconName.ARROW_UP}
  label={t.campaign_prioritize()}
  busyLabel={busy ? busyLabel(busy) : ""}
  busy={busy === CampaignAction.FIRST}
  disabled={busy !== null || selected.length === 0}
  reason={unminable}
  variant={ButtonVariant.PRIMARY}
  size={ButtonSize.MD}
  type={ButtonType.BUTTON}
  onclick={() =>
    act(CampaignAction.FIRST, t.change_prioritized({ item }), (settings) =>
      prioritizeIn(settings, campaignId),
    )}
/>
