import { ServerMessageKind } from "../bindings/ServerMessageKind";
import { openUrl as tauriOpenUrl } from "@tauri-apps/plugin-opener";

import type { AppInfoType } from "../bindings/AppInfoType";
import type { AuthResponseType } from "../bindings/AuthResponseType";
import type { AuthStatusType } from "../bindings/AuthStatusType";
import type { BurstProgressType } from "../bindings/BurstProgressType";
import type { RefreshCountType } from "../bindings/RefreshCountType";
import type { ChannelQueryType } from "../bindings/ChannelQueryType";
import type { ChannelsLookupRequestType } from "../bindings/ChannelsLookupRequestType";
import type { ChannelsLookupResponseType } from "../bindings/ChannelsLookupResponseType";
import type { ChannelType } from "../bindings/ChannelType";
import type { CloseAction } from "../bindings/CloseAction";
import type { CommandType } from "../bindings/CommandType";
import type { DashboardLoginRequestType } from "../bindings/DashboardLoginRequestType";
import { DesktopEvent } from "../bindings/DesktopEvent";
import type { FollowsQueryType } from "../bindings/FollowsQueryType";
import type { HistoryQueryType } from "../bindings/HistoryQueryType";
import type { HistoryResponseType } from "../bindings/HistoryResponseType";
import type { LoginFlowViewType } from "../bindings/LoginFlowViewType";
import type { LogsQueryType } from "../bindings/LogsQueryType";
import type { BadgesResponseType } from "../bindings/BadgesResponseType";
import type { CatalogResponseType } from "../bindings/CatalogResponseType";
import type { LogsResponseType } from "../bindings/LogsResponseType";
import type { MinerEventType } from "../bindings/MinerEventType";
import type { NotificationTestType } from "../bindings/NotificationTestType";
import type { ProxyTestRequestType } from "../bindings/ProxyTestRequestType";
import type { RedeemRequestType } from "../bindings/RedeemRequestType";
import type { RequestsQueryType } from "../bindings/RequestsQueryType";
import type { RequestsResponseType } from "../bindings/RequestsResponseType";
import type { SearchQueryType } from "../bindings/SearchQueryType";
import type { SearchResponseType } from "../bindings/SearchResponseType";
import type { ChannelBannersRequestType } from "../bindings/ChannelBannersRequestType";
import { RoutesApi } from "../bindings/RoutesApi";
import type { ServerMessageType } from "../bindings/ServerMessageType";
import type { SnapshotType } from "../bindings/SnapshotType";
import type { SpamBalanceType } from "../bindings/SpamBalanceType";
import type { SpamChannelQueryType } from "../bindings/SpamChannelQueryType";
import type { SpamChannelViewType } from "../bindings/SpamChannelViewType";
import type { StartupProblemType } from "../bindings/StartupProblemType";
import type { UpdateCheckResponseType } from "../bindings/UpdateCheckResponseType";
import {
  HttpMethod,
  RECONNECT_DELAY_MAX_MS,
  RECONNECT_DELAY_MIN_MS,
  TauriCommand,
  UpdateEvent,
} from "./api.constants";
import { http } from "./http";
import { invoke, isTauri, listen, listenUpdate } from "./tauri";
import type {
  ArgumentsType,
  TauriCommandsType,
  FailedUpdateType,
  UnsubscribeType,
  UpdateInfoType,
  UpdateProgressType,
} from "./tauri";

type ListenersType = {
  onSnapshot: (snapshot: SnapshotType) => void;
  onEvent: (event: MinerEventType) => void;
  onBurst: (progress: BurstProgressType) => void;
  onConnection: (connected: boolean) => void;
};

type HttpRouteType = { route: RoutesApi; method: HttpMethod };

type HttpBodyType = { params?: object; data?: unknown };

