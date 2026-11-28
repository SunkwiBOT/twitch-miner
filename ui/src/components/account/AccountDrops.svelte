<script lang="ts">
  import AccountSaveBar from "./AccountSaveBar.svelte";
  import {
    CopySection,
    REWARDS_PAGE_TAB_SLUGS,
    RewardsPageTab,
  } from "../../lib/accounts/accounts.constants";
  import {
    CAMPAIGN_GROUP_SLUGS,
    CAMPAIGN_GROUPS,
    CampaignGroup,
  } from "../../lib/accounts/campaign-groups.constants";
  import { accountCampaigns, visibleGroups } from "../../lib/accounts/campaign-groups.utils";
  import { groupOptions, groupsCount } from "../../lib/accounts/campaign-states.utils";
  import { ignoredItems } from "../../lib/accounts/drops.utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { campaignsOf } from "../../lib/catalogue/catalogue.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { openTab, showTab } from "../../lib/state/router";
  import { openCampaignSheet } from "../../lib/state/sheets";
  import { strong } from "../../lib/text.utils";
  import TextButton from "../ui/TextButton.svelte";
  import Notice from "../ui/Notice.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { NoticeTone } from "../ui/ui.constants";
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

  let query = $state("");
  let hidden = $state<string[]>([]);

  const shownGroups = $derived(visibleGroups(account, groups, query, hidden));

  const tab = $derived(
    openTab(Object.values(RewardsPageTab), REWARDS_PAGE_TAB_SLUGS, RewardsPageTab.ORDER),
  );
  const group = $derived(openTab(CAMPAIGN_GROUPS, CAMPAIGN_GROUP_SLUGS, CampaignGroup.TO_MINE));
  let copying = $state(false);

  const openCampaign = (campaign: CampaignType): void => openCampaignSheet(campaign, account.id);
</script>

<PageBody>
  {#if account.drops_held}
    <Notice
      tone={NoticeTone.DANGER}
      text={t.account_drops_hold({ count: strong(account.drops_held) })}
    />
  {/if}
  <div class="flex flex-wrap items-center justify-between gap-3">
    <Segmented
      value={tab}
      label={t.drops_tabs_label()}
      onchange={(next) => showTab(REWARDS_PAGE_TAB_SLUGS, next)}
      options={[
        {
          value: RewardsPageTab.ORDER,
          label: t.queue_title(),
          icon: IconName.LIST_ORDERED,
          count: account.queue.length,
        },
        {
          value: RewardsPageTab.CAMPAIGNS,
          label: t.drops_tab_campaigns(),
          icon: IconName.GRID,
          count: groupsCount(shownGroups),
        },
        { value: RewardsPageTab.SETTINGS, label: t.drops_tab_settings(), icon: IconName.SETTINGS },
        {
          value: RewardsPageTab.EXCLUSIONS,
          label: t.drops_tab_exclusions(),
          icon: IconName.EYE_OFF,
          count: ignored.length + drops.exclude_games.length + drops.exclude_names.length,
        },
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

  {#if tab === RewardsPageTab.ORDER}
    <DropsOrderTab {account} onopen={openCampaign} />
  {:else if tab === RewardsPageTab.SETTINGS}
    <DropsSettingsTab {account} />
  {:else if tab === RewardsPageTab.EXCLUSIONS}
    <DropsExclusionsTab {account} {ignored} onopen={openCampaign} />
  {:else}
    <Segmented
      value={group}
      label={t.rewards_groups_label()}
      onchange={(next) => showTab(CAMPAIGN_GROUP_SLUGS, next)}
      options={groupOptions(shownGroups)}
    />
    <RewardsTab
      {account}
      {group}
      entries={groups[group]}
      icon={IconName.GIFT}
      bind:query
      bind:hidden
      onopen={openCampaign}
    />
  {/if}
  <AccountSaveBar {account} />
</PageBody>

<CopyFromDialog
  open={copying}
  {account}
  section={CopySection.DROPS}
  onclose={() => (copying = false)}
/>
