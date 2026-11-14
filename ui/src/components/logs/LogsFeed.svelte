<script lang="ts" generics="QueryType, ItemType, CursorType">
  import type { Snippet } from "svelte";

  import type { PagedFeed } from "../../lib/feed/feed.svelte";
  import { IconName } from "../../lib/icons.constants";
  import EmptyState from "../ui/EmptyState.svelte";
  import LoadMoreSentinel from "../ui/LoadMoreSentinel.svelte";
  import LogSkeleton from "./LogSkeleton.svelte";
  import { LOGS_MORE_SKELETON_ROWS, LOGS_SKELETON_ROWS } from "./logs.constants";

  type LogsFeedPropsType = {
    feed: PagedFeed<QueryType, ItemType, CursorType>;
    failedTitle: string;
    emptyText: string;
    moreLabel: string;
    children: Snippet;
  };

  let { feed, failedTitle, emptyText, moreLabel, children }: LogsFeedPropsType = $props();
</script>

{#if feed.loading && feed.items.length === 0}
  <LogSkeleton rows={LOGS_SKELETON_ROWS} />
{:else if feed.items.length === 0 && feed.problem}
  <EmptyState title={failedTitle} text={feed.problem} icon={IconName.ALERT} />
{:else if feed.items.length === 0}
  <EmptyState text={emptyText} />
{:else}
  {@render children()}
  {#if feed.loadingMore}<LogSkeleton rows={LOGS_MORE_SKELETON_ROWS} />{/if}
  <LoadMoreSentinel
    label={moreLabel}
    busy={feed.loadingMore}
    more={feed.hasMore}
    onmore={feed.loadMore}
  />
{/if}
