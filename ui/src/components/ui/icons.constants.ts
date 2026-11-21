import ArrowDownIcon from "@lucide/svelte/icons/arrow-down";
import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
import ArrowUpIcon from "@lucide/svelte/icons/arrow-up";
import AtSignIcon from "@lucide/svelte/icons/at-sign";
import AwardIcon from "@lucide/svelte/icons/award";
import BellIcon from "@lucide/svelte/icons/bell";
import CalendarIcon from "@lucide/svelte/icons/calendar";
import ChartColumnIcon from "@lucide/svelte/icons/chart-column";
import CheckIcon from "@lucide/svelte/icons/check";
import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
import ChevronUpIcon from "@lucide/svelte/icons/chevron-up";
import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
import ClockIcon from "@lucide/svelte/icons/clock";
import CopyIcon from "@lucide/svelte/icons/copy";
import DownloadIcon from "@lucide/svelte/icons/download";
import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
import EyeIcon from "@lucide/svelte/icons/eye";
import EyeOffIcon from "@lucide/svelte/icons/eye-off";
import FileTextIcon from "@lucide/svelte/icons/file-text";
import FlameIcon from "@lucide/svelte/icons/flame";
import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
import GiftIcon from "@lucide/svelte/icons/gift";
import GlobeIcon from "@lucide/svelte/icons/globe";
import GripVerticalIcon from "@lucide/svelte/icons/grip-vertical";
import HouseIcon from "@lucide/svelte/icons/house";
import InfinityIcon from "@lucide/svelte/icons/infinity";
import InfoIcon from "@lucide/svelte/icons/info";
import LayoutDashboardIcon from "@lucide/svelte/icons/layout-dashboard";
import LayoutGridIcon from "@lucide/svelte/icons/layout-grid";
import LinkIcon from "@lucide/svelte/icons/link";
import ListOrderedIcon from "@lucide/svelte/icons/list-ordered";
import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
import LockIcon from "@lucide/svelte/icons/lock";
import LockOpenIcon from "@lucide/svelte/icons/lock-open";
import LogInIcon from "@lucide/svelte/icons/log-in";
import LogOutIcon from "@lucide/svelte/icons/log-out";
import MenuIcon from "@lucide/svelte/icons/menu";
import MinusIcon from "@lucide/svelte/icons/minus";
import MoonIcon from "@lucide/svelte/icons/moon";
import OctagonXIcon from "@lucide/svelte/icons/octagon-x";
import PackageIcon from "@lucide/svelte/icons/package";
import PauseIcon from "@lucide/svelte/icons/pause";
import PencilIcon from "@lucide/svelte/icons/pencil";
import PinIcon from "@lucide/svelte/icons/pin";
import PlayIcon from "@lucide/svelte/icons/play";
import PlusIcon from "@lucide/svelte/icons/plus";
import RadioIcon from "@lucide/svelte/icons/radio";
import RotateCcwClockIcon from "@lucide/svelte/icons/rotate-ccw-clock";
import RotateCwIcon from "@lucide/svelte/icons/rotate-cw";
import SearchIcon from "@lucide/svelte/icons/search";
import SettingsIcon from "@lucide/svelte/icons/settings";
import StarIcon from "@lucide/svelte/icons/star";
import SunIcon from "@lucide/svelte/icons/sun";
import TargetIcon from "@lucide/svelte/icons/target";
import TrashIcon from "@lucide/svelte/icons/trash";
import TriangleExclamationPointIcon from "@lucide/svelte/icons/triangle-exclamation-point";
import TrophyIcon from "@lucide/svelte/icons/trophy";
import TvIcon from "@lucide/svelte/icons/tv";
import UsersIcon from "@lucide/svelte/icons/users";
import WebhookIcon from "@lucide/svelte/icons/webhook";
import XIcon from "@lucide/svelte/icons/x";
import ZapIcon from "@lucide/svelte/icons/zap";

import { IconName } from "../../lib/icons.constants";

// ! Typed from one per-icon module: importing the type from the package root
// makes the type check load all ~4,500 icon declarations.
type LucideIconType = typeof ArrowDownIcon;

export type IconDrawingType = { viewBox: string; path: string };

