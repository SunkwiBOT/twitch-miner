<script lang="ts">
  import { BadgeFilter } from "../../lib/badges/badges.constants";
  import { badgeList } from "../../lib/badges/badges.svelte";
  import { visibleBadges } from "../../lib/badges/badges.utils";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { describeCode } from "../../lib/errors/errors.utils";
  import { mmss, shortDuration } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DROPS_API_NAME, DROPS_WEB_URL } from "../../lib/partners.constants";
  import { normalizeSearch } from "../../lib/search/search.utils";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { linkMarker, strong } from "../../lib/text.utils";
  import { HOUR_MS } from "../../lib/time/time.constants";
  import { relativeTime } from "../../lib/time/time.utils";
  import BadgeCard from "../badges/BadgeCard.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import RichText from "../ui/RichText.svelte";
  import SearchField from "../ui/SearchField.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { IconName } from "../ui/ui.constants";

  type BadgesPropsType = { snapshot: SnapshotType };

  let { snapshot }: BadgesPropsType = $props();

  let query = $state("");
  let filter = $state(BadgeFilter.WATCH);

  const list = $derived(snapshot.badges);

  const retryIn = $derived(
    list.next_refresh_at ? Date.parse(list.next_refresh_at) - ticker.now : 0,
  );
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

  const filters = $derived([
    { value: BadgeFilter.WATCH, label: t.badges_filter_watch() },
    { value: BadgeFilter.SUB, label: t.badges_filter_sub() },
    { value: BadgeFilter.ALL, label: t.badges_filter_all() },
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
        search={null}
      />
    </div>
    <span class="flex-1"></span>
    <Segmented
      value={filter}
      options={filters}
      label={t.badges_filter()}
      onchange={(next) => (filter = next)}
    />
  </PageHeader>

  {#if shown.length === 0}
    <EmptyState
      text={badgeList.badges.length === 0 ? problem || t.badges_none() : t.badges_empty()}
      icon={IconName.AWARD}
    />
  {:else}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(min(300px,100%),1fr))] gap-3">
      {#each shown as badge (`${badge.set_id}/${badge.version_id}`)}
        <BadgeCard {badge} now={ticker.now} />
      {/each}
    </div>
  {/if}
</PageBody>
