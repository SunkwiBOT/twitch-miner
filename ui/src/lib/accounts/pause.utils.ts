// ! The datetime-local input reads and writes wall-clock time with no zone suffix: parsed
// and formatted against the configured time zone, not the browser's, so it matches the
// badges and hints that already show times in that zone.

import { wallMsToZoneMs, zoneWallMs } from "../time/time.utils";

export const PAUSE_CUSTOM_DEFAULT_MINUTES = 60;

export const PAUSE_TOMORROW_HOUR = 9;

export const untilInMinutes = (minutes: number, from: number): Date =>
  new Date(from + minutes * 60_000);

export const untilTomorrowAt = (hour: number, from: number, zone: string): Date => {
  const wall = new Date(zoneWallMs(from, zone));

  wall.setUTCDate(wall.getUTCDate() + 1);
  wall.setUTCHours(hour, 0, 0, 0);

  return new Date(wallMsToZoneMs(wall.getTime(), zone));
};

const pad = (value: number): string => String(value).padStart(2, "0");

export const toLocalInputValue = (date: Date, zone: string): string => {
  const wall = new Date(zoneWallMs(date.getTime(), zone));

  return `${wall.getUTCFullYear()}-${pad(wall.getUTCMonth() + 1)}-${pad(wall.getUTCDate())}T${pad(wall.getUTCHours())}:${pad(wall.getUTCMinutes())}`;
};

export const fromLocalInputValue = (value: string, zone: string): Date | null => {
  const match = /^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2})$/.exec(value);

  if (!match) {
    return null;
  }

  const [, year, month, day, hour, minute] = match;
  const wallMs = Date.UTC(
    Number(year),
    Number(month) - 1,
    Number(day),
    Number(hour),
    Number(minute),
  );

  return new Date(wallMsToZoneMs(wallMs, zone));
};
