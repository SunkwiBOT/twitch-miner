<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { signOut } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import { ErrorApiMessage } from "../../lib/bindings/ErrorApiMessage";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { Theme } from "../../lib/bindings/Theme";
  import { VaultMode } from "../../lib/bindings/VaultMode";
  import { describeError, toApiError } from "../../lib/errors/errors.utils";
  import { languageOptions, locale, t } from "../../lib/i18n/i18n.svelte";
  import { overlay } from "../../lib/state/overlay.svelte";
  import { Page, ToastKind } from "../../lib/state/state.constants";
  import { app } from "../../lib/state/app.svelte";
  import { setPreference, toggleTheme } from "../../lib/state/preferences.svelte";
  import { navigate } from "../../lib/state/router";
  import { run, sessionEnded } from "../../lib/state/session";
  import { toast } from "../../lib/state/toasts";
  import Dialog from "../ui/Dialog.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import Select from "../ui/Select.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MenuAlign } from "../ui/menu.constants";

  type ShellControlsPropsType = { snapshot: SnapshotType; align: MenuAlign };

  let { snapshot, align }: ShellControlsPropsType = $props();

  const languages = $derived(languageOptions());

  const passwordSet = $derived(!isTauri() || snapshot.vault.mode === VaultMode.PASSPHRASE);

  const padlockLabel = $derived.by((): string => {
    if (!isTauri()) {
      return t.lock_dashboard();
    }

    return passwordSet ? t.lock_now() : t.lock_set_password();
  });

  let locking = $state(false);
  let confirmingLock = $state(false);

  const lock = async (): Promise<void> => {
    confirmingLock = false;
    locking = true;

    if (isTauri()) {
      await run({ command: CommandName.LOCK });
      locking = false;

      return;
    }

    try {
      await signOut();
    } catch (e: any) {
      if (toApiError(e).code !== ErrorApiMessage.UNAUTHORIZED) {
        toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
        locking = false;

        return;
      }
    }

    sessionEnded();
  };

  const askLock = (): void => {
    if (overlay.unsaved) {
      confirmingLock = true;

      return;
    }

    void lock();
  };

  const showPasswordCard = (): void => {
    app.revealPassword = true;
    navigate(Page.SETTINGS);
  };
</script>

<div class="min-w-28 max-w-56 grow-[100] basis-0">
  <Select
    value={locale.preference}
    options={languages}
    label={t.topbar_language()}
    {align}
    onchange={(language) => setPreference({ language })}
  />
</div>
<IconButton
  icon={snapshot.settings.theme === Theme.LIGHT ? IconName.MOON : IconName.SUN}
  label={t.topbar_theme()}
  busy={false}
  disabled={false}
  variant={ButtonVariant.DEFAULT}
  size={ButtonSize.MD}
  onclick={toggleTheme}
/>
<IconButton
  icon={IconName.LOCK}
  label={padlockLabel}
  busy={locking}
  disabled={false}
  variant={ButtonVariant.DEFAULT}
  size={ButtonSize.MD}
  onclick={passwordSet ? askLock : showPasswordCard}
/>

<Dialog
  open={confirmingLock}
  title={t.lock_unsaved_title()}
  wide={false}
  closable={true}
  closeLabel={t.common_close()}
  onclose={() => (confirmingLock = false)}
>
  <p class="text-hint text-muted-foreground">{t.lock_unsaved_text()}</p>

  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      icon={IconName.X}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => (confirmingLock = false)}
    />
    <TextButton
      label={t.lock_anyway()}
      icon={IconName.LOCK}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={lock}
    />
  {/snippet}
</Dialog>
