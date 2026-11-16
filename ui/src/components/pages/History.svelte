<script lang="ts">
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { queryAccountIds, visibleAccountIds } from "../../lib/accounts/account-filter.utils";
  import { Card } from "$lib/components/ui/card";
  import AccountFilter from "../ui/AccountFilter.svelte";
  import HistoryList from "../HistoryList.svelte";
  import PageBody from "../ui/PageBody.svelte";

  type HistoryPropsType = { snapshot: SnapshotType };

  let { snapshot }: HistoryPropsType = $props();

  let hiddenAccounts = $state<string[]>([]);

  const accountIds = $derived(visibleAccountIds(snapshot.accounts, hiddenAccounts));
  const queryIds = $derived(queryAccountIds(accountIds, snapshot.accounts.length));
</script>

<PageBody>
  <Card class="px-5 py-4">
    <HistoryList accountIds={queryIds} showAccount={accountIds.length !== 1}>
      {#snippet lead()}
        <AccountFilter
          accounts={snapshot.accounts}
          hidden={hiddenAccounts}
          onchange={(next) => (hiddenAccounts = next)}
        />
      {/snippet}
    </HistoryList>
  </Card>
</PageBody>
