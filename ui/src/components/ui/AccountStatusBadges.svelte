<script lang="ts">
  import {
    ACCOUNT_STATE_ICONS,
    ACCOUNT_STATE_TONES,
    AccountState,
  } from "../../lib/accounts/account-state.constants";
  import { accountState } from "../../lib/accounts/account-state.utils";
  import { accountStateHint, accountStateLabel } from "../../lib/accounts/accounts.utils";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { MinerKind } from "../../lib/bindings/MinerKind";
  import { ticker } from "../../lib/state/ticker.svelte";
  import Badge from "./Badge.svelte";
  import MiningBadge from "./MiningBadge.svelte";

  type AccountStatusBadgesPropsType = { account: AccountSnapshotType };

  let { account }: AccountStatusBadgesPropsType = $props();

  const current = $derived(accountState(account, ticker.now));

  const miners = $derived(
    current === AccountState.PAUSED || account.status !== AccountStatus.MINING
      ? []
      : Object.values(MinerKind).filter((kind) =>
          account.slots.some((slot) => slot.miner === kind),
        ),
  );
</script>

{#each miners as kind (kind)}<MiningBadge {kind} />{/each}
{#if miners.length === 0 || current === AccountState.PROBLEM}
  <Badge
    text={accountStateLabel(account, current, ticker.now)}
    tone={ACCOUNT_STATE_TONES[current]}
    dot={false}
    icon={ACCOUNT_STATE_ICONS[current]}
    hint={accountStateHint(account, current, ticker.now)}
  />
{/if}
