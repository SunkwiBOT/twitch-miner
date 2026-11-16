<script lang="ts">
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import type { RewardType } from "../../../lib/catalogue/campaign.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { AccountField } from "../../../lib/state/state.constants";
  import { editsOf } from "../../../lib/state/edits.svelte";
  import { escapeEmphasis } from "../../../lib/text.utils";
  import ActionButton from "../../ui/ActionButton.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import List from "../../ui/List.svelte";
  import ListRow from "../../ui/ListRow.svelte";
  import Panel from "../../ui/Panel.svelte";
  import RewardThumb from "../../ui/RewardThumb.svelte";
  import Setting from "../../ui/Setting.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../../ui/button.constants";
  import { IconName } from "../../../lib/icons.constants";
  import { ThumbSize } from "../../ui/picture.constants";
  import { untrack } from "svelte";

  type InventoryClaimableTabPropsType = {
    account: AccountSnapshotType;
    claimable: RewardType[];
    claiming: string | null;
    claimingAll: boolean;
    onclaim: (instanceId: string) => void;
    onclaimall: () => void;
    onopen: (campaign: CampaignType) => void;
  };

  let {
    account,
    claimable,
    claiming,
    claimingAll,
    onclaim,
    onclaimall,
    onopen,
  }: InventoryClaimableTabPropsType = $props();

  const edits = editsOf(untrack(() => account.id));

  const setAutoClaim = (value: boolean): void => {
    edits.set(AccountField.AUTO_CLAIM, (settings) => (settings.drops.auto_claim = value));
  };
</script>

<Panel title={t.inventory_claimable()} count={claimable.length}>
  {#snippet actions()}
    {#if claimable.length > 1}
      <ActionButton
        icon={IconName.GIFT}
        label={t.inventory_claim_all()}
        busyLabel={t.campaign_claiming()}
        busy={claimingAll}
        disabled={claiming !== null}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        onclick={onclaimall}
      />
    {/if}
  {/snippet}
  <Setting
    title={t.account_drops_auto_claim()}
    description={t.drops_auto_claim_hint()}
    checked={edits.applied(account.settings).drops.auto_claim}
    disabled={false}
    onchange={setAutoClaim}
  />
  {#if claimable.length === 0}
    <EmptyState text={t.inventory_empty_claimable()} />
  {:else}
    <List>
      {#each claimable as item (item.drop.id)}
        {@const instanceId = item.drop.instance_id ?? ""}
        <ListRow
          title={item.drop.name}
          subtitle={`${escapeEmphasis(item.campaign.game.name)} · ${escapeEmphasis(item.campaign.name)}`}
          onopen={() => onopen(item.campaign)}
          muted={false}
        >
          {#snippet lead()}<RewardThumb {item} size={ThumbSize.MD} />{/snippet}
          {#snippet trailing()}
            <ActionButton
              icon={IconName.GIFT}
              label={t.reward_claim()}
              busyLabel={t.campaign_claiming()}
              busy={claiming === instanceId}
              disabled={claimingAll || (claiming !== null && claiming !== instanceId)}
              variant={ButtonVariant.PRIMARY}
              size={ButtonSize.SM}
              type={ButtonType.BUTTON}
              onclick={() => onclaim(instanceId)}
            />
          {/snippet}
        </ListRow>
      {/each}
    </List>
  {/if}
</Panel>
