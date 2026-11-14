import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
import { LogLevel } from "../../lib/bindings/LogLevel";
import { t } from "../../lib/i18n/i18n.svelte";

export const LOG_LEVEL_LABELS: Partial<Record<LogLevel, () => string>> = {
  [LogLevel.ERROR]: () => t.logs_level_error(),
  [LogLevel.WARN]: () => t.logs_level_warn(),
  [LogLevel.INFO]: () => t.logs_level_info(),
};

export const accountOptions = (accounts: AccountSnapshotType[]) => [
  { value: "", label: t.logs_filter_all() },
  ...accounts.map((a) => ({ value: a.id, label: a.display_name, image: a.avatar_url })),
];
