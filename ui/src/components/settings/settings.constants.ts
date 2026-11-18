import { HistoryRetention } from "../../lib/bindings/HistoryRetention";
import { t } from "../../lib/i18n/i18n.svelte";

export const PASSWORD_MIN_LENGTH = 4;

export const FIELD_GRID_CLASSES = "grid gap-3 @md:grid-cols-2";

// ! A day above 12 in every time zone: day/month and month/day never look alike.
export const SAMPLE_DATE = Date.UTC(2026, 11, 24, 18, 41);

// ! Single label source for both the settings dropdown and the stats period
// selector's disabled tooltip, in the order shown.
export const RETENTION_OPTIONS: HistoryRetention[] = [
  HistoryRetention.DAYS30,
  HistoryRetention.DAYS90,
  HistoryRetention.YEAR1,
  HistoryRetention.UNLIMITED,
];

export const RETENTION_LABELS: Record<HistoryRetention, () => string> = {
  [HistoryRetention.DAYS30]: () => t.retention_30(),
  [HistoryRetention.DAYS90]: () => t.retention_90(),
  [HistoryRetention.YEAR1]: () => t.retention_365(),
  [HistoryRetention.UNLIMITED]: () => t.retention_unlimited(),
};
