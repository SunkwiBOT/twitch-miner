<script lang="ts">
  import {
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
  import { categoryUrl, periodsText, watchText } from "../../lib/catalogue/campaign.utils";
  import { DROPS_WEB_HOST } from "../../lib/partners.constants";
  import { ProgressState } from "../../lib/catalogue/progress.constants";
  import { gameArt } from "../../lib/catalogue/games.utils";
  import { ResolveKind } from "../ui/external-button.constants";
  import { ensureChannels, knownChannels } from "../../lib/channels/channels.svelte";
  import { minutes } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { strong } from "../../lib/text.utils";
  import { formatDate } from "../../lib/time/time.utils";
  import AccountProgressList from "../catalogue/AccountProgressList.svelte";
  import CampaignActions from "../catalogue/CampaignActions.svelte";
  import { CampaignAction } from "../catalogue/catalogue.constants";
  import Badge from "../ui/Badge.svelte";
  import ChannelLink from "../ui/ChannelLink.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import GameName from "../ui/GameName.svelte";
  import Icon from "../ui/Icon.svelte";
  import RichText from "../ui/RichText.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import BadgeTile from "../ui/BadgeTile.svelte";
  import { ThumbSize } from "../ui/picture.constants";
  import { BadgeTone } from "../../lib/tones.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";

  type BadgeDialogPropsType = {
    badge: GlobalBadgeType | null;
    badges: GlobalBadgeType[];
    contextAccountId: string | null;
    onopencampaign: (campaign: CampaignType) => void;
    onopenbadge: (badge: GlobalBadgeType) => void;
    onclose: () => void;
  };

  let {
    badge,
    badges,
    contextAccountId,
    onopencampaign,
    onopenbadge,
    onclose,
  }: BadgeDialogPropsType = $props();

  const accounts = $derived(app.snapshot?.accounts ?? []);
  const context = $derived(accounts.find((a) => a.id === contextAccountId) ?? null);

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

  let chosen = $state<string[] | null>(null);
  let busy = $state<CampaignAction | null>(null);

  const selected = $derived(chosen ?? (context ? [context.id] : accounts.map((a) => a.id)));
  const progress = $derived(
    badge
      ? accounts
          .filter((a) => selected.includes(a.id))
          .map((account) => badgeProgress(account, badge))
      : [],
  );

  const running = $derived.by((): string | null => {
    const id = requirement?.campaign_id;
    const ends =
      campaign?.ends_at ??
      accounts.flatMap((a) => a.quests).find((quest) => quest.id === id)?.ends_at;

    return id && ends && Date.parse(ends) > ticker.now ? id : null;
  });
  const actionable = $derived(running !== null && accounts.length > 0);
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
    entry.state !== ProgressState.OWNED &&
    !entry.account.owned_badges.some((owned) => badgeKey(owned.set_id, owned.version) === fromKey);

  const minable = $derived(
    progress.some(
      (entry) =>
        entry.minable &&
        (evolution === null || (!lacksFrom(entry) && entry.account.settings.badges.evolutions)),
    ),
  );

  const close = (): void => {
    chosen = null;
    onclose();
  };

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

{#snippet actions()}
  {#if running && accounts.length}
    <CampaignActions
      campaignId={running}
      {selected}
      {minable}
      bind:busy
      onselect={(values) => (chosen = values)}
    />
  {/if}
{/snippet}

<Dialog
  open={badge !== null}
  title={badge?.title ?? ""}
  wide={true}
  closable={busy === null}
  closeLabel={t.common_close()}
  onclose={close}
  footer={actionable ? actions : undefined}
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
              <span class="font-semibold">
                {watchText(minutes(requirement.required_minutes), requirement.repeat)}
              </span>
            {/if}
            {#if requirement.required_subs > 0}
              <span class="text-muted-foreground">
                <RichText
                  text={t.badge_dialog_subs({ count: strong(requirement.required_subs) })}
                />
              </span>
            {/if}
            {#if dates}
              <span class="text-muted-foreground"><RichText text={dates} /></span>
            {/if}
            <span class="text-muted-foreground">
              <RichText
                text={requirement.channel_count > 0
                  ? t.campaign_channels_some({ count: strong(requirement.channel_count) })
                  : categories.length > 1
                    ? t.campaign_channels_categories({ count: strong(categories.length) })
                    : t.campaign_channels_any()}
              />
            </span>
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
              resolve={{
                kind: ResolveKind.BADGE,
                badgeKey: badgeKey(badge.set_id, badge.version_id),
              }}
              label={t.campaign_open_twitch()}
              variant={campaign ? ButtonVariant.OUTLINE : ButtonVariant.PRIMARY}
              size={ButtonSize.SM}
            />
          {/if}
          {#if badge.web_url}
            {@const webUrl = badge.web_url}
            <ExternalButton
              url={webUrl}
              label={t.campaign_view_details()}
              ariaLabel={t.campaign_view_details_web({ domain: DROPS_WEB_HOST })}
              variant={ButtonVariant.OUTLINE}
              size={ButtonSize.SM}
              icon={IconName.GLOBE}
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
              <li>
                <BadgeTile image={possible.image_url} title={possible.title} small={true} />
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
              <BadgeTile
                image={source.image_url}
                title={source.title}
                small={true}
                link={true}
                onclick={() => onopenbadge(known)}
              />
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
      <AccountProgressList entries={progress}>
        {#snippet tags(entry)}
          {#if entry.repeat && entry.state !== ProgressState.OWNED}
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
      </AccountProgressList>
    {/if}
  {/if}
</Dialog>
