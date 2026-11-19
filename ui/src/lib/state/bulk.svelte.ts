import { number } from "../format.utils";
import { t } from "../i18n/i18n.svelte";
import { strong } from "../text.utils";
import { BULK_TOAST_KEY, ToastKind } from "./state.constants";
import { updateAccountSettings } from "./account-settings.svelte";
import { app } from "./app.svelte";
import { type AccountChangeType, wouldChange } from "./bulk.utils";
import { accountImage, toast } from "./toasts";
import type { ToastSubjectType } from "./toasts";

type BulkStateType = { open: boolean; sourceId: string | null; changes: AccountChangeType[] };

export const bulk = $state<BulkStateType>({ open: false, sourceId: null, changes: [] });

export const closeBulk = (): void => {
  bulk.open = false;
  bulk.sourceId = null;
  bulk.changes = [];
};

export const changedWhenSaved = async (
  saved: Promise<boolean>,
  sourceId: string,
  text: string | undefined,
  change: AccountChangeType,
  subject?: ToastSubjectType,
): Promise<boolean> => {
  if (!(await saved)) {
    return false;
  }

  changed(sourceId, text, change, subject);

  return true;
};

export const changed = (
  sourceId: string,
  text: string | undefined,
  change: AccountChangeType,
  subject?: ToastSubjectType,
): void => {
  const image = accountImage(sourceId, "");

  if ((app.snapshot?.accounts.length ?? 0) < 2) {
    toast(ToastKind.SUCCESS, t.change_settings(), {
      description: text,
      account: subject ?? image,
    });

    return;
  }

  if (bulk.sourceId !== sourceId) {
    bulk.sourceId = sourceId;
    bulk.changes = [];
  }

  bulk.changes.push(change);

  const count = bulk.changes.length;
  const others = (app.snapshot?.accounts ?? []).filter((a) => a.id !== sourceId);
  const applicable = others.some((a) => wouldChange(a, bulk.changes));

  toast(ToastKind.SUCCESS, t.change_settings(), {
    key: BULK_TOAST_KEY,
    description: count === 1 ? text : t.bulk_changes({ count, shown: strong(number(count)) }),
    account: count === 1 ? (subject ?? image) : image,
    action: applicable
      ? { label: t.bulk_apply_others(), run: () => (bulk.open = true) }
      : undefined,
    onend: () => {
      if (!bulk.open) {
        closeBulk();
      }
    },
  });
};

export const applyToAccounts = async (accountIds: string[]): Promise<void> => {
  const changes = [...bulk.changes];
  const results = await Promise.all(
    accountIds.map((id) => {
      const target = app.snapshot?.accounts.find((a) => a.id === id);

      if (!target || !wouldChange(target, changes)) {
        return Promise.resolve(false);
      }

      return updateAccountSettings(id, (settings) => {
        for (const change of changes) {
          change(settings, target);
        }
      });
    }),
  );

  const applied = results.filter(Boolean).length;

  if (applied > 0) {
    toast(ToastKind.SUCCESS, t.bulk_applied_title(), {
      description: t.common_accounts_count({ count: strong(number(applied)) }),
    });
  }
};
