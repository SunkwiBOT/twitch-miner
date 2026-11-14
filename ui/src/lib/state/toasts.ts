import type { Component } from "svelte";
import { toast as sonner } from "svelte-sonner";

import { openUrl } from "../api/client";
import { isTauri } from "../api/tauri";
import { describeError } from "../errors/errors.utils";
import { t } from "../i18n/i18n.svelte";
import { strong } from "../text.utils";
import { app } from "./app.svelte";
import {
  LINK_TOAST_KEY,
  TOAST_ACTION_DURATION_MS,
  TOAST_DURATION_MS,
  ToastKind,
} from "./state.constants";

export type ToastImageType = { url: string | undefined; name: string; channel: string | null };

export type ToastActionType = { label: string; run: () => void };

export type ToastCardPropsType = {
  kind: ToastKind;
  text: string;
  image: ToastImageType | null;
  title: string | null;
  action: ToastActionType | null;
  onclose: () => void;
};

type ToastOptionsType = {
  image?: ToastImageType;
  title?: string;
  action?: ToastActionType;
  key?: string;
  onend?: () => void;
};

let toastId = 0;

let card: Component<ToastCardPropsType>;

export const setToastCard = (component: Component<ToastCardPropsType>): void => {
  card = component;
};

export const dismissToast = (id: string): void => {
  sonner.dismiss(id);
};

export const dismissToasts = (): void => {
  sonner.dismiss();
};

export const toast = (kind: ToastKind, text: string, options: ToastOptionsType = {}): void => {
  toastId += 1;

  const id = options.key ?? `toast-${toastId}`;
  const onend = (): void => options.onend?.();

  sonner.custom(card, {
    id,
    duration: options.action ? TOAST_ACTION_DURATION_MS : TOAST_DURATION_MS[kind],
    componentProps: {
      kind,
      text,
      image: options.image ?? null,
      title: options.title ?? null,
      action: options.action ?? null,
      onclose: () => dismissToast(id),
    },
    onAutoClose: onend,
    onDismiss: onend,
  });
};

export const openLink = async (url: string): Promise<void> => {
  try {
    await openUrl(url);
  } catch (e: any) {
    toast(ToastKind.ERROR, describeError(e));

    return;
  }

  if (isTauri()) {
    const site = new URL(url).host.replace(/^www\./, "");

    toast(ToastKind.INFO, t.link_opened({ site: strong(site) }), { key: LINK_TOAST_KEY });
  }
};

export const accountImage = (accountId: string, fallback: string): ToastImageType => {
  const found = app.snapshot?.accounts.find((a) => a.id === accountId);

  return { url: found?.avatar_url, name: found?.display_name ?? fallback, channel: null };
};
