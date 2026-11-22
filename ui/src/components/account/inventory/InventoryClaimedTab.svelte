<script lang="ts">
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import { campaignSubtitle, type RewardType } from "../../../lib/catalogue/campaign.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { IconName } from "../../../lib/icons.constants";
  import { strong } from "../../../lib/text.utils";
  import { formatDate } from "../../../lib/time/time.utils";
  import BadgeTile from "../../ui/BadgeTile.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import Panel from "../../ui/Panel.svelte";

  type InventoryClaimedTabPropsType = {
    account: AccountSnapshotType;
    claimed: RewardType[];
    onopen: (campaign: CampaignType) => void;
  };

  let { account, claimed, onopen }: InventoryClaimedTabPropsType = $props();

  // ! The same tile as an owned badge: the reward's category and campaign, then its claim date when Twitch gave one.
  const details = (item: RewardType): string[] => {
    const at = account.drop_dates[item.drop.id];
    const campaign = campaignSubtitle(item.campaign);

    return [
      item.campaign.game.name,
      ...(campaign ? [campaign] : []),
      ...(at ? [t.owned_badge_unlocked({ date: strong(formatDate(at, false)) })] : []),
    ];
  };
</script>

<Panel title={t.inventory_claimed()} count={claimed.length}>
  {#if claimed.length === 0}
    <EmptyState text={t.inventory_empty_claimed()} />
  {:else}
    <ul class="grid grid-cols-[repeat(auto-fill,minmax(min(250px,100%),1fr))] gap-2.5">
      {#each claimed as item (item.drop.id)}
        <li class="grid">
          <BadgeTile
            image={item.drop.benefits.at(0)?.image_url}
            title={item.drop.name}
            details={details(item)}
            fallback={IconName.GIFT}
            tooltip={true}
            onclick={() => onopen(item.campaign)}
          />
        </li>
      {/each}
    </ul>
  {/if}
</Panel>
