<script lang="ts">
  import { laneEnabled } from "../../lib/accounts/mining-order.utils";
  import type { MiningLaneType } from "../../lib/bindings/MiningLaneType";
  import { saveAccountSettings } from "../../lib/state/state.svelte";
  import {
    AccountBadgeState,
    accountBadges,
    ownedBadges,
    type AccountBadgeType,
  } from "../../lib/badges/badges.utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { MiningLaneKind } from "../../lib/bindings/MiningLaneKind";
  import { campaignEnd } from "../../lib/catalogue/campaign.utils";
  import { minutes } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { escapeEmphasis } from "../../lib/text.utils";
  import { badgeList } from "../../lib/badges/badges.svelte";
  import { relativeTime } from "../../lib/time/time.utils";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
  import Badge from "../ui/Badge.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import MiningOrderList from "./drops/MiningOrderList.svelte";
  import EmptyState from "../ui/EmptyState.svelte";
  import List from "../ui/List.svelte";
  import ListRow from "../ui/ListRow.svelte";
  import Notice from "../ui/Notice.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PageHeader from "../ui/PageHeader.svelte";
  import Panel from "../ui/Panel.svelte";
  import ProgressLine from "../ui/ProgressLine.svelte";
  import Thumb from "../ui/Thumb.svelte";
  import {
    BadgeTone,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    IconName,
    NoticeTone,
    ThumbShape,
    ThumbSize,
  } from "../ui/ui.constants";

  type AccountBadgesPropsType = { account: AccountSnapshotType };

  let { account }: AccountBadgesPropsType = $props();

  const sections = $derived(accountBadges(account, ticker.now));
  const owned = $derived(ownedBadges(account, badgeList.badges));
  const badgesOn = $derived(
    laneEnabled(account.settings.drops.mining_order, MiningLaneKind.BADGES),
  );

  let open = $state<CampaignType | null>(null);

  let order = $state<MiningLaneType[] | null>(null);
  let saving = $state(false);

  const saveOrder = async (lanes: MiningLaneType[]): Promise<void> => {
    saving = true;

    const settings = account.settings;
    const saved = await saveAccountSettings(account.id, {
      ...settings,
      drops: { ...settings.drops, mining_order: lanes },
    });

    saving = false;

    if (saved) {
      order = null;
    }
  };
</script>

{#snippet rows(items: AccountBadgeType[], progress: boolean)}
  <List>
    {#each items as item (item.key)}
      <ListRow
        title={item.title}
        details={[escapeEmphasis(item.source), campaignEnd(item.endsAt, ticker.now)]}
        onopen={item.campaign ? () => (open = item.campaign) : undefined}
        muted={false}
      >
        {#snippet lead()}
          <Thumb
            src={item.image}
            round={false}
            contain={true}
            size={ThumbSize.MD}
            shape={ThumbShape.SQUARE}
            fallback={IconName.AWARD}
          />
        {/snippet}
        {#snippet badges()}
          {#if item.quest}
            <Badge
              text={t.quest_label()}
              tone={BadgeTone.INFO}
              dot={false}
              hint={t.badge_hint_quest()}
            />
          {/if}
          {#if item.state === AccountBadgeState.CLAIMABLE}
            <Badge
              text={t.reward_state_ready()}
              tone={BadgeTone.PRIMARY}
              dot={false}
              icon={IconName.GIFT}
            />
          {:else if item.state === AccountBadgeState.NOT_MINABLE}
            <Badge
              text={t.quest_not_minable()}
              tone={BadgeTone.MUTED}
              dot={false}
              hint={t.badge_hint_quest_not_minable()}
            />
          {:else if item.state === AccountBadgeState.SUB_ONLY}
            <Badge
              text={t.reward_sub_only()}
              tone={BadgeTone.WARNING}
              dot={false}
              hint={t.badge_hint_sub_only()}
            />
          {:else if item.state === AccountBadgeState.WAITING && item.nextAt}
            <Badge
              text={t.quest_next_window({ time: relativeTime(item.nextAt, ticker.now) })}
              tone={BadgeTone.MUTED}
              dot={false}
              icon={IconName.CLOCK}
            />
          {:else if !progress && item.required > 0}
            <Badge
              text={t.reward_watch({ time: minutes(item.required) })}
              tone={BadgeTone.NEUTRAL}
              dot={false}
            />
          {/if}
        {/snippet}
        {#snippet below()}
          {#if progress && item.required > 0}
            <ProgressLine current={item.current} required={item.required} stacked={false} />
          {/if}
        {/snippet}
      </ListRow>
    {/each}
  </List>
{/snippet}

<PageBody>
  <PageHeader intro={t.account_badges_hint()}>
    {#snippet actions()}
      <TextButton
        label={t.mining_order_title()}
        icon={IconName.LIST_ORDERED}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => (order = account.settings.drops.mining_order)}
      />
    {/snippet}
  </PageHeader>
  {#if !account.badges_read_at}
    <Notice tone={NoticeTone.INFO} text={t.account_badges_not_read()} />
  {/if}
  {#if !badgesOn}
    <Notice tone={NoticeTone.WARNING} text={t.account_badges_mining_off()} />
  {/if}

  <Panel title={t.inventory_in_progress()} count={sections.inProgress.length}>
    {#if sections.inProgress.length === 0}
      <EmptyState text={t.account_badges_empty_progress()} />
    {:else}
      {@render rows(sections.inProgress, true)}
    {/if}
  </Panel>

  <Panel title={t.account_badges_available()} count={sections.available.length}>
    {#if sections.available.length === 0}
      <EmptyState text={t.account_badges_empty_available()} />
    {:else}
      {@render rows(sections.available, false)}
    {/if}
  </Panel>

  {#if sections.notMined.length > 0}
    <Panel
      title={t.account_badges_not_mined()}
      count={sections.notMined.length}
      description={t.account_badges_not_mined_hint()}
    >
      {@render rows(sections.notMined, false)}
    </Panel>
  {/if}

  <Panel title={t.account_badges_owned()} count={owned.length}>
    {#if owned.length === 0}
      <EmptyState text={t.account_badges_empty_owned()} />
    {:else}
      <ul class="grid grid-cols-[repeat(auto-fill,minmax(min(170px,100%),1fr))] gap-2.5">
        {#each owned as badge (badge.key)}
          <li class="flex min-w-0 items-center gap-2.5 rounded-lg border p-2">
            <Thumb
              src={badge.image}
              round={false}
              contain={true}
              size={ThumbSize.MD}
              shape={ThumbShape.SQUARE}
              fallback={IconName.AWARD}
            />
            <span class="truncate text-xs font-semibold" title={badge.title}>{badge.title}</span>
          </li>
        {/each}
      </ul>
    {/if}
  </Panel>
</PageBody>

<CampaignDialog campaign={open} contextAccountId={account.id} onclose={() => (open = null)} />

<Dialog
  open={order !== null}
  title={t.mining_order_title()}
  wide={false}
  closable={!saving}
  closeLabel={t.common_close()}
  onclose={() => (order = null)}
>
  <p class="text-hint text-muted-foreground">{t.mining_order_hint()}</p>
  {#if order}
    <MiningOrderList {order} onchange={(next) => (order = next)} />
  {/if}

  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={saving}
      onclick={() => (order = null)}
    />
    <TextButton
      label={t.common_save()}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={saving}
      onclick={() => order && saveOrder(order)}
    />
  {/snippet}
</Dialog>
