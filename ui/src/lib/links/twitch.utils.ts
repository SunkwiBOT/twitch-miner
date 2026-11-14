export const channelUrl = (login: string): string => `https://www.twitch.tv/${login}`;
export const campaignUrl = (campaignId: string): string =>
  `https://www.twitch.tv/drops/campaigns?dropID=${encodeURIComponent(campaignId)}`;
