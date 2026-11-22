<script lang="ts">
  import AccountSaveBar from "./AccountSaveBar.svelte";
  import { dropDone, type RewardType } from "../../lib/catalogue/campaign.utils";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { CommandName } from "../../lib/bindings/CommandName";
  import { campaignsOf } from "../../lib/catalogue/catalogue.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { InventoryTab, INVENTORY_TAB_SLUGS } from "../../lib/accounts/accounts.constants";
  import { openTab, showTab } from "../../lib/state/router";
  import { openCampaignSheet } from "../../lib/state/sheets";
  import { run } from "../../lib/state/session";
  import { ToastKind } from "../../lib/state/state.constants";
  import { toast } from "../../lib/state/toasts";
  import { strong } from "../../lib/text.utils";
  import Notice from "../ui/Notice.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import Segmented from "../ui/Segmented.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { NoticeTone } from "../ui/ui.constants";
  import InventoryClaimableTab from "./inventory/InventoryClaimableTab.svelte";
  import InventoryClaimedTab from "./inventory/InventoryClaimedTab.svelte";
  import InventoryInProgressTab from "./inventory/InventoryInProgressTab.svelte";
  import InventoryWaitingLinkTab from "./inventory/InventoryWaitingLinkTab.svelte";

  type AccountInventoryPropsType = { account: AccountSnapshotType };

  let { account }: AccountInventoryPropsType = $props();

  const rewards = $derived<RewardType[]>(
    campaignsOf(account).flatMap((campaign) => campaign.drops.map((drop) => ({ campaign, drop }))),
  );

  let claimedHere = $state<string[]>([]);

  const ready = $derived(
    rewards.filter(
      ({ drop }) => !drop.claimed && drop.instance_id && !claimedHere.includes(drop.instance_id),
    ),
  );

  // ! Twitch takes a claim on a game the account has not linked and gives nothing: those drops wait for the link.
  const claimable = $derived(ready.filter(({ campaign }) => campaign.linked !== false));
  const waitingLink = $derived(ready.filter(({ campaign }) => campaign.linked === false));

  const markClaimed = (instanceId: string): void => {
    claimedHere = [...claimedHere, instanceId];
  };

  // ! Kept here, not in the tab: switching tabs mid-"Claim all" must not lose the loop's state or let it restart.
  let claiming = $state<string | null>(null);
  let claimingAll = $state(false);

  const claim = async (instanceId: string): Promise<void> => {
    claiming = instanceId;

    if (
      await run({
        command: CommandName.CLAIM_DROP,
        account_id: account.id,
        drop_instance_id: instanceId,
      })
    ) {
      markClaimed(instanceId);
    }

    claiming = null;
  };

  const claimAll = async (): Promise<void> => {
    claimingAll = true;
    let done = 0;

    for (const { drop } of claimable) {
      if (drop.instance_id) {
        const instanceId = drop.instance_id;
        const claimedOne = await run({
          command: CommandName.CLAIM_DROP,
          account_id: account.id,
          drop_instance_id: instanceId,
        });

        if (claimedOne) {
          markClaimed(instanceId);
          done += 1;
        }
      }
    }

    claimingAll = false;

    if (done > 0) {
      toast(ToastKind.SUCCESS, t.inventory_claimed_title(), {
        description: t.inventory_claimed_all({ claimCount: strong(done) }),
        account: { url: account.avatar_url, name: account.display_name, channel: null },
      });
    }
  };

  const openCampaign = (campaign: CampaignType): void => openCampaignSheet(campaign, account.id);

  const inProgress = $derived(
    campaignsOf(account)
      .filter((campaign) => campaign.in_inventory)
      .map((campaign) => ({
        campaign,
        drops: campaign.drops.filter(
          (drop) => !dropDone(drop) && !drop.badge_owned && !(drop.instance_id && !drop.claimed),
        ),
      }))
      .filter((group) => group.drops.length > 0),
  );

  const claimed = $derived(rewards.filter(({ drop }) => dropDone(drop) || drop.badge_owned));

  const tab = $derived(
    openTab(Object.values(InventoryTab), INVENTORY_TAB_SLUGS, InventoryTab.CLAIMABLE),
  );
</script>

<PageBody>
  <PageHeader intro={t.inventory_hint()} />
  {#if account.status === AccountStatus.PAUSED && campaignsOf(account).length === 0}
    <Notice tone={NoticeTone.INFO} text={t.inventory_paused()} />
  {/if}

  <div class="flex flex-wrap items-center justify-between gap-3">
    <Segmented
      value={tab}
      label={t.inventory_tabs_label()}
      onchange={(next) => showTab(INVENTORY_TAB_SLUGS, next)}
      options={[
        {
          value: InventoryTab.CLAIMABLE,
          label: t.inventory_claimable(),
          icon: IconName.GIFT,
          count: claimable.length,
        },
        {
          value: InventoryTab.WAITING_LINK,
          label: t.inventory_waiting_link(),
          icon: IconName.LINK,
          count: waitingLink.length,
        },
        {
          value: InventoryTab.IN_PROGRESS,
          label: t.inventory_in_progress(),
          icon: IconName.CLOCK,
          count: inProgress.length,
        },
        {
          value: InventoryTab.CLAIMED,
          label: t.inventory_claimed(),
          icon: IconName.CHECK,
          count: claimed.length,
        },
      ]}
    />
  </div>

  {#if tab === InventoryTab.CLAIMABLE}
    <InventoryClaimableTab
      {account}
      {claimable}
      {claiming}
      {claimingAll}
      onclaim={claim}
      onclaimall={claimAll}
      onopen={openCampaign}
    />
  {:else if tab === InventoryTab.WAITING_LINK}
    <InventoryWaitingLinkTab {account} {waitingLink} onopen={openCampaign} />
  {:else if tab === InventoryTab.IN_PROGRESS}
    <InventoryInProgressTab {inProgress} onopen={openCampaign} />
  {:else}
    <InventoryClaimedTab {account} {claimed} onopen={openCampaign} />
  {/if}
  <AccountSaveBar {account} />
</PageBody>
