<script lang="ts">
  import { cn } from "$lib/utils";
  import {
    ACCOUNT_STATE_ICON_CLASSES,
    ACCOUNT_STATE_ICONS,
  } from "../../lib/accounts/account-state.constants";
  import { accountState } from "../../lib/accounts/account-state.utils";
  import { accountStateHint, accountStateLabel } from "../../lib/accounts/accounts.utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { ticker } from "../../lib/state/ticker.svelte";
  import Icon from "./Icon.svelte";
  import Tip from "./Tip.svelte";

  type AccountStateIconPropsType = { account: AccountSnapshotType };

  let { account }: AccountStateIconPropsType = $props();

  const current = $derived(accountState(account, ticker.now));
  const label = $derived(accountStateLabel(account, current, ticker.now));
  const hint = $derived(accountStateHint(account, current, ticker.now));
</script>

<!-- ! Never focusable: it sits inside the account's row button, whose name already reads the label. -->
<Tip text={hint ? `${label} · ${hint}` : label}>
  {#snippet children(props)}
    <span
      {...props}
      role="img"
      aria-label={label}
      tabindex="-1"
      class={cn("inline-flex shrink-0 cursor-help", ACCOUNT_STATE_ICON_CLASSES[current])}
    >
      <Icon name={ACCOUNT_STATE_ICONS[current]} size={14} />
    </span>
  {/snippet}
</Tip>
