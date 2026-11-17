import { reportActivity } from "../api/client";
import type { AutoLock } from "../bindings/AutoLock";
import { MINUTE_MS } from "../time/time.constants";
import { warningLeadMs } from "./autolock.utils";
import {
  ACTIVITY_REPORT_GAP_MS,
  AUTO_LOCK_MINUTES,
  LOCK_CHECK_DELAY_MS,
  LOCK_CHECK_TIMEOUT_MS,
} from "./state.constants";
import { checkSession, sessionEnded, sessionEndedBy } from "./session";
import { remember } from "./storage.utils";
import { dismissLockWarning, showLockWarning } from "./toasts";

type WatchedType = { open: boolean; idleMs: number | null };

type TimerType = ReturnType<typeof setTimeout>;

let watched: WatchedType = { open: false, idleMs: null };
let reportedAt = Number.NEGATIVE_INFINITY;
let heardAt = performance.now();
let pendingReport: TimerType | null = null;
let lockTimer: TimerType | null = null;
let warnTimer: TimerType | null = null;
let warningShown = false;

const stopLockTimer = (): void => {
  if (lockTimer) {
    clearTimeout(lockTimer);
    lockTimer = null;
  }
};

const stopWarning = (): void => {
  if (warnTimer) {
    clearTimeout(warnTimer);
    warnTimer = null;
  }

  if (warningShown) {
    warningShown = false;
    dismissLockWarning();
  }
};

// ! Skipped while a report is already due: that report arms a fresh
// deadline, so showing the warning now would only flash and vanish.
const armWarning = (realDeadline: number): void => {
  const lead = warningLeadMs(watched.idleMs);

  warnTimer = setTimeout(
    () => {
      warnTimer = null;

      if (pendingReport) {
        return;
      }

      warningShown = true;
      showLockWarning(realDeadline);
    },
    Math.max(0, realDeadline - lead - performance.now()),
  );
};

const armLock = (from: number): void => {
  stopLockTimer();
  stopWarning();

  if (!watched.open || watched.idleMs === null) {
    return;
  }

  const realDeadline = from + watched.idleMs;

  lockTimer = setTimeout(
    () => void lockIfIdle(),
    Math.max(0, realDeadline + LOCK_CHECK_DELAY_MS - performance.now()),
  );
  armWarning(realDeadline);
};

// ! No server answer within LOCK_CHECK_TIMEOUT_MS: nothing says the session was in use, so the page locks.
const lockIfIdle = async (): Promise<void> => {
  lockTimer = null;

  let timeout: TimerType | undefined;
  const answered = await Promise.race([
    checkSession(),
    new Promise<boolean>((resolve) => {
      timeout = setTimeout(() => resolve(false), LOCK_CHECK_TIMEOUT_MS);
    }),
  ]);

  clearTimeout(timeout);

  if (!answered) {
    sessionEnded();

    return;
  }

  armLock(performance.now());
};

const LAST_HEARD_STORAGE_KEY = "autolock:lastHeardAt";

const toPerformanceNow = (epochMs: number): number => performance.now() - (Date.now() - epochMs);

// ! Another tab's own activity keeps the shared server session alive: a
// tab with no local interaction must re-arm from that tab's last report
// instead of warning of a lock that the server will never apply.
const onStorage = (event: StorageEvent): void => {
  if (event.key !== LAST_HEARD_STORAGE_KEY || event.newValue === null) {
    return;
  }

  const from = toPerformanceNow(Number(event.newValue));

  if (!Number.isFinite(from) || from <= heardAt) {
    return;
  }

  heardAt = from;
  armLock(from);
};

if (typeof window !== "undefined") {
  window.addEventListener("storage", onStorage);
}

const report = async (): Promise<void> => {
  pendingReport = null;
  reportedAt = performance.now();
  heardAt = reportedAt;
  armLock(heardAt);

  remember(LAST_HEARD_STORAGE_KEY, String(Date.now()));

  try {
    await reportActivity();
  } catch (e: any) {
    sessionEndedBy(e);
  }
};

// ! Reported at the end of ACTIVITY_REPORT_GAP_MS: the server never takes the user for idle too early.
// While the warning toast is up, a report is sent at once instead: the user
// is already down to the last minute, with no gap left to spare.
export const noteActivity = (): void => {
  if (!watched.open) {
    return;
  }

  if (warningShown) {
    if (pendingReport) {
      clearTimeout(pendingReport);
      pendingReport = null;
    }

    void report();

    return;
  }

  if (pendingReport) {
    return;
  }

  const wait = reportedAt + ACTIVITY_REPORT_GAP_MS - performance.now();

  if (wait <= 0) {
    void report();
  } else {
    pendingReport = setTimeout(() => void report(), wait);
  }
};

export const noteReturn = (): void => {
  if (!watched.open || document.visibilityState !== "visible") {
    return;
  }

  if (pendingReport) {
    clearTimeout(pendingReport);
  }

  void report();
};

export const watchSession = (open: boolean, setting: AutoLock | null): void => {
  const minutes = setting === null ? null : AUTO_LOCK_MINUTES[setting];
  const idleMs = minutes === null ? null : minutes * MINUTE_MS;

  if (open === watched.open && idleMs === watched.idleMs) {
    return;
  }

  if (open && !watched.open) {
    heardAt = performance.now();
  }

  watched = { open, idleMs };

  if (!open && pendingReport) {
    clearTimeout(pendingReport);
    pendingReport = null;
  }

  armLock(heardAt);
};
