import type { BadgeViewType } from "./badge-views.constants";
import type { IconName } from "./icons.constants";
import type { BadgeTone } from "./tones.constants";

export type BadgePropsType = { text: string; tone: BadgeTone; icon?: IconName; hint?: string };

export const badgeProps = (view: BadgeViewType): BadgePropsType => ({
  text: view.text(),
  tone: view.tone,
  icon: view.icon,
  hint: view.hint?.(),
});
