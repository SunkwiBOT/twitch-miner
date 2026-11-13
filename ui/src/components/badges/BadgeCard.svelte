<script lang="ts">
  import {
    badgeEnded,
    badgeIsNew,
    badgeMinable,
    badgeNeedsSub,
  } from "../../lib/badges/badges.utils";
  import { BadgeSource } from "../../lib/bindings/BadgeSource";
  import type { GlobalBadgeType } from "../../lib/bindings/GlobalBadgeType";
  import { categoryUrl } from "../../lib/catalogue/campaign.utils";
  import { minutes } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { formatDate } from "../../lib/time/time.utils";
  import Badge from "../ui/Badge.svelte";
  import ExternalLink from "../ui/ExternalLink.svelte";
  import Thumb from "../ui/Thumb.svelte";
  import {
    BadgeTone,
    IconName,
    LinkLook,
    LinkMark,
    LinkSize,
    ThumbShape,
    ThumbSize,
  } from "../ui/ui.constants";

  type BadgeCardPropsType = { badge: GlobalBadgeType; now: number };

  let { badge, now }: BadgeCardPropsType = $props();

  const requirement = $derived(badge.requirement);
  const ended = $derived(badgeEnded(badge, now));
</script>

<article class="flex min-w-0 gap-3 rounded-lg border bg-card p-3">
  <Thumb
    src={badge.image_url}
    round={false}
    contain={true}
    size={ThumbSize.XL}
    shape={ThumbShape.SQUARE}
    fallback={IconName.AWARD}
  />
  <div class="flex min-w-0 flex-1 flex-col gap-1.5">
    <h3 class="truncate text-sm font-semibold" title={badge.title}>{badge.title}</h3>
    <div class="flex flex-wrap gap-1.5">
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
        <Badge
          text={t.reward_sub_only()}
          tone={BadgeTone.WARNING}
          dot={false}
          hint={t.badge_hint_sub_only()}
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
          {#if requirement.required_minutes > 0 && requirement.required_subs === 0}
            <span class="font-semibold"
              >{t.reward_watch({ time: minutes(requirement.required_minutes) })}</span
            >
          {/if}
          {#if requirement.game_name}
            <ExternalLink
              url={categoryUrl(requirement.game_name, undefined)}
              text={requirement.game_name}
              mark={LinkMark.END}
              look={LinkLook.HOVER}
              size={LinkSize.MD}
            />
          {/if}
        </span>
        {#if requirement.source === BadgeSource.DROPS}
          <span class="truncate text-muted-foreground">
            {requirement.channel_count > 0
              ? `${t.campaign_channels_some({ count: requirement.channel_count })} · ${requirement.channels.join(", ")}`
              : t.campaign_channels_any()}
          </span>
        {/if}
        {#if requirement.campaign_name}
          <span class="truncate text-muted-foreground">{requirement.campaign_name}</span>
        {/if}
        {#if requirement.ends_at && !ended}
          <span class="text-muted-foreground"
            >{t.badges_until({ date: formatDate(requirement.ends_at, true) })}</span
          >
        {/if}
      </div>
    {/if}
  </div>
</article>
