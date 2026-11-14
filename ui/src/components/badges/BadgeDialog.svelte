<script lang="ts">
  import {
    BadgeProgressState,
    badgeCampaign,
    badgeEvolution,
    badgeProgress,
    chestBadges,
    type BadgeProgressType,
  } from "../../lib/badges/badge-details.utils";
  import {
    badgeEnded,
    badgeIsNew,
    badgeKey,
    badgeMinable,
    badgeNeedsSub,
    collectionText,
  } from "../../lib/badges/badges.utils";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import type { GlobalBadgeType } from "../../lib/bindings/GlobalBadgeType";
  import {
    categoryUrl,
    periodTotals,
    periodsText,
    watchText,
  } from "../../lib/catalogue/campaign.utils";
  import { gameArt } from "../../lib/catalogue/games.utils";
  import { ensureChannels, knownChannels } from "../../lib/channels/channels.svelte";
  import { minutes } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { strong } from "../../lib/text.utils";
  import { formatDate } from "../../lib/time/time.utils";
  import Avatar from "../ui/Avatar.svelte";
  import Badge from "../ui/Badge.svelte";
  import ChannelLink from "../ui/ChannelLink.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import GameName from "../ui/GameName.svelte";
  import Icon from "../ui/Icon.svelte";
  import ProgressLine from "../ui/ProgressLine.svelte";
  import RichText from "../ui/RichText.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import Thumb from "../ui/Thumb.svelte";
  import { AvatarSize, ThumbShape, ThumbSize } from "../ui/picture.constants";
  import { BadgeTone } from "../../lib/tones.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";

  type BadgeDialogPropsType = {
    badge: GlobalBadgeType | null;
    badges: GlobalBadgeType[];
    onopencampaign: (campaign: CampaignType) => void;
    onopenbadge: (badge: GlobalBadgeType) => void;
    onclose: () => void;
  };

  let { badge, badges, onopencampaign, onopenbadge, onclose }: BadgeDialogPropsType = $props();

  const accounts = $derived(app.snapshot?.accounts ?? []);
  const requirement = $derived(badge?.requirement);
  const campaign = $derived(badge ? badgeCampaign(badge, accounts) : null);
  const chest = $derived(chestBadges(campaign));
  const evolution = $derived(badge ? badgeEvolution(badge, badges, accounts) : null);
  const fromKey = $derived(
    evolution?.from ? badgeKey(evolution.from.set_id, evolution.from.version) : null,
  );
  const from = $derived(
    badges.find((other) => badgeKey(other.set_id, other.version_id) === fromKey),
  );
  const progress = $derived(badge ? accounts.map((account) => badgeProgress(account, badge)) : []);
  const categories = $derived(
    requirement?.categories.length
      ? requirement.categories
      : requirement?.game_name
        ? [{ id: requirement.game_id ?? "", name: requirement.game_name }]
        : [],
  );

  $effect(() => {
    if (requirement?.channels.length) {
      ensureChannels(requirement.channels);
    }
  });

  const dates = $derived.by((): string => {
    const starts = requirement?.starts_at;
    const ends = requirement?.ends_at;

    if (starts && ends) {
      return t.badge_dialog_dates({
        start: strong(formatDate(starts, true)),
        end: strong(formatDate(ends, true)),
      });
    }

    if (ends) {
      return t.badges_until({ date: strong(formatDate(ends, true)) });
    }

    return starts ? t.badge_dialog_from({ date: strong(formatDate(starts, true)) }) : "";
  });

  const lacksFrom = (entry: BadgeProgressType): boolean =>
    fromKey !== null &&
    entry.state !== BadgeProgressState.OWNED &&
    !entry.account.owned_badges.some((owned) => badgeKey(owned.set_id, owned.version) === fromKey);

  let failedImage = $state<string | null>(null);
</script>