const ROUTES = {
  [TauriCommand.SNAPSHOT]: { route: RoutesApi.SNAPSHOT, method: HttpMethod.GET },
  [TauriCommand.COMMAND]: { route: RoutesApi.COMMAND, method: HttpMethod.POST },
  [TauriCommand.REFRESH_CATALOG]: { route: RoutesApi.CATALOG_REFRESH, method: HttpMethod.POST },
  [TauriCommand.REFRESH_BADGES]: { route: RoutesApi.BADGES_REFRESH, method: HttpMethod.POST },
  [TauriCommand.START_LOGIN]: { route: RoutesApi.LOGIN_FLOW, method: HttpMethod.POST },
  [TauriCommand.LOOKUP_CHANNEL]: { route: RoutesApi.CHANNEL, method: HttpMethod.GET },
  [TauriCommand.CHANNEL_BANNERS]: { route: RoutesApi.CHANNELS_BANNERS, method: HttpMethod.POST },
  [TauriCommand.SEARCH]: { route: RoutesApi.SEARCH, method: HttpMethod.GET },
  [TauriCommand.LOOKUP_CHANNELS]: { route: RoutesApi.CHANNELS_LOOKUP, method: HttpMethod.POST },
  [TauriCommand.FOLLOWS]: { route: RoutesApi.FOLLOWS, method: HttpMethod.GET },
  [TauriCommand.SPAM_CHANNEL]: { route: RoutesApi.SPAM_CHANNEL, method: HttpMethod.GET },
  [TauriCommand.SPAM_BALANCE]: { route: RoutesApi.SPAM_BALANCE, method: HttpMethod.GET },
  [TauriCommand.REDEEM]: { route: RoutesApi.SPAM_REDEEM, method: HttpMethod.POST },
  [TauriCommand.TEST_NOTIFICATIONS]: {
    route: RoutesApi.NOTIFICATIONS_TEST,
    method: HttpMethod.POST,
  },
  [TauriCommand.TEST_PROXY]: { route: RoutesApi.PROXY_TEST, method: HttpMethod.POST },
  [TauriCommand.HISTORY]: { route: RoutesApi.HISTORY, method: HttpMethod.GET },
  [TauriCommand.BADGES]: { route: RoutesApi.BADGES, method: HttpMethod.GET },
  [TauriCommand.CATALOG]: { route: RoutesApi.CATALOG, method: HttpMethod.GET },
  [TauriCommand.LOGS]: { route: RoutesApi.LOGS, method: HttpMethod.GET },
  [TauriCommand.LOGS_REPORT]: { route: RoutesApi.LOGS_REPORT, method: HttpMethod.GET },
  [TauriCommand.REQUESTS]: { route: RoutesApi.REQUESTS, method: HttpMethod.GET },
  [TauriCommand.REQUESTS_REPORT]: { route: RoutesApi.REQUESTS_REPORT, method: HttpMethod.GET },
} satisfies Partial<Record<TauriCommand, HttpRouteType>>;

type RoutedCommandType = keyof typeof ROUTES;

const transport = async <K extends RoutedCommandType>(
  command: K,
  request: HttpBodyType,
  ...args: ArgumentsType<K>
): Promise<TauriCommandsType[K]["result"]> => {
  if (isTauri()) {
    return invoke(command, ...args);
  }

  const { route, method } = ROUTES[command];
  const response = await http<TauriCommandsType[K]["result"]>(route, { method, ...request });

  return response.data;
};

export const fetchSnapshot = async (): Promise<SnapshotType> =>
  (await transport(TauriCommand.SNAPSHOT, {})).snapshot;

export const sendCommand = async (command: CommandType): Promise<void> => {
  await transport(TauriCommand.COMMAND, { data: command }, { command });
};

export const requestCatalogRefresh = async (): Promise<RefreshCountType> =>
  (await transport(TauriCommand.REFRESH_CATALOG, {})).refresh;

export const requestBadgesRefresh = async (): Promise<RefreshCountType> =>
  (await transport(TauriCommand.REFRESH_BADGES, {})).refresh;

export const startLogin = async (): Promise<LoginFlowViewType> =>
  (await transport(TauriCommand.START_LOGIN, {})).login;

export const lookupChannel = async (login: string): Promise<ChannelType> =>
  (
    await transport(
      TauriCommand.LOOKUP_CHANNEL,
      { params: { login } satisfies ChannelQueryType },
      { login },
    )
  ).channel;

export const fetchBanners = async (logins: string[]): Promise<Record<string, string>> =>
  (
    await transport(
      TauriCommand.CHANNEL_BANNERS,
      { data: { logins } satisfies ChannelBannersRequestType },
      { logins },
    )
  ).banners;

export const searchTwitch = async (query: string): Promise<SearchResponseType> =>
  transport(TauriCommand.SEARCH, { params: { query } satisfies SearchQueryType }, { query });

export const lookupChannels = async (logins: string[]): Promise<ChannelsLookupResponseType> =>
  transport(
    TauriCommand.LOOKUP_CHANNELS,
    { data: { logins } satisfies ChannelsLookupRequestType },
    { logins },
  );

