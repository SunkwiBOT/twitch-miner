<script lang="ts">
  import { untrack } from "svelte";

  import { fetchHistory } from "../../../lib/api/client";
  import type { OwnedBadgeType } from "../../../lib/badges/badges.utils";
  import { badgeList } from "../../../lib/badges/badges.svelte";
  import { unlockDates } from "../../../lib/badges/owned-dates.utils";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import { EventKind } from "../../../lib/bindings/EventKind";
  import type { GlobalBadgeType } from "../../../lib/bindings/GlobalBadgeType";
  import type { MinerEventType } from "../../../lib/bindings/MinerEventType";
  import { describeError } from "../../../lib/errors/errors.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { matchesSearch, normalizeSearch } from "../../../lib/search/search.utils";
  import { ToastKind } from "../../../lib/state/state.constants";
  import { toast } from "../../../lib/state/toasts";
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
  let claims = $state.raw<MinerEventType[] | null>(null);

  const shown = $derived.by(() => {
    const needle = normalizeSearch(query);

    return owned.filter((badge) => !needle || matchesSearch(badge.title, needle));
  });

  const dates = $derived(claims && unlockDates(account, claims, badgeList.badges));

  const loadClaims = async (accountId: string): Promise<void> => {
    const found: MinerEventType[] = [];
    let before: number | undefined;

    try {
      do {
        const page = await fetchHistory({
          account_ids: [accountId],
          kinds: [EventKind.DROP_CLAIMED],
          before_id: before,
        });

        found.push(...page.events);
        before = page.next_before_id;
      } while (before !== undefined);

      claims = found;
    } catch (e: unknown) {
      toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
    }
  };

  $effect(() => {
    void loadClaims(untrack(() => account.id));
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

<Panel title={t.account_badges_owned()} count={owned.length}>
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
