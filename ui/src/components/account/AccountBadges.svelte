<script lang="ts">
  import PanelColumns from "../ui/PanelColumns.svelte";
  import { untrack } from "svelte";

  import {
    BADGES_PAGE_TABS,
    BADGES_TAB_SLUGS,
    BadgesTab,
  } from "../../lib/accounts/accounts.constants";
  import { accountCampaigns } from "../../lib/accounts/campaign-groups.utils";
  import { groupOptions } from "../../lib/accounts/campaign-states.utils";
  import { laneEnabled } from "../../lib/accounts/mining-order.utils";
  import { accountBadges, ownedBadges } from "../../lib/badges/badges.utils";
  import { badgeList } from "../../lib/badges/badges.svelte";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import type { GlobalBadgeType } from "../../lib/bindings/GlobalBadgeType";
  import { MiningLaneKind } from "../../lib/bindings/MiningLaneKind";
  import { campaignGivesBadge } from "../../lib/catalogue/campaign.utils";
  import { campaignsOf } from "../../lib/catalogue/catalogue.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { editsOf } from "../../lib/state/edits.svelte";
  import { AccountField } from "../../lib/state/state.constants";
  import { openTab, showTab } from "../../lib/state/router";
  import { ticker } from "../../lib/state/ticker.svelte";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
  import BadgeDialog from "../badges/BadgeDialog.svelte";
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

  const badgesOn = $derived(
    laneEnabled(account.settings.drops.mining_order, MiningLaneKind.BADGES),
  );

  const tab = $derived(openTab(BADGES_PAGE_TABS, BADGES_TAB_SLUGS, BadgesTab.ORDER));

  let open = $state<CampaignType | null>(null);
  let openBadge = $state<GlobalBadgeType | null>(null);
</script>

<PageBody>
  <div class="flex flex-wrap items-center justify-between gap-3">
    <Segmented
      value={tab}
      label={t.account_badges_tabs_label()}
      onchange={(next) => showTab(BADGES_TAB_SLUGS, next)}
      options={[
        {
          value: BadgesTab.ORDER,
          label: t.queue_title(),
          icon: IconName.LIST_ORDERED,
          count: sections.minable.length,
        },
        { value: BadgesTab.SETTINGS, label: t.drops_tab_settings(), icon: IconName.SETTINGS },
        {
          value: BadgesTab.EXCLUSIONS,
          label: t.drops_tab_exclusions(),
          icon: IconName.EYE_OFF,
          count:
            sections.ignored.length +
            account.settings.badges.exclude_games.length +
            account.settings.badges.exclude_names.length,
        },
        ...groupOptions(groups),
        {
          value: BadgesTab.OWNED,
          label: t.account_badges_owned(),
          icon: IconName.AWARD,
          count: owned.length,
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

  {#if tab === BadgesTab.ORDER}
    <BadgesOrderTab {account} {sections} onopen={(campaign) => (open = campaign)} />
  {:else if tab === BadgesTab.EXCLUSIONS}
    <BadgesExclusionsTab {account} {sections} onopen={(campaign) => (open = campaign)} />
  {:else if tab === BadgesTab.SETTINGS}
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
  {:else if tab === BadgesTab.OWNED}
    <OwnedBadgesTab {account} {owned} onopen={(badge) => (openBadge = badge)} />
  {:else}
    {#key tab}
      <RewardsTab
        {account}
        group={tab}
        entries={groups[tab]}
        icon={IconName.AWARD}
        onopen={(campaign) => (open = campaign)}
      />
    {/key}
  {/if}
  <AccountSaveBar {account} />
</PageBody>

<CampaignDialog campaign={open} contextAccountId={account.id} onclose={() => (open = null)} />
<BadgeDialog
  badge={openBadge}
  badges={badgeList.badges}
  contextAccountId={account.id}
  onopencampaign={(next) => {
    openBadge = null;
    open = next;
  }}
  onclose={() => (openBadge = null)}
/>
