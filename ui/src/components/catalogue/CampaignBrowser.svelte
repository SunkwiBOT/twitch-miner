<script lang="ts">
  import type { Snippet } from "svelte";

  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import {
    TIMELINE_ZOOM_SLUGS,
    TIMELINE_ZOOMS,
    TimelineView,
    TimelineZoom,
  } from "../../lib/catalogue/timeline.constants";
  import { campaignTimelineRows, type TimelineTagType } from "../../lib/catalogue/timeline.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { openTab, showTab } from "../../lib/state/router";
  import { ticker } from "../../lib/state/ticker.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import CampaignCard from "./CampaignCard.svelte";
  import Timeline from "./Timeline.svelte";
  import { CAMPAIGN_SORT_SLUGS, type CampaignSort } from "./catalogue.constants";
  import { sortCampaigns, sortOptions, zoomOptions } from "./catalogue.utils";

  type CampaignBrowserPropsType = {
    campaigns: CampaignType[];
    view: TimelineView;
    sorts: readonly [CampaignSort, ...CampaignSort[]];
    ordered: (campaign: CampaignType) => boolean;
    tagOf: (campaign: CampaignType) => TimelineTagType | undefined;
    cover?: Snippet<[CampaignType]>;
    tags?: Snippet<[CampaignType]>;
    empty: Snippet;
    onopen: (campaign: CampaignType) => void;
  };

  let {
    campaigns,
    view,
    sorts,
    ordered,
    tagOf,
    cover,
    tags,
    empty,
    onopen,
  }: CampaignBrowserPropsType = $props();

  const sort = $derived(openTab(sorts, CAMPAIGN_SORT_SLUGS, sorts[0]));
  const zoom = $derived(openTab(TIMELINE_ZOOMS, TIMELINE_ZOOM_SLUGS, TimelineZoom.TWO_WEEKS));
  const sorted = $derived(sortCampaigns(campaigns, sort));
  const rows = $derived(campaignTimelineRows(campaigns, tagOf));
</script>

{#if view === TimelineView.CARDS}
  <Segmented
    value={sort}
    options={sortOptions(sorts)}
    label={t.catalogue_sort()}
    onchange={(next) => showTab(CAMPAIGN_SORT_SLUGS, next)}
  />
{:else}
  <Segmented
    value={zoom}
    options={zoomOptions()}
    label={t.timeline_zoom()}
    onchange={(next) => showTab(TIMELINE_ZOOM_SLUGS, next)}
  />
{/if}

{#if campaigns.length === 0}
  {@render empty()}
{:else if view === TimelineView.TIMELINE}
  <Timeline
    {rows}
    now={ticker.now}
    {zoom}
    moreLabel={t.timeline_show_all({ count: rows.length })}
    {onopen}
  />
{:else}
  <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-4">
    {#each sorted as campaign (campaign.id)}
      {#snippet ownCover()}{@render cover?.(campaign)}{/snippet}
      {#snippet ownTags()}{@render tags?.(campaign)}{/snippet}
      <CampaignCard
        {campaign}
        ordered={ordered(campaign)}
        cover={ownCover}
        tags={ownTags}
        {onopen}
      />
    {/each}
  </div>
{/if}
