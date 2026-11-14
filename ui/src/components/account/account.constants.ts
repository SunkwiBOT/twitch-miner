import { ImportEntryStatus } from "../../lib/accounts/accounts.constants";
import { t } from "../../lib/i18n/i18n.svelte";
import { BADGE_TONE_CLASSES } from "../ui/badge.constants";
import { BadgeTone } from "../../lib/tones.constants";

export const QUEUE_PREVIEW = 6;
export const RECENT_MAX = 12;

export const PROXY_EXAMPLE = "socks5://user:pass@host:1080";

export const STREAMER_BOX_CLASS =
  "border-muted-foreground bg-card data-[state=indeterminate]:border-primary data-[state=indeterminate]:bg-primary data-[state=indeterminate]:text-primary-foreground";

export const SPAM_ROW_CLASS =
  "flex min-h-13 items-center gap-3 border-b px-3 py-2 transition-colors last:border-b-0";

export const IMPORT_ENTRY_CLASSES: Record<ImportEntryStatus, string> = {
  [ImportEntryStatus.PENDING]: "border-dashed border-input text-muted-foreground",
  [ImportEntryStatus.FOUND]: BADGE_TONE_CLASSES[BadgeTone.SUCCESS],
  [ImportEntryStatus.ALREADY]: BADGE_TONE_CLASSES[BadgeTone.MUTED],
  [ImportEntryStatus.MISSING]: BADGE_TONE_CLASSES[BadgeTone.DANGER],
  [ImportEntryStatus.UNCHECKED]: BADGE_TONE_CLASSES[BadgeTone.WARNING],
  [ImportEntryStatus.INVALID]: BADGE_TONE_CLASSES[BadgeTone.DANGER],
};

export const IMPORT_ENTRY_TITLES: Record<ImportEntryStatus, (() => string) | null> = {
  [ImportEntryStatus.PENDING]: null,
  [ImportEntryStatus.FOUND]: null,
  [ImportEntryStatus.ALREADY]: () => t.import_entry_already(),
  [ImportEntryStatus.MISSING]: () => t.import_entry_missing(),
  [ImportEntryStatus.UNCHECKED]: () => t.import_entry_unchecked(),
  [ImportEntryStatus.INVALID]: () => t.import_entry_invalid(),
};