export const fetchFollows = async (accountId: string): Promise<ChannelType[]> =>
  (
    await transport(
      TauriCommand.FOLLOWS,
      { params: { account_id: accountId } satisfies FollowsQueryType },
      { accountId },
    )
  ).channels;

export const fetchSpamChannel = async (
  accountId: string,
  login: string,
): Promise<SpamChannelViewType> =>
  (
    await transport(
      TauriCommand.SPAM_CHANNEL,
      { params: { account_id: accountId, login } satisfies SpamChannelQueryType },
      { accountId, login },
    )
  ).channel;

export const fetchSpamBalance = async (
  accountId: string,
  login: string,
): Promise<SpamBalanceType> =>
  (
    await transport(
      TauriCommand.SPAM_BALANCE,
      { params: { account_id: accountId, login } satisfies SpamChannelQueryType },
      { accountId, login },
    )
  ).balance;

export const redeemRewards = async (request: RedeemRequestType): Promise<string> =>
  (await transport(TauriCommand.REDEEM, { data: request }, { request })).burst_id;

export const testNotifications = async (): Promise<NotificationTestType[]> =>
  (await transport(TauriCommand.TEST_NOTIFICATIONS, {})).targets;

export const testProxy = async (url: string): Promise<number> =>
  (
    await transport(
      TauriCommand.TEST_PROXY,
      { data: { url } satisfies ProxyTestRequestType },
      { url },
    )
  ).latency_ms;

export const fetchHistory = async (query: HistoryQueryType): Promise<HistoryResponseType> =>
  transport(TauriCommand.HISTORY, { params: query }, { query });

export const fetchBadges = async (): Promise<BadgesResponseType> =>
  transport(TauriCommand.BADGES, {});

export const fetchCatalog = async (): Promise<CatalogResponseType> =>
  transport(TauriCommand.CATALOG, {});

export const fetchLogs = async (query: LogsQueryType): Promise<LogsResponseType> =>
  transport(TauriCommand.LOGS, { params: query }, { query });

export const fetchLogsReport = async (query: LogsQueryType): Promise<string> =>
  (await transport(TauriCommand.LOGS_REPORT, { params: query }, { query })).report;

export const fetchRequests = async (query: RequestsQueryType): Promise<RequestsResponseType> =>
  transport(TauriCommand.REQUESTS, { params: query }, { query });

export const fetchRequestsReport = async (query: RequestsQueryType): Promise<string> =>
  (await transport(TauriCommand.REQUESTS_REPORT, { params: query }, { query })).report;

const isServerMessage = (value: unknown): value is ServerMessageType =>
  typeof value === "object" &&
  value !== null &&
  "type" in value &&
  ((value.type === ServerMessageKind.SNAPSHOT && "snapshot" in value) ||
    (value.type === ServerMessageKind.EVENT && "event" in value) ||
    (value.type === ServerMessageKind.BURST && "progress" in value));

const parseServerMessage = (raw: unknown): ServerMessageType | null => {
  try {
    const value: unknown = JSON.parse(String(raw));

    return isServerMessage(value) ? value : null;
  } catch (e: any) {
    console.warn("dropped a malformed dashboard frame", e);

    return null;
  }
};

const subscribeDesktop = async (listeners: ListenersType): Promise<UnsubscribeType> => {
  const stopSnapshot = await listen(DesktopEvent.SNAPSHOT, listeners.onSnapshot);
  const stopEvent = await listen(DesktopEvent.MINER_EVENT, listeners.onEvent);
  const stopBurst = await listen(DesktopEvent.BURST_PROGRESS, listeners.onBurst);

  listeners.onConnection(true);

  return () => {
    stopSnapshot();
    stopEvent();
    stopBurst();
  };
};

const subscribeDashboard = (listeners: ListenersType): UnsubscribeType => {
  let socket: WebSocket | null = null;
  let retry: ReturnType<typeof setTimeout> | null = null;
  let closed = false;
  let delay = RECONNECT_DELAY_MIN_MS;

  const connect = (): void => {
    const protocol = location.protocol === "https:" ? "wss" : "ws";
    const current = new WebSocket(`${protocol}://${location.host}${RoutesApi.EVENTS}`);

    socket = current;
    current.onopen = () => {
      delay = RECONNECT_DELAY_MIN_MS;
      listeners.onConnection(true);
    };
    current.onmessage = (frame) => {
      const message = parseServerMessage(frame.data);

      if (message?.type === ServerMessageKind.SNAPSHOT) {
        listeners.onSnapshot(message.snapshot);
      } else if (message?.type === ServerMessageKind.EVENT) {
        listeners.onEvent(message.event);
      } else if (message?.type === ServerMessageKind.BURST) {
        listeners.onBurst(message.progress);
      }
    };
    current.onclose = () => {
      listeners.onConnection(false);

      if (closed) {
        return;
      }

      retry = setTimeout(connect, delay);
      delay = Math.min(delay * 2, RECONNECT_DELAY_MAX_MS);
    };
    current.onerror = () => current.close();
  };

  connect();

  return () => {
    closed = true;

    if (retry) {
      clearTimeout(retry);
    }

    socket?.close();
  };
};

