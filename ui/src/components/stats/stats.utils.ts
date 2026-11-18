import type { DropsByCategoryType } from "../../lib/bindings/DropsByCategoryType";
import type { PointsByDayType } from "../../lib/bindings/PointsByDayType";
import { t } from "../../lib/i18n/i18n.svelte";
import { DAY_MS, HOUR_MS } from "../../lib/time/time.constants";
import { timePreferences } from "../../lib/time/time.svelte";
import { formatDayMonth, wallMsToZoneMs, zoneWallMs } from "../../lib/time/time.utils";
import {
  STATS_CATEGORY_TONES,
  STATS_KNOWN_REASONS,
  STATS_PERIODS,
  STATS_REASON_TONES,
} from "./stats.constants";
import type {
  StatsBarRowType,
  StatsDayBucketType,
  StatsPeriodDaysType,
  StatsReasonToneType,
} from "./stats.types";

// ! Day boundaries in the user's time zone (`timePreferences.zone`), not
// UTC: a period of "7 days" means 7 of the user's own calendar days, and
// the bucket a point lands in must match the day shown under it.
const startOfDayInZone = (ms: number, zone: string): number => {
  const wall = new Date(zoneWallMs(ms, zone));

  wall.setUTCHours(0, 0, 0, 0);

  return wallMsToZoneMs(wall.getTime(), zone);
};

// ! Steps by more than 24h then re-snaps to that zone's own day start, so a
// 23h or 25h DST day never gets skipped or repeated.
const nextDayInZone = (dayStart: number, zone: string): number =>
  startOfDayInZone(dayStart + DAY_MS + HOUR_MS, zone);

// ! The exact instants `HistoryStatsQuery.day_boundaries` sends and
// `PointsByDayType.day_start` echoes back: bucketing matches on these ISO
// strings directly instead of a recomputed date key, so there is one source
// for "which day" rather than two that could drift apart across a DST span.
export const dayBoundaries = (period: StatsPeriodDaysType, now: number): string[] => {
  const zone = timePreferences.zone;
  const boundaries: number[] = [startOfDayInZone(now, zone)];

  for (let i = 1; i < period; i++) {
    const previous = startOfDayInZone(boundaries[0] - HOUR_MS, zone);

    boundaries.unshift(previous);
  }

  return boundaries.map((ms) => new Date(ms).toISOString());
};

export const periodRange = (
  period: StatsPeriodDaysType,
  now: number,
): { since: string; until: string } => {
  const boundaries = dayBoundaries(period, now);
  const since = boundaries[0];
  const lastBoundary = Date.parse(boundaries.at(-1) ?? since);
  const until = new Date(nextDayInZone(lastBoundary, timePreferences.zone)).toISOString();

  return { since, until };
};

export const clampPeriod = (
  period: StatsPeriodDaysType,
  retentionDays: number | null,
): StatsPeriodDaysType => {
  if (retentionDays === null || period <= retentionDays) {
    return period;
  }

  return STATS_PERIODS.filter((candidate) => candidate <= retentionDays).at(-1) ?? STATS_PERIODS[0];
};

const hashIndex = (value: string, length: number): number => {
  let hash = 0;

  for (const char of value) {
    hash = (hash * 31 + char.charCodeAt(0)) % length;
  }

  return hash;
};

export const reasonTone = (reason: string): StatsReasonToneType => {
  const known = STATS_KNOWN_REASONS.indexOf(reason);

  return STATS_REASON_TONES[known >= 0 ? known : hashIndex(reason, STATS_REASON_TONES.length)];
};

export const buildDayBuckets = (
  pointsByDay: PointsByDayType[],
  boundaries: string[],
): { days: StatsDayBucketType[]; reasons: string[] } => {
  const byDay = new Map<number, Map<string, number>>();
  const totals = new Map<string, number>();

  for (const row of pointsByDay) {
    const ms = Date.parse(row.day_start);
    const dayReasons = byDay.get(ms) ?? new Map<string, number>();

    dayReasons.set(row.reason, (dayReasons.get(row.reason) ?? 0) + row.points);
    byDay.set(ms, dayReasons);
    totals.set(row.reason, (totals.get(row.reason) ?? 0) + row.points);
  }

  const reasons = [...totals.entries()].sort((a, b) => b[1] - a[1]).map(([key]) => key);
  const days: StatsDayBucketType[] = boundaries.map((boundary) => {
    const ms = Date.parse(boundary);
    const dayReasons = byDay.get(ms);
    const byReason: Record<string, number> = {};
    let total = 0;

    for (const reason of reasons) {
      const value = dayReasons?.get(reason) ?? 0;

      byReason[reason] = value;
      total += value;
    }

    return { ms, dateLabel: formatDayMonth(ms), byReason, total };
  });

  return { days, reasons };
};

export const categoryRows = (rows: DropsByCategoryType[]): StatsBarRowType[] =>
  [...rows]
    .sort((a, b) => b.count - a.count)
    .map((row) => ({
      key: row.category ?? "",
      label: row.category ?? t.stats_category_unknown(),
      value: row.count,
    }));

export const categoryTone = (index: number): string =>
  STATS_CATEGORY_TONES[index % STATS_CATEGORY_TONES.length];
