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
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
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
    campaignId: string | null;
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
  const campaignOrReward = $derived(strong(name));
  const idle = $derived(campaignId === null ? t.badge_no_running_campaign() : undefined);
  const unminable = $derived(idle ?? (minable ? undefined : t.campaign_nothing_to_mine()));

  const allOf = (ids: (account: AccountSnapshotType) => string[]): boolean =>
    campaignId !== null &&
    chosen.length > 0 &&
    chosen.every((account) => ids(account).includes(campaignId));

  const allIgnore = $derived(allOf((a) => a.settings.drops.exclude_campaign_ids));
  const allSelected = $derived(allOf((a) => a.settings.drops.selected_campaign_ids));

  const act = async (
    key: CampaignAction,
    text: string,
    change: (settings: AccountSettingsType, id: string) => void,
  ): Promise<void> => {
    if (campaignId === null) {
      return;
    }

    const id = campaignId;

    busy = key;
    await changeAccounts(selected, text, (settings) => change(settings, id));
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
    countLabel={(count) => t.common_accounts_count({ accountCount: strong(count) })}
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
  reason={idle}
  variant={ButtonVariant.DEFAULT}
  size={ButtonSize.MD}
  type={ButtonType.BUTTON}
  onclick={() =>
    allIgnore
      ? act(CampaignAction.RESTORE, t.change_unignored({ campaignOrReward }), (settings, id) =>
          setCampaignIgnoredIn(settings, id, false),
        )
      : act(CampaignAction.IGNORE, t.change_ignored({ campaignOrReward }), (settings, id) =>
          setCampaignIgnoredIn(settings, id, true),
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
      ? act(CampaignAction.UNSELECT, t.change_unselected({ campaignOrReward }), (settings, id) =>
          setCampaignSelectedIn(settings, id, false),
        )
      : act(CampaignAction.SELECT, t.change_selected({ campaignOrReward }), (settings, id) =>
          setCampaignSelectedIn(settings, id, true),
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
    act(CampaignAction.FIRST, t.change_prioritized({ campaignOrReward }), (settings, id) =>
      prioritizeIn(settings, id),
    )}
/>
