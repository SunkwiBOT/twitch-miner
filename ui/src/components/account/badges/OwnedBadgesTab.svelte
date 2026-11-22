<script lang="ts">
  import type { OwnedBadgeType } from "../../../lib/badges/badges.utils";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { GlobalBadgeType } from "../../../lib/bindings/GlobalBadgeType";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { matchesSearch, normalizeSearch } from "../../../lib/search/search.utils";
  import { strong } from "../../../lib/text.utils";
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

  const shown = $derived.by(() => {
    const needle = normalizeSearch(query);

    return owned.filter((badge) => !needle || matchesSearch(badge.title, needle));
  });

  // ! Only a badge won through a Drops reward has a date: one Twitch gives otherwise shows none.
  const details = (badge: OwnedBadgeType): string[] => {
    const at = account.owned_badge_dates[badge.key];

    return at ? [t.owned_badge_unlocked({ date: strong(formatDate(at, false)) })] : [];
  };
</script>

<Panel title={t.account_badges_owned()} count={shown.length}>
  {#if owned.length === 0}
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
              details={details(badge)}
              tooltip={true}
              onclick={known ? () => onopen(known) : undefined}
            />
          </li>
        {/each}
      </ul>
    {/if}
  {/if}
</Panel>
