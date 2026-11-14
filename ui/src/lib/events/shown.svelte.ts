import type { AccountSnapshotType } from "../bindings/AccountSnapshotType";
import type { CampaignType } from "../bindings/CampaignType";
import type { ChannelType } from "../bindings/ChannelType";
import type { MinerEventType } from "../bindings/MinerEventType";
import type { SnapshotType } from "../bindings/SnapshotType";
import { campaignOf, catalogueCampaign } from "../catalogue/catalogue.svelte";
import { knownChannels } from "../channels/channels.svelte";
import { app } from "../state/app.svelte";

type SnapshotIndexType = {
  accounts: Map<string, AccountSnapshotType>;
  channels: Map<string, ChannelType>;
};

const indexes = new WeakMap<SnapshotType, SnapshotIndexType>();

const indexOf = (snapshot: SnapshotType): SnapshotIndexType => {
  const known = indexes.get(snapshot);

  if (known) {
    return known;
  }

  const index: SnapshotIndexType = { accounts: new Map(), channels: new Map() };

  for (const account of snapshot.accounts) {
    index.accounts.set(account.id, account);

    for (const channel of [
      ...account.slots.map((slot) => slot.channel),
      ...account.streamers.map((view) => view.channel),
    ]) {
      if (channel && !index.channels.has(channel.login)) {
        index.channels.set(channel.login, channel);
      }
    }
  }

  indexes.set(snapshot, index);

  return index;
};

const eventCampaign = (accountId: string, id: string): CampaignType | undefined => {
  const owner = app.snapshot ? indexOf(app.snapshot).accounts.get(accountId) : undefined;

  return owner ? campaignOf(owner, id) : catalogueCampaign(id);
};

const channelOf = (login: string): ChannelType | undefined =>
  (app.snapshot ? indexOf(app.snapshot).channels.get(login) : undefined) ?? knownChannels[login];

export const unnamedChannels = (events: MinerEventType[]): string[] => [
  ...new Set(
    events.flatMap((event) => event.context.channel ?? []).filter((login) => !channelOf(login)),
  ),
];

export const shownEvent = (event: MinerEventType): MinerEventType => {
  const context = { ...event.context };
  const owner = app.snapshot ? indexOf(app.snapshot).accounts.get(event.account_id) : undefined;

  context.account_name ??= owner?.display_name;
  context.account_avatar ??= owner?.avatar_url;

  if (context.channel && (!context.channel_name || !context.avatar)) {
    const channel = channelOf(context.channel);

    context.channel_name ??= channel?.display_name;
    context.avatar ??= channel?.avatar_url;
  }

  const campaign =
    context.campaign_id && !context.campaign_name
      ? eventCampaign(event.account_id, context.campaign_id)
      : undefined;

  if (campaign) {
    const drop = campaign.drops.find((item) => item.id === context.drop_id);

    context.campaign_name = campaign.name;
    context.game ??= campaign.game.name;
    context.game_slug ??= campaign.game.slug;
    context.art ??= campaign.game.box_art_url;
    context.drop_name ??= drop?.name;
    context.reward ??= drop?.benefits[0]?.image_url;
  }

  return { ...event, context };
};
