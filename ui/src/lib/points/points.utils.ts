import { number } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { strong } from "../text.utils";
import {
  TWITCH_SUB_POINTS_MULTIPLIER_MAX,
  TWITCH_SUB_POINTS_MULTIPLIER_MIN,
} from "./points.constants";

export const subscriberPriorityHint = (): string =>
  t.points_subscriber_priority_hint({
    minMultiplier: strong(number(TWITCH_SUB_POINTS_MULTIPLIER_MIN)),
    maxMultiplier: strong(number(TWITCH_SUB_POINTS_MULTIPLIER_MAX)),
  });
