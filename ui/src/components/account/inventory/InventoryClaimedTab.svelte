<script lang="ts">
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import type { RewardType } from "../../../lib/catalogue/campaign.utils";
  import { gameArt } from "../../../lib/catalogue/games.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import GameName from "../../ui/GameName.svelte";
  import Panel from "../../ui/Panel.svelte";
  import RewardThumb from "../../ui/RewardThumb.svelte";
  import { ThumbSize } from "../../ui/picture.constants";

  type InventoryClaimedTabPropsType = {
    claimed: RewardType[];
    onopen: (campaign: CampaignType) => void;
  };

  let { claimed, onopen }: InventoryClaimedTabPropsType = $props();
</script>

<Panel title={t.inventory_claimed()} count={claimed.length}>
  {#if claimed.length === 0}
    <EmptyState text={t.inventory_empty_claimed()} />
  {:else}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-2.5">
      {#each claimed as item (item.drop.id)}
        <button
          class="flex items-center gap-2.5 rounded-lg border p-2 text-left transition-colors hover:bg-accent"
          type="button"
          onclick={() => onopen(item.campaign)}
        >
          <RewardThumb {item} size={ThumbSize.MD} />
          <span class="flex min-w-0 flex-col">
            <span class="truncate text-xs font-semibold">{item.drop.name}</span>
            <span class="text-xs text-muted-foreground">
              <GameName
                name={item.campaign.game.name}
                url={null}
                art={gameArt(item.campaign.game.name, item.campaign.game.box_art_url)}
                size={ThumbSize.XS}
              />
            </span>
          </span>
        </button>
      {/each}
    </div>
  {/if}
</Panel>
