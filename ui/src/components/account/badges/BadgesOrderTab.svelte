<script lang="ts">
  import { QUEUE_BLOCK_HINTS, QUEUE_BLOCK_LABELS } from "../../../lib/accounts/accounts.utils";
  import { miningOrder } from "../../../lib/accounts/changes.utils";
  import { reorderWithin, setBadgesIgnored, setSelected } from "../../../lib/accounts/drops.utils";
  import {
    badgeRows,
    type AccountBadgeType,
    type AccountBadgesType,
    type BadgeRowType,
  } from "../../../lib/badges/badges.utils";
  import type { AccountSnapshotType } from "../../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import type { QueueItemType } from "../../../lib/bindings/QueueItemType";
  import type { SnapshotType } from "../../../lib/bindings/SnapshotType";
  import { campaignEnd } from "../../../lib/catalogue/campaign.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { ticker } from "../../../lib/state/ticker.svelte";
  import { escapeEmphasis } from "../../../lib/text.utils";
  import Badge from "../../ui/Badge.svelte";
  import DetailLine from "../../ui/DetailLine.svelte";
  import IconButton from "../../ui/IconButton.svelte";
  import List from "../../ui/List.svelte";
  import ListRow from "../../ui/ListRow.svelte";
  import Panel from "../../ui/Panel.svelte";
  import ProgressLine from "../../ui/ProgressLine.svelte";
  import SortableList from "../../ui/SortableList.svelte";
  import Thumb from "../../ui/Thumb.svelte";
  import {
    BadgeTone,
    ButtonSize,
    ButtonVariant,
    IconName,
    ThumbShape,
    ThumbSize,
  } from "../../ui/ui.constants";
  import BadgeStateBadges from "./BadgeStateBadges.svelte";

  type BadgesOrderTabPropsType = {
    account: AccountSnapshotType;
    snapshot: SnapshotType;
    sections: AccountBadgesType;
    onopen: (campaign: CampaignType) => void;
  };

  let { account, snapshot, sections, onopen }: BadgesOrderTabPropsType = $props();

  const rows = $derived(badgeRows(sections.minable, miningOrder(account.settings, account)));

  const mining = (id: string): boolean =>
    account.slots.some((slot) => slot.drop?.campaign_id === id);

  const queued = (id: string): QueueItemType | undefined =>
    account.queue.find((item) => item.campaign_id === id);

  const chosen = (id: string): boolean => account.settings.drops.selected_campaign_ids.includes(id);

  const titleOf = (row: BadgeRowType): string => row.items.map((item) => item.title).join(" · ");
</script>

{#snippet thumb(item: AccountBadgeType)}
  <Thumb
    src={item.image}
    round={false}
    contain={true}
    size={ThumbSize.MD}
    shape={ThumbShape.SQUARE}
    fallback={IconName.AWARD}
  />
{/snippet}

<Panel
  title={t.account_badges_order_title()}
  count={rows.length}
  description={t.account_badges_order_hint()}
>
  <SortableList
    items={rows}
    key={(row) => row.campaignId}
    empty={t.account_badges_order_empty()}
    upLabel={t.common_up()}
    downLabel={t.common_down()}
    onreorder={(list, moved) =>
      reorderWithin(
        account,
        snapshot,
        list.map((row) => row.campaignId),
        moved.campaignId,
      )}
  >
    {#snippet row(row)}
      {@const first = row.items[0]}
      {@const item = queued(row.campaignId)}
      {@render thumb(first)}
      <button
        class="flex min-w-0 flex-1 flex-col gap-0.5 text-left"
        type="button"
        onclick={() => row.campaign && onopen(row.campaign)}
      >
        <span class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1">
          <span class="truncate font-semibold text-link">{titleOf(row)}</span>
          <span class="inline-flex max-w-full flex-wrap items-center gap-1.5">
            {#if mining(row.campaignId)}<Badge
                text={t.queue_mining()}
                tone={BadgeTone.SUCCESS}
                dot={true}
                hint={t.badge_hint_queue_mining()}
              />{:else if item?.blocked}<Badge
                text={QUEUE_BLOCK_LABELS[item.blocked]()}
                tone={BadgeTone.WARNING}
                dot={false}
                hint={QUEUE_BLOCK_HINTS[item.blocked]()}
              />{:else if account.settings.badges.selection_only && !chosen(row.campaignId)}<Badge
                text={t.selection_not_selected()}
                tone={BadgeTone.MUTED}
                dot={false}
                hint={t.badges_not_selected_hint()}
              />{/if}
            <BadgeStateBadges item={first} watch={row.current === 0} />
          </span>
        </span>
        <DetailLine parts={[escapeEmphasis(first.source), campaignEnd(first.endsAt, ticker.now)]} />
        {#if row.current > 0}
          <span class="mt-1 block">
            <ProgressLine current={row.current} required={row.required} stacked={false} />
          </span>
        {/if}
      </button>
    {/snippet}
    {#snippet actions(row)}
      {@const on = chosen(row.campaignId)}
      <IconButton
        icon={on ? IconName.STAR_FILLED : IconName.STAR}
        label={on ? t.campaign_unselect() : t.campaign_select()}
        busy={false}
        disabled={false}
        variant={ButtonVariant.GHOST}
        size={ButtonSize.SM}
        onclick={() => setSelected(account, row.campaignId, titleOf(row), !on)}
      />
      <IconButton
        icon={IconName.EYE_OFF}
        label={t.account_badges_ignore()}
        busy={false}
        disabled={false}
        variant={ButtonVariant.GHOST}
        size={ButtonSize.SM}
        onclick={() =>
          setBadgesIgnored(
            account,
            row.items.map((item) => item.dropId),
            titleOf(row),
          )}
      />
    {/snippet}
  </SortableList>
</Panel>

{#if sections.notMined.length > 0}
  <Panel
    title={t.account_badges_not_mined()}
    count={sections.notMined.length}
    description={t.account_badges_not_mined_hint()}
  >
    <List>
      {#each sections.notMined as item (item.key)}
        <ListRow
          title={item.title}
          details={[escapeEmphasis(item.source), campaignEnd(item.endsAt, ticker.now)]}
          onopen={item.campaign ? () => item.campaign && onopen(item.campaign) : undefined}
          muted={false}
        >
          {#snippet lead()}{@render thumb(item)}{/snippet}
          {#snippet badges()}<BadgeStateBadges {item} watch={false} />{/snippet}
        </ListRow>
      {/each}
    </List>
  </Panel>
{/if}
