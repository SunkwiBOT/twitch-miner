<script lang="ts">
  import PanelColumns from "../ui/PanelColumns.svelte";
  import { CommandName } from "../../lib/bindings/CommandName";
  import {
    campaignArtUrl,
    campaignEnd,
    campaignHeading,
    campaignSubtitle,
  } from "../../lib/catalogue/campaign.utils";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { campaignOf } from "../../lib/catalogue/catalogue.svelte";
  import { QUEUE_PREVIEW, RECENT_MAX } from "./account.constants";
  import { ErrorApiMessage } from "../../lib/bindings/ErrorApiMessage";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { describeCode } from "../../lib/errors/errors.utils";
  import { recentEvents } from "../../lib/events/events.utils";
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { escapeEmphasis, strong } from "../../lib/text.utils";
  import { PAGE_ICONS, Page } from "../../lib/state/state.constants";
  import { setDropIgnored } from "../../lib/state/account-settings.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { navigate } from "../../lib/state/router";
  import { openCampaignSheet } from "../../lib/state/sheets";
  import { run } from "../../lib/state/session";
  import { ticker } from "../../lib/state/ticker.svelte";
  import AccountActivity from "../AccountActivity.svelte";
  import ActivityFeed from "../ActivityFeed.svelte";
  import LinkBadge from "../catalogue/LinkBadge.svelte";
  import { linkFacet } from "../../lib/accounts/campaign-groups.utils";
  import RewardTiles from "../catalogue/RewardTiles.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { Card } from "$lib/components/ui/card";
  import EmptyState from "../ui/EmptyState.svelte";
  import List from "../ui/List.svelte";
  import ListRow from "../ui/ListRow.svelte";
  import Notice from "../ui/Notice.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import Panel from "../ui/Panel.svelte";
  import Thumb from "../ui/Thumb.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { NoticeTone, PanelWidth } from "../ui/ui.constants";
  import { ThumbShape, ThumbSize } from "../ui/picture.constants";

  type AccountOverviewPropsType = { account: AccountSnapshotType; snapshot: SnapshotType };

  let { account, snapshot }: AccountOverviewPropsType = $props();

  const current = $derived.by<CampaignType | null>(() => {
    const id = account.slots.find((slot) => slot.drop)?.drop?.campaign_id;

    return id ? (campaignOf(account, id) ?? null) : null;
  });

  const events = $derived(
    recentEvents(
      app.liveEvents,
      snapshot.recent_events,
      (e) => e.account_id === account.id,
      RECENT_MAX,
    ),
  );

  const claim = (instanceId: string): Promise<boolean> =>
    run({ command: CommandName.CLAIM_DROP, account_id: account.id, drop_instance_id: instanceId });
</script>

<PageBody>
  {#if account.problem === ErrorApiMessage.BEACON_BLOCKED}
    <Notice tone={NoticeTone.WARNING} text={describeCode(ErrorApiMessage.BEACON_BLOCKED)} />
  {:else if account.problem && account.status !== AccountStatus.LOGIN_REQUIRED}
    <Notice tone={NoticeTone.DANGER} text={describeCode(account.problem)} />
  {/if}

  <Card class="gap-0 py-0"><AccountActivity {account} /></Card>

  <!-- ! Wide enough for a queue row's details on one line before the panels sit side by side. -->
  <PanelColumns width={PanelWidth.XL}>
    <Panel title={t.overview_queue()}>
      {#snippet actions()}
        <TextButton
          label={t.overview_manage_queue()}
          icon={PAGE_ICONS[Page.ACCOUNT_DROPS]}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.SM}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => navigate(Page.ACCOUNT_DROPS)}
        />
      {/snippet}
      {#if account.queue.length === 0}
        <EmptyState text={t.queue_empty()} />
      {:else}
        <List>
          {#each account.queue.slice(0, QUEUE_PREVIEW) as q, i (q.campaign_id)}
            {@const c = campaignOf(account, q.campaign_id)}
            <ListRow
              title={c ? campaignHeading(c) : q.campaign_id}
              details={[
                escapeEmphasis(c ? campaignSubtitle(c) : ""),
                t.dashboard_remaining({ minutes: strong(q.remaining_minutes) }),
                campaignEnd(q.ends_at, ticker.now),
              ]}
              onopen={c ? () => openCampaignSheet(c, account.id) : undefined}
              muted={false}
            >
              {#snippet lead()}
                <span class="w-4 shrink-0 text-right text-xs text-muted-foreground tabular-nums">
                  {i + 1}
                </span>
                <Thumb
                  src={boxArt(c && campaignArtUrl(c), BoxArtSize.ROW)}
                  round={false}
                  contain={false}
                  size={ThumbSize.LG}
                  shape={ThumbShape.BOX_ART}
                  fallback={IconName.GIFT}
                />
              {/snippet}
              {#snippet trailing()}{#if c}<LinkBadge link={linkFacet(c, q.linked)} />{/if}{/snippet}
            </ListRow>
          {/each}
        </List>
      {/if}
    </Panel>
    <Panel title={t.overview_rewards()}>
      {#snippet actions()}
        {#if current}
          <TextButton
            label={t.campaign_details()}
            icon={IconName.EYE}
            variant={ButtonVariant.OUTLINE}
            size={ButtonSize.SM}
            type={ButtonType.BUTTON}
            disabled={false}
            onclick={() => openCampaignSheet(current, account.id)}
          />
        {/if}
      {/snippet}
      {#if current}
        <RewardTiles
          campaign={current}
          ignored={account.settings.drops.exclude_drop_ids}
          onclaim={claim}
          onignore={(drop, ignored) => setDropIgnored([account.id], drop, ignored)}
        />
      {:else}
        <EmptyState text={t.overview_no_campaign()} />
      {/if}
    </Panel>
  </PanelColumns>

  <Panel title={t.overview_recent()}>
    {#snippet actions()}
      <TextButton
        label={t.nav_history()}
        icon={PAGE_ICONS[Page.ACCOUNT_HISTORY]}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => navigate(Page.ACCOUNT_HISTORY)}
      />
    {/snippet}
    <ActivityFeed {events} showAccount={false} wide={false} empty={t.history_empty()} />
  </Panel>
</PageBody>
