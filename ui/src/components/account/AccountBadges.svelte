<script lang="ts">
  import PanelColumns from "../ui/PanelColumns.svelte";
  import { untrack } from "svelte";

  import {
    BADGES_GROUP_SLUGS,
    BADGES_GROUPS,
    BadgesGroup,
    REWARDS_PAGE_TAB_SLUGS,
    RewardsPageTab,
  } from "../../lib/accounts/accounts.constants";
  import { CampaignGroup } from "../../lib/accounts/campaign-groups.constants";
  import { accountCampaigns, visibleGroups } from "../../lib/accounts/campaign-groups.utils";
  import { groupOptions, groupsCount } from "../../lib/accounts/campaign-states.utils";
  import { laneEnabled } from "../../lib/accounts/mining-order.utils";
  import { accountBadges, ownedBadges } from "../../lib/badges/badges.utils";
  import { badgeList } from "../../lib/badges/badges.svelte";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { MiningLaneKind } from "../../lib/bindings/MiningLaneKind";
  import { campaignGivesBadge } from "../../lib/catalogue/campaign.utils";
  import { campaignsOf } from "../../lib/catalogue/catalogue.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { editsOf } from "../../lib/state/edits.svelte";
  import { AccountField } from "../../lib/state/state.constants";
  import { openTab, showTab } from "../../lib/state/router";
  import { openBadgeSheet, openCampaignSheet } from "../../lib/state/sheets";
  import { ticker } from "../../lib/state/ticker.svelte";
  import Notice from "../ui/Notice.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Panel from "../ui/Panel.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import Setting from "../ui/Setting.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { NoticeTone, PanelWidth } from "../ui/ui.constants";
  import AccountSaveBar from "./AccountSaveBar.svelte";
  import BadgesExclusionsTab from "./badges/BadgesExclusionsTab.svelte";
  import BadgesOrderTab from "./badges/BadgesOrderTab.svelte";
  import OwnedBadgesTab from "./badges/OwnedBadgesTab.svelte";
  import CampaignScopeFields from "./drops/CampaignScopeFields.svelte";
  import MiningOrderList from "./drops/MiningOrderList.svelte";
  import RewardsTab from "./rewards/RewardsTab.svelte";

  type AccountBadgesPropsType = { account: AccountSnapshotType };

  let { account }: AccountBadgesPropsType = $props();

  const edits = editsOf(untrack(() => account.id));
  const badges = $derived(edits.applied(account.settings).badges);

  const sections = $derived(accountBadges(account, ticker.now));
  const owned = $derived(ownedBadges(account, badgeList.badges));
  const groups = $derived(
    accountCampaigns(account, campaignsOf(account).filter(campaignGivesBadge)),
  );

  let query = $state("");
  let hidden = $state<string[]>([]);

  const shownGroups = $derived(visibleGroups(account, groups, query, hidden));

  const badgesOn = $derived(
    laneEnabled(account.settings.drops.mining_order, MiningLaneKind.BADGES),
  );

  const tab = $derived(
    openTab(Object.values(RewardsPageTab), REWARDS_PAGE_TAB_SLUGS, RewardsPageTab.ORDER),
  );
  const group = $derived(openTab(BADGES_GROUPS, BADGES_GROUP_SLUGS, CampaignGroup.TO_MINE));

  const openCampaign = (campaign: CampaignType): void => openCampaignSheet(campaign, account.id);
</script>

<PageBody>
  <div class="flex flex-wrap items-center justify-between gap-3">
    <Segmented
      value={tab}
      label={t.account_badges_tabs_label()}
      onchange={(next) => showTab(REWARDS_PAGE_TAB_SLUGS, next)}
      options={[
        {
          value: RewardsPageTab.ORDER,
          label: t.queue_title(),
          icon: IconName.LIST_ORDERED,
          count: sections.minable.length,
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
          count:
            sections.ignored.length +
            account.settings.badges.exclude_games.length +
            account.settings.badges.exclude_names.length,
        },
      ]}
    />
  </div>
  {#if !account.badges_read_at}
    <Notice tone={NoticeTone.INFO} text={t.account_badges_not_read()} />
  {/if}
  {#if !badgesOn}
    <Notice tone={NoticeTone.WARNING} text={t.account_badges_mining_off()} />
  {/if}

  {#if tab === RewardsPageTab.ORDER}
    <BadgesOrderTab {account} {sections} onopen={openCampaign} />
  {:else if tab === RewardsPageTab.EXCLUSIONS}
    <BadgesExclusionsTab {account} {sections} onopen={openCampaign} />
  {:else if tab === RewardsPageTab.SETTINGS}
    <PanelColumns width={PanelWidth.LG}>
      <Panel title={t.mining_order_title()} description={t.mining_order_hint()}>
        <MiningOrderList
          order={edits.applied(account.settings).drops.mining_order}
          onchange={(next) =>
            edits.set(AccountField.MINING_ORDER, (s) => (s.drops.mining_order = next))}
        />
      </Panel>
      <Panel title={t.drops_scope()}>
        <CampaignScopeFields
          id="badges-order"
          scope={badges.scope}
          linkedFirst={badges.linked_first}
          order={badges.order}
          selectionOnly={badges.selection_only}
          selectionHint={t.badges_selection_only_hint()}
          onselectiononly={(value) =>
            edits.set(AccountField.BADGE_SELECTION_ONLY, (s) => (s.badges.selection_only = value))}
          onscope={(scope) => edits.set(AccountField.BADGE_SCOPE, (s) => (s.badges.scope = scope))}
          onlinkedfirst={(value) =>
            edits.set(AccountField.BADGE_LINKED_FIRST, (s) => (s.badges.linked_first = value))}
          onorder={(next) => edits.set(AccountField.BADGE_ORDER, (s) => (s.badges.order = next))}
        />
        <Setting
          title={t.badges_evolutions_title()}
          description={t.badges_evolutions_hint()}
          checked={badges.evolutions}
          disabled={false}
          onchange={(value) =>
            edits.set(AccountField.BADGE_EVOLUTIONS, (s) => (s.badges.evolutions = value))}
        />
      </Panel>
    </PanelColumns>
  {:else}
    <Segmented
      value={group}
      label={t.rewards_groups_label()}
      onchange={(next) => showTab(BADGES_GROUP_SLUGS, next)}
      options={[
        ...groupOptions(shownGroups),
        {
          value: BadgesGroup.OWNED,
          label: t.account_badges_owned(),
          icon: IconName.AWARD,
          count: owned.length,
        },
      ]}
    />
    {#if group === BadgesGroup.OWNED}
      <OwnedBadgesTab {account} {owned} onopen={(badge) => openBadgeSheet(badge, account.id)} />
    {:else}
      <RewardsTab
        {account}
        {group}
        entries={groups[group]}
        icon={IconName.AWARD}
        bind:query
        bind:hidden
        onopen={openCampaign}
      />
    {/if}
  {/if}
  <AccountSaveBar {account} />
</PageBody>
