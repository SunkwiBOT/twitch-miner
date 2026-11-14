<script lang="ts">
  import { Card } from "$lib/components/ui/card";
  import { fetchRequests, fetchRequestsReport } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import type { RequestEntryType } from "../../lib/bindings/RequestEntryType";
  import type { RequestKind } from "../../lib/bindings/RequestKind";
  import type { RequestsQueryType } from "../../lib/bindings/RequestsQueryType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { PagedFeed } from "../../lib/feed/feed.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import {
    DESKTOP_ONLY_KINDS,
    REQUEST_KIND_GROUPS,
    REQUEST_LIST_CLASSES,
    REQUESTS_PAGE_SIZE,
  } from "../../lib/requests/requests.constants";
  import {
    isOlder,
    nextCursor,
    REQUEST_KIND_GROUP_LABELS,
    REQUEST_KIND_LABELS,
    requestKey,
    type RequestCursorType,
  } from "../../lib/requests/requests.utils";
  import MultiSelect from "../ui/MultiSelect.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import Select from "../ui/Select.svelte";
  import Toggle from "../ui/Toggle.svelte";
  import { MenuAlign } from "../ui/menu.constants";
  import LogsActions from "./LogsActions.svelte";
  import LogsFeed from "./LogsFeed.svelte";
  import { LOGS_REFRESH_MS } from "./logs.constants";
  import { accountOptions } from "./logs.utils";
  import RequestRow from "./RequestRow.svelte";

  type RequestListPropsType = { snapshot: SnapshotType };

  let { snapshot }: RequestListPropsType = $props();

  const groups = REQUEST_KIND_GROUPS.map((entry) => ({
    group: entry.group,
    kinds: entry.kinds.filter((kind) => isTauri() || !DESKTOP_ONLY_KINDS.has(kind)),
  }));
  const offered = groups.flatMap((entry) => entry.kinds);

  let accountId = $state("");
  let hidden = $state<RequestKind[]>([]);
  let errorsOnly = $state(false);

  const ticked = $derived(offered.filter((kind) => !hidden.includes(kind)));

  const options = $derived(accountOptions(snapshot.accounts));

  const kindOptions = $derived(
    groups.flatMap((entry) =>
      entry.kinds.map((kind) => ({
        value: kind,
        label: REQUEST_KIND_LABELS[kind](),
        group: REQUEST_KIND_GROUP_LABELS[entry.group](),
      })),
    ),
  );

  const accountOf = (id: string | undefined) =>
    id === undefined ? undefined : snapshot.accounts.find((a) => a.id === id);

  const filters = (): RequestsQueryType => ({
    account_id: accountId || undefined,
    kinds: ticked.length === offered.length ? undefined : ticked,
    errors_only: errorsOnly || undefined,
  });

  const feed = new PagedFeed<RequestsQueryType, RequestEntryType, RequestCursorType>({
    fetch: async (query, from) => {
      const page = await fetchRequests({ ...query, ...from, limit: REQUESTS_PAGE_SIZE });

      return { items: page.requests, next: nextCursor(page) };
    },
    isOlder,
  });

  feed.poll(filters, LOGS_REFRESH_MS);

  const report = (): Promise<string> => fetchRequestsReport(filters());
</script>

<PageHeader intro={t.requests_subtitle()}>
  {#snippet actions()}
    <LogsActions {report} copiedLabel={t.requests_copied()} />
  {/snippet}
  <div class="w-56">
    <Select
      value={accountId}
      {options}
      label={t.common_account()}
      align={MenuAlign.LEFT}
      onchange={(next) => (accountId = next)}
    />
  </div>
  <div class="w-56">
    <MultiSelect
      values={ticked}
      options={kindOptions}
      label={t.requests_types()}
      allLabel={t.requests_types_all()}
      noneLabel={t.requests_types_none()}
      countLabel={(count) => t.requests_types_count({ count })}
      align={MenuAlign.LEFT}
      onchange={(next) => (hidden = offered.filter((kind) => !next.includes(kind)))}
    />
  </div>
  <label class="flex cursor-pointer items-center gap-2 text-hint">
    <Toggle
      checked={errorsOnly}
      disabled={false}
      label={t.requests_errors_only()}
      onchange={(next) => (errorsOnly = next)}
    />
    <span>{t.requests_errors_only()}</span>
  </label>
</PageHeader>

<Card class="@container gap-0 px-5 py-3">
  <LogsFeed
    {feed}
    failedTitle={t.requests_load_failed()}
    emptyText={errorsOnly ? t.requests_empty_errors() : t.requests_empty()}
    moreLabel={t.requests_load_more()}
  >
    <div class={REQUEST_LIST_CLASSES} role="list" aria-label={t.logs_tab_requests()}>
      {#each feed.items as entry (requestKey(entry))}
        <RequestRow {entry} account={accountOf(entry.account_id)} />
      {/each}
    </div>
  </LogsFeed>
</Card>
