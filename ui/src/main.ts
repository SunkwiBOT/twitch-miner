import { mount } from "svelte";

import "@fontsource-variable/inter";
import "./theme.css";

import App from "./components/layout/App.svelte";
import ToastCard from "./components/layout/ToastCard.svelte";
import { isTauri } from "./lib/api/tauri";
import { prepareLanguage } from "./lib/i18n/i18n.svelte";
import { applySavedTheme } from "./lib/state/preferences.svelte";
import { start } from "./lib/state/session";
import { setToastCard } from "./lib/state/toasts";
import { followUpdates } from "./lib/updates/updates.svelte";

setToastCard(ToastCard);
applySavedTheme();

if (isTauri()) {
  window.addEventListener("contextmenu", (event) => {
    const editable =
      event.target instanceof Element && event.target.closest("input, textarea, [contenteditable]");

    if (!editable && !window.getSelection()?.toString()) {
      event.preventDefault();
    }
  });
}

// ! Web dashboard only, where a browser allows a service worker (HTTPS or localhost): an offline "no connection" screen.
if (!isTauri() && import.meta.env.PROD && window.isSecureContext && "serviceWorker" in navigator) {
  navigator.serviceWorker
    .register("/sw.js", { updateViaCache: "none" })
    .catch((e: unknown) => console.warn("the offline screen could not be set up", e));
}

const target = document.getElementById("app");

if (target) {
  prepareLanguage()
    .then(() => {
      mount(App, { target });

      return start();
    })
    .then(followUpdates)
    .catch((e: unknown) => console.error("the interface could not start", e));
}
