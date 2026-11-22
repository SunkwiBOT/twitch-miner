import { KEY_STORAGE_SIDEBAR_RAIL } from "./storage.constants";
import { recall, remember } from "./storage.utils";

const RAIL_ON = "1";
const RAIL_OFF = "0";

type SidebarStateType = { rail: boolean };

export const sidebar = $state<SidebarStateType>({
  rail: recall(KEY_STORAGE_SIDEBAR_RAIL) === RAIL_ON,
});

export const toggleSidebarRail = (): void => {
  sidebar.rail = !sidebar.rail;
  remember(KEY_STORAGE_SIDEBAR_RAIL, sidebar.rail ? RAIL_ON : RAIL_OFF);
};
