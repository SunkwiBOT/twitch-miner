<script lang="ts">
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import type { RewardType } from "../../../lib/catalogue/campaign.utils";
  import { refreshWhenBack } from "../../../lib/accounts/refresh.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { escapeEmphasis } from "../../../lib/text.utils";
  import LinkBadge from "../../catalogue/LinkBadge.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import ExternalButton from "../../ui/ExternalButton.svelte";
  import List from "../../ui/List.svelte";
  import ListRow from "../../ui/ListRow.svelte";
  import Panel from "../../ui/Panel.svelte";
  import RewardThumb from "../../ui/RewardThumb.svelte";
  import { ButtonSize, ButtonVariant } from "../../ui/button.constants";
  import { IconName } from "../../../lib/icons.constants";
  import { ThumbSize } from "../../ui/picture.constants";

  type InventoryWaitingLinkTabPropsType = {
    account: AccountSnapshotType;
    waitingLink: RewardType[];
    onopen: (campaign: CampaignType) => void;
  };

  let { account, waitingLink, onopen }: InventoryWaitingLinkTabPropsType = $props();
</script>

<Panel
  title={t.inventory_waiting_link()}
  count={waitingLink.length}
  description={t.badge_hint_not_linked()}
>
  {#if waitingLink.length === 0}
    <EmptyState text={t.inventory_empty_waiting_link()} />
  {:else}
    <List>
      {#each waitingLink as item (item.drop.id)}
        <ListRow
          title={item.drop.name}
          subtitle={`${escapeEmphasis(item.campaign.game.name)} · ${escapeEmphasis(item.campaign.name)}`}
          onopen={() => onopen(item.campaign)}
          muted={false}
        >
          {#snippet lead()}<RewardThumb {item} size={ThumbSize.MD} />{/snippet}
          {#snippet trailing()}
            {#if item.campaign.link_url}
              <ExternalButton
                url={item.campaign.link_url}
                label={t.campaign_link_game()}
                variant={ButtonVariant.OUTLINE}
                size={ButtonSize.SM}
                icon={IconName.LINK}
                onfollow={() => refreshWhenBack(account.id)}
              />
            {:else}
              <LinkBadge linked={false} />
            {/if}
          {/snippet}
        </ListRow>
      {/each}
    </List>
  {/if}
</Panel>
