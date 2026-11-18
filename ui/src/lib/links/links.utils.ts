import { openBadgeLink, openCampaignLink, openLink } from "../state/toasts";

// ! The page never follows an external link itself: the desktop app opens it in the system browser.
export const followLink = (event: MouseEvent, url: string): void => {
  event.preventDefault();
  void openLink(url);
};

// ! The backend resolves the campaign's link from the account's own campaigns first, then
// its catalogue, by id: `url` is only the known value shown for the toast before the real
// one comes back.
export const followCampaignLink = (
  event: MouseEvent,
  accountId: string,
  campaignId: string,
  url: string,
): void => {
  event.preventDefault();
  void openCampaignLink(accountId, campaignId, url);
};

export const followBadgeLink = (event: MouseEvent, badgeKey: string, url: string): void => {
  event.preventDefault();
  void openBadgeLink(badgeKey, url);
};
