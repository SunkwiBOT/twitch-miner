<script lang="ts">
  import {
    badgeEnded,
    badgeIsNew,
    badgeMinable,
    badgeNeedsSub,
    collectionText,
  } from "../../lib/badges/badges.utils";
  import { BadgeSource } from "../../lib/bindings/BadgeSource";
  import type { GlobalBadgeType } from "../../lib/bindings/GlobalBadgeType";
  import { categoryUrl, watchText } from "../../lib/catalogue/campaign.utils";
  import { catalogueCampaign } from "../../lib/catalogue/catalogue.svelte";
  import { gameArt } from "../../lib/catalogue/games.utils";
  import { ensureChannels, knownChannels } from "../../lib/channels/channels.svelte";
  import { minutes } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { strong } from "../../lib/text.utils";
  import { formatDate } from "../../lib/time/time.utils";
  import { openCampaignSheet } from "../../lib/state/sheets";
  import Badge from "../ui/Badge.svelte";
  import SubRequiredBadge from "../catalogue/SubRequiredBadge.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import TextLink from "../ui/TextLink.svelte";
  import ChannelLink from "../ui/ChannelLink.svelte";
  import GameName from "../ui/GameName.svelte";
  import Thumb from "../ui/Thumb.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { CARD_LIFT_CLASSES, RAISED_CLASSES, STRETCHED_BUTTON_CLASSES } from "../ui/ui.constants";
  import { IconName } from "../../lib/icons.constants";
  import { LinkSize } from "../ui/link.constants";
  import { ThumbShape, ThumbSize } from "../ui/picture.constants";

  type BadgeCardPropsType = {
    badge: GlobalBadgeType;
    now: number;
    evolvesFrom: string | null;
    onopen: () => void;
  };

  let { badge, now, evolvesFrom, onopen }: BadgeCardPropsType = $props();

  const requirement = $derived(badge.requirement);
  const linkedCampaign = $derived(
    requirement?.campaign_id ? catalogueCampaign(requirement.campaign_id) : undefined,
  );
  const ended = $derived(badgeEnded(badge, now));

  $effect(() => {
    if (requirement?.channels.length) {
      ensureChannels(requirement.channels);
    }
  });
</script>

<!-- ! The title button covers the whole card; links, hinted badges and the details button sit above it. -->
<div class="group flex min-w-0">
  <article
    class="relative flex min-w-0 flex-1 gap-3 rounded-lg border bg-card p-3 {CARD_LIFT_CLASSES}"
  >
    <Thumb
      src={badge.image_url}
      round={false}
      contain={true}
      size={ThumbSize.XL}
      shape={ThumbShape.SQUARE}
      fallback={IconName.AWARD}
    />
    <div class="flex min-w-0 flex-1 flex-col gap-1.5">
      <h3 class="truncate text-sm font-semibold" title={badge.title}>
        <button
          class={[STRETCHED_BUTTON_CLASSES, "after:rounded-lg"]}
          type="button"
          onclick={onopen}
        >
          {badge.title}
        </button>
      </h3>
      <TextButton
        label={t.campaign_details()}
        icon={IconName.EYE}
        name={t.badge_card_details({ badge: badge.title })}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        class="absolute top-2 right-2 z-10 opacity-0 group-hover:opacity-100 group-has-focus-visible:opacity-100"
        onclick={onopen}
      />
      <div class={[RAISED_CLASSES, "flex flex-wrap gap-1.5 empty:hidden"]}>
        {#if badgeMinable(badge, now)}
          <Badge
            text={t.badges_minable()}
            tone={BadgeTone.SUCCESS}
            dot={false}
            icon={IconName.CHECK}
            hint={t.badge_hint_badge_minable()}
          />
        {/if}
        {#if badgeNeedsSub(badge)}
          <SubRequiredBadge />
        {/if}
        {#if evolvesFrom}
          <Badge
            text={t.badge_evolution({ badge: evolvesFrom })}
            tone={BadgeTone.INFO}
            dot={false}
            hint={t.badge_hint_evolution({ badge: strong(evolvesFrom) })}
          />
        {/if}
        {#if requirement?.ends_at && ended}
          <Badge text={t.campaign_ended()} tone={BadgeTone.MUTED} dot={false} />
        {/if}
        {#if badgeIsNew(badge, now)}
          <Badge text={t.catalogue_new()} tone={BadgeTone.INFO} dot={false} />
        {/if}
      </div>
      {#if badge.description}
        <p class="line-clamp-2 text-xs text-muted-foreground">{badge.description}</p>
      {/if}
      {#if requirement}
        <div class="flex flex-col gap-0.5 text-xs">
          <span class="flex min-w-0 flex-wrap items-center gap-x-1.5">
            {#if requirement.required_minutes > 0}
              <span class="font-semibold">
                {watchText(minutes(requirement.required_minutes), requirement.repeat)}
              </span>
            {/if}
            {#if requirement.game_name}
              {@const game = requirement.categories.find((c) => c.name === requirement.game_name)}
              <span class="relative z-10 inline-flex max-w-full min-w-0">
                <GameName
                  name={requirement.game_name}
                  url={categoryUrl(requirement.game_name, game?.slug)}
                  art={gameArt(requirement.game_name, game?.box_art_url)}
                  size={ThumbSize.XS}
                  inSentence={false}
                />
              </span>
            {/if}
            {#if requirement.categories.length > 1}
              <span class="text-muted-foreground">+{requirement.categories.length - 1}</span>
            {/if}
          </span>
          {#if requirement.source === BadgeSource.DROPS}
            <span class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1 text-muted-foreground">
              {#if requirement.channel_count > 0}
                <span>{t.campaign_channels_some({ channelCount: requirement.channel_count })}</span>
                {#each requirement.channels as login (login)}{@const channel =
                    knownChannels[login]}<span class="relative z-10">
                    <ChannelLink
                      {login}
                      name={channel?.display_name ?? login}
                      avatar={channel?.avatar_url}
                    />
                  </span>{/each}
              {:else if requirement.categories.length > 1}
                {t.campaign_channels_categories({ categoryCount: requirement.categories.length })}
              {:else}
                {t.campaign_channels_any()}
              {/if}
            </span>
          {/if}
          {#if requirement.collection}
            <span class="text-muted-foreground">{collectionText(requirement)}</span>
          {:else if requirement.campaign_name}
            {#if linkedCampaign}
              <span class="relative z-10 min-w-0 truncate text-muted-foreground">
                <TextLink
                  text={requirement.campaign_name}
                  target={{ open: () => openCampaignSheet(linkedCampaign, null) }}
                  size={LinkSize.SM}
                />
              </span>
            {:else}
              <span class="truncate text-muted-foreground">{requirement.campaign_name}</span>
            {/if}
          {/if}
          {#if requirement.ends_at && !ended}
            <span class="text-muted-foreground">
              {t.badges_until({ date: formatDate(requirement.ends_at, true) })}
            </span>
          {/if}
        </div>
      {/if}
    </div>
  </article>
</div>
