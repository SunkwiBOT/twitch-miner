<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import {
    campaignArtUrl,
    campaignEnd,
    campaignHeading,
    campaignSubtitle,
    categoryUrl,
    onDropsPage,
    sitewideQuest,
  } from "../../lib/catalogue/campaign.utils";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { campaignOf, catalogueCampaign } from "../../lib/catalogue/catalogue.svelte";
  import {
    campaignProgress,
    type CampaignProgressType,
  } from "../../lib/catalogue/campaign-progress.utils";
  import { ProgressState } from "../../lib/catalogue/progress.constants";
  import { setAsideLabel } from "../../lib/accounts/accounts.utils";
  import { refreshWhenBack } from "../../lib/accounts/refresh.utils";
  import { campaignUrl } from "../../lib/links/twitch.utils";
  import { boxArt, shortDuration } from "../../lib/format.utils";
  import { BoxArtSize } from "../../lib/format.constants";
  import { ResolveKind } from "../ui/external-button.constants";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import { DROPS_WEB_HOST, TWITCH_CATEGORY_URL } from "../../lib/partners.constants";
  import { setDropIgnored } from "../../lib/state/account-settings.svelte";
  import { app, type SheetType } from "../../lib/state/app.svelte";
  import { run } from "../../lib/state/session";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { CampaignAction, URGENT_MS, URGENT_TONE } from "./catalogue.constants";
  import ActionButton from "../ui/ActionButton.svelte";
  import Badge from "../ui/Badge.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import ChannelLink from "../ui/ChannelLink.svelte";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import GameName from "../ui/GameName.svelte";
  import OverflowNames from "../ui/OverflowNames.svelte";
  import RichText from "../ui/RichText.svelte";
  import { NAMES_SHOWN } from "../ui/name-list.constants";
  import { ThumbSize } from "../ui/picture.constants";
  import { gameArt } from "../../lib/catalogue/games.utils";
  import { ensureChannels, knownChannels } from "../../lib/channels/channels.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import AccountProgressList from "./AccountProgressList.svelte";
  import CampaignActions from "./CampaignActions.svelte";
  import LinkBadge from "./LinkBadge.svelte";
  import RewardTiles from "./RewardTiles.svelte";

  type CampaignDialogPropsType = {
    campaign: CampaignType | null;
    sheet: SheetType | null;
    onclose: () => void;
  };

  let { campaign, sheet, onclose }: CampaignDialogPropsType = $props();

  const accounts = $derived(app.snapshot?.accounts ?? []);
  const context = $derived(accounts.find((a) => a.id === sheet?.accountId) ?? null);

  let chosen = $state<string[] | null>(null);
  let claimed = $state<string[]>([]);
  let claiming = $state<string | null>(null);
  let busy = $state<CampaignAction | null>(null);

  // ! The state belongs to one opening, never to the live campaign each snapshot rebuilds.
  $effect.pre(() => {
    void sheet;
    chosen = null;
    claimed = [];
  });

  const selected = $derived(chosen ?? (context ? [context.id] : accounts.map((a) => a.id)));
  const picked = $derived(accounts.filter((a) => selected.includes(a.id)));
  const linker = $derived(picked.at(0) ?? context);

  // ! One account: the rewards show its own progress; several: the campaign itself, each account's own below.
  const view = $derived.by((): CampaignType | null => {
    if (!campaign) {
      return null;
    }

    const own = picked.length === 1 ? campaignOf(picked[0], campaign.id) : undefined;
    const shown = own ?? catalogueCampaign(campaign.id) ?? campaign;

    return {
      ...shown,
      drops: shown.drops.map((drop) =>
        drop.instance_id && claimed.includes(drop.instance_id) ? { ...drop, claimed: true } : drop,
      ),
    };
  });

  const actionable = $derived(view !== null && accounts.length > 0);

  const progress = $derived(
    campaign ? picked.map((account) => campaignProgress(account, campaign.id, claimed)) : [],
  );

  const ignoredDrops = $derived(picked.flatMap((a) => a.settings.drops.exclude_drop_ids));

  const left = (c: CampaignType): number => new Date(c.ends_at).getTime() - ticker.now;

  const claim = async (entry: CampaignProgressType): Promise<void> => {
    claiming = entry.account.id;

    for (const instanceId of entry.ready) {
      if (
        await run({
          command: CommandName.CLAIM_DROP,
          account_id: entry.account.id,
          drop_instance_id: instanceId,
        })
      ) {
        claimed = [...claimed, instanceId];
      }
    }

    claiming = null;
  };
</script>