export const subscribe = async (listeners: ListenersType): Promise<UnsubscribeType> =>
  isTauri() ? subscribeDesktop(listeners) : subscribeDashboard(listeners);

export const fetchAppInfo = async (): Promise<AppInfoType | null> => {
  if (!isTauri()) {
    return null;
  }

  return (await invoke(TauriCommand.APP_INFO)).app;
};

export const fetchStartupProblem = async (): Promise<StartupProblemType | null> =>
  isTauri() ? ((await invoke(TauriCommand.STARTUP_PROBLEM)).problem ?? null) : null;

export const quitApp = async (): Promise<void> => {
  await invoke(TauriCommand.QUIT);
};

export const fetchAutostart = async (): Promise<boolean> =>
  isTauri() ? (await invoke(TauriCommand.AUTOSTART_STATUS)).enabled : false;

export const setAutostart = async (enabled: boolean): Promise<void> => {
  await invoke(TauriCommand.SET_AUTOSTART, { enabled });
};

export const closeWindow = async (action: CloseAction): Promise<void> => {
  await invoke(TauriCommand.CLOSE_WINDOW, { action });
};

export const onCloseRequested = async (callback: () => void): Promise<UnsubscribeType | null> =>
  isTauri() ? listen(DesktopEvent.CLOSE_REQUESTED, callback) : null;

export const openLogsDirectory = async (): Promise<void> => {
  await invoke(TauriCommand.OPEN_LOGS_DIRECTORY);
};

export const openDataDirectory = async (): Promise<void> => {
  await invoke(TauriCommand.OPEN_DATA_DIRECTORY);
};

export const checkForUpdate = async (): Promise<string | null> => {
  if (isTauri()) {
    return (await invoke(TauriCommand.CHECK_FOR_UPDATE))?.version ?? null;
  }

  const response = await http<UpdateCheckResponseType>(RoutesApi.UPDATES_CHECK, {
    method: HttpMethod.POST,
  });

  return response.data.check.available ?? null;
};

export const fetchFoundUpdate = async (): Promise<UpdateInfoType | null> =>
  invoke(TauriCommand.FOUND_UPDATE);

export const takeFailedUpdate = async (): Promise<FailedUpdateType | null> =>
  invoke(TauriCommand.TAKE_FAILED_UPDATE);

export const onUpdateAvailable = (
  callback: (found: UpdateInfoType) => void,
): Promise<UnsubscribeType> => listenUpdate(UpdateEvent.AVAILABLE, callback);

export const downloadUpdate = async (): Promise<string> => invoke(TauriCommand.DOWNLOAD_UPDATE);

export const applyUpdate = async (version: string): Promise<void> => {
  await invoke(TauriCommand.APPLY_UPDATE, { version });
};

export const onUpdateProgress = (
  callback: (progress: UpdateProgressType) => void,
): Promise<UnsubscribeType> => listenUpdate(UpdateEvent.PROGRESS, callback);

export const openUrl = async (url: string): Promise<void> => {
  if (isTauri()) {
    await tauriOpenUrl(url);

    return;
  }

  window.open(url, "_blank", "noopener,noreferrer");
};

export const fetchAuthStatus = async (): Promise<AuthStatusType> => {
  if (isTauri()) {
    return { authorized: true, public: false };
  }

  const response = await http<AuthResponseType>(RoutesApi.AUTH, { method: HttpMethod.GET });

  return response.data.auth;
};

export const signIn = async (password: string): Promise<void> => {
  await http<null>(RoutesApi.AUTH, {
    method: HttpMethod.POST,
    data: { password } satisfies DashboardLoginRequestType,
  });
};

export const signOut = async (): Promise<void> => {
  await http<null>(RoutesApi.AUTH, { method: HttpMethod.DELETE });
};

export const reportActivity = async (): Promise<void> => {
  await http<null>(RoutesApi.ACTIVITY, { method: HttpMethod.POST });
};
