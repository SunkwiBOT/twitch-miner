import { isTauri } from "../api/tauri";
import type { SnapshotType } from "../bindings/SnapshotType";
import { account, app } from "./app.svelte";
import { parseRoute, routePath } from "./routes.utils";
import { ACCOUNT_PAGES, HistoryMode, MouseButton, Page, PAGE_PATHS } from "./state.constants";
import { KEY_STORAGE_ACCOUNT, KEY_STORAGE_PAGE } from "./storage.constants";
import { recall, remember } from "./storage.utils";

export const isAccountPage = (page: Page): boolean => ACCOUNT_PAGES.has(page);

const addressTab = (): string => location.hash.slice(1);

export const openTab = <T extends string>(
  tabs: readonly T[],
  slugs: Record<T, string>,
  fallback: T,
): T => tabs.find((tab) => slugs[tab] === app.tab) ?? fallback;

export const showTab = <T extends string>(slugs: Record<T, string>, tab: T): void => {
  app.tab = slugs[tab];
  history.replaceState(history.state, "", `${location.pathname}${location.search}#${app.tab}`);
};

const PAGES: ReadonlySet<string> = new Set(Object.values(Page));
const isPage = (value: string | null): value is Page => value !== null && PAGES.has(value);

let pendingLogin: string | null = null;

const writeAddress = (mode: HistoryMode): void => {
  const login = isAccountPage(app.page) ? (account(app.selectedAccount)?.login ?? null) : null;

  if ((isAccountPage(app.page) && !login) || pendingLogin !== null) {
    return;
  }

  const path = routePath(app.page, login);

  if (path === location.pathname) {
    return;
  }

  app.tab = "";

  if (mode === HistoryMode.PUSH) {
    history.pushState(null, "", path);
  } else {
    history.replaceState(null, "", path);
  }
};

const followAddress = (): void => {
  const route = parseRoute(location.pathname) ?? { page: Page.HOME, login: null };

  app.page = route.page;
  app.tab = addressTab();

  if (route.login !== null) {
    const found = app.snapshot?.accounts.find((a) => a.login === route.login);

    if (found) {
      app.selectedAccount = found.id;
    } else if (app.snapshot) {
      app.page = Page.HOME;
    } else {
      pendingLogin = route.login;
    }
  }

  remember(KEY_STORAGE_PAGE, app.page);

  if (app.selectedAccount) {
    remember(KEY_STORAGE_ACCOUNT, app.selectedAccount);
  }

  writeAddress(HistoryMode.REPLACE);
};

export const followSnapshotAccounts = (snapshot: SnapshotType): void => {
  if (pendingLogin !== null) {
    const found = snapshot.accounts.find((a) => a.login === pendingLogin);

    pendingLogin = null;

    if (found) {
      app.selectedAccount = found.id;
    } else {
      app.page = Page.HOME;
    }
  }

  if (app.selectedAccount && !snapshot.accounts.some((a) => a.id === app.selectedAccount)) {
    app.selectedAccount = null;

    if (isAccountPage(app.page)) {
      app.page = Page.HOME;
    }
  }

  writeAddress(HistoryMode.REPLACE);
};

export const navigate = (page: Page, accountId?: string): void => {
  const next = isAccountPage(page) && !(accountId ?? app.selectedAccount) ? Page.HOME : page;

  app.page = next;
  app.navDrawer = false;
  remember(KEY_STORAGE_PAGE, next);

  if (accountId !== undefined) {
    app.selectedAccount = accountId;
    remember(KEY_STORAGE_ACCOUNT, accountId);
  }

  writeAddress(HistoryMode.PUSH);
};

// ! The desktop app's window does not follow the mouse's back and forward buttons everywhere.
const followMouseButtons = (event: MouseEvent): void => {
  if (event.button === MouseButton.BACK) {
    event.preventDefault();
    history.back();
  } else if (event.button === MouseButton.FORWARD) {
    event.preventDefault();
    history.forward();
  }
};

export const startRouting = (): void => {
  const selected = recall(KEY_STORAGE_ACCOUNT);

  if (selected) {
    app.selectedAccount = selected;
  }

  if (location.pathname === PAGE_PATHS[Page.HOME]) {
    const page = recall(KEY_STORAGE_PAGE);

    if (isPage(page) && (!isAccountPage(page) || selected)) {
      app.page = page;
    }
  } else {
    followAddress();
  }

  app.tab = addressTab();
  window.addEventListener("popstate", followAddress);
  window.addEventListener("hashchange", () => (app.tab = addressTab()));

  if (isTauri()) {
    window.addEventListener("mouseup", followMouseButtons);
  }
};
