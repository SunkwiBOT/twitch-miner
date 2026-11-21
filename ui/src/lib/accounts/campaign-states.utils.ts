import { BenefitKind } from "../bindings/BenefitKind";
import type { CampaignType } from "../bindings/CampaignType";
import { shortDuration } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { IconName } from "../icons.constants";
import type { TimelineTagType } from "../catalogue/timeline.utils";
import { relativeTime } from "../time/time.utils";
import { BadgeTone } from "../tones.constants";
import { QUEUE_REASON_LABELS } from "./accounts.utils";
import {
  facetKey,
  type AccountCampaignType,
  type CampaignGroupsType,
  type FacetValueType,
} from "./campaign-groups.utils";
import {
  CAMPAIGN_GROUPS,
  CampaignFacet,
  CampaignGroup,
  CampaignState,
  LinkFacet,
  RequirementFacet,
  STATE_GROUPS,
} from "./campaign-groups.constants";

export type CampaignStateViewType = {
  text: string;
  tone: BadgeTone;
  icon: IconName;
  hint: string;
};

type StateViewType = {
  text: () => string;
  tone: BadgeTone;
  icon: IconName;
  hint: (entry: AccountCampaignType, now: number) => string;
};

const evolutionHint = (campaign: CampaignType): string => {
  const from = campaign.drops.find((drop) => drop.evolution && !drop.evolution.owned)?.evolution
    ?.from?.title;

  return from ? t.badge_hint_evolution_locked({ badge: from }) : t.badge_hint_evolution_unknown();
};

