<script lang="ts">
  import { CampaignGroup } from "../../../lib/accounts/campaign-groups.constants";
  import {
    expectedStartIn,
    facetOptions,
    facetsShown,
    type AccountCampaignType,
  } from "../../../lib/accounts/campaign-groups.utils";
  import {
    campaignStateView,
    etaView,
    filterOptions,
    groupEmpty,
    overlaid,
    timelineTag,
  } from "../../../lib/accounts/campaign-states.utils";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import { campaignMatches } from "../../../lib/catalogue/campaign.utils";
  import { TIMELINE_VIEW_SLUGS, TimelineView } from "../../../lib/catalogue/timeline.constants";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import type { IconName } from "../../../lib/icons.constants";
  import { normalizeSearch } from "../../../lib/search/search.utils";
  import { openTab, showTab } from "../../../lib/state/router";
  import { ticker } from "../../../lib/state/ticker.svelte";
  import CampaignBrowser from "../../catalogue/CampaignBrowser.svelte";
  import CampaignOverlay from "../../catalogue/CampaignOverlay.svelte";
  import { CATALOGUE_SORTS, QUEUE_SORTS } from "../../catalogue/catalogue.constants";
  import { viewOptions } from "../../catalogue/catalogue.utils";
  import Badge from "../../ui/Badge.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import MultiSelect from "../../ui/MultiSelect.svelte";
  import SearchField from "../../ui/SearchField.svelte";
  import Segmented from "../../ui/Segmented.svelte";
  import { MenuAlign } from "../../ui/menu.constants";

  type RewardsTabPropsType = {
    account: AccountSnapshotType;
    group: CampaignGroup;
    entries: AccountCampaignType[];
    icon: IconName;
    onopen: (campaign: CampaignType) => void;
  };

  let { account, group, entries, icon, onopen }: RewardsTabPropsType = $props();

  let query = $state("");
  let hidden = $state<string[]>([]);

  const view = $derived(
    openTab(Object.values(TimelineView), TIMELINE_VIEW_SLUGS, TimelineView.CARDS),
  );
  const options = $derived(filterOptions(facetOptions(entries)));
  const offered = $derived(options.map((option) => option.value));
  const ticked = $derived(offered.filter((key) => !hidden.includes(key)));

  const shown = $derived.by(() => {
    const needle = normalizeSearch(query);
    const off = new Set(hidden);

    return entries.filter(
      (entry) => campaignMatches(entry.campaign, needle) && facetsShown(entry, off),
    );
  });

  const byId = $derived(new Map(entries.map((entry) => [entry.campaign.id, entry])));

  const startsIn = (entry: AccountCampaignType): number | null =>
    entry.queued ? expectedStartIn(account, entry.queued, ticker.now) : null;

  const showFacets = (next: string[]): void => {
    hidden = [
      ...hidden.filter((key) => !offered.includes(key)),
      ...offered.filter((key) => !next.includes(key)),
    ];
  };
</script>

<div class="flex flex-wrap items-center gap-3">
  <div class="w-full max-w-96">
    <SearchField
      placeholder={t.campaigns_search_placeholder()}
      aria-label={t.campaigns_search_placeholder()}
      bind:value={query}
      busy={false}
    />
  </div>
  {#if options.length > 0}
    <div class="w-56">
      <MultiSelect
        values={ticked}
        {options}
        label={t.rewards_filter()}
        allLabel={t.rewards_filter_all()}
        noneLabel={t.rewards_filter_none()}
        countLabel={(count) => t.rewards_filter_count({ count })}
        align={MenuAlign.LEFT}
        onchange={showFacets}
      />
    </div>
  {/if}
  <span class="flex-1"></span>
  <Segmented
    value={view}
    options={viewOptions()}
    label={t.catalogue_view()}
    onchange={(next) => showTab(TIMELINE_VIEW_SLUGS, next)}
  />
</div>

<CampaignBrowser
  campaigns={shown.map((entry) => entry.campaign)}
  {view}
  sorts={group === CampaignGroup.TO_MINE ? QUEUE_SORTS : CATALOGUE_SORTS}
  ordered={(campaign) => account.settings.drops.campaign_order.includes(campaign.id)}
  tagOf={(campaign) => {
    const entry = byId.get(campaign.id);

    return entry && timelineTag(entry, startsIn(entry), ticker.now);
  }}
  {onopen}
>
  {#snippet cover(campaign)}
    {@const entry = byId.get(campaign.id)}
    {#if entry && overlaid(entry.state)}
      <CampaignOverlay {...campaignStateView(entry, ticker.now)} />
    {/if}
  {/snippet}
  {#snippet tags(campaign)}
    {@const entry = byId.get(campaign.id)}
    {#if entry && !overlaid(entry.state)}
      {@const next = startsIn(entry)}
      {@const pill = next === null ? campaignStateView(entry, ticker.now) : etaView(next)}
      <Badge text={pill.text} tone={pill.tone} dot={false} icon={pill.icon} hint={pill.hint} />
    {/if}
  {/snippet}
  {#snippet empty()}
    <EmptyState text={entries.length ? t.campaigns_empty() : groupEmpty(group)} {icon} />
  {/snippet}
</CampaignBrowser>
