import { DROPS_WEB_URL } from "../partners.constants";

// ! Must match drops.sunkwi.com's badge addresses: only the id slug is guaranteed stable.

const BADGE_SLUG_FALLBACK_PREFIX = "badge";
const DEFAULT_VERSION_ID = "1";

const FNV_OFFSET_BASIS = 0x81_1c_9d_c5;
const FNV_PRIME = 0x01_00_01_93;
const HASH_HEX_DIGITS = 8;
const HEX_RADIX = 16;

const COMBINING_MARKS = /\p{M}/gu;
const NON_ALPHANUMERIC = /[^a-z0-9]+/g;
const EDGE_DASHES = /^-+|-+$/g;
const MAX_SLUG_LENGTH = 80;

// ! Lower-case letters Unicode does not split into a base letter and an accent.
const LETTER_SPELLINGS: Readonly<Record<string, string>> = {
  ß: "ss",
  æ: "ae",
  ð: "d",
  ø: "o",
  þ: "th",
  đ: "d",
  ł: "l",
  œ: "oe",
};
const SPELLED_LETTERS = /[æðđłøœßþ]/g;

const foldText = (text: string): string =>
  text
    .normalize("NFKD")
    .replace(COMBINING_MARKS, "")
    .toLowerCase()
    .replace(SPELLED_LETTERS, (letter) => LETTER_SPELLINGS[letter] ?? letter);

const toSlugBase = (name: string): string =>
  foldText(name)
    .replaceAll("&", " and ")
    .replace(NON_ALPHANUMERIC, "-")
    .replace(EDGE_DASHES, "")
    .slice(0, MAX_SLUG_LENGTH)
    .replace(EDGE_DASHES, "");

const hashText = (text: string): string => {
  let hash = FNV_OFFSET_BASIS;

  for (const byte of new TextEncoder().encode(text)) {
    hash = Math.imul(hash ^ byte, FNV_PRIME) >>> 0;
  }

  return hash.toString(HEX_RADIX).padStart(HASH_HEX_DIGITS, "0");
};

const toSetSlug = (setId: string): string => toSlugBase(setId) || BADGE_SLUG_FALLBACK_PREFIX;

const toBadgeSlug = (setId: string, versionId: string): string => {
  const version = toSlugBase(versionId);

  return versionId === DEFAULT_VERSION_ID || version === ""
    ? toSetSlug(setId)
    : `${toSetSlug(setId)}-${version}`;
};

const toBadgeIdSlug = (setId: string, versionId: string): string =>
  `${toBadgeSlug(setId, versionId)}-${hashText(`${setId}/${versionId}`)}`;

export const badgeUrl = (setId: string, versionId: string): string =>
  `${DROPS_WEB_URL}/badges/${toBadgeIdSlug(setId, versionId)}`;
