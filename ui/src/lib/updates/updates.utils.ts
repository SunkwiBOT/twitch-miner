import { locale } from "../i18n/i18n.svelte";
import { BYTES_PER_MEGABYTE, BrowserPermission, MEGABYTE_UNIT } from "./updates.constants";

type UpdateBannerInputType = {
  desktop: boolean;
  permission: BrowserPermission | null;
  enabled: boolean;
  dismissed: boolean;
};

// ! Only a browser still undecided is asked: a blocked one needs its site settings, the Settings page says so.
export const updateBannerShown = ({
  desktop,
  permission,
  enabled,
  dismissed,
}: UpdateBannerInputType): boolean =>
  !desktop && enabled && !dismissed && permission === BrowserPermission.DEFAULT;

export const megabytes = (bytes: number): string =>
  new Intl.NumberFormat(locale.current, {
    style: "unit",
    unit: MEGABYTE_UNIT,
    maximumFractionDigits: 1,
  }).format(bytes / BYTES_PER_MEGABYTE);
