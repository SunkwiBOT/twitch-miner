import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import { QueueBlock } from "../bindings/QueueBlock";
import type { TimedDropType } from "../bindings/TimedDropType";
import { campaignOf } from "./catalogue.svelte";
import { dropDone, dropTotals, evolutionBlocked, subsMissing } from "./campaign.utils";
import { ProgressState, type AccountProgressType } from "./progress.constants";

export type CampaignProgressType = AccountProgressType & {
  ready: string[];
  linked: boolean | undefined;
};

export const minesCampaign = (account: AccountSnapshotType, id: string): boolean =>
  account.slots.some((slot) => slot.drop?.campaign_id === id);

// ! `claimed` holds the claims made here: Twitch's inventory counts them a moment later.
export const campaignProgress = (
  account: AccountSnapshotType,
  id: string,
  claimed: string[],
): CampaignProgressType => {
  const campaign = campaignOf(account, id);

  if (!campaign) {
    return {
      account,
      state: ProgressState.NOT_OFFERED,
      current: 0,
      required: 0,
      minable: false,
      ready: [],
      linked: undefined,
      setAsideUntil: null,
    };
  }

  const drops = account.settings.drops;
  const queued = account.queue.find((item) => item.campaign_id === id);
  const done = (drop: TimedDropType): boolean =>
    dropDone(drop) ||
    (!drop.repeat && drop.instance_id !== undefined && claimed.includes(drop.instance_id));
  const open = campaign.drops.filter((drop) => !done(drop) && !drop.badge_owned);
  const wanted = open.filter((drop) => !drops.exclude_drop_ids.includes(drop.id));
  const totals = campaign.drops
    .filter(
      (drop) =>
        !subsMissing(drop) && !drop.badge_owned && !drops.exclude_drop_ids.includes(drop.id),
    )
    .map(dropTotals);
  const current = totals.reduce((sum, item) => sum + item.current, 0);
  const waiting = campaign.drops.flatMap((drop) =>
    drop.instance_id && !drop.claimed && !claimed.includes(drop.instance_id)
      ? [drop.instance_id]
      : [],
  );

  const state = (): ProgressState => {
    if (waiting.length > 0) {
      return ProgressState.CLAIMABLE;
    }

    if (open.length === 0) {
      return campaign.drops.some(done) ? ProgressState.CLAIMED : ProgressState.OWNED;
    }

    if (drops.exclude_campaign_ids.includes(id) || wanted.length === 0) {
      return ProgressState.IGNORED;
    }

    if (wanted.every(subsMissing)) {
      return ProgressState.SUB_ONLY;
    }

    if (queued?.blocked === QueueBlock.SET_ASIDE) {
      return ProgressState.SET_ASIDE;
    }

    if (minesCampaign(account, id)) {
      return ProgressState.MINING;
    }

    return campaign.in_inventory || current > 0
      ? ProgressState.IN_PROGRESS
      : ProgressState.NOT_STARTED;
  };

  return {
    account,
    state: state(),
    current,
    required: totals.reduce((sum, item) => sum + item.required, 0),
    minable: open.some(
      (drop) =>
        !(drop.instance_id && !drop.claimed) &&
        !subsMissing(drop) &&
        !evolutionBlocked(drop.evolution, account.settings.badges.evolutions),
    ),
    ready: waiting,
    linked: campaign.linked,
    setAsideUntil: queued?.set_aside_until ?? null,
  };
};
