<script lang="ts">
  import PanelColumns from "../ui/PanelColumns.svelte";
  import { untrack } from "svelte";

  import { BADGES_TAB_SLUGS, BadgesTab } from "../../lib/accounts/accounts.constants";
  import { laneEnabled } from "../../lib/accounts/mining-order.utils";
  import { accountBadges, ownedBadges } from "../../lib/badges/badges.utils";
  import { badgeList } from "../../lib/badges/badges.svelte";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { MiningLaneKind } from "../../lib/bindings/MiningLaneKind";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { matchesSearch, normalizeSearch } from "../../lib/search/search.utils";
  import { editsOf } from "../../lib/state/edits.svelte";
  import { AccountField } from "../../lib/state/state.constants";
  import { openTab, showTab } from "../../lib/state/state.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import Notice from "../ui/Notice.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Panel from "../ui/Panel.svelte";
  import SearchField from "../ui/SearchField.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import Thumb from "../ui/Thumb.svelte";
  import { IconName, NoticeTone, PanelWidth, ThumbShape, ThumbSize } from "../ui/ui.constants";
  import AccountSaveBar from "./AccountSaveBar.svelte";
  import BadgesExclusionsTab from "./badges/BadgesExclusionsTab.svelte";
  import BadgesOrderTab from "./badges/BadgesOrderTab.svelte";
  import CampaignScopeFields from "./drops/CampaignScopeFields.svelte";
  import MiningOrderList from "./drops/MiningOrderList.svelte";

  type AccountBadgesPropsType = { account: AccountSnapshotType; snapshot: SnapshotType };

  let { account, snapshot }: AccountBadgesPropsType = $props();

  const edits = editsOf(untrack(() => account.id));
  const badges = $derived(edits.applied(account.settings).badges);

  const sections = $derived(accountBadges(account, ticker.now));
  const owned = $derived(ownedBadges(account, badgeList.badges));

  let query = $state("");

  const ownedShown = $derived.by(() => {
    const needle = normalizeSearch(query);

    return owned.filter((badge) => !needle || matchesSearch(badge.title, needle));
  });
  const badgesOn = $derived(
    laneEnabled(account.settings.drops.mining_order, MiningLaneKind.BADGES),
  );

  const tab = $derived(openTab(Object.values(BadgesTab), BADGES_TAB_SLUGS, BadgesTab.ORDER));

  let open = $state<CampaignType | null>(null);
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
    <BadgesOrderTab {account} {snapshot} {sections} onopen={(campaign) => (open = campaign)} />
  {:else if tab === BadgesTab.EXCLUSIONS}
    <BadgesExclusionsTab {account} {snapshot} {sections} onopen={(campaign) => (open = campaign)} />
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
      </Panel>
    </PanelColumns>
  {:else}
    <Panel title={t.account_badges_owned()} count={owned.length}>
      {#if owned.length === 0}
        <EmptyState text={t.account_badges_empty_owned()} />
      {:else}
        <div class="w-full max-w-96">
          <SearchField
            placeholder={t.account_badges_search_placeholder()}
            aria-label={t.account_badges_search_placeholder()}
            bind:value={query}
            busy={false}
          />
        </div>
        {#if ownedShown.length === 0}
          <EmptyState text={t.badges_empty()} />
        {:else}
          <ul class="grid grid-cols-[repeat(auto-fill,minmax(min(170px,100%),1fr))] gap-2.5">
            {#each ownedShown as badge (badge.key)}
              <li class="flex min-w-0 items-center gap-2.5 rounded-lg border p-2">
                <Thumb
                  src={badge.image}
                  round={false}
                  contain={true}
                  size={ThumbSize.MD}
                  shape={ThumbShape.SQUARE}
                  fallback={IconName.AWARD}
                />
                <span class="truncate text-xs font-semibold" title={badge.title}>{badge.title}</span
                >
              </li>
            {/each}
          </ul>
        {/if}
      {/if}
    </Panel>
  {/if}
  <AccountSaveBar {account} />
</PageBody>

<CampaignDialog campaign={open} contextAccountId={account.id} onclose={() => (open = null)} />
