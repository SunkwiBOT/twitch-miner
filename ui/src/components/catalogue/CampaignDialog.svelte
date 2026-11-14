<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import {
    campaignArtUrl,
    campaignEnd,
    campaignHeading,
    campaignSubtitle,
    categoryNames,
    dropDone,
    onDropsPage,
    sitewideQuest,
    subsMissing,
  } from "../../lib/catalogue/campaign.utils";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { campaignOf } from "../../lib/catalogue/catalogue.svelte";
  import { refreshWhenBack } from "../../lib/accounts/refresh.utils";
  import { campaignUrl } from "../../lib/links/twitch.utils";
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt, shortDuration } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import { TWITCH_CATEGORY_URL } from "../../lib/partners.constants";
  import {
    prioritizeCampaign,
    setCampaignIgnored,
    setCampaignSelected,
    setDropIgnored,
  } from "../../lib/state/account-settings.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { run } from "../../lib/state/session";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { CampaignAction, URGENT_MS, URGENT_TONE } from "./catalogue.constants";
  import ActionButton from "../ui/ActionButton.svelte";
  import Badge from "../ui/Badge.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import MultiSelect from "../ui/MultiSelect.svelte";
  import RichText from "../ui/RichText.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MenuAlign } from "../ui/menu.constants";
  import LinkBadge from "./LinkBadge.svelte";
  import RewardTiles from "./RewardTiles.svelte";

  type CampaignDialogPropsType = {
    campaign: CampaignType | null;
    contextAccountId: string | null;
    onclose: () => void;
  };

  let { campaign, contextAccountId, onclose }: CampaignDialogPropsType = $props();

  const accounts = $derived(app.snapshot?.accounts ?? []);
  const context = $derived(accounts.find((a) => a.id === contextAccountId) ?? null);

  let chosen = $state<string[] | null>(null);

  const selected = $derived(chosen ?? (context ? [context.id] : accounts.map((a) => a.id)));
  const viewer = $derived(accounts.find((a) => selected.includes(a.id)) ?? context);

  const allIgnore = $derived(
    campaign !== null &&
      selected.length > 0 &&
      accounts
        .filter((a) => selected.includes(a.id))
        .every((a) => a.settings.drops.exclude_campaign_ids.includes(campaign.id)),
  );

  const allSelected = $derived(
    campaign !== null &&
      selected.length > 0 &&
      accounts
        .filter((a) => selected.includes(a.id))
        .every((a) => a.settings.drops.selected_campaign_ids.includes(campaign.id)),
  );

  const view = $derived(
    campaign ? ((viewer && campaignOf(viewer, campaign.id)) ?? campaign) : null,
  );

  const ignoredDrops = $derived(
    accounts
      .filter((a) => selected.includes(a.id))
      .flatMap((a) => a.settings.drops.exclude_drop_ids),
  );

  // ! Pinning a campaign no selected account can mine (subscription only, done, owned) would hold a place in the order that is never mined.
  const minable = $derived(
    campaign !== null &&
      accounts
        .filter((a) => selected.includes(a.id))
        .some((a) =>
          (campaignOf(a, campaign.id) ?? campaign).drops.some(
            (drop) =>
              !subsMissing(drop) &&
              !dropDone(drop) &&
              !drop.badge_owned &&
              !(drop.instance_id && !drop.claimed),
          ),
        ),
  );

  let busy = $state<CampaignAction | null>(null);

  const act = async (key: CampaignAction, action: () => Promise<unknown>): Promise<void> => {
    busy = key;
    await action();
    busy = null;
  };

  const left = (c: CampaignType): number => new Date(c.ends_at).getTime() - ticker.now;

  const claim = (instanceId: string): Promise<boolean> =>
    viewer
      ? run({
          command: CommandName.CLAIM_DROP,
          account_id: viewer.id,
          drop_instance_id: instanceId,
        })
      : Promise.resolve(false);

  const close = (): void => {
    chosen = null;
    onclose();
  };
</script>

<Dialog
  open={campaign !== null}
  title={campaign
    ? [campaignHeading(campaign), campaignSubtitle(campaign)].filter(Boolean).join(" · ")
    : ""}
  wide={true}
  closable={busy === null}
  closeLabel={t.common_close()}
  onclose={close}
