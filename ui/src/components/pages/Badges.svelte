<script lang="ts">
  import { BadgeFilter } from "../../lib/badges/badges.constants";
  import { badgeList } from "../../lib/badges/badges.svelte";
  import { badgeKey, badgeTimelineRows, visibleBadges } from "../../lib/badges/badges.utils";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import type { GlobalBadgeType } from "../../lib/bindings/GlobalBadgeType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import {
    TIMELINE_VIEW_SLUGS,
    TimelineView,
    TimelineZoom,
  } from "../../lib/catalogue/timeline.constants";
  import { describeCode } from "../../lib/errors/errors.utils";
  import { mmss, shortDuration } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DROPS_API_NAME, DROPS_WEB_URL } from "../../lib/partners.constants";
  import { normalizeSearch } from "../../lib/search/search.utils";
  import { openTab, showTab } from "../../lib/state/router";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { linkMarker, strong } from "../../lib/text.utils";
  import { HOUR_MS } from "../../lib/time/time.constants";
  import { secondClock } from "../../lib/time/clock.svelte";
  import { relativeTime } from "../../lib/time/time.utils";
  import BadgeCard from "../badges/BadgeCard.svelte";
  import BadgeDialog from "../badges/BadgeDialog.svelte";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
  import Timeline from "../catalogue/Timeline.svelte";
  import { viewOptions, zoomOptions } from "../catalogue/catalogue.utils";
  import EmptyState from "../ui/EmptyState.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import RichText from "../ui/RichText.svelte";
  import SearchField from "../ui/SearchField.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { IconName } from "../../lib/icons.constants";

  type BadgesPropsType = { snapshot: SnapshotType };

  let { snapshot }: BadgesPropsType = $props();

  let query = $state("");
  let filter = $state(BadgeFilter.WATCH);
  let zoom = $state(TimelineZoom.TWO_WEEKS);
  let open = $state<GlobalBadgeType | null>(null);
  let campaign = $state<CampaignType | null>(null);

  const view = $derived(
    openTab(Object.values(TimelineView), TIMELINE_VIEW_SLUGS, TimelineView.TIMELINE),
  );

  const titles = $derived(
    new Map(
      badgeList.badges.map((badge) => [badgeKey(badge.set_id, badge.version_id), badge.title]),
    ),
  );

  const list = $derived(snapshot.badges);

  const retryAt = $derived(
    list.problem && list.next_refresh_at ? Date.parse(list.next_refresh_at) : 0,
  );
  const clock = secondClock(() => retryAt);
  const retryIn = $derived(Math.max(0, retryAt - clock.now));

  const problem = $derived.by((): string => {
    if (!list.problem) {
      return "";
    }

    const error = t.badges_error({ error: describeCode(list.problem) });

    return retryIn > 0
      ? `${error} ${t.badges_retry({ time: retryIn < HOUR_MS ? mmss(retryIn) : shortDuration(retryIn) })}`
      : error;
  });
  const shown = $derived(
    visibleBadges(badgeList.badges, filter, normalizeSearch(query), ticker.now),
  );
  const rows = $derived(badgeTimelineRows(shown, ticker.now));
  const offCalendar = $derived(shown.length - rows.reduce((sum, row) => sum + row.items.length, 0));

  const countOf = (kind: BadgeFilter): number =>
    visibleBadges(badgeList.badges, kind, normalizeSearch(query), ticker.now).length;

  const filters = $derived([
    { value: BadgeFilter.WATCH, label: t.badges_filter_watch(), count: countOf(BadgeFilter.WATCH) },
    { value: BadgeFilter.SUB, label: t.badges_filter_sub(), count: countOf(BadgeFilter.SUB) },
    { value: BadgeFilter.ALL, label: t.badges_filter_all(), count: countOf(BadgeFilter.ALL) },
  ]);
</script>

<PageBody>
  <PageHeader
    intro={t.badges_intro({ site: linkMarker(0) })}
    links={[{ label: DROPS_API_NAME, url: DROPS_WEB_URL }]}
  >
    {#snippet meta()}
      {#if list.updated_at}
        <RichText
          text={t.catalogue_last_update({
            time: strong(relativeTime(list.updated_at, ticker.now)),
          })}
        />{#if list.fetched_at}{" · "}<RichText
            text={t.catalogue_fetched({
              time: strong(relativeTime(list.fetched_at, ticker.now)),
              minutes: strong(Math.round(list.interval_seconds / 60)),
            })}
          />{/if}
      {/if}
    {/snippet}
    <div class="w-full max-w-96">
      <SearchField
        placeholder={t.badges_search_placeholder()}
        aria-label={t.badges_search_placeholder()}
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

  <div class="flex flex-wrap items-center gap-3">
    <Segmented
      value={filter}
      options={filters}
      label={t.badges_filter()}
      onchange={(next) => (filter = next)}
    />
    {#if view === TimelineView.TIMELINE}
      <Segmented
        value={zoom}
        options={zoomOptions()}
        label={t.timeline_zoom()}
        onchange={(next) => (zoom = next)}
      />
    {/if}
  </div>

  {#if shown.length === 0}
    <EmptyState
      text={badgeList.badges.length === 0 ? problem || t.badges_none() : t.badges_empty()}
      icon={IconName.AWARD}
    />
  {:else if view === TimelineView.TIMELINE}
    {#if offCalendar > 0}
      <p class="text-hint text-muted-foreground">
        {t.badges_off_calendar({ count: offCalendar })}
        <button
          class="font-semibold text-foreground underline decoration-link underline-offset-3 transition-colors hover:text-link focus-visible:text-link"
          type="button"
          onclick={() => showTab(TIMELINE_VIEW_SLUGS, TimelineView.CARDS)}
          >{t.badges_show_cards()}</button
        >
      </p>
    {/if}
    {#if rows.length > 0}
      <Timeline
        {rows}
        now={ticker.now}
        {zoom}
        moreLabel={t.badges_show_all_rows({ count: rows.length })}
        onopen={(badge) => (open = badge)}
      />
    {/if}
  {:else}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(min(300px,100%),1fr))] gap-3">
      {#each shown as badge (badgeKey(badge.set_id, badge.version_id))}
        <BadgeCard
          {badge}
          now={ticker.now}
          evolvesFrom={titles.get(badge.requirement?.evolves_from ?? "") ?? null}
          onopen={() => (open = badge)}
        />
      {/each}
    </div>
  {/if}
</PageBody>

<BadgeDialog
  badge={open}
  badges={badgeList.badges}
  onopencampaign={(next) => {
    open = null;
    campaign = next;
  }}
  onopenbadge={(next) => (open = next)}
  onclose={() => (open = null)}
/>
<CampaignDialog {campaign} contextAccountId={null} onclose={() => (campaign = null)} />
