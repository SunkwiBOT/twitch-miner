<script lang="ts">
  import { type Snippet, untrack } from "svelte";

  import EmptyState from "./ui/EmptyState.svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
  } from "$lib/components/ui/table";
  import { exportCsvToFile, fetchHistory } from "../lib/api/client";
  import type { CategoryCountType } from "../lib/bindings/CategoryCountType";
  import type { EventKind } from "../lib/bindings/EventKind";
  import type { HistoryQueryType } from "../lib/bindings/HistoryQueryType";
  import type { MinerEventType } from "../lib/bindings/MinerEventType";
  import { ensureChannels } from "../lib/channels/channels.svelte";
  import { describeError } from "../lib/errors/errors.utils";
  import { EVENT_ICONS, JOURNAL_ALL, JOURNAL_FILTER_SLUGS } from "../lib/events/events.constants";
  import {
    EVENT_KIND_LABELS,
    JOURNAL_FILTERS,
    KIND_GROUP_LABELS,
  } from "../lib/events/labels.utils";
  import { kindGroupsOf } from "../lib/events/events.utils";
  import { shownEvent, unnamedChannels } from "../lib/events/shown.svelte";
  import { PagedFeed } from "../lib/feed/feed.svelte";
  import { t } from "../lib/i18n/i18n.svelte";
  import { app } from "../lib/state/app.svelte";
  import { openTab, showTab } from "../lib/state/router";
  import { ToastKind } from "../lib/state/state.constants";
  import { toast } from "../lib/state/toasts";
  import { code, strong } from "../lib/text.utils";
  import {
    HISTORY_EVENT_MIN_WIDTH_PX,
    HISTORY_MORE_SKELETON_ROWS,
    HISTORY_PAGE_SIZE,
    HISTORY_SKELETON_ROWS,
  } from "./history.constants";
  import HistoryEvent from "./HistoryEvent.svelte";
  import ActionButton from "./ui/ActionButton.svelte";
  import Dialog from "./ui/Dialog.svelte";
  import LoadMoreSentinel from "./ui/LoadMoreSentinel.svelte";
  import MultiSelect from "./ui/MultiSelect.svelte";
  import Segmented from "./ui/Segmented.svelte";
  import { IconName } from "../lib/icons.constants";
  import { MenuAlign } from "./ui/menu.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "./ui/button.constants";

  type HistoryListPropsType = {
    accountIds: string[] | undefined;
    showAccount: boolean;
    lead?: Snippet;
  };

  let { accountIds, showAccount, lead }: HistoryListPropsType = $props();

  const filter = $derived(
    openTab(
      JOURNAL_FILTERS.map((item) => item.value),
      JOURNAL_FILTER_SLUGS,
      JOURNAL_ALL,
    ),
  );

  let hidden = $state<EventKind[]>([]);

  const groups = $derived(kindGroupsOf(filter));
  const offered = $derived([...new Set(groups.flatMap((entry) => entry.kinds))]);
  const ticked = $derived(offered.filter((kind) => !hidden.includes(kind)));

  const kindOptions = $derived(
    groups.flatMap((entry) =>
      entry.kinds.map((kind) => ({
        value: kind,
        label: EVENT_KIND_LABELS[kind](),
        icon: EVENT_ICONS[kind],
        group: KIND_GROUP_LABELS[entry.group](),
      })),
    ),
  );

  const query = (): HistoryQueryType => ({
    account_ids: accountIds,
    category: filter === JOURNAL_ALL ? undefined : filter,
    kinds: ticked.length === offered.length ? undefined : ticked,
  });

  const feed = new PagedFeed<HistoryQueryType, MinerEventType, number>({
    fetch: async (request, before) => {
      const page = await fetchHistory({
        ...request,
        before_id: before ?? undefined,
        limit: HISTORY_PAGE_SIZE,
      });

      return { items: page.events, next: page.next_before_id ?? null };
    },
    isOlder: (event, oldest) => event.id < oldest.id,
  });

  const shown = $derived(feed.items.map(shownEvent));

  const showKinds = (values: EventKind[]): void => {
    hidden = [
      ...hidden.filter((kind) => !offered.includes(kind)),
      ...offered.filter((kind) => !values.includes(kind)),
    ];
  };

  const serializedQuery = $derived(JSON.stringify(query()));

  $effect(() => {
    void serializedQuery;
    void app.liveEvents.length;
    void feed.show(untrack(query));
  });

  let exporting = $state(false);
  let counts = $state<CategoryCountType[] | null>(null);

  const exportHistory = async (): Promise<void> => {
    exporting = true;

    try {
      const filename = `claimed-drops-${new Date().toISOString().slice(0, 10)}.csv`;
      const result = await exportCsvToFile({ account_ids: accountIds }, filename);

      if (result) {
        counts = result.counts;

        if (result.truncated) {
          toast(ToastKind.INFO, t.export_csv_truncated());
        }

        if (result.path) {
          toast(ToastKind.SUCCESS, t.export_csv_saved(), { description: code(result.path) });
        }
      }
    } catch (e: any) {
      toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
    } finally {
      exporting = false;
    }
  };

  $effect(() => {
    ensureChannels(unnamedChannels(feed.items));
  });

  let room = $state(0);
  let table = $state<HTMLTableElement | null>(null);
  let eventHead = $state<HTMLTableCellElement | null>(null);
  // ! Every column but the event sizes to its content, so the table's need is measured while it shows and kept while the cards do.
  let tableNeed = $state(0);
  const tableFits = $derived(room >= tableNeed);

  // ! New content (a filter, new events) may need less room: the table is tried again, and measured before it paints.
  $effect(() => {
    void shown;

    untrack(() => {
      if (!tableFits) {
        tableNeed = 0;
      }
    });
  });

  $effect(() => {
    void shown;
    void room;

    if (table && eventHead) {
      tableNeed = Math.ceil(
        table.getBoundingClientRect().width -
          eventHead.getBoundingClientRect().width +
          HISTORY_EVENT_MIN_WIDTH_PX,
      );
    }
  });
