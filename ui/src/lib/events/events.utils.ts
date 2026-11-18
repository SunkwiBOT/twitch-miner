import { EventKind } from "../bindings/EventKind";
import type { MinerEventType } from "../bindings/MinerEventType";
import { pointsEarnedIcon } from "./event-reasons.utils";
import { IconName } from "../icons.constants";
import {
  EVENT_ICONS,
  JOURNAL_ALL,
  KIND_GROUPS,
  POINTS_ROUTINE,
  TwitchPointsReason,
  type JournalFilterType,
} from "./events.constants";

export const eventIcon = (event: MinerEventType): IconName => {
  const streakBonus =
    event.kind === EventKind.POINTS_EARNED &&
    event.context.reason === TwitchPointsReason.WATCH_STREAK;

  if (streakBonus || event.kind === EventKind.STREAK_RECOVERED) {
    return IconName.WATCH_STREAK;
  }

  const reasonIcon =
    event.kind === EventKind.POINTS_EARNED ? pointsEarnedIcon(event.context.reason) : undefined;

  return reasonIcon ?? EVENT_ICONS[event.kind];
};

export const kindGroupsOf = (filter: JournalFilterType) =>
  KIND_GROUPS.filter((entry) => filter === JOURNAL_ALL || entry.categories.includes(filter));

export const recentEvents = (
  live: MinerEventType[],
  recent: MinerEventType[],
  keep: (event: MinerEventType) => boolean,
  max: number,
): MinerEventType[] =>
  [...live, ...recent]
    .filter((e) => keep(e) && !POINTS_ROUTINE.has(e.kind))
    .filter((e, i, all) => all.findIndex((o) => o.at === e.at && o.title === e.title) === i)
    .sort((a, b) => Date.parse(b.at) - Date.parse(a.at) || b.id - a.id)
    .slice(0, max);
