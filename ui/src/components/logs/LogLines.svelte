<script lang="ts">
  import { untrack } from "svelte";
  import { fetchLogs, fetchLogsReport, openLogsDirectory } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import { queryAccountIds, visibleAccountIds } from "../../lib/accounts/account-filter.utils";
  import type { LogLevel } from "../../lib/bindings/LogLevel";
  import type { LogLineType } from "../../lib/bindings/LogLineType";
  import type { LogsQueryType } from "../../lib/bindings/LogsQueryType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { describeError } from "../../lib/errors/errors.utils";
  import { PagedFeed } from "../../lib/feed/feed.svelte";
  import { formatClockWithSeconds } from "../../lib/time/time.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { app } from "../../lib/state/app.svelte";
  import { toast } from "../../lib/state/toasts";
  import TextButton from "../ui/TextButton.svelte";
  import { Card } from "$lib/components/ui/card";
  import { cn } from "$lib/utils";
  import AccountFilter from "../ui/AccountFilter.svelte";
  import Avatar from "../ui/Avatar.svelte";
  import MultiSelect from "../ui/MultiSelect.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import { AvatarSize } from "../ui/picture.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MenuAlign } from "../ui/menu.constants";
  import LogsActions from "./LogsActions.svelte";
  import LogsFeed from "./LogsFeed.svelte";
  import {
    LOG_LEVEL_CLASSES,
    LOG_LEVEL_PHONE_CLASSES,
    LOG_LEVELS_KEPT,
    LOG_LINE_CLASSES,
    LOG_LIST_CLASSES,
    LOGS_PAGE_SIZE,
    LOGS_REFRESH_MS,
  } from "./logs.constants";
  import { LOG_LEVEL_LABELS } from "./logs.utils";

  type LogLinesPropsType = { snapshot: SnapshotType };

  let { snapshot }: LogLinesPropsType = $props();

  const accountOfLogin = (login: string) =>
    app.snapshot?.accounts.find((a) => a.login === login.toLowerCase());

  let hiddenAccounts = $state<string[]>(
    app.logsAccount
      ? untrack(() => snapshot.accounts.map((a) => a.id).filter((id) => id !== app.logsAccount))
      : [],
  );
  app.logsAccount = null;

  let levels = $state<LogLevel[]>([...LOG_LEVELS_KEPT]);

  const accountIds = $derived(visibleAccountIds(snapshot.accounts, hiddenAccounts));

  const filters = (): LogsQueryType => ({
    account_ids: queryAccountIds(accountIds, snapshot.accounts.length),
    levels: levels.length === LOG_LEVELS_KEPT.length ? undefined : levels,
  });

  const feed = new PagedFeed<LogsQueryType, LogLineType, number>({
    fetch: async (query, before) => {
      const page = await fetchLogs({
        ...query,
        before_id: before ?? undefined,
        limit: LOGS_PAGE_SIZE,
      });

      return { items: [...page.lines].reverse(), next: page.next_before_id ?? null };
    },
    isOlder: (line, oldest) => line.id < oldest.id,
  });

  feed.poll(filters, LOGS_REFRESH_MS);

  const filterLevels = (next: LogLevel[]): void => {
    levels = LOG_LEVELS_KEPT.filter((level) => next.includes(level));
  };

  const report = (): Promise<string> => fetchLogsReport(filters());

  const openFolder = async (): Promise<void> => {
    try {
      await openLogsDirectory();
    } catch (e: any) {
      toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
    }
  };
</script>

<PageHeader intro={t.logs_subtitle()}>
  {#snippet actions()}
    <LogsActions {report} copiedLabel={t.logs_copied()}>
      {#if isTauri()}
        <TextButton
          label={t.logs_open_folder()}
          icon={IconName.FILE}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={openFolder}
        />
      {/if}
    </LogsActions>
  {/snippet}
  <AccountFilter
    accounts={snapshot.accounts}
    hidden={hiddenAccounts}
    onchange={(next) => (hiddenAccounts = next)}
  />
  <div class="w-56">
    <MultiSelect
      values={levels}
      options={LOG_LEVELS_KEPT.map((level) => ({
        value: level,
        label: LOG_LEVEL_LABELS[level]?.() ?? level,
      }))}
      label={t.logs_levels()}
      allLabel={t.logs_levels_all()}
      noneLabel={t.logs_levels_none()}
      countLabel={(count) => t.logs_levels_count({ levelCount: count })}
      align={MenuAlign.LEFT}
      onchange={filterLevels}
    />
  </div>
</PageHeader>

<Card class="gap-0 px-5 py-3">
  <LogsFeed
    {feed}
    failedTitle={t.logs_load_failed()}
    emptyText={t.logs_empty()}
    moreLabel={t.logs_load_more()}
  >
    <div class={LOG_LIST_CLASSES}>
      {#each feed.items as line (line.id)}
        <div class={LOG_LINE_CLASSES}>
          <span class="whitespace-nowrap text-muted-foreground">
            {formatClockWithSeconds(line.at)}
          </span>
          <span class={cn("hidden md:block", LOG_LEVEL_CLASSES[line.level])}>{line.level}</span>
          <span class="hidden min-w-0 items-center gap-1.5 md:flex">
            {#if line.account}
              {@const owner = accountOfLogin(line.account)}
              <Avatar
                url={owner?.avatar_url}
                name={owner?.display_name ?? line.account}
                size={AvatarSize.XS}
              />
              <span class="truncate text-link">{owner?.display_name ?? line.account}</span>
            {/if}
          </span>
          <span class={LOG_LEVEL_PHONE_CLASSES[line.level]}>{line.text}</span>
        </div>
      {/each}
    </div>
  </LogsFeed>
</Card>
