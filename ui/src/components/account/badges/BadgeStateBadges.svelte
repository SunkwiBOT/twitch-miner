<script lang="ts">
  import { AccountBadgeState, type AccountBadgeType } from "../../../lib/badges/badges.utils";
  import { minutes } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { ticker } from "../../../lib/state/ticker.svelte";
  import { relativeTime } from "../../../lib/time/time.utils";
  import Badge from "../../ui/Badge.svelte";
  import { BadgeTone, IconName } from "../../ui/ui.constants";

  type BadgeStateBadgesPropsType = { item: AccountBadgeType; watch: boolean };

  let { item, watch }: BadgeStateBadgesPropsType = $props();
</script>

{#if item.quest}
  <Badge text={t.quest_label()} tone={BadgeTone.INFO} dot={false} hint={t.badge_hint_quest()} />
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
{:else if watch && item.current === 0 && item.required > 0}
  <Badge
    text={t.reward_watch({ time: minutes(item.required) })}
    tone={BadgeTone.NEUTRAL}
    dot={false}
  />
{/if}
