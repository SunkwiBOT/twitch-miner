import { sendCommand } from "../api/client";
import { isTauri } from "../api/tauri";
import { ClockFormat } from "../bindings/ClockFormat";
import { CommandName } from "../bindings/CommandName";
import { DateFormat } from "../bindings/DateFormat";
import type { GlobalSettingsType } from "../bindings/GlobalSettingsType";
import type { SnapshotType } from "../bindings/SnapshotType";
import type { SpamPointsSettingsType } from "../bindings/SpamPointsSettingsType";
import { Theme } from "../bindings/Theme";
import { sameValue } from "../compare.utils";
import { applyLanguage, t } from "../i18n/i18n.svelte";
import { applyTimePreferences } from "../time/time.svelte";
import { app } from "./app.svelte";
import { refresh, reportFailure, run } from "./session";
import { ToastKind } from "./state.constants";
import { KEY_STORAGE_PREFERENCES } from "./storage.constants";
import { recall, remember } from "./storage.utils";
import { toast } from "./toasts";

type PreferencesType = { theme?: Theme; language?: string };

type ViewerPreferencesType = Partial<
  Pick<GlobalSettingsType, "language" | "theme" | "time_zone" | "date_format" | "clock_format">
>;

const pending: PreferencesType = {};

let pendingSpam: SpamPointsSettingsType | null = null;

let serverPreferences: ViewerPreferencesType = {};

const perViewer = (): boolean => !isTauri();

const isChoice = (choices: Record<string, string>, value: string | undefined): boolean =>
  value === undefined || Object.values(choices).includes(value);

const viewerPreferences = (): ViewerPreferencesType => {
  try {
    const saved = JSON.parse(recall(KEY_STORAGE_PREFERENCES) ?? "{}") as ViewerPreferencesType;

    if (!isChoice(Theme, saved.theme)) {
      delete saved.theme;
    }

    if (!isChoice(DateFormat, saved.date_format)) {
      delete saved.date_format;
    }

    if (!isChoice(ClockFormat, saved.clock_format)) {
      delete saved.clock_format;
    }

    return saved;
  } catch (e: any) {
    return {};
  }
};

const pickViewer = (settings: GlobalSettingsType): ViewerPreferencesType => ({
  language: settings.language,
  theme: settings.theme,
  time_zone: settings.time_zone,
  date_format: settings.date_format,
  clock_format: settings.clock_format,
});

const switchLanguage = (preference: string): void => {
  void applyLanguage(preference).then((shown) => {
    if (!shown) {
      toast(ToastKind.ERROR, t.language_load_failed());
    }
  });
};

const applyTheme = (theme: Theme): void => {
  const root = document.documentElement;

  root.dataset.theme = theme;
  document
    .querySelector('meta[name="theme-color"]')
    ?.setAttribute("content", getComputedStyle(root).getPropertyValue("--background").trim());
};

export const preferencesPending = (): boolean =>
  pending.theme !== undefined || pending.language !== undefined || pendingSpam !== null;

export const keepPendingPreferences = (snapshot: SnapshotType): void => {
  if (perViewer()) {
    serverPreferences = pickViewer(snapshot.settings);
    Object.assign(snapshot.settings, viewerPreferences());
  }

  if (pending.theme !== undefined) {
    if (snapshot.settings.theme === pending.theme) {
      delete pending.theme;
    } else {
      snapshot.settings.theme = pending.theme;
    }
  }

  if (pending.language !== undefined) {
    if (snapshot.settings.language === pending.language) {
      delete pending.language;
    } else {
      snapshot.settings.language = pending.language;
    }
  }

  if (pendingSpam) {
    if (sameValue(snapshot.settings.spam_points, pendingSpam)) {
      pendingSpam = null;
    } else {
      snapshot.settings.spam_points = pendingSpam;
    }
  }
};

export const showPreferences = (settings: GlobalSettingsType): void => {
  switchLanguage(settings.language);
  applyTimePreferences(settings);
  applyTheme(settings.theme);
};

export const saveGlobalSettings = (settings: GlobalSettingsType): Promise<boolean> => {
  if (!perViewer()) {
    return run({ command: CommandName.SET_GLOBAL_SETTINGS, settings }, t.common_saved());
  }

  remember(KEY_STORAGE_PREFERENCES, JSON.stringify(pickViewer(settings)));
  showPreferences(settings);

  if (app.snapshot) {
    app.snapshot.settings = settings;
  }

  return run(
    { command: CommandName.SET_GLOBAL_SETTINGS, settings: { ...settings, ...serverPreferences } },
    t.common_saved(),
  );
};

export const setPreference = async (change: PreferencesType): Promise<void> => {
  if (!app.snapshot) {
    return;
  }

  const settings = { ...$state.snapshot(app.snapshot.settings), ...change };

  if (perViewer()) {
    remember(KEY_STORAGE_PREFERENCES, JSON.stringify({ ...viewerPreferences(), ...change }));

    if (change.theme !== undefined) {
      applyTheme(change.theme);
    }

    if (change.language !== undefined) {
      switchLanguage(change.language);
    }

    app.snapshot.settings = settings;

    return;
  }

  if (change.theme !== undefined) {
    pending.theme = change.theme;
    applyTheme(change.theme);
  }

  if (change.language !== undefined) {
    pending.language = change.language;
    switchLanguage(change.language);
  }

  app.snapshot.settings = settings;

  try {
    await sendCommand({ command: CommandName.SET_GLOBAL_SETTINGS, settings });
  } catch (e: any) {
    delete pending.theme;
    delete pending.language;
    reportFailure(e);
  }
};

export const saveSpamSettings = async (spam: SpamPointsSettingsType): Promise<void> => {
  if (!app.snapshot) {
    return;
  }

  pendingSpam = spam;
  app.snapshot.settings.spam_points = spam;

  const settings = $state.snapshot(app.snapshot.settings);

  try {
    await sendCommand({
      command: CommandName.SET_GLOBAL_SETTINGS,
      settings: perViewer() ? { ...settings, ...serverPreferences } : settings,
    });
  } catch (e: any) {
    pendingSpam = null;

    if (reportFailure(e)) {
      await refresh();
    }
  }
};

export const toggleTheme = (): Promise<void> =>
  setPreference({ theme: app.snapshot?.settings.theme === Theme.LIGHT ? Theme.DARK : Theme.LIGHT });

export const applySavedTheme = (): void => {
  const theme = perViewer() ? viewerPreferences().theme : undefined;

  if (theme) {
    applyTheme(theme);
  }
};