const STATE_VIEWS: Record<CampaignState, StateViewType> = {
  [CampaignState.MINING]: {
    text: () => t.queue_mining(),
    tone: BadgeTone.SUCCESS,
    icon: IconName.PLAY,
    hint: () => t.badge_hint_queue_mining(),
  },
  [CampaignState.QUEUED]: {
    text: () => t.dashboard_queue_blocked_one_campaign_at_a_time(),
    tone: BadgeTone.MUTED,
    icon: IconName.LIST_ORDERED,
    hint: () => t.badge_hint_blocked_one_at_a_time(),
  },
  [CampaignState.NO_LIVE_CHANNEL]: {
    text: () => t.dashboard_queue_blocked_no_live_channel(),
    tone: BadgeTone.WARNING,
    icon: IconName.RADIO,
    hint: () => t.badge_hint_blocked_no_live(),
  },
  [CampaignState.SET_ASIDE]: {
    text: () => t.badge_progress_set_aside(),
    tone: BadgeTone.WARNING,
    icon: IconName.CLOCK,
    hint: (entry, now) =>
      t.badge_hint_blocked_set_aside({
        time: relativeTime(entry.queued?.set_aside_until, now),
      }),
  },
  [CampaignState.CLAIMED_LINKED]: {
    text: () => t.campaign_state_claimed_linked(),
    tone: BadgeTone.SUCCESS,
    icon: IconName.LINK,
    hint: () => t.badge_hint_claimed(),
  },
  [CampaignState.CLAIMED]: {
    text: () => t.reward_state_claimed(),
    tone: BadgeTone.SUCCESS,
    icon: IconName.CHECK,
    hint: () => t.badge_hint_claimed(),
  },
  [CampaignState.DONE]: {
    text: () => t.campaign_state_done(),
    tone: BadgeTone.PRIMARY,
    icon: IconName.GIFT,
    hint: () => t.badge_hint_ready(),
  },
  [CampaignState.DONE_NOT_LINKED]: {
    text: () => t.campaign_state_done_not_linked(),
    tone: BadgeTone.WARNING,
    icon: IconName.ALERT,
    hint: () => t.badge_hint_not_linked(),
  },
  [CampaignState.DONE_SUB_REQUIRED]: {
    text: () => t.campaign_state_done_sub_required(),
    tone: BadgeTone.WARNING,
    icon: IconName.ALERT,
    hint: () => t.campaign_hint_done_sub_required(),
  },
  [CampaignState.CLAIM_FAILED]: {
    text: () => t.campaign_state_claim_failed(),
    tone: BadgeTone.DANGER,
    icon: IconName.ALERT,
    hint: () => t.campaign_hint_claim_failed({ journal: t.nav_history() }),
  },
  [CampaignState.OWNED]: {
    text: () => t.reward_state_owned(),
    tone: BadgeTone.SUCCESS,
    icon: IconName.CHECK,
    hint: () => t.badge_hint_owned(),
  },
  [CampaignState.IGNORED]: {
    text: () => t.reward_ignored(),
    tone: BadgeTone.NEUTRAL,
    icon: IconName.EYE_OFF,
    hint: () => t.campaign_hint_ignored({ tab: t.drops_tab_exclusions() }),
  },
  [CampaignState.FILTERED]: {
    text: () => t.campaign_state_filtered(),
    tone: BadgeTone.NEUTRAL,
    icon: IconName.EYE_OFF,
    hint: () => t.campaign_hint_filtered({ setting: t.drops_exclusions_title() }),
  },
  [CampaignState.EVOLUTIONS_OFF]: {
    text: () => t.badge_evolution_off(),
    tone: BadgeTone.NEUTRAL,
    icon: IconName.SETTINGS,
    hint: () => t.badge_hint_evolution_off(),
  },
  [CampaignState.POINTS_ONLY]: {
    text: () => t.mode_points_title(),
    tone: BadgeTone.NEUTRAL,
    icon: IconName.CHANNEL_POINTS,
    hint: () =>
      t.campaign_hint_points_only({
        setting: t.account_mode_title(),
        value: t.mode_points_title(),
      }),
  },
  [CampaignState.SUB_REQUIRED]: {
    text: () => t.badge_progress_sub_required(),
    tone: BadgeTone.WARNING,
    icon: IconName.ALERT,
    hint: () => t.badge_hint_sub_only(),
  },
  [CampaignState.EVOLUTION_LOCKED]: {
    text: () => t.campaign_state_evolution_locked(),
    tone: BadgeTone.WARNING,
    icon: IconName.LOCK,
    hint: (entry) => evolutionHint(entry.campaign),
  },
  [CampaignState.WAITING]: {
    text: () => t.campaign_state_waiting(),
    tone: BadgeTone.INFO,
    icon: IconName.CLOCK,
    hint: () => t.campaign_hint_waiting(),
  },
  [CampaignState.UNFINISHABLE]: {
    text: () => t.campaign_state_unfinishable(),
    tone: BadgeTone.NEUTRAL,
    icon: IconName.CLOCK,
    hint: () => t.campaign_hint_unfinishable({ setting: t.account_drops_skip_unfinishable() }),
  },
  [CampaignState.NOT_SELECTED]: {
    text: () => t.selection_not_selected(),
    tone: BadgeTone.NEUTRAL,
    icon: IconName.STAR,
    hint: () => t.campaign_hint_not_selected({ setting: t.selection_only_title() }),
  },
  [CampaignState.NOT_LINKED]: {
    text: () => t.dashboard_not_linked(),
    tone: BadgeTone.WARNING,
    icon: IconName.ALERT,
    hint: () =>
      t.campaign_hint_not_linked({
        setting: t.drops_scope(),
        value: t.drops_scope_linked_only(),
      }),
  },
  [CampaignState.LANE_OFF]: {
    text: () => t.campaign_state_lane_off(),
    tone: BadgeTone.NEUTRAL,
    icon: IconName.LIST_ORDERED,
    hint: () => t.campaign_hint_lane_off({ setting: t.mining_order_title() }),
  },
  [CampaignState.NOT_ACTIVE]: {
    text: () => t.campaign_state_not_active(),
    tone: BadgeTone.NEUTRAL,
    icon: IconName.CALENDAR,
    hint: () => t.campaign_hint_not_active(),
  },
};

export const stateLabel = (state: CampaignState): string => STATE_VIEWS[state].text();

export const campaignStateView = (
  entry: AccountCampaignType,
  now: number,
): CampaignStateViewType => {
  const view = STATE_VIEWS[entry.state];

  return { text: view.text(), tone: view.tone, icon: view.icon, hint: view.hint(entry, now) };
};

// ! A campaign out of the queue is blurred under its state; one in it stays clear.
export const overlaid = (state: CampaignState): boolean =>
  STATE_GROUPS[state] !== CampaignGroup.TO_MINE;

export const etaView = (startsIn: number): CampaignStateViewType => ({
  text: t.rewards_eta({ time: shortDuration(startsIn) }),
  tone: BadgeTone.INFO,
  icon: IconName.CLOCK,
  hint: t.rewards_eta_hint(),
});

export const timelineTag = (
  entry: AccountCampaignType,
  startsIn: number | null,
  now: number,
): TimelineTagType => ({
  ...(startsIn === null ? campaignStateView(entry, now) : etaView(startsIn)),
  muted: overlaid(entry.state),
});

