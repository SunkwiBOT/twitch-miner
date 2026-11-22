<script lang="ts">
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { accountOptions, visibleAccountIds } from "../../lib/accounts/account-filter.utils";
  import MultiSelect from "./MultiSelect.svelte";
  import { MenuAlign } from "./menu.constants";

  type AccountFilterPropsType = {
    accounts: AccountSnapshotType[];
    hidden: string[];
    onchange: (hidden: string[]) => void;
  };

  let { accounts, hidden, onchange }: AccountFilterPropsType = $props();

  const options = $derived(accountOptions(accounts));
  const values = $derived(visibleAccountIds(accounts, hidden));
</script>

<div class="w-56">
  <MultiSelect
    {values}
    {options}
    label={t.common_account()}
    allLabel={t.common_accounts_all()}
    noneLabel={t.common_accounts_none()}
    countLabel={(count) => t.common_accounts_count({ accountCount: count })}
    align={MenuAlign.LEFT}
    onchange={(next) => onchange(accounts.map((a) => a.id).filter((id) => !next.includes(id)))}
  />
</div>
