import type { BinaryAssetType } from "../bindings/BinaryAssetType";
import { locale } from "../i18n/i18n.svelte";
import {
  BINARY_PATH,
  BINARY_SERVICE,
  BYTES_PER_MEGABYTE,
  BrowserPermission,
  MEGABYTE_UNIT,
} from "./updates.constants";

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

// ! The exact file for this server's own processor, from `binary_asset` in the snapshot.
export const binaryUpdateCommands = (version: string, asset: BinaryAssetType): string[] => [
  `sudo install -m 755 ${asset.prefix}${version}_${asset.arch} ${BINARY_PATH}`,
  `sudo systemctl restart ${BINARY_SERVICE}`,
];
