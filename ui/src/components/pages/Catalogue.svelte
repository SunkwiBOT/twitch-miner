<script lang="ts">
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { describeCode } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DROPS_API_NAME, DROPS_WEB_URL } from "../../lib/partners.constants";
  import { openTab, showTab } from "../../lib/state/router";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { campaignTimelineRows, runningCampaigns } from "../../lib/catalogue/timeline.utils";
  import { matchesSearch, normalizeSearch } from "../../lib/search/search.utils";
  import { linkMarker, strong } from "../../lib/text.utils";
  import { relativeTime } from "../../lib/time/time.utils";
  import CampaignCard from "../catalogue/CampaignCard.svelte";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
  import Timeline from "../catalogue/Timeline.svelte";
  import {
    TIMELINE_VIEW_SLUGS,
    TIMELINE_ZOOM_SLUGS,
    TIMELINE_ZOOMS,
    TimelineView,
    TimelineZoom,
  } from "../../lib/catalogue/timeline.constants";
  import EmptyState from "../ui/EmptyState.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import RichText from "../ui/RichText.svelte";
  import SearchField from "../ui/SearchField.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { CATALOGUE_SORT_SLUGS, CatalogueSort } from "../catalogue/catalogue.constants";
  import { viewOptions, zoomOptions } from "../catalogue/catalogue.utils";
  import { catalogue } from "../../lib/catalogue/catalogue.svelte";

  type CataloguePropsType = { snapshot: SnapshotType };

  let { snapshot }: CataloguePropsType = $props();

  let query = $state("");
  const view = $derived(
    openTab(Object.values(TimelineView), TIMELINE_VIEW_SLUGS, TimelineView.TIMELINE),
  );
  const sort = $derived(
    openTab(Object.values(CatalogueSort), CATALOGUE_SORT_SLUGS, CatalogueSort.ENDING),
  );
  let open = $state<CampaignType | null>(null);

  const time = (iso: string): number => new Date(iso).getTime();

  const running = $derived(runningCampaigns(catalogue.campaigns, ticker.now));

  const active = $derived.by(() => {
    const needle = normalizeSearch(query);

    return running.filter(
      (c) => !needle || matchesSearch(c.game.name, needle) || matchesSearch(c.name, needle),
    );
  });

  const sorted = $derived.by(() => {
    const list = [...active];

    switch (sort) {
      case CatalogueSort.ENDING:
        return list.sort((a, b) => time(a.ends_at) - time(b.ends_at));
      case CatalogueSort.NEW:
        return list.sort((a, b) => time(b.starts_at) - time(a.starts_at));
      case CatalogueSort.GAME:
        return list.sort(
          (a, b) => a.game.name.localeCompare(b.game.name) || a.name.localeCompare(b.name),
        );
    }
  });

  const zoom = $derived(openTab(TIMELINE_ZOOMS, TIMELINE_ZOOM_SLUGS, TimelineZoom.TWO_WEEKS));

  const sorts = $derived([
    { value: CatalogueSort.ENDING, label: t.catalogue_sort_ending() },
    { value: CatalogueSort.NEW, label: t.catalogue_sort_new() },
    { value: CatalogueSort.GAME, label: t.catalogue_sort_game() },
  ]);
</script>

<PageBody>
  <PageHeader
    intro={t.catalogue_intro({
      site: linkMarker(0),
      minutes: Math.round(snapshot.catalog.interval_seconds / 60),
    })}
    links={[{ label: DROPS_API_NAME, url: DROPS_WEB_URL }]}
  >
    {#snippet meta()}
      {#if snapshot.catalog.updated_at}
        <RichText
          text={t.catalogue_last_update({
            time: strong(relativeTime(snapshot.catalog.updated_at, ticker.now)),
          })}
        />{#if snapshot.catalog.fetched_at}{" · "}<RichText
            text={t.catalogue_fetched({
              time: strong(relativeTime(snapshot.catalog.fetched_at, ticker.now)),
              minutes: strong(Math.round(snapshot.catalog.interval_seconds / 60)),
            })}
          />{/if}
      {/if}
    {/snippet}
    <div class="w-full max-w-96">
      <SearchField
        placeholder={t.campaigns_search_placeholder()}
        aria-label={t.campaigns_search_placeholder()}
        bind:value={query}
        busy={false}
      />
    </div>
    <span class="flex-1"></span>
    <Segmented
      value={view}
      options={viewOptions()}
      label={t.catalogue_view()}
      onchange={(next) => showTab(TIMELINE_VIEW_SLUGS, next)}
    />
  </PageHeader>

  {#if view === TimelineView.CARDS}
    <Segmented
      value={sort}
      options={sorts}
      label={t.catalogue_sort()}
      onchange={(next) => showTab(CATALOGUE_SORT_SLUGS, next)}
    />
  {:else}
    <Segmented
      value={zoom}
      options={zoomOptions()}
      label={t.timeline_zoom()}
      onchange={(next) => showTab(TIMELINE_ZOOM_SLUGS, next)}
    />
  {/if}

  {#if active.length === 0}
    <EmptyState
      text={snapshot.catalog.problem
        ? t.dashboard_catalog_error({ error: describeCode(snapshot.catalog.problem) })
        : running.length
          ? t.catalogue_empty()
          : t.catalogue_none()}
      icon={IconName.GIFT}
    />
  {:else if view === TimelineView.TIMELINE}
    {@const rows = campaignTimelineRows(active)}
    <Timeline
      {rows}
      now={ticker.now}
      {zoom}
      moreLabel={t.timeline_show_all({ count: rows.length })}
      onopen={(campaign) => (open = campaign)}
    />
  {:else}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-4">
      {#each sorted as c (c.id)}
        <CampaignCard campaign={c} ordered={false} onopen={(campaign) => (open = campaign)} />
      {/each}
    </div>
  {/if}
</PageBody>

<CampaignDialog campaign={open} contextAccountId={null} onclose={() => (open = null)} />