{#snippet actions()}
  {#if view && accounts.length}
    <CampaignActions
      campaignId={view.id}
      name={view.name}
      {selected}
      minable={progress.some((entry) => entry.minable)}
      bind:busy
      onselect={(values) => (chosen = values)}
    />
  {/if}
{/snippet}

<Dialog
  open={campaign !== null}
  title={campaign
    ? [campaignHeading(campaign), campaignSubtitle(campaign)].filter(Boolean).join(" · ")
    : ""}
  wide={true}
  closable={busy === null && claiming === null}
  closeOnOutside={true}
  closeLabel={t.common_close()}
  {onclose}
  footer={actionable ? actions : undefined}
>
  {#if view}
    {@const art = boxArt(campaignArtUrl(view), BoxArtSize.CARD)}
    {@const id = view.id}
    <div class="@container">
      <div class="flex items-start gap-4 @max-[26rem]:flex-col @max-[26rem]:items-stretch">
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
                ? t.campaign_time_left({ remainingTime: shortDuration(left(view)) })
                : t.campaign_ended()}
              tone={left(view) <= 0
                ? BadgeTone.MUTED
                : left(view) < URGENT_MS
                  ? URGENT_TONE
                  : BadgeTone.NEUTRAL}
              icon={IconName.CLOCK}
            />
            <Badge
              text={t.catalogue_rewards({ rewardCount: view.drops.length })}
              tone={BadgeTone.NEUTRAL}
              icon={IconName.GIFT}
            />
            <LinkBadge linked={view.linked} needed={view.link_url !== undefined} />
            {#if view.in_inventory}<Badge
                text={t.campaigns_filter_in_progress()}
                tone={BadgeTone.ACCENT}
                hint={t.badge_hint_in_progress()}
              />{/if}
          </div>
          <span class="text-hint text-muted-foreground">
            <RichText
              text={view.allowed_channels.length
                ? t.campaign_channels_some({ channelCount: strong(view.allowed_channels.length) })
                : view.categories.length > 1
                  ? t.campaign_channels_categories({
                      categoryCount: strong(view.categories.length),
                    })
                  : t.campaign_channels_any()}
            />{" · "}<RichText text={campaignEnd(view.ends_at, ticker.now)} />
          </span>
          {#if view.allowed_channels.length}
            <OverflowNames
              items={view.allowed_channels}
              shown={NAMES_SHOWN}
              title={t.list_channels_title()}
              key={(channel) => channel.login}
              words={(channel) => [channel.login, channel.display_name]}
              class="gap-x-4 gap-y-2 text-hint"
              itemClass="min-w-0"
              onshow={(names) => ensureChannels(names.map((channel) => channel.login))}
            >
              {#snippet item(channel)}
                <ChannelLink
                  login={channel.login}
                  name={channel.display_name}
                  avatar={knownChannels[channel.login]?.avatar_url}
                />
              {/snippet}
            </OverflowNames>
          {:else if view.categories.length > 1}
            <OverflowNames
              items={view.categories}
              shown={NAMES_SHOWN}
              title={t.list_categories_title()}
              key={(category) => category.name}
              words={(category) => [category.name]}
              class="gap-x-4 gap-y-2 text-hint"
              itemClass="min-w-0"
            >
              {#snippet item(category)}
                <GameName
                  name={category.name}
                  url={categoryUrl(category.name, category.slug)}
                  art={gameArt(category.name, category.box_art_url)}
                  size={ThumbSize.XS}
                />
              {/snippet}
            </OverflowNames>
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
                resolve={{
                  kind: ResolveKind.CAMPAIGN,
                  accountId: linker?.id ?? "",
                  campaignId: id,
                }}
                label={t.campaign_link_game()}
                variant={ButtonVariant.OUTLINE}
                size={ButtonSize.SM}
                icon={IconName.LINK}
                onfollow={linker ? () => linker && refreshWhenBack(linker.id) : undefined}
              />
            {/if}
            {#if view.web_url}
              {@const webUrl = view.web_url}
              <ExternalButton
                url={webUrl}
                label={t.campaign_view_details()}
                ariaLabel={t.campaign_view_details_web({ domain: DROPS_WEB_HOST })}
                variant={ButtonVariant.OUTLINE}
                size={ButtonSize.SM}
                icon={IconName.GLOBE}
              />
            {/if}
            {#if view.game.slug && !sitewideQuest(view)}
              {@const slug = view.game.slug}
              <ExternalButton
                url={`${TWITCH_CATEGORY_URL}/${slug}`}
                label={t.campaign_view_category()}
                variant={onDropsPage(view) ? ButtonVariant.OUTLINE : ButtonVariant.PRIMARY}
                size={ButtonSize.SM}
                icon={IconName.TV}
              />
            {/if}
          </div>
        </div>
      </div>
    </div>
    <RewardTiles
      campaign={view}
      ignored={ignoredDrops}
      onignore={selected.length
        ? (drop, ignored) => setDropIgnored(selected, drop, ignored)
        : undefined}
    />
    {#if progress.length > 0}
      <AccountProgressList entries={progress}>
        {#snippet tags(entry)}
          {#if entry.linked === false}<LinkBadge linked={false} />{/if}
          {#if entry.state === ProgressState.SET_ASIDE && entry.setAsideUntil}
            <Badge text={setAsideLabel(entry.setAsideUntil, ticker.now)} tone={BadgeTone.WARNING} />
          {/if}
        {/snippet}
        {#snippet action(entry)}
          {#if entry.ready.length > 0 && entry.linked !== false}
            <ActionButton
              icon={IconName.GIFT}
              label={entry.ready.length > 1 ? t.inventory_claim_all() : t.reward_claim()}
              busyLabel={t.campaign_claiming()}
              busy={claiming === entry.account.id}
              disabled={claiming !== null}
              variant={ButtonVariant.PRIMARY}
              size={ButtonSize.SM}
              type={ButtonType.BUTTON}
              onclick={() => claim(entry)}
              class="ml-auto"
            />
          {/if}
        {/snippet}
      </AccountProgressList>
    {/if}
  {/if}
</Dialog>