>
  {#if view}
    {@const art = boxArt(campaignArtUrl(view), BoxArtSize.CARD)}
    {@const id = view.id}
    <div class="flex items-start gap-4">
      {#if art}<img
          class="block w-24 shrink-0 rounded-lg object-cover"
          style:aspect-ratio="3 / 4"
          src={art}
          alt=""
        />{/if}
      <div class="flex min-w-0 flex-1 flex-col gap-2.5">
        <div class="flex flex-wrap gap-1.5">
          <Badge
            text={left(view) > 0
              ? t.campaign_time_left({ time: shortDuration(left(view)) })
              : t.campaign_ended()}
            tone={left(view) < URGENT_MS ? URGENT_TONE : BadgeTone.NEUTRAL}
            dot={false}
            icon={IconName.CLOCK}
          />
          <Badge
            text={t.catalogue_rewards({ count: view.drops.length })}
            tone={BadgeTone.NEUTRAL}
            dot={false}
            icon={IconName.GIFT}
          />
          {#if view.linked === undefined && view.link_url}
            <Badge
              text={t.catalogue_link_required()}
              tone={BadgeTone.MUTED}
              dot={false}
              hint={t.badge_hint_link_required()}
              icon={IconName.LINK}
            />
          {:else}
            <LinkBadge linked={view.linked} />
          {/if}
          {#if view.in_inventory}<Badge
              text={t.campaigns_filter_in_progress()}
              tone={BadgeTone.ACCENT}
              dot={false}
              hint={t.badge_hint_in_progress()}
            />{/if}
        </div>
        <span class="text-hint text-muted-foreground">
          <RichText
            text={view.allowed_channels.length
              ? t.campaign_channels_some({ count: strong(view.allowed_channels.length) })
              : view.categories.length > 1
                ? t.campaign_channels_categories({ count: strong(view.categories.length) })
                : t.campaign_channels_any()}
          />{" · "}<RichText text={campaignEnd(view.ends_at, ticker.now)} />
        </span>
        {#if !view.allowed_channels.length && view.categories.length > 1}
          <span class="line-clamp-2 text-hint text-muted-foreground"
            >{categoryNames(view.categories)}</span
          >
        {/if}
        <div class="flex flex-wrap gap-2">
          {#if onDropsPage(view)}
            <ExternalButton
              url={campaignUrl(id)}
              label={t.campaign_open_twitch()}
              variant={ButtonVariant.PRIMARY}
              size={ButtonSize.SM}
            />
          {/if}
          {#if view.link_url && view.linked !== true}
            {@const link = view.link_url}
            <ExternalButton
              url={link}
              label={t.campaign_link_game()}
              variant={ButtonVariant.OUTLINE}
              size={ButtonSize.SM}
              icon={IconName.LINK}
              onfollow={viewer ? () => viewer && refreshWhenBack(viewer.id) : undefined}
            />
          {/if}
          {#if view.game.slug && !sitewideQuest(view)}
            {@const slug = view.game.slug}
            <ExternalButton
              url={`${TWITCH_CATEGORY_URL}/${slug}`}
              label={view.game.name}
              variant={onDropsPage(view) ? ButtonVariant.OUTLINE : ButtonVariant.PRIMARY}
              size={ButtonSize.SM}
              icon={IconName.TV}
            />
          {/if}
        </div>
      </div>
    </div>
    {#if viewer && selected.length > 1}
      <span class="text-hint text-muted-foreground"
        ><RichText text={t.campaign_progress_of({ account: strong(viewer.display_name) })} /></span
      >
    {/if}
    <RewardTiles
      campaign={view}
      ignored={ignoredDrops}
      onclaim={viewer ? claim : undefined}
      onignore={selected.length
        ? (drop, ignored) => setDropIgnored(selected, drop, ignored)
        : undefined}
    />
  {/if}

  {#snippet footer()}
    {#if view && accounts.length}
      {@const id = view.id}
      <div class="w-56 sm:mr-auto">
        <MultiSelect
          values={selected}
          options={accounts.map((a) => ({
            value: a.id,
            label: a.display_name,
            image: a.avatar_url,
          }))}
          label={t.common_account()}
          allLabel={t.accounts_all()}
          noneLabel={t.accounts_none()}
          countLabel={(count) => t.accounts_count({ count })}
          align={MenuAlign.LEFT}
          onchange={(values) => (chosen = values)}
        />
      </div>
      <ActionButton
        icon={allIgnore ? IconName.EYE : IconName.EYE_OFF}
        label={allIgnore ? t.campaign_unignore() : t.campaign_ignore()}
        busyLabel={t.common_saving()}
        busy={busy === CampaignAction.IGNORE}
        disabled={busy !== null || selected.length === 0}
        variant={ButtonVariant.DEFAULT}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        onclick={() =>
          act(CampaignAction.IGNORE, () => setCampaignIgnored(selected, id, !allIgnore))}
      />
      {#if minable}
        <ActionButton
          icon={allSelected ? IconName.STAR_FILLED : IconName.STAR}
          label={allSelected ? t.campaign_unselect() : t.campaign_select()}
          busyLabel={t.common_saving()}
          busy={busy === CampaignAction.SELECT}
          disabled={busy !== null || selected.length === 0}
          variant={ButtonVariant.DEFAULT}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          onclick={() =>
            act(CampaignAction.SELECT, () => setCampaignSelected(selected, id, !allSelected))}
        />
        <ActionButton
          icon={IconName.ARROW_UP}
          label={t.campaign_prioritize()}
          busyLabel={t.common_saving()}
          busy={busy === CampaignAction.FIRST}
          disabled={busy !== null || selected.length === 0}
          variant={ButtonVariant.PRIMARY}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          onclick={() => act(CampaignAction.FIRST, () => prioritizeCampaign(selected, id))}
        />
      {/if}
    {/if}
  {/snippet}
</Dialog>
