import type { Component } from "svelte";
import { toast as sonner } from "svelte-sonner";

import {
  openBadgeLink as openBadgeLinkCommand,
  openCampaignLink as openCampaignLinkCommand,
  openUrl,
} from "../api/client";
import { isTauri } from "../api/tauri";
import { describeError } from "../errors/errors.utils";
import type { IconName } from "../icons.constants";
import { t } from "../i18n/i18n.svelte";
import { strong, type TextCategoryType, type TextPersonType } from "../text.utils";
import { app } from "./app.svelte";
import {
  LINK_TOAST_KEY,
  LOCK_WARNING_TOAST_KEY,
  TOAST_ACTION_DURATION_MS,
  TOAST_DURATION_MS,
  ToastKind,
} from "./state.constants";

export type ToastSubjectType = { url: string | undefined; name: string; channel: string | null };

export type ToastActionType = { label: string; icon: IconName; run: () => void };

export type ToastCardPropsType = {
  kind: ToastKind;
  title: string;
  description: string | null;
  people: TextPersonType[];
  categories: TextCategoryType[];
  account: ToastSubjectType | null;
  action: ToastActionType | null;
  onclose: () => void;
};

export type LockWarningToastPropsType = { deadline: number; onclose: () => void };

type ToastOptionsType = {
  description?: string;
  people?: TextPersonType[];
  categories?: TextCategoryType[];
  account?: ToastSubjectType;
  action?: ToastActionType;
  key?: string;
  onend?: () => void;
};

let toastId = 0;

let card: Component<ToastCardPropsType>;
let lockWarningCard: Component<LockWarningToastPropsType>;

export const setToastCard = (component: Component<ToastCardPropsType>): void => {
  card = component;
};

export const setLockWarningCard = (component: Component<LockWarningToastPropsType>): void => {
  lockWarningCard = component;
};

export const dismissToast = (id: string): void => {
  sonner.dismiss(id);
};

export const dismissToasts = (): void => {
  sonner.dismiss();
};

export const toast = (kind: ToastKind, title: string, options: ToastOptionsType = {}): void => {
  toastId += 1;

  const id = options.key ?? `toast-${toastId}`;
  const onend = (): void => options.onend?.();

  sonner.custom(card, {
    id,
    duration: options.action ? TOAST_ACTION_DURATION_MS : TOAST_DURATION_MS[kind],
    componentProps: {
      kind,
      title,
      description: options.description ?? null,
      people: options.people ?? [],
      categories: options.categories ?? [],
      account: options.account ?? null,
      action: options.action ?? null,
      onclose: () => dismissToast(id),
    },
    onAutoClose: onend,
    onDismiss: onend,
  });
};

// ! Infinite duration, no auto-dismiss: only an interaction or the lock
// itself (which clears every toast) ends it.
export const showLockWarning = (deadline: number): void => {
  sonner.custom(lockWarningCard, {
    id: LOCK_WARNING_TOAST_KEY,
    duration: Number.POSITIVE_INFINITY,
    componentProps: {
      deadline,
      onclose: dismissLockWarning,
    },
  });
};

export const dismissLockWarning = (): void => {
  dismissToast(LOCK_WARNING_TOAST_KEY);
};

const notifyOpened = (host: string): void => {
  if (!isTauri()) {
    return;
  }

  const site = host.replace(/^www\./, "");

  toast(ToastKind.INFO, t.link_opened_title(), {
    description: t.link_opened({ site: strong(site) }),
    key: LINK_TOAST_KEY,
  });
};

const openAndNotify = async (url: string, open: () => Promise<void>): Promise<void> => {
  try {
    await open();
  } catch (e: unknown) {
    toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });

    return;
  }

  notifyOpened(new URL(url).host);
};

export const openLink = (url: string): Promise<void> => openAndNotify(url, () => openUrl(url));

export const openCampaignLink = async (
  accountId: string,
  campaignId: string,
  url: string,
): Promise<void> => {
  let host: string;

  try {
    host = await openCampaignLinkCommand(accountId, campaignId, url);
  } catch (e: unknown) {
    toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });

    return;
  }

  notifyOpened(host);
};

export const openBadgeLink = (badgeKey: string, url: string): Promise<void> =>
  openAndNotify(url, () => openBadgeLinkCommand(badgeKey, url));

export const accountImage = (accountId: string, fallback: string): ToastSubjectType => {
  const found = app.snapshot?.accounts.find((a) => a.id === accountId);

  return { url: found?.avatar_url, name: found?.display_name ?? fallback, channel: null };
};
