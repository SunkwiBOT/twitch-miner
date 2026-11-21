export type StatsPeriodDaysType = 7 | 30 | 90;

export type StatsDayBucketType = {
  ms: number;
  dateLabel: string;
  byReason: Record<string, number>;
  total: number;
};

export type StatsReasonToneType = { bg: string; fill: string; text: string };

export type StatsBarRowType = { key: string; label: string; url: string | null; value: number };
