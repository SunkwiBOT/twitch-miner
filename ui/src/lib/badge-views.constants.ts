import { t } from "./i18n/i18n.svelte";
import { IconName } from "./icons.constants";
import { BadgeTone } from "./tones.constants";

// ! One look per meaning: every badge that says the same thing reads its text, tone and icon here.
export type BadgeViewType = {
  text: () => string;
  tone: BadgeTone;
  icon?: IconName;
  hint?: () => string;
};

export const READY_VIEW = {
  text: () => t.reward_state_ready(),
  tone: BadgeTone.PRIMARY,
  icon: IconName.GIFT,
  hint: () => t.badge_hint_ready(),
} satisfies BadgeViewType;

export const CLAIMED_VIEW = {
  text: () => t.reward_state_claimed(),
  tone: BadgeTone.SUCCESS,
  icon: IconName.CHECK,
  hint: () => t.badge_hint_claimed(),
} satisfies BadgeViewType;

export const OWNED_VIEW = {
  text: () => t.reward_state_owned(),
  tone: BadgeTone.SUCCESS,
  icon: IconName.CHECK,
  hint: () => t.badge_hint_owned(),
} satisfies BadgeViewType;

export const MINING_VIEW = {
  text: () => t.queue_mining(),
  tone: BadgeTone.SUCCESS,
  icon: IconName.PLAY,
  hint: () => t.badge_hint_queue_mining(),
} satisfies BadgeViewType;

// ! Its hint names the retry time, so each place builds it.
export const SET_ASIDE_VIEW = {
  text: () => t.badge_progress_set_aside(),
  tone: BadgeTone.WARNING,
  icon: IconName.CLOCK,
} satisfies BadgeViewType;

export const NOT_MINABLE_VIEW = {
  text: () => t.quest_not_minable(),
  tone: BadgeTone.MUTED,
  hint: () => t.badge_hint_quest_not_minable(),
} satisfies BadgeViewType;

export const EVOLUTION_OFF_VIEW = {
  text: () => t.badge_evolution_off(),
  tone: BadgeTone.NEUTRAL,
  icon: IconName.SETTINGS,
  hint: () => t.badge_hint_evolution_off(),
} satisfies BadgeViewType;

// ! Its hint names where the choice was made, so each place builds it.
export const IGNORED_VIEW = {
  text: () => t.reward_ignored(),
  tone: BadgeTone.NEUTRAL,
  icon: IconName.EYE_OFF,
} satisfies BadgeViewType;

export const NOT_SELECTED_VIEW = {
  text: () => t.selection_not_selected(),
  tone: BadgeTone.NEUTRAL,
  icon: IconName.STAR,
} satisfies BadgeViewType;

export const NOT_LINKED_VIEW = {
  text: () => t.dashboard_not_linked(),
  tone: BadgeTone.WARNING,
  icon: IconName.ALERT,
  hint: () => t.badge_hint_not_linked(),
} satisfies BadgeViewType;

export const SUB_REQUIRED_VIEW = {
  text: () => t.campaign_progress_sub_required(),
  tone: BadgeTone.WARNING,
  icon: IconName.ALERT,
  hint: () => t.badge_hint_sub_only(),
} satisfies BadgeViewType;

export const NEW_VIEW = {
  text: () => t.catalogue_new(),
  tone: BadgeTone.INFO,
} satisfies BadgeViewType;
