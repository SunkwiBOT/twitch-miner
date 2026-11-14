<script lang="ts">
  import { STATUS_HINTS, STATUS_LABELS, STATUS_TONES } from "../../lib/accounts/accounts.utils";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { MinerKind } from "../../lib/bindings/MinerKind";
  import Badge from "./Badge.svelte";
  import MiningBadge from "./MiningBadge.svelte";

  type AccountStatusBadgesPropsType = { account: AccountSnapshotType };

  let { account }: AccountStatusBadgesPropsType = $props();

  const miners = $derived(
    Object.values(MinerKind).filter((kind) => account.slots.some((slot) => slot.miner === kind)),
  );
</script>

{#if account.status === AccountStatus.MINING && miners.length > 0}
  {#each miners as kind (kind)}<MiningBadge {kind} />{/each}
{:else}
  <Badge
    text={STATUS_LABELS[account.status]()}
    tone={STATUS_TONES[account.status]}
    dot={true}
    hint={STATUS_HINTS[account.status]?.()}
  />
{/if}
