<script lang="ts">
  import AccountSaveBar from "./AccountSaveBar.svelte";
  import {
    CopySection,
    DROPS_PAGE_TABS,
    DROPS_TAB_SLUGS,
    DropsTab,
  } from "../../lib/accounts/accounts.constants";
  import { accountCampaigns } from "../../lib/accounts/campaign-groups.utils";
  import { groupOptions } from "../../lib/accounts/campaign-states.utils";
  import { ignoredItems } from "../../lib/accounts/drops.utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { campaignsOf } from "../../lib/catalogue/catalogue.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { openTab, showTab } from "../../lib/state/router";
  import TextButton from "../ui/TextButton.svelte";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import CopyFromDialog from "./CopyFromDialog.svelte";
  import DropsExclusionsTab from "./drops/DropsExclusionsTab.svelte";
  import DropsOrderTab from "./drops/DropsOrderTab.svelte";
  import DropsSettingsTab from "./drops/DropsSettingsTab.svelte";
  import RewardsTab from "./rewards/RewardsTab.svelte";

  type AccountDropsPropsType = { account: AccountSnapshotType };

  let { account }: AccountDropsPropsType = $props();

  const drops = $derived(account.settings.drops);
  const hasOthers = $derived((app.snapshot?.accounts.length ?? 0) > 1);
  const ignored = $derived(ignoredItems(account));
  const groups = $derived(accountCampaigns(account, campaignsOf(account)));

  const tab = $derived(openTab(DROPS_PAGE_TABS, DROPS_TAB_SLUGS, DropsTab.ORDER));
  let copying = $state(false);
  let open = $state<CampaignType | null>(null);

  const openCampaign = (campaign: CampaignType): void => {
    open = campaign;
  };
</script>

<PageBody>
  <div class="flex flex-wrap items-center justify-between gap-3">
    <Segmented
      value={tab}
      label={t.drops_tabs_label()}
      onchange={(next) => showTab(DROPS_TAB_SLUGS, next)}
      options={[
        {
          value: DropsTab.ORDER,
          label: t.queue_title(),
          icon: IconName.LIST_ORDERED,
          count: account.queue.length,
        },
        { value: DropsTab.SETTINGS, label: t.drops_tab_settings(), icon: IconName.SETTINGS },
        {
          value: DropsTab.EXCLUSIONS,
          label: t.drops_tab_exclusions(),
          icon: IconName.EYE_OFF,
          count: ignored.length + drops.exclude_games.length + drops.exclude_names.length,
        },
        ...groupOptions(groups),
      ]}
    />
    {#if hasOthers}
      <TextButton
        label={t.copy_drops_from_account()}
        icon={IconName.COPY}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => (copying = true)}
      />
    {/if}
  </div>

  {#if tab === DropsTab.ORDER}
    <DropsOrderTab {account} onopen={openCampaign} />
  {:else if tab === DropsTab.SETTINGS}
    <DropsSettingsTab {account} />
  {:else if tab === DropsTab.EXCLUSIONS}
    <DropsExclusionsTab {account} {ignored} onopen={openCampaign} />
  {:else}
    {#key tab}
      <RewardsTab
        {account}
        group={tab}
        entries={groups[tab]}
        icon={IconName.GIFT}
        onopen={openCampaign}
      />
    {/key}
  {/if}
  <AccountSaveBar {account} />
</PageBody>

<CampaignDialog campaign={open} contextAccountId={account.id} onclose={() => (open = null)} />

<CopyFromDialog
  open={copying}
  {account}
  section={CopySection.DROPS}
  onclose={() => (copying = false)}
/>
