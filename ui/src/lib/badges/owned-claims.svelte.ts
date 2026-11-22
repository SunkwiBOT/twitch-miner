import { fetchHistory } from "../api/client";
import { EventKind } from "../bindings/EventKind";
import type { MinerEventType } from "../bindings/MinerEventType";
import { reportFailure } from "../state/session";

export type OwnedClaimsStateType = { events: MinerEventType[] | null; loading: boolean };

const claims: Record<string, OwnedClaimsStateType> = $state({});

export const ownedClaimsOf = (accountId: string): OwnedClaimsStateType =>
  claims[accountId] ?? { events: null, loading: false };

// ! Loaded once per account; a claim made afterwards joins it as it happens (`noteOwnedClaim`).
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

export const noteOwnedClaim = (event: MinerEventType): void => {
  const known = claims[event.account_id]?.events;

  if (event.kind === EventKind.DROP_CLAIMED && known && !known.some((e) => e.id === event.id)) {
    claims[event.account_id] = { events: [event, ...known], loading: false };
  }
};

export const forgetOwnedClaims = (): void => {
  for (const accountId of Object.keys(claims)) {
    delete claims[accountId];
  }
};
