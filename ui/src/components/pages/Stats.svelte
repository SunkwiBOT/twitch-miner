<script lang="ts">
  import type { HistoryStatsResponseType } from "../../lib/bindings/HistoryStatsResponseType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { queryAccountIds, visibleAccountIds } from "../../lib/accounts/account-filter.utils";
  import { fetchHistoryStats } from "../../lib/api/client";
  import { ensureChannels } from "../../lib/channels/channels.svelte";
  import { describeError } from "../../lib/errors/errors.utils";
  import { number, shortDuration } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { MINUTE_MS } from "../../lib/time/time.constants";
  import { app } from "../../lib/state/app.svelte";
  import { navigate } from "../../lib/state/router";
  import { Page } from "../../lib/state/state.constants";
  import { Card } from "$lib/components/ui/card";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import AccountFilter from "../ui/AccountFilter.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import Panel from "../ui/Panel.svelte";
  import PanelGrid from "../ui/PanelGrid.svelte";
  import CategoryBarList from "../stats/charts/CategoryBarList.svelte";
  import DailyPointsChart from "../stats/charts/DailyPointsChart.svelte";
  import StatsPeriodSelector from "../stats/StatsPeriodSelector.svelte";
  import StatsTile from "../stats/StatsTile.svelte";
  import StatsTopCategory from "../stats/StatsTopCategory.svelte";
  import StatsTopStreamer from "../stats/StatsTopStreamer.svelte";
  import TopStreamersList from "../stats/TopStreamersList.svelte";
  import type { StatsPeriodDaysType } from "../stats/stats.types";
  import {
    buildDayBuckets,
    categoryRows,
    clampPeriod,
    dayBoundaries,
    periodRange,
  } from "../stats/stats.utils";

  type StatsPropsType = { snapshot: SnapshotType };

  let { snapshot }: StatsPropsType = $props();

  let hiddenAccounts = $state<string[]>([]);
  let selectedPeriod = $state<StatsPeriodDaysType>(7);
  let stats = $state<HistoryStatsResponseType | null>(null);
  let loading = $state(true);
  let problem = $state<string | null>(null);

  const accountIds = $derived(visibleAccountIds(snapshot.accounts, hiddenAccounts));
  const queryIds = $derived(queryAccountIds(accountIds, snapshot.accounts.length));
  const retentionDays = $derived(snapshot.history_retention_days ?? null);
  const period = $derived(clampPeriod(selectedPeriod, retentionDays));

  // ! Bumped by every new query: a slower, now-stale answer to an earlier
  // account/period combination is dropped instead of landing after a newer
  // one, so switching periods or accounts quickly never shows totals from
  // the wrong period.
  let statsGeneration = 0;

  $effect(() => {
    const ids = queryIds;
    const days = period;
    const now = Date.now();
    const { since, until } = periodRange(days, now);
    const boundaries = dayBoundaries(days, now);
    const requestGeneration = ++statsGeneration;

    loading = true;
    problem = null;

    fetchHistoryStats({ account_ids: ids, since, until, day_boundaries: boundaries })
      .then((result) => {
        if (requestGeneration === statsGeneration) {
          stats = result;
        }
      })
      .catch((e: unknown) => {
        if (requestGeneration === statsGeneration) {
          problem = describeError(e);
        }
      })
      .finally(() => {
        if (requestGeneration === statsGeneration) {
          loading = false;
        }
      });
  });

  $effect(() => {
    const data = stats;

    if (!data) {
      return;
    }

    const logins = new Set<string>();

    if (data.top_points_streamer) {
      logins.add(data.top_points_streamer.login);
    }

    if (data.top_watched_streamer) {
      logins.add(data.top_watched_streamer.login);
    }

    if (data.top_predictions_streamer) {
      logins.add(data.top_predictions_streamer.login);
    }

    for (const row of data.top_streamers_by_points) {
      logins.add(row.login);
    }

    ensureChannels([...logins]);
  });

  const pointsTotal = $derived(stats?.points_total ?? 0);
  const dropsTotal = $derived(stats?.drops_claimed_total ?? 0);
  const minutesTotal = $derived(stats?.minutes_watched_total ?? 0);
  const predictionsTotal = $derived(stats?.predictions_won_total ?? 0);

  const topPointsLogin = $derived(stats?.top_points_streamer?.login);
  const topWatchedLogin = $derived(stats?.top_watched_streamer?.login);
  const topPredictionsLogin = $derived(stats?.top_predictions_streamer?.login);
  const topCategory = $derived(stats?.top_category);

  const buckets = $derived(
    buildDayBuckets(stats?.points_by_day ?? [], dayBoundaries(period, Date.now())),
  );
  const categories = $derived(categoryRows(stats?.drops_by_category ?? []));
  const topStreamers = $derived(stats?.top_streamers_by_points ?? []);

  const openCurve = (login: string): void => {
    const owner = accountIds.find((id) =>
      snapshot.accounts.find((a) => a.id === id)?.streamers.some((s) => s.login === login),
    );

    app.statsCurveTarget = {
      accountId: owner ?? accountIds[0] ?? snapshot.accounts[0]?.id ?? "",
      channel: login,
      period,
    };
    navigate(Page.STATS_STREAMER);
  };

  const empty = $derived(
    pointsTotal === 0 && dropsTotal === 0 && minutesTotal === 0 && predictionsTotal === 0,
  );
