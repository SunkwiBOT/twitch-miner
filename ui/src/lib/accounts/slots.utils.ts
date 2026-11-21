import { coreLimits } from "../state/limits";
import { strong } from "../text.utils";

export const maxWatchedChannelsParams = (): {
  maxWatchedChannels: number;
  formattedMaxWatchedChannels: string;
} => {
  const max = coreLimits().max_watched_channels;

  return { maxWatchedChannels: max, formattedMaxWatchedChannels: strong(max) };
};
