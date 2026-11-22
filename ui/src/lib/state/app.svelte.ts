import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { AppInfoType } from "../bindings/AppInfoType";
import type { AuthStatusType } from "../bindings/AuthStatusType";
import type { MinerEventType } from "../bindings/MinerEventType";
import type { SnapshotType } from "../bindings/SnapshotType";
import type { StartupProblemType } from "../bindings/StartupProblemType";
import { Page } from "./state.constants";

// ! The item itself is never kept: a sheet names a campaign id or badge key, SheetHost looks the live one up.
// Each opening stores a new record, so a dialog ties its own state to the record, never to the live item.
export type SheetType = { key: string; accountId: string | null };

type AuthStateType = AuthStatusType & { checked: boolean };

type AppStateType = {
  snapshot: SnapshotType | null;
  connected: boolean;
  page: Page;
  selectedAccount: string | null;
  info: AppInfoType | null;
  startupProblem: StartupProblemType | null;
  auth: AuthStateType;
  loginDialog: boolean;
  closeDialog: boolean;
  feedbackDialog: boolean;
  campaignSheet: SheetType | null;
  badgeSheet: SheetType | null;
  navDrawer: boolean;
  liveEvents: MinerEventType[];
  logsAccount: string | null;
  statsCurveTarget: { accountId: string; channel: string; period: number } | null;
  tab: string;
  revealPassword: boolean;
};

export const app = $state<AppStateType>({
  snapshot: null,
  connected: false,
  page: Page.HOME,
  selectedAccount: null,
  info: null,
  startupProblem: null,
  auth: { authorized: false, public: false, checked: false },
  loginDialog: false,
  closeDialog: false,
  feedbackDialog: false,
  campaignSheet: null,
  badgeSheet: null,
  navDrawer: false,
  liveEvents: [],
  logsAccount: null,
  statsCurveTarget: null,
  tab: "",
  revealPassword: false,
});

export const account = (id: string | null): AccountSnapshotType | null =>
  app.snapshot?.accounts.find((a) => a.id === id) ?? null;
