// ! Quiet window times are minutes since midnight in wall-clock time, with no date or zone
// of their own: formatted from a synthetic UTC instant so no local or configured zone can
// shift the digits shown.

import type { ClockFormat } from "../bindings/ClockFormat";
import { locale } from "../i18n/i18n.svelte";
import { hour12 } from "./time.utils";
import { timePreferences } from "./time.svelte";

export const minutesToClockInputValue = (minutes: number): string =>
  `${String(Math.floor(minutes / 60)).padStart(2, "0")}:${String(minutes % 60).padStart(2, "0")}`;

export const clockInputValueToMinutes = (value: string): number | null => {
  const match = /^(\d{1,2}):(\d{2})$/.exec(value);

  if (!match) {
    return null;
  }

  const hours = Number(match[1]);
  const mins = Number(match[2]);

  return hours >= 0 && hours <= 23 && mins >= 0 && mins <= 59 ? hours * 60 + mins : null;
};

export const minutesToClock = (
  minutes: number,
  format: ClockFormat = timePreferences.clockFormat,
): string =>
  new Intl.DateTimeFormat(locale.current, {
    timeZone: "UTC",
    hour: "2-digit",
    minute: "2-digit",
    hour12: hour12(format),
  }).format(Date.UTC(2000, 0, 1, Math.floor(minutes / 60), minutes % 60));
