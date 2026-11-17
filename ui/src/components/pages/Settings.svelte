<script lang="ts">
  import { untrack } from "svelte";

  import { fetchAutostart, setAutostart } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { sameValue } from "../../lib/compare.utils";
  import { describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { AUTO_TIME_ZONE } from "../../lib/time/time.constants";
  import { deviceTimeZone } from "../../lib/time/time.svelte";
  import { saveGlobalSettings } from "../../lib/state/preferences.svelte";
  import { toast } from "../../lib/state/toasts";
  import SettingsAbout from "../settings/SettingsAbout.svelte";
  import SettingsCatalogue from "../settings/SettingsCatalogue.svelte";
  import SettingsDesktop from "../settings/SettingsDesktop.svelte";
  import SettingsFeedback from "../settings/SettingsFeedback.svelte";
  import SettingsGeneral from "../settings/SettingsGeneral.svelte";
  import SettingsNotifications from "../settings/SettingsNotifications.svelte";
  import SettingsProxy from "../settings/SettingsProxy.svelte";
  import SettingsSecurity from "../settings/SettingsSecurity.svelte";
  import SettingsTime from "../settings/SettingsTime.svelte";
  import UnsavedBar from "../settings/UnsavedBar.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PanelGrid, { type PanelGridCellType } from "../ui/PanelGrid.svelte";

  type SettingsPropsType = { snapshot: SnapshotType };

  let { snapshot }: SettingsPropsType = $props();

  const current = (): GlobalSettingsType => $state.snapshot(snapshot.settings);

  let base = $state<GlobalSettingsType>(untrack(current));
  let draft = $state<GlobalSettingsType>(untrack(current));

  const edited = $derived(!sameValue(draft, base));

  $effect(() => {
    const next = current();

    if (!untrack(() => edited)) {
      base = next;
      draft = structuredClone(next);
    }
  });

  let autostart = $state<boolean | null>(null);
  let autostartDraft = $state(false);

  $effect(() => {
    if (!isTauri()) {
      return;
    }

    fetchAutostart()
      .then((enabled) => {
        autostart = enabled;
        autostartDraft = enabled;
      })
      .catch((e: unknown) => console.warn("autostart state unavailable", e));
  });

  const autostartEdited = $derived(autostart !== null && autostartDraft !== autostart);

  const dirty = $derived(edited || autostartEdited);

  let saving = $state(false);

  const save = async (): Promise<void> => {
    const globalEdited = edited;

    saving = true;

    if (globalEdited) {
      const before = new Map(Object.entries(base));
      const changes = Object.fromEntries(
        Object.entries($state.snapshot(draft)).filter(
          ([key, value]) => !sameValue(value, before.get(key)),
        ),
      );
      const proxy = draft.default_proxy.trim();
      const resolvedZone = draft.time_zone === AUTO_TIME_ZONE ? deviceTimeZone() : draft.time_zone;
      const quietWindow = { ...draft.quiet_window, time_zone: resolvedZone };

      if (
        await saveGlobalSettings({
          ...current(),
          ...changes,
          default_proxy: proxy,
          quiet_window: quietWindow,
        })
      ) {
        draft.default_proxy = proxy;
        draft.quiet_window.time_zone = resolvedZone;
        base = $state.snapshot(draft);
      }
    }

    if (autostartEdited) {
      try {
        await setAutostart(autostartDraft);
        autostart = autostartDraft;

        if (!globalEdited) {
          toast(ToastKind.SUCCESS, t.common_saved());
        }
      } catch (e: any) {
        toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
      }
    }

    saving = false;
  };

  const reset = (): void => {
    base = current();
    draft = current();
    autostartDraft = autostart ?? false;
  };

  // ! Panels of similar height share a row; the desktop has one more panel, so its order differs to keep rows full.
  const cells: PanelGridCellType[] = isTauri()
    ? [
        { key: "general", wide: false, panel: general },
        { key: "time", wide: false, panel: time },
        { key: "security", wide: false, panel: security },
        { key: "desktop", wide: false, panel: desktop },
        { key: "proxy", wide: false, panel: proxy },
        { key: "catalogue", wide: false, panel: catalogue },
        { key: "notifications", wide: true, panel: notifications },
        { key: "about", wide: false, panel: about },
        { key: "feedback", wide: false, panel: feedback },
      ]
    : [
        { key: "general", wide: false, panel: general },
        { key: "time", wide: false, panel: time },
        { key: "security", wide: false, panel: security },
        { key: "about", wide: false, panel: about },
        { key: "proxy", wide: false, panel: proxy },
        { key: "catalogue", wide: false, panel: catalogue },
        { key: "notifications", wide: true, panel: notifications },
        { key: "feedback", wide: true, panel: feedback },
      ];
</script>

{#snippet general()}<SettingsGeneral bind:draft />{/snippet}

{#snippet time()}<SettingsTime bind:draft />{/snippet}

{#snippet security()}<SettingsSecurity {snapshot} bind:draft />{/snippet}

{#snippet desktop()}<SettingsDesktop bind:draft bind:autostart={autostartDraft} />{/snippet}

{#snippet proxy()}<SettingsProxy bind:draft />{/snippet}

{#snippet catalogue()}<SettingsCatalogue />{/snippet}

{#snippet notifications()}<SettingsNotifications bind:draft />{/snippet}

{#snippet about()}<SettingsAbout {snapshot} bind:draft />{/snippet}

{#snippet feedback()}<SettingsFeedback />{/snippet}

<PageBody>
  <PanelGrid {cells} />

  {#if dirty}
    <UnsavedBar {saving} oncancel={reset} onsave={save} />
  {/if}
</PageBody>
