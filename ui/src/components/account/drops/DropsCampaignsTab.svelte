<script lang="ts">
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { matchesSearch, normalizeSearch } from "../../../lib/search/search.utils";
  import CampaignCard from "../../catalogue/CampaignCard.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import Panel from "../../ui/Panel.svelte";
  import SearchField from "../../ui/SearchField.svelte";

  type DropsCampaignsTabPropsType = {
    account: AccountSnapshotType;
    campaigns: CampaignType[];
    onopen: (campaign: CampaignType) => void;
  };

  let { account, campaigns, onopen }: DropsCampaignsTabPropsType = $props();

  let query = $state("");

  const shown = $derived.by(() => {
    const needle = normalizeSearch(query);

    return campaigns.filter(
      (c) => !needle || matchesSearch(c.game.name, needle) || matchesSearch(c.name, needle),
    );
  });
</script>

<Panel title={t.drops_all_campaigns()}>
  <div class="w-full max-w-96">
    <SearchField
      placeholder={t.campaigns_search_placeholder()}
      aria-label={t.campaigns_search_placeholder()}
      bind:value={query}
      busy={false}
    />
  </div>
  {#if shown.length === 0}
    <EmptyState text={t.campaigns_empty()} />
  {:else}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-4">
      {#each shown as c (c.id)}
        <CampaignCard
          campaign={c}
          ordered={account.settings.drops.campaign_order.includes(c.id)}
          {onopen}
        />
      {/each}
    </div>
  {/if}
</Panel>