const GROUP_LABELS: Record<CampaignGroup, () => string> = {
  [CampaignGroup.TO_MINE]: () => t.rewards_tab_to_mine(),
  [CampaignGroup.BLOCKED]: () => t.rewards_tab_blocked(),
  [CampaignGroup.CLAIMED]: () => t.rewards_tab_claimed(),
};

const GROUP_ICONS: Record<CampaignGroup, IconName> = {
  [CampaignGroup.TO_MINE]: IconName.PLAY,
  [CampaignGroup.BLOCKED]: IconName.LOCK,
  [CampaignGroup.CLAIMED]: IconName.CHECK,
};

const GROUP_EMPTY: Record<CampaignGroup, () => string> = {
  [CampaignGroup.TO_MINE]: () => t.rewards_empty_to_mine(),
  [CampaignGroup.BLOCKED]: () => t.rewards_empty_blocked(),
  [CampaignGroup.CLAIMED]: () => t.rewards_empty_claimed(),
};

export const groupEmpty = (group: CampaignGroup): string => GROUP_EMPTY[group]();

export const groupOptions = (
  groups: CampaignGroupsType,
): { value: CampaignGroup; label: string; icon: IconName; count: number }[] =>
  CAMPAIGN_GROUPS.map((group) => ({
    value: group,
    label: GROUP_LABELS[group](),
    icon: GROUP_ICONS[group],
    count: groups[group].length,
  }));

export const groupsCount = (groups: CampaignGroupsType): number =>
  CAMPAIGN_GROUPS.reduce((sum, group) => sum + groups[group].length, 0);

const FACET_LABELS: Record<CampaignFacet, () => string> = {
  [CampaignFacet.STATE]: () => t.rewards_filter_state(),
  [CampaignFacet.REASON]: () => t.rewards_filter_reason(),
  [CampaignFacet.LINK]: () => t.rewards_filter_link(),
  [CampaignFacet.REQUIREMENT]: () => t.rewards_filter_requirement(),
  [CampaignFacet.REWARD]: () => t.rewards_filter_reward(),
};

const LINK_LABELS: Record<LinkFacet, () => string> = {
  [LinkFacet.LINKED]: () => t.dashboard_linked(),
  [LinkFacet.NOT_LINKED]: () => t.dashboard_not_linked(),
  [LinkFacet.NO_LINK]: () => t.rewards_link_none(),
};

const LINK_ICONS: Record<LinkFacet, IconName> = {
  [LinkFacet.LINKED]: IconName.LINK,
  [LinkFacet.NOT_LINKED]: IconName.ALERT,
  [LinkFacet.NO_LINK]: IconName.CHECK,
};

const REQUIREMENT_LABELS: Record<RequirementFacet, () => string> = {
  [RequirementFacet.WATCH_TIME]: () => t.badges_filter_watch(),
  [RequirementFacet.SUBSCRIPTION]: () => t.badges_filter_sub(),
};

const REWARD_LABELS: Record<BenefitKind, () => string> = {
  [BenefitKind.BADGE]: () => t.reward_kind_badge(),
  [BenefitKind.EMOTE]: () => t.reward_kind_emote(),
  [BenefitKind.IN_GAME_ITEM]: () => t.reward_kind_item(),
  [BenefitKind.OTHER]: () => t.reward_kind_other(),
};

const facetLabel = (option: FacetValueType): { label: string; icon?: IconName } => {
  switch (option.facet) {
    case CampaignFacet.STATE:
      return { label: stateLabel(option.value), icon: STATE_VIEWS[option.value].icon };
    case CampaignFacet.REASON:
      return { label: QUEUE_REASON_LABELS[option.value]() };
    case CampaignFacet.LINK:
      return { label: LINK_LABELS[option.value](), icon: LINK_ICONS[option.value] };
    case CampaignFacet.REQUIREMENT:
      return { label: REQUIREMENT_LABELS[option.value]() };
    case CampaignFacet.REWARD:
      return { label: REWARD_LABELS[option.value]() };
  }
};

export const filterOptions = (
  options: readonly FacetValueType[],
): { value: string; label: string; icon?: IconName; group: string }[] =>
  options.map((option) => ({
    value: facetKey(option),
    group: FACET_LABELS[option.facet](),
    ...facetLabel(option),
  }));
