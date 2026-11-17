import type { TokenCheckType } from "../bindings/TokenCheckType";

export enum ImportLineStatus {
  PENDING = "pending",
  CHECKING = "checking",
  VALID = "valid",
  ALREADY_ADDED = "already_added",
  DUPLICATE = "duplicate",
  INVALID = "invalid",
  UNUSABLE_CLIENT = "unusable_client",
  OTHER_ERROR = "other_error",
  NOT_IMPORTED = "not_imported",
}

export enum ImportStep {
  LIST = "list",
  SETTINGS = "settings",
  RESULT = "result",
}

export type ParsedTokenLineType = {
  token: string;
  status: ImportLineStatus.PENDING | ImportLineStatus.DUPLICATE;
};

export const parseTokenLines = (value: string): string[] =>
  value
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line !== "");

export const buildTokenLines = (tokens: string[]): ParsedTokenLineType[] => {
  const seen = new Set<string>();

  // ! Only an exact duplicate paste is caught here, before any check
  // request: two tokens that only differ by an `oauth:`/`OAuth ` prefix
  // or casing still each get checked, and `dedupeByAccount` below catches
  // them once the backend's own `parse_pasted_token` has resolved both to
  // the same account — no hand-mirrored copy of that parser here.
  return tokens.map((token) => {
    const key = token.trim();

    if (seen.has(key)) {
      return { token, status: ImportLineStatus.DUPLICATE };
    }

    seen.add(key);

    return { token, status: ImportLineStatus.PENDING };
  });
};

// ! A later line resolving to an account already seen among earlier,
// already-checked lines is marked a duplicate too: the raw-string dedupe
// above cannot catch two different tokens of the same account.
export const dedupeByAccount = <T extends { check: TokenCheckType | null }>(
  lines: (T & { status: ImportLineStatus })[],
): (T & { status: ImportLineStatus })[] => {
  const seen = new Set<string>();

  return lines.map((line) => {
    if (!line.check) {
      return line;
    }

    if (seen.has(line.check.login)) {
      return { ...line, status: ImportLineStatus.DUPLICATE };
    }

    seen.add(line.check.login);

    return line;
  });
};

// ! Never show a pasted token in full: a screenshot or a shoulder-surfed
// screen must not leak it. Scales down for a short input so at least one
// character on each side stays hidden.
export const maskToken = (token: string): string => {
  const visible = Math.min(4, Math.floor(token.length / 3));

  return visible < 1 ? "…" : `${token.slice(0, visible)}…${token.slice(-visible)}`;
};