</script>

<div class="mb-3 flex flex-wrap items-center gap-2.5">
  {#if lead}{@render lead()}{/if}
  <ActionButton
    icon={IconName.FILE}
    label={t.export_csv_button()}
    busyLabel={t.export_csv_downloading()}
    busy={exporting}
    disabled={false}
    variant={ButtonVariant.DEFAULT}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    onclick={exportHistory}
  />
  <span class="flex-1"></span>
  <div class="w-56">
    <MultiSelect
      values={ticked}
      options={kindOptions}
      label={t.journal_types()}
      allLabel={t.journal_types_all()}
      noneLabel={t.journal_types_none()}
      countLabel={(count) => t.journal_types_count({ typeCount: strong(count) })}
      align={MenuAlign.RIGHT}
      onchange={showKinds}
    />
  </div>
  <Segmented
    value={filter}
    options={JOURNAL_FILTERS.map((item) => ({
      value: item.value,
      label: item.label(),
      icon: item.icon,
    }))}
    label={t.journal_category()}
    onchange={(next) => showTab(JOURNAL_FILTER_SLUGS, next)}
  />
</div>

{#snippet head()}
  <TableHeader>
    <TableRow class="hover:bg-transparent">
      <TableHead class="pl-0">{t.journal_col_time()}</TableHead>
      {#if showAccount}<TableHead>{t.journal_col_account()}</TableHead>{/if}
      <TableHead
        class="w-full"
        style="min-width: {HISTORY_EVENT_MIN_WIDTH_PX}px"
        bind:ref={eventHead}
      >
        {t.journal_col_event()}
      </TableHead>
      <TableHead>{t.journal_col_channel()}</TableHead>
      <TableHead>{t.journal_col_type()}</TableHead>
      <TableHead>{t.journal_col_miner()}</TableHead>
    </TableRow>
  </TableHeader>
{/snippet}

{#snippet skeleton(rows: number)}
  {#each { length: rows }, index (index)}
    <TableRow class="hover:bg-transparent" aria-hidden="true">
      <TableCell class="pl-0"><Skeleton class="h-3 w-24" /></TableCell>
      {#if showAccount}<TableCell><Skeleton class="h-5 w-20 rounded-full" /></TableCell>{/if}
      <TableCell>
        <span class="flex items-center gap-2.5">
          <Skeleton class="size-7 shrink-0 rounded-md" />
          <Skeleton class="h-3.5 w-2/3" />
        </span>
      </TableCell>
      <TableCell><Skeleton class="h-5 w-24 rounded-full" /></TableCell>
      <TableCell><Skeleton class="h-5 w-20 rounded-full" /></TableCell>
      <TableCell><Skeleton class="h-5 w-24 rounded-full" /></TableCell>
    </TableRow>
  {/each}
{/snippet}

{#snippet cardSkeleton(rows: number)}
  {#each { length: rows }, index (index)}
    <li class="flex gap-2.5 py-3" aria-hidden="true">
      <Skeleton class="size-7 shrink-0 rounded-md" />
      <span class="flex flex-1 flex-col gap-2">
        <Skeleton class="h-3.5 w-4/5" />
        <Skeleton class="h-3 w-1/2" />
      </span>
    </li>
  {/each}
{/snippet}

{#if feed.loading && feed.items.length === 0}
  <div bind:clientWidth={room}>
    {#if tableFits}
      <Table class="text-hint" bind:ref={table}>
        {@render head()}
        <TableBody>{@render skeleton(HISTORY_SKELETON_ROWS)}</TableBody>
      </Table>
    {:else}
      <ul class="flex flex-col divide-y text-hint">
        {@render cardSkeleton(HISTORY_SKELETON_ROWS)}
      </ul>
    {/if}
  </div>
{:else if feed.items.length === 0 && feed.problem}
  <EmptyState title={t.history_load_failed()} text={feed.problem} icon={IconName.ALERT} />
{:else if feed.items.length === 0}
  <EmptyState text={t.history_empty()} />
{:else}
  <div bind:clientWidth={room}>
    {#if tableFits}
      <Table class="text-hint" bind:ref={table}>
        {@render head()}
        <TableBody>
          {#each shown as event (event.id)}
            <HistoryEvent {event} {showAccount} wide={true} />
          {/each}
          {#if feed.loadingMore}{@render skeleton(HISTORY_MORE_SKELETON_ROWS)}{/if}
        </TableBody>
      </Table>
    {:else}
      <!-- ! Narrower than the table's columns, it would scroll sideways or cut its badges: one card per event. -->
      <ul class="flex flex-col divide-y text-hint">
        {#each shown as event (event.id)}
          <HistoryEvent {event} {showAccount} wide={false} />
        {/each}
        {#if feed.loadingMore}{@render cardSkeleton(HISTORY_MORE_SKELETON_ROWS)}{/if}
      </ul>
    {/if}
  </div>
  <LoadMoreSentinel
    label={t.history_load_more()}
    busy={feed.loadingMore}
    more={feed.hasMore}
    onmore={feed.loadMore}
  />
{/if}

<Dialog
  open={counts !== null}
  title={t.export_counts_title()}
  wide={false}
  closable={true}
  closeOnOutside={true}
  closeLabel={t.common_close()}
  onclose={() => (counts = null)}
>
  {#if counts && counts.length > 0}
    <ul class="flex flex-col divide-y text-sm">
      {#each counts as entry (entry.category)}
        <li class="flex items-center justify-between gap-3 py-2">
          <span class="min-w-0 truncate">
            {entry.category.trim() === "" ? t.journal_filter_other() : entry.category}
          </span>
          <span class="shrink-0 font-medium tabular-nums text-muted-foreground">
            {(entry.count === 1 ? t.export_counts_count_one : t.export_counts_count)({
              claimCount: entry.count,
            })}
          </span>
        </li>
      {/each}
    </ul>
  {:else}
    <EmptyState text={t.export_counts_empty()} />
  {/if}
</Dialog>