{#snippet section(title: string, count: number | null)}
  <h3 class="flex items-center gap-2 text-xs font-semibold text-muted-foreground">
    {title}{#if count !== null}<Badge
        text={String(count)}
        tone={BadgeTone.NEUTRAL}
        dot={false}
      />{/if}
  </h3>
{/snippet}

{#snippet state(entry: BadgeProgressType)}
  {#if entry.state === BadgeProgressState.OWNED}
    <Badge
      text={t.reward_state_owned()}
      tone={BadgeTone.SUCCESS}
      dot={false}
      icon={IconName.CHECK}
    />
  {:else if entry.state === BadgeProgressState.CLAIMABLE}
    <Badge
      text={t.reward_state_ready()}
      tone={BadgeTone.PRIMARY}
      dot={false}
      icon={IconName.GIFT}
      hint={t.badge_hint_ready()}
    />
  {:else if entry.state === BadgeProgressState.SUB_ONLY}
    <Badge
      text={t.reward_sub_only()}
      tone={BadgeTone.WARNING}
      dot={false}
      hint={t.badge_hint_sub_only()}
    />
  {:else if entry.state === BadgeProgressState.IN_PROGRESS}
    <Badge
      text={t.campaigns_filter_in_progress()}
      tone={BadgeTone.ACCENT}
      dot={false}
      hint={t.badge_hint_in_progress()}
    />
  {:else if entry.state === BadgeProgressState.NOT_STARTED}
    <Badge text={t.badge_progress_not_started()} tone={BadgeTone.MUTED} dot={false} />
  {:else}
    <Badge text={t.badge_progress_not_offered()} tone={BadgeTone.MUTED} dot={false} />
  {/if}
  {#if entry.repeat && entry.state !== BadgeProgressState.OWNED}
    <Badge
      text={periodsText(entry.done, entry.repeat)}
      tone={BadgeTone.NEUTRAL}
      dot={false}
      icon={IconName.CALENDAR}
    />
  {/if}
  {#if evolution?.from && lacksFrom(entry)}
    <Badge
      text={t.badge_evolution_locked({ badge: evolution.from.title })}
      tone={BadgeTone.WARNING}
      dot={false}
      hint={t.badge_hint_evolution_locked({ badge: evolution.from.title })}
    />
  {/if}
{/snippet}

<Dialog
  open={badge !== null}
  title={badge?.title ?? ""}
  wide={true}
  closable={true}
  closeLabel={t.common_close()}
  {onclose}
>
  {#if badge}
    <div class="flex items-start gap-4">
      <span
        class="grid size-24 shrink-0 place-items-center rounded-lg border bg-muted/40 text-muted-foreground"
      >
        {#if badge.image_url && failedImage !== badge.image_url}<img
            class="size-18 object-contain"
            src={badge.image_url}
            alt=""
            onerror={() => (failedImage = badge?.image_url ?? null)}
          />{:else}<Icon name={IconName.AWARD} size={40} />{/if}
      </span>
      <div class="flex min-w-0 flex-1 flex-col gap-2.5">
        <div class="flex flex-wrap gap-1.5 empty:hidden">
          {#if badgeMinable(badge, ticker.now)}
            <Badge
              text={t.badges_minable()}
              tone={BadgeTone.SUCCESS}
              dot={false}
              icon={IconName.CHECK}
              hint={t.badge_hint_badge_minable()}
            />
          {/if}
          {#if badgeNeedsSub(badge)}
            <Badge
              text={t.reward_sub_only()}
              tone={BadgeTone.WARNING}
              dot={false}
              hint={t.badge_hint_sub_only()}
            />
          {/if}
          {#if requirement?.ends_at && badgeEnded(badge, ticker.now)}
            <Badge text={t.campaign_ended()} tone={BadgeTone.MUTED} dot={false} />
          {/if}
          {#if badgeIsNew(badge, ticker.now)}
            <Badge text={t.catalogue_new()} tone={BadgeTone.INFO} dot={false} />
          {/if}
        </div>
        {#if badge.description}
          <p class="text-hint text-muted-foreground">{badge.description}</p>
        {/if}
        {#if requirement}
          <div class="flex flex-col gap-1 text-hint">
            {#if requirement.required_minutes > 0}
              <span class="font-semibold"
                >{watchText(minutes(requirement.required_minutes), requirement.repeat)}</span
              >
            {/if}
            {#if requirement.required_subs > 0}
              <span class="text-muted-foreground"
                ><RichText
                  text={t.badge_dialog_subs({ count: strong(requirement.required_subs) })}
                /></span
              >
            {/if}
            {#if dates}
              <span class="text-muted-foreground"><RichText text={dates} /></span>
            {/if}
            <span class="text-muted-foreground"
              ><RichText
                text={requirement.channel_count > 0
                  ? t.campaign_channels_some({ count: strong(requirement.channel_count) })
                  : categories.length > 1
                    ? t.campaign_channels_categories({ count: strong(categories.length) })
                    : t.campaign_channels_any()}
              /></span
            >
          </div>
        {/if}
        <div class="flex flex-wrap gap-2 empty:hidden">
          {#if campaign}
            {@const linked = campaign}
            <TextButton
              label={t.badge_dialog_open_campaign()}
              icon={IconName.GIFT}
              variant={ButtonVariant.PRIMARY}
              size={ButtonSize.SM}
              type={ButtonType.BUTTON}
              disabled={false}
              onclick={() => onopencampaign(linked)}
            />
          {/if}
          {#if badge.click_url}
            <ExternalButton
              url={badge.click_url}
              label={t.campaign_open_twitch()}
              variant={campaign ? ButtonVariant.OUTLINE : ButtonVariant.PRIMARY}
              size={ButtonSize.SM}
            />
          {/if}
        </div>
      </div>
    </div>

    {#if categories.length > 0}
      <section class="flex flex-col gap-2">
        {@render section(t.badge_dialog_categories(), categories.length)}
        <ul class="flex flex-wrap gap-x-4 gap-y-2 text-hint">
          {#each categories as category (category.name)}
            <li class="min-w-0">
              <GameName
                name={category.name}
                url={categoryUrl(category.name, category.slug)}
                art={gameArt(category.name, category.box_art_url)}
                size={ThumbSize.XS}
                inSentence={true}
              />
            </li>
          {/each}
        </ul>
      </section>
    {/if}

    {#if requirement && requirement.channels.length > 0}
      <section class="flex flex-col gap-2">
        {@render section(t.badge_dialog_channels(), requirement.channel_count)}
        <ul class="flex flex-wrap gap-x-4 gap-y-2 text-hint">
          {#each requirement.channels as login (login)}
            {@const channel = knownChannels[login]}
            <li class="min-w-0">
              <ChannelLink
                {login}
                name={channel?.display_name ?? login}
                avatar={channel?.avatar_url}
              />
            </li>
          {/each}
        </ul>
      </section>
    {/if}

    {#if requirement?.collection || chest.length > 0}
      <section class="flex flex-col gap-2">
        {@render section(t.badge_dialog_collection(), null)}
        {#if requirement?.collection}
          <p class="text-hint text-muted-foreground">{collectionText(requirement)}</p>
        {/if}
        {#if chest.length > 0}
          <ul class="grid grid-cols-[repeat(auto-fill,minmax(min(170px,100%),1fr))] gap-2">
            {#each chest as possible (badgeKey(possible.set_id, possible.version))}
              <li class="flex min-w-0 items-center gap-2.5 rounded-lg border p-2">
                <Thumb
                  src={possible.image_url}
                  round={false}
                  contain={true}
                  size={ThumbSize.MD}
                  shape={ThumbShape.SQUARE}
                  fallback={IconName.AWARD}
                />
                <span class="truncate text-xs font-semibold">{possible.title}</span>
              </li>
            {/each}
          </ul>
        {/if}
      </section>
    {/if}

    {#if evolution}
      <section class="flex flex-col gap-2">
        {@render section(t.badge_dialog_evolution(), null)}
        {#if evolution.from}
          {@const source = evolution.from}
          <div class="flex items-center gap-3">
            {#if from}
              {@const known = from}
              <button
                class="flex min-w-0 items-center gap-2.5 rounded-lg border p-2 text-left transition-colors hover:bg-accent"
                type="button"
                onclick={() => onopenbadge(known)}
              >
                <Thumb
                  src={source.image_url}
                  round={false}
                  contain={true}
                  size={ThumbSize.MD}
                  shape={ThumbShape.SQUARE}
                  fallback={IconName.AWARD}
                />
                <span class="truncate text-xs font-semibold text-link">{source.title}</span>
              </button>
            {/if}
            <p class="min-w-0 flex-1 text-hint text-muted-foreground">
              <RichText text={t.badge_dialog_evolves_from({ badge: strong(source.title) })} />
            </p>
          </div>
        {:else}
          <p class="text-hint text-muted-foreground">{t.badge_hint_evolution_unknown()}</p>
        {/if}
      </section>
    {/if}

    {#if progress.length > 0}
      <section class="flex flex-col gap-2">
        {@render section(t.badge_dialog_accounts(), null)}
        <ul class="flex flex-col gap-2">
          {#each progress as entry (entry.account.id)}
            {@const totals = periodTotals(entry)}
            <li class="flex items-center gap-3 rounded-lg border p-2.5">
              <Avatar
                url={entry.account.avatar_url}
                name={entry.account.display_name}
                size={AvatarSize.SM}
              />
              <span class="flex min-w-0 flex-1 flex-col gap-1.5">
                <span class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1">
                  <span class="truncate text-sm font-semibold">{entry.account.display_name}</span>
                  {@render state(entry)}
                </span>
                {#if totals.required > 0 && entry.state !== BadgeProgressState.OWNED && entry.state !== BadgeProgressState.SUB_ONLY}
                  <ProgressLine
                    current={totals.current}
                    required={totals.required}
                    stacked={false}
                  />
                {/if}
              </span>
            </li>
          {/each}
        </ul>
      </section>
    {/if}
  {/if}
</Dialog>
