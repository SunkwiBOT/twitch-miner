<script lang="ts">
  import { untrack } from "svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { fetchHistoryCurve } from "../../lib/api/client";
  import type { HistoryCurveResponseType } from "../../lib/bindings/HistoryCurveResponseType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import {
    accountChannel,
    ensureChannels,
    knownChannels,
  } from "../../lib/channels/channels.svelte";
  import { describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { MenuAlign } from "../ui/menu.constants";
  import BalanceCurveChart from "../stats/charts/BalanceCurveChart.svelte";
  import CurveLegend from "../stats/charts/CurveLegend.svelte";
  import { STATS_PERIODS } from "../stats/stats.constants";
  import type { StatsPeriodDaysType } from "../stats/stats.types";
  import { clampPeriod, periodRange } from "../stats/stats.utils";
  import StatsPeriodSelector from "../stats/StatsPeriodSelector.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import Panel from "../ui/Panel.svelte";
  import Select from "../ui/Select.svelte";
  import { IconName } from "../../lib/icons.constants";

  type StreamerCurvePropsType = { snapshot: SnapshotType };

  let { snapshot }: StreamerCurvePropsType = $props();

  const isStatsPeriod = (value: number): value is StatsPeriodDaysType =>
    STATS_PERIODS.some((period) => period === value);

  const urlParams = untrack(() => new URLSearchParams(location.search));
  const urlAccount = urlParams.get("account");
  const urlChannel = urlParams.get("channel");
  const urlPeriod = Number(urlParams.get("period"));

  const target =
    untrack(() => app.statsCurveTarget) ??
    (urlAccount && urlChannel ? { accountId: urlAccount, channel: urlChannel } : null);

  app.statsCurveTarget = null;

  let accountId = $state<string | null>(
    target?.accountId ?? untrack(() => snapshot.accounts[0]?.id ?? null),
  );
  let selectedChannel = $state<string | null>(target?.channel ?? null);
  // ! Opened from the stats page's top streamers list: starts on the period
  // that list was already showing, not always 7 days.
  let selectedPeriod = $state<StatsPeriodDaysType>(
    target && "period" in target && isStatsPeriod(target.period)
      ? target.period
      : isStatsPeriod(urlPeriod)
        ? urlPeriod
        : 7,
  );

  const account = $derived(snapshot.accounts.find((a) => a.id === accountId) ?? null);
  const retentionDays = $derived(snapshot.history_retention_days ?? null);
  const period = $derived(clampPeriod(selectedPeriod, retentionDays));

  const accountOptions = $derived(
    snapshot.accounts.map((a) => ({ value: a.id, label: a.display_name, image: a.avatar_url })),
  );

  // ! Keeps the requested streamer as an option even when it is no longer in
  // the account's live `streamers` list, so a reload or a direct link never
  // silently swaps it for another streamer; falls back to a channel known
  // only by login through `knownChannels`/`ensureChannels` below.
  const streamerOptions = $derived.by(() => {
    const resolved = (account?.streamers ?? [])
      .filter((streamer) => streamer.channel)
      .map((streamer) => ({
        value: streamer.login,
        label: streamer.channel?.display_name ?? streamer.login,
        image: streamer.channel?.avatar_url,
      }));

    if (selectedChannel && !resolved.some((option) => option.value === selectedChannel)) {
      const known = account
        ? accountChannel(account, selectedChannel)
        : knownChannels[selectedChannel];

      resolved.push({
        value: selectedChannel,
        label: known?.display_name ?? selectedChannel,
        image: known?.avatar_url,
      });
    }

    return resolved.sort((a, b) => a.label.localeCompare(b.label));
  });

  // ! `selectedChannel` is the user's own choice (kept as an option above
  // even when no longer live); falls back to the first known streamer only
  // while nothing has been chosen yet, with no effect writing state.
  const channel = $derived(selectedChannel ?? streamerOptions[0]?.value ?? null);

  $effect(() => {
    if (channel) {
      ensureChannels([channel]);
    }
  });

  const currentStreamer = $derived(streamerOptions.find((option) => option.value === channel));

  const now = Date.now();

  const { since, until } = $derived(periodRange(period, now));

  // ! Bumped by every new query so a slower, now-stale answer to an earlier
  // account/channel/period combination is dropped instead of landing after
  // a newer one.
  let curveGeneration = 0;

  let curve = $state<HistoryCurveResponseType | null>(null);
  let loading = $state(true);
  let problem = $state<string | null>(null);

  // ! The server clamps `since`/`until` to the account's retention: the
  // curve's X domain follows that clamped span, not the client's request.
  const domainStart = $derived(Date.parse(curve?.since ?? since));
  const domainEnd = $derived(Date.parse(curve?.until ?? until));

  $effect(() => {
    const currentAccountId = accountId;
    const currentChannel = channel;
    const sinceValue = since;
    const untilValue = until;
    const requestGeneration = ++curveGeneration;

    if (!currentAccountId || !currentChannel) {
      curve = null;
      loading = false;

      return;
    }

    loading = true;
    problem = null;

    fetchHistoryCurve({
      account_id: currentAccountId,
      channel: currentChannel,
      since: sinceValue,
      until: untilValue,
    })
      .then((result) => {
        if (requestGeneration === curveGeneration) {
          curve = result;
        }
      })
      .catch((e: unknown) => {
        if (requestGeneration === curveGeneration) {
          problem = describeError(e);
        }
      })
      .finally(() => {
        if (requestGeneration === curveGeneration) {
          loading = false;
        }
      });
  });

  // ! Keeps a reload or a copied link on the same account/streamer/period.
  $effect(() => {
    const params = new URLSearchParams();

    if (accountId) {
      params.set("account", accountId);
    }

    if (channel) {
      params.set("channel", channel);
    }

    params.set("period", String(period));

    const next = `${location.pathname}?${params.toString()}${location.hash}`;

    if (next !== `${location.pathname}${location.search}${location.hash}`) {
      history.replaceState(history.state, "", next);
    }
  });

  const events = $derived(curve?.events ?? []);
  const empty = $derived(events.length === 0);
</script>

<PageBody>
  <PageHeader>
    <div class="w-48">
      <Select
        value={accountId}
        options={accountOptions}
        label={t.common_account()}
        align={MenuAlign.LEFT}
        onchange={(next) => (accountId = next)}
      />
    </div>
    <div class="w-56">
      <Select
        value={channel}
        options={streamerOptions}
        label={t.stats_curve_streamer_label()}
        placeholder={t.stats_curve_streamer_empty()}
        align={MenuAlign.LEFT}
        channelShape={true}
        onchange={(next) => (selectedChannel = next)}
      />
    </div>
    <span class="flex-1"></span>
    <StatsPeriodSelector
      value={period}
      retention={snapshot.settings.history_retention}
      {retentionDays}
      onchange={(next) => (selectedPeriod = next)}
    />
  </PageHeader>

  {#if !account || streamerOptions.length === 0}
    <EmptyState text={t.stats_curve_no_streamer()} icon={IconName.CHART} />
  {:else if loading && curve === null && !problem}
    <Panel title={t.stats_curve_title()}><Skeleton class="h-56 w-full" /></Panel>
  {:else if problem}
    <EmptyState title={t.stats_load_failed()} text={problem} icon={IconName.ALERT} />
  {:else if empty}
    <EmptyState text={t.stats_curve_empty()} icon={IconName.CHANNEL_POINTS} />
  {:else}
    <Panel title={t.stats_curve_title()}>
      {#snippet actions()}
        {#if currentStreamer}
          <span class="text-hint text-muted-foreground">{currentStreamer.label}</span>
        {/if}
      {/snippet}
      <BalanceCurveChart
        {events}
        {domainStart}
        {domainEnd}
        ariaLabel={t.stats_curve_aria({ streamer: currentStreamer?.label ?? "" })}
      />
      <CurveLegend />
    </Panel>
  {/if}
</PageBody>
