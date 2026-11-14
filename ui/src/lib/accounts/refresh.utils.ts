import { CommandName } from "../bindings/CommandName";
import { run } from "../state/session";

let waiting: (() => void) | null = null;

// ! Twitch says nothing when a game account gets linked: the account is read again once, as soon as the viewer comes back.
export const refreshWhenBack = (accountId: string): void => {
  waiting?.();

  const back = (): void => {
    if (document.visibilityState !== "visible") {
      return;
    }

    waiting?.();
    void run({ command: CommandName.REFRESH_ACCOUNT, account_id: accountId });
  };

  window.addEventListener("focus", back);
  document.addEventListener("visibilitychange", back);
  waiting = () => {
    window.removeEventListener("focus", back);
    document.removeEventListener("visibilitychange", back);
    waiting = null;
  };
};
