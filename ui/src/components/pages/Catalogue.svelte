<script lang="ts">
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { describeCode } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DROPS_API_NAME, DROPS_WEB_URL } from "../../lib/partners.constants";
  import { openTab, showTab } from "../../lib/state/router";
  import { openCampaignSheet } from "../../lib/state/sheets";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { campaignMatches } from "../../lib/catalogue/campaign.utils";
  import { runningCampaigns } from "../../lib/catalogue/timeline.utils";
  import { normalizeSearch } from "../../lib/search/search.utils";
  import { linkMarker, strong } from "../../lib/text.utils";
  import { relativeTime } from "../../lib/time/time.utils";
  import CampaignBrowser from "../catalogue/CampaignBrowser.svelte";
  import { TIMELINE_VIEW_SLUGS, TimelineView } from "../../lib/catalogue/timeline.constants";
  import EmptyState from "../ui/EmptyState.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import RichText from "../ui/RichText.svelte";
  import SearchField from "../ui/SearchField.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { CATALOGUE_SORTS } from "../catalogue/catalogue.constants";
  import { viewOptions } from "../catalogue/catalogue.utils";
  import { catalogue } from "../../lib/catalogue/catalogue.svelte";

  type CataloguePropsType = { snapshot: SnapshotType };

  let { snapshot }: CataloguePropsType = $props();

  let query = $state("");
  const view = $derived(
    openTab(Object.values(TimelineView), TIMELINE_VIEW_SLUGS, TimelineView.TIMELINE),
  );

  const running = $derived(runningCampaigns(catalogue.campaigns, ticker.now));

  const active = $derived.by(() => {
    const needle = normalizeSearch(query);

    return running.filter((c) => campaignMatches(c, needle));
  });
</script>

<PageBody>
  <PageHeader
    intro={t.catalogue_intro({
      site: linkMarker(0),
      minutes: strong(Math.round(snapshot.catalog.interval_seconds / 60)),
    })}
    links={[{ label: DROPS_API_NAME, url: DROPS_WEB_URL }]}
  >
    {#snippet meta()}
      {#if snapshot.catalog.updated_at}
        <RichText
          text={t.catalogue_last_update({
            updatedTime: strong(relativeTime(snapshot.catalog.updated_at, ticker.now)),
          })}
        />{#if snapshot.catalog.fetched_at}{" · "}<RichText
            text={t.catalogue_fetched({
              fetchedTime: strong(relativeTime(snapshot.catalog.fetched_at, ticker.now)),
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

  <CampaignBrowser
    campaigns={active}
    {view}
    sorts={CATALOGUE_SORTS}
    ordered={() => false}
    tagOf={() => undefined}
    onopen={(campaign) => openCampaignSheet(campaign, null)}
  >
    {#snippet empty()}
      <EmptyState
        text={snapshot.catalog.problem
          ? t.dashboard_catalog_error({ error: strong(describeCode(snapshot.catalog.problem)) })
          : running.length
            ? t.catalogue_empty()
            : t.catalogue_none()}
        icon={IconName.GIFT}
      />
    {/snippet}
  </CampaignBrowser>
</PageBody>
