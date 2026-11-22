<script lang="ts">
  import {
    ACCOUNT_STATE_ICONS,
    ACCOUNT_STATE_TONES,
    AccountState,
  } from "../../lib/accounts/account-state.constants";
  import { accountState } from "../../lib/accounts/account-state.utils";
  import {
    accountStateHint,
    accountStateLabel,
    activeMiners,
  } from "../../lib/accounts/accounts.utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { ticker } from "../../lib/state/ticker.svelte";
  import Badge from "./Badge.svelte";
  import MiningBadge from "./MiningBadge.svelte";

  type AccountStatusBadgesPropsType = { account: AccountSnapshotType };

  let { account }: AccountStatusBadgesPropsType = $props();

  const current = $derived(accountState(account, ticker.now));

  const miners = $derived(current === AccountState.PAUSED ? [] : activeMiners(account));
</script>

{#each miners as kind (kind)}<MiningBadge {kind} />{/each}
{#if miners.length === 0 || current === AccountState.PROBLEM}
  <Badge
    text={accountStateLabel(account, current, ticker.now)}
    tone={ACCOUNT_STATE_TONES[current]}
    icon={ACCOUNT_STATE_ICONS[current]}
    hint={accountStateHint(account, current, ticker.now)}
  />
{/if}