export const ICONS: Record<IconName, LucideIconType | IconDrawingType> = {
  [IconName.DASHBOARD]: LayoutDashboardIcon,
  [IconName.USERS]: UsersIcon,
  [IconName.GIFT]: GiftIcon,
  [IconName.AWARD]: AwardIcon,
  [IconName.TV]: TvIcon,
  [IconName.HISTORY]: RotateCcwClockIcon,
  [IconName.SETTINGS]: SettingsIcon,
  [IconName.PLUS]: PlusIcon,
  [IconName.MINUS]: MinusIcon,
  [IconName.INFO]: InfoIcon,
  [IconName.X]: XIcon,
  [IconName.MENU]: MenuIcon,
  [IconName.REFRESH]: RotateCwIcon,
  [IconName.TRASH]: TrashIcon,
  [IconName.UP]: ChevronUpIcon,
  [IconName.DOWN]: ChevronDownIcon,
  [IconName.CHEVRONS_UP_DOWN]: ChevronsUpDownIcon,
  [IconName.EXTERNAL]: ExternalLinkIcon,
  [IconName.CHECK]: CheckIcon,
  [IconName.CIRCLE_CHECK]: CircleCheckIcon,
  [IconName.OCTAGON_X]: OctagonXIcon,
  [IconName.COPY]: CopyIcon,
  [IconName.PLAY]: PlayIcon,
  [IconName.PAUSE]: PauseIcon,
  [IconName.LINK]: LinkIcon,
  [IconName.ALERT]: TriangleExclamationPointIcon,
  [IconName.LOCK]: LockIcon,
  [IconName.SUN]: SunIcon,
  [IconName.MOON]: MoonIcon,
  [IconName.GLOBE]: GlobeIcon,
  [IconName.SEARCH]: SearchIcon,
  [IconName.BELL]: BellIcon,
  [IconName.LOGOUT]: LogOutIcon,
  [IconName.FILE]: FileTextIcon,
  [IconName.FOLDER]: FolderOpenIcon,
  [IconName.PIN]: PinIcon,
  [IconName.HOME]: HouseIcon,
  [IconName.UNLOCK]: LockOpenIcon,
  [IconName.CLOCK]: ClockIcon,
  [IconName.ARROW_UP]: ArrowUpIcon,
  [IconName.ARROW_DOWN]: ArrowDownIcon,
  [IconName.ARROW_LEFT]: ArrowLeftIcon,
  [IconName.GRIP]: GripVerticalIcon,
  [IconName.LOADER]: LoaderCircleIcon,
  [IconName.AT]: AtSignIcon,
  [IconName.PACKAGE]: PackageIcon,
  [IconName.TROPHY]: TrophyIcon,
  [IconName.TARGET]: TargetIcon,
  [IconName.ZAP]: ZapIcon,
  [IconName.INFINITY]: InfinityIcon,
  [IconName.STAR]: StarIcon,
  [IconName.STAR_FILLED]: StarIcon,
  [IconName.FLAME]: FlameIcon,
  [IconName.RADIO]: RadioIcon,
  [IconName.EYE_OFF]: EyeOffIcon,
  [IconName.EYE]: EyeIcon,
  [IconName.LOG_IN]: LogInIcon,
  [IconName.CALENDAR]: CalendarIcon,
  [IconName.GRID]: LayoutGridIcon,
  [IconName.EDIT]: PencilIcon,
  [IconName.WEBHOOK]: WebhookIcon,
  [IconName.IMPORT]: DownloadIcon,
  [IconName.LIST_ORDERED]: ListOrderedIcon,
  [IconName.CHART]: ChartColumnIcon,
  // ! Lucide has no brand logos and none of Twitch's own glyphs: these few are drawn here, filled.
  [IconName.CHANNEL_POINTS]: {
    viewBox: "0 0 20 20",
    path: "M10 6a4 4 0 0 1 4 4h-2a2 2 0 0 0-2-2V6zM2 10a8 8 0 1 1 16 0 8 8 0 0 1-16 0zm2 0a6 6 0 1 1 12 0 6 6 0 0 1-12 0z",
  },
  [IconName.VIEWERS]: {
    viewBox: "0 0 24 24",
    path: "M6 8a6 6 0 1 1 7.025 5.913l.012.036A3 3 0 0 0 15.883 16H17a4 4 0 0 1 4 4v2h-2v-2a2 2 0 0 0-2-2h-1.117A5 5 0 0 1 12 16.15 5 5 0 0 1 8.117 18H7a2 2 0 0 0-2 2v2H3v-2a4 4 0 0 1 4-4h1.117a3 3 0 0 0 2.846-2.051l.012-.036A6.002 6.002 0 0 1 6 8Zm6 4a4 4 0 1 1 0-8 4 4 0 0 1 0 8Z",
  },
  [IconName.GITHUB]: {
    viewBox: "0 0 24 24",
    path: "M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12",
  },
  [IconName.WATCH_STREAK]: {
    viewBox: "0 0 24 24",
    path: "M5.295 8.05 10 2l3 4 2-3 3.8 5.067a11 11 0 0 1 2.2 6.6A7.333 7.333 0 0 1 13.667 22h-3.405A7.262 7.262 0 0 1 3 14.738c0-2.423.807-4.776 2.295-6.688Zm7.801 1.411 2-3L17.2 9.267a9 9 0 0 1 1.8 5.4 5.334 5.334 0 0 1-4.826 5.31 3 3 0 0 0 .174-3.748L12 13l-2.348 3.229a3 3 0 0 0 .18 3.754A5.263 5.263 0 0 1 5 14.738c0-1.978.66-3.9 1.873-5.46l3.098-3.983 3.125 4.166Z",
  },
  [IconName.DISCORD]: {
    viewBox: "0 0 24 24",
    path: "M20.317 4.37a19.79 19.79 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.211.375-.445.865-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.618-1.25.077.077 0 0 0-.079-.037A19.74 19.74 0 0 0 3.677 4.37a.07.07 0 0 0-.032.028C.533 9.046-.32 13.58.099 18.058a.082.082 0 0 0 .031.056 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.1 14.1 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.1 13.1 0 0 1-1.872-.892.077.077 0 0 1-.008-.128c.126-.094.252-.192.372-.291a.074.074 0 0 1 .078-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.009c.12.099.246.198.373.292a.077.077 0 0 1-.007.128 12.3 12.3 0 0 1-1.873.891.077.077 0 0 0-.041.107c.36.698.772 1.363 1.225 1.993a.076.076 0 0 0 .084.029 19.84 19.84 0 0 0 6.002-3.03.077.077 0 0 0 .032-.055c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.029zM8.02 15.331c-1.183 0-2.157-1.086-2.157-2.419s.956-2.419 2.157-2.419c1.21 0 2.176 1.095 2.157 2.42 0 1.332-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.086-2.157-2.419s.955-2.419 2.157-2.419c1.21 0 2.176 1.095 2.157 2.42 0 1.332-.946 2.418-2.157 2.418z",
  },
  [IconName.TELEGRAM]: {
    viewBox: "0 0 24 24",
    path: "M11.944 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0a12 12 0 0 0-.056 0zm4.962 7.224c.1-.002.321.023.465.14a.506.506 0 0 1 .171.325c.016.093.036.306.02.472-.18 1.898-.962 6.502-1.36 8.627-.168.9-.499 1.201-.82 1.23-.696.065-1.225-.46-1.9-.902-1.056-.693-1.653-1.124-2.678-1.8-1.185-.78-.417-1.21.258-1.91.177-.184 3.247-2.977 3.307-3.23.007-.032.014-.15-.056-.212s-.174-.041-.249-.024c-.106.024-1.793 1.14-5.061 3.345-.48.33-.913.49-1.302.48-.428-.008-1.252-.241-1.865-.44-.752-.245-1.349-.374-1.297-.789.027-.216.325-.437.893-.663 3.498-1.524 5.83-2.529 6.998-3.014 3.332-1.386 4.025-1.627 4.476-1.635z",
  },
  [IconName.VERIFIED]: {
    viewBox: "0 0 16 16",
    path: "M12.5 3.5 8 2 3.5 3.5 2 8l1.5 4.5L8 14l4.5-1.5L14 8l-1.5-4.5ZM7 11l4.5-4.5L10 5 7 8 5.5 6.5 4 8l3 3Z",
  },
};

export const FILLED_ICONS: ReadonlySet<IconName> = new Set([IconName.STAR_FILLED]);
