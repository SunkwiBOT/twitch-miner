import { openLink } from "../state/toasts";

// ! The page never follows an external link itself: the desktop app opens it in the system browser.
export const followLink = (event: MouseEvent, url: string): void => {
  event.preventDefault();
  void openLink(url);
};
