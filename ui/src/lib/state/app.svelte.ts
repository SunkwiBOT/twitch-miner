import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { AppInfoType } from "../bindings/AppInfoType";
import type { AuthStatusType } from "../bindings/AuthStatusType";
import type { CampaignType } from "../bindings/CampaignType";
import type { GlobalBadgeType } from "../bindings/GlobalBadgeType";
import type { MinerEventType } from "../bindings/MinerEventType";
import type { SnapshotType } from "../bindings/SnapshotType";
import type { StartupProblemType } from "../bindings/StartupProblemType";
import { Page } from "./state.constants";

type SheetType<T> = { item: T; accountId: string | null };

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
  campaignSheet: SheetType<CampaignType> | null;
  badgeSheet: SheetType<GlobalBadgeType> | null;
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
