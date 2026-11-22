<script lang="ts">
  import { AccountBadgeState, type AccountBadgeType } from "../../../lib/badges/badges.utils";
  import { periodsText, watchText } from "../../../lib/catalogue/campaign.utils";
  import { minutes } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { strong } from "../../../lib/text.utils";
  import { ticker } from "../../../lib/state/ticker.svelte";
  import { relativeTime } from "../../../lib/time/time.utils";
  import Badge from "../../ui/Badge.svelte";
  import { BadgeTone } from "../../../lib/tones.constants";
  import { IconName } from "../../../lib/icons.constants";
  import SubRequiredBadge from "../../catalogue/SubRequiredBadge.svelte";

  type BadgeStateBadgesPropsType = { item: AccountBadgeType; watch: boolean };

  let { item, watch }: BadgeStateBadgesPropsType = $props();

  const from = $derived(item.evolution?.from?.title ?? "");
</script>

{#if item.quest}
  <Badge text={t.quest_label()} tone={BadgeTone.INFO} dot={false} hint={t.badge_hint_quest()} />
{/if}
{#if item.chest && item.badges.length > 1}
  <Badge
    text={t.reward_chest({ chestCount: item.chest })}
    tone={BadgeTone.ACCENT}
    dot={false}
    icon={IconName.GIFT}
    hint={t.reward_chest_hint({
      badges: strong(item.badges.map((badge) => badge.title).join(", ")),
    })}
  />
{/if}
{#if item.repeat && item.state !== AccountBadgeState.SUB_ONLY}
  <Badge
    text={periodsText(item.periodsDone, item.repeat)}
    tone={BadgeTone.NEUTRAL}
    dot={false}
    icon={IconName.CALENDAR}
  />
{/if}
{#if item.state === AccountBadgeState.CLAIMABLE}
  <Badge text={t.reward_state_ready()} tone={BadgeTone.PRIMARY} dot={false} icon={IconName.GIFT} />
{:else if item.state === AccountBadgeState.NOT_MINABLE}
  <Badge
    text={t.quest_not_minable()}
    tone={BadgeTone.MUTED}
    dot={false}
    hint={t.badge_hint_quest_not_minable()}
  />
{:else if item.state === AccountBadgeState.SUB_ONLY}
  <SubRequiredBadge />
{:else if item.state === AccountBadgeState.EVOLUTION_UNKNOWN}
  <Badge
    text={t.badge_evolution_unknown()}
    tone={BadgeTone.WARNING}
    dot={false}
    icon={IconName.LOCK}
    hint={t.badge_hint_evolution_unknown()}
  />
{:else if item.state === AccountBadgeState.EVOLUTION_LOCKED}
  <Badge
    text={t.badge_evolution_locked({ badge: from })}
    tone={BadgeTone.WARNING}
    dot={false}
    icon={IconName.LOCK}
    hint={t.badge_hint_evolution_locked({ badge: strong(from) })}
  />
{:else if item.state === AccountBadgeState.EVOLUTION_OFF}
  <Badge
    text={t.badge_evolution_off()}
    tone={BadgeTone.NEUTRAL}
    dot={false}
    icon={IconName.SETTINGS}
    hint={t.badge_hint_evolution_off()}
  />
{:else}
  {#if from}
    <Badge
      text={t.badge_evolution({ badge: from })}
      tone={BadgeTone.INFO}
      dot={false}
      hint={t.badge_hint_evolution({ badge: strong(from) })}
    />
  {/if}
  {#if item.state === AccountBadgeState.WAITING && item.nextAt}
    <Badge
      text={t.quest_next_window({ windowTime: relativeTime(item.nextAt, ticker.now) })}
      tone={BadgeTone.INFO}
      dot={false}
      icon={IconName.CLOCK}
    />
  {:else if watch && item.current === 0 && item.required > 0}
    <Badge
      text={watchText(minutes(item.required), item.repeat)}
      tone={BadgeTone.NEUTRAL}
      dot={false}
    />
  {/if}
{/if}
