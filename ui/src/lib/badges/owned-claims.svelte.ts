import { fetchHistory } from "../api/client";
import { EventKind } from "../bindings/EventKind";
import type { MinerEventType } from "../bindings/MinerEventType";
import { reportFailure } from "../state/session";

export type OwnedClaimsStateType = { events: MinerEventType[] | null; loading: boolean };

const claims: Record<string, OwnedClaimsStateType> = $state({});

export const ownedClaimsOf = (accountId: string): OwnedClaimsStateType =>
  claims[accountId] ?? { events: null, loading: false };

// ! Cached once per account: switching tabs never pages the whole claim history again.
export const loadOwnedClaims = async (accountId: string): Promise<void> => {
  if (claims[accountId]?.events || claims[accountId]?.loading) {
    return;
  }

  claims[accountId] = { events: null, loading: true };

  const found: MinerEventType[] = [];
  let before: number | undefined;

  try {
    do {
      const page = await fetchHistory({
        account_ids: [accountId],
        kinds: [EventKind.DROP_CLAIMED],
        before_id: before,
      });

      found.push(...page.events);
      before = page.next_before_id;
    } while (before !== undefined);

    claims[accountId] = { events: found, loading: false };
  } catch (e: unknown) {
    reportFailure(e);
    claims[accountId] = { events: null, loading: false };
  }
};
