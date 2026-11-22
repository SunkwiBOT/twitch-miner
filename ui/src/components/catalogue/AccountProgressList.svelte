<script lang="ts" generics="T extends AccountProgressType">
  import type { Snippet } from "svelte";

  import {
    ALL_PROGRESS,
    NO_BAR_STATES,
    ProgressState,
    type AccountProgressType,
  } from "../../lib/catalogue/progress.constants";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { matchesSearch, normalizeSearch } from "../../lib/search/search.utils";
  import { ticker } from "../../lib/state/ticker.svelte";
  import Avatar from "../ui/Avatar.svelte";
  import Badge from "../ui/Badge.svelte";
  import ProgressLine from "../ui/ProgressLine.svelte";
  import SearchField from "../ui/SearchField.svelte";
  import SearchNoResult from "../ui/SearchNoResult.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { SELECT_SEARCH_FROM } from "../ui/menu.constants";
  import { AvatarSize } from "../ui/picture.constants";
  import { PROGRESS_BADGES, progressSummary } from "./progress.utils";
  import SubRequiredBadge from "./SubRequiredBadge.svelte";

  type AccountProgressListPropsType = {
    entries: T[];
    tags?: Snippet<[T]>;
    action?: Snippet<[T]>;
  };

  let { entries, tags, action }: AccountProgressListPropsType = $props();

  let picked = $state<ProgressState | typeof ALL_PROGRESS>(ALL_PROGRESS);
  let query = $state("");

  const summary = $derived(progressSummary(entries));
  const filter = $derived(summary.some((item) => item.state === picked) ? picked : ALL_PROGRESS);
  const searchable = $derived(entries.length >= SELECT_SEARCH_FROM);
  const shown = $derived.by(() => {
    const needle = searchable ? normalizeSearch(query) : "";

    return entries.filter(
      (entry) =>
        (filter === ALL_PROGRESS || entry.state === filter) &&
        (!needle ||
          matchesSearch(entry.account.display_name, needle) ||
          matchesSearch(entry.account.login, needle)),
    );
  });
</script>

<section class="flex flex-col gap-2">
  <h3 class="text-xs font-semibold text-muted-foreground">{t.badge_dialog_accounts()}</h3>
  {#if summary.length > 1}
    <Segmented
      value={filter}
      label={t.badge_dialog_accounts()}
      onchange={(next) => (picked = next)}
      options={[
        { value: ALL_PROGRESS, label: t.common_accounts_all(), count: entries.length },
        ...summary.map((item) => ({
          value: item.state,
          label: PROGRESS_BADGES[item.state].text(),
          icon: PROGRESS_BADGES[item.state].icon,
          count: item.count,
        })),
      ]}
    />
  {/if}
  {#if searchable}
    <SearchField
      placeholder={t.common_search()}
      aria-label={t.common_search()}
      bind:value={query}
      busy={false}
    />
    <SearchNoResult shown={shown.length === 0} {query} />
  {/if}
  <ul
    class="flex max-h-96 flex-col divide-y overflow-y-auto overscroll-contain rounded-lg border"
    hidden={shown.length === 0}
    data-account-progress-list
  >
    {#each shown as entry (entry.account.id)}
      {@const badge = PROGRESS_BADGES[entry.state]}
      <li class="flex flex-wrap items-center gap-x-3 gap-y-1.5 px-2.5 py-2">
        <Avatar
          url={entry.account.avatar_url}
          name={entry.account.display_name}
          size={AvatarSize.XS}
        />
        <span class="flex min-w-0 flex-[1_1_14rem] flex-wrap items-center gap-x-2 gap-y-1">
          <span class="truncate text-sm font-semibold">{entry.account.display_name}</span>
          <Badge
            text={badge.text()}
            tone={badge.tone}
            icon={badge.icon}
            hint={badge.hint?.(entry, ticker.now)}
          />
          {#if entry.subRequired}<SubRequiredBadge />{/if}
          {@render tags?.(entry)}
        </span>
        {#if entry.required > 0 && !NO_BAR_STATES.has(entry.state)}
          <span class="flex flex-[0_1_14rem]">
            <ProgressLine current={entry.current} required={entry.required} stacked={false} />
          </span>
        {/if}
        {@render action?.(entry)}
      </li>
    {/each}
  </ul>
</section>
