<script lang="ts">
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { badgeList } from "../../../lib/badges/badges.svelte";
  import {
    BADGE_TILE_CLASSES,
    OWNED_BADGES_SKELETON_COUNT,
  } from "../../../lib/badges/badges.constants";
  import type { OwnedBadgeType } from "../../../lib/badges/badges.utils";
  import { loadOwnedClaims, ownedClaimsOf } from "../../../lib/badges/owned-claims.svelte";
  import { unlockDates } from "../../../lib/badges/owned-dates.utils";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { GlobalBadgeType } from "../../../lib/bindings/GlobalBadgeType";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { matchesSearch, normalizeSearch } from "../../../lib/search/search.utils";
  import { formatDate } from "../../../lib/time/time.utils";
  import BadgeTile from "../../ui/BadgeTile.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import Panel from "../../ui/Panel.svelte";
  import SearchField from "../../ui/SearchField.svelte";

  type OwnedBadgesTabPropsType = {
    account: AccountSnapshotType;
    owned: OwnedBadgeType[];
    onopen: (badge: GlobalBadgeType) => void;
  };

  let { account, owned, onopen }: OwnedBadgesTabPropsType = $props();

  let query = $state("");

  const claims = $derived(ownedClaimsOf(account.id));

  const shown = $derived.by(() => {
    const needle = normalizeSearch(query);

    return owned.filter((badge) => !needle || matchesSearch(badge.title, needle));
  });

  const dates = $derived(claims.events && unlockDates(account, claims.events, badgeList.badges));

  $effect(() => {
    void loadOwnedClaims(account.id);
  });

  const detail = (badge: OwnedBadgeType): string | undefined => {
    if (!dates) {
      return undefined;
    }

    const at = dates.get(badge.key);

    return at
      ? t.owned_badge_unlocked({ date: formatDate(at, true) })
      : t.owned_badge_date_unknown();
  };
</script>

{#snippet tileSkeleton()}
  <li class={BADGE_TILE_CLASSES} aria-hidden="true">
    <Skeleton class="size-9 shrink-0 rounded-md" />
    <span class="flex min-w-0 flex-1 flex-col gap-1.5">
      <Skeleton class="h-3.5 w-2/3" />
      <Skeleton class="h-3 w-1/2" />
    </span>
  </li>
{/snippet}

<Panel title={t.account_badges_owned()} count={shown.length}>
  {#if claims.loading && claims.events === null}
    <ul class="grid grid-cols-[repeat(auto-fill,minmax(min(250px,100%),1fr))] gap-2.5">
      {#each { length: OWNED_BADGES_SKELETON_COUNT }, index (index)}
        {@render tileSkeleton()}
      {/each}
    </ul>
  {:else if owned.length === 0}
    <EmptyState text={t.account_badges_empty_owned()} />
  {:else}
    <div class="w-full max-w-96">
      <SearchField
        placeholder={t.account_badges_search_placeholder()}
        aria-label={t.account_badges_search_placeholder()}
        bind:value={query}
        busy={false}
      />
    </div>
    {#if shown.length === 0}
      <EmptyState text={t.badges_empty()} />
    {:else}
      <ul class="grid grid-cols-[repeat(auto-fill,minmax(min(250px,100%),1fr))] gap-2.5">
        {#each shown as badge (badge.key)}
          {@const known = badge.badge}
          <li class="grid">
            <BadgeTile
              image={badge.image}
              title={badge.title}
              detail={detail(badge)}
              tooltip={true}
              onclick={known ? () => onopen(known) : undefined}
            />
          </li>
        {/each}
      </ul>
    {/if}
  {/if}
</Panel>