</script>

{#snippet pointsPanel()}
  <Panel title={t.stats_chart_points_by_day()}>
    <DailyPointsChart
      days={buckets.days}
      reasons={buckets.reasons}
      ariaLabel={t.stats_chart_points_by_day_aria({ days: period })}
    />
  </Panel>
{/snippet}

{#snippet categoriesPanel()}
  <Panel title={t.stats_chart_drops_by_category()}>
    <CategoryBarList
      rows={categories}
      ariaLabel={t.stats_chart_drops_by_category()}
      emptyText={t.stats_chart_drops_by_category_empty()}
    />
  </Panel>
{/snippet}

{#snippet streamersPanel()}
  <Panel title={t.stats_chart_top_streamers()}>
    <TopStreamersList
      rows={topStreamers}
      ariaLabel={t.stats_chart_top_streamers()}
      emptyText={t.stats_chart_top_streamers_empty()}
      onopencurve={openCurve}
    />
  </Panel>
{/snippet}

{#snippet pointsSkeleton()}
  <Panel title={t.stats_chart_points_by_day()}><Skeleton class="h-48 w-full" /></Panel>
{/snippet}

{#snippet categoriesSkeleton()}
  <Panel title={t.stats_chart_drops_by_category()}><Skeleton class="h-40 w-full" /></Panel>
{/snippet}

{#snippet streamersSkeleton()}
  <Panel title={t.stats_chart_top_streamers()}><Skeleton class="h-40 w-full" /></Panel>
{/snippet}

<PageBody>
  <PageHeader>
    <AccountFilter
      accounts={snapshot.accounts}
      hidden={hiddenAccounts}
      onchange={(next) => (hiddenAccounts = next)}
    />
    <span class="flex-1"></span>
    <StatsPeriodSelector
      value={period}
      retention={snapshot.settings.history_retention}
      {retentionDays}
      onchange={(next) => (selectedPeriod = next)}
    />
  </PageHeader>

  {#if loading && stats === null && !problem}
    <div class="grid grid-cols-2 gap-3 @lg:grid-cols-4">
      {#each { length: 4 }, index (index)}
        <Card class="gap-2.5 p-4">
          <Skeleton class="h-7 w-16" />
          <Skeleton class="h-3.5 w-24" />
          <Skeleton class="mt-1 h-5 w-28" />
        </Card>
      {/each}
    </div>
    <PanelGrid
      cells={[
        { key: "points", wide: true, panel: pointsSkeleton },
        { key: "categories", wide: false, panel: categoriesSkeleton },
        { key: "streamers", wide: false, panel: streamersSkeleton },
      ]}
    />
  {:else if problem}
    <EmptyState title={t.stats_load_failed()} text={problem} icon={IconName.ALERT} />
  {:else if stats === null || empty}
    <EmptyState text={t.stats_empty()} icon={IconName.CHART} />
  {:else}
    <div class="grid grid-cols-2 gap-3 @lg:grid-cols-4">
      <StatsTile
        icon={IconName.CHANNEL_POINTS}
        value={number(pointsTotal)}
        label={t.stats_tile_points()}
      >
        {#snippet topItem()}<StatsTopStreamer login={topPointsLogin} />{/snippet}
      </StatsTile>
      <StatsTile icon={IconName.GIFT} value={number(dropsTotal)} label={t.stats_tile_drops()}>
        {#snippet topItem()}<StatsTopCategory {topCategory} />{/snippet}
      </StatsTile>
      <StatsTile
        icon={IconName.TV}
        value={shortDuration(minutesTotal * MINUTE_MS)}
        label={t.stats_tile_watched()}
      >
        {#snippet topItem()}<StatsTopStreamer login={topWatchedLogin} />{/snippet}
      </StatsTile>
      <StatsTile
        icon={IconName.TROPHY}
        value={number(predictionsTotal)}
        label={t.stats_tile_predictions()}
      >
        {#snippet topItem()}<StatsTopStreamer login={topPredictionsLogin} />{/snippet}
      </StatsTile>
    </div>

    <PanelGrid
      cells={[
        { key: "points", wide: true, panel: pointsPanel },
        { key: "categories", wide: false, panel: categoriesPanel },
        { key: "streamers", wide: false, panel: streamersPanel },
      ]}
    />
  {/if}
</PageBody>
