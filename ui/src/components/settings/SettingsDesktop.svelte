<script lang="ts">
  import { openDataDirectory } from "../../lib/api/client";
  import { CloseAction } from "../../lib/bindings/CloseAction";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import { describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { app } from "../../lib/state/app.svelte";
  import { toast } from "../../lib/state/toasts";
  import { Input } from "$lib/components/ui/input";
  import CopyButton from "../ui/CopyButton.svelte";
  import FieldActions from "../ui/FieldActions.svelte";
  import FormField from "../ui/FormField.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import Panel from "../ui/Panel.svelte";
  import Select from "../ui/Select.svelte";
  import Setting from "../ui/Setting.svelte";
  import { ButtonSize, ButtonVariant } from "../ui/button.constants";
  import { CopyLayout } from "../ui/ui.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MenuAlign } from "../ui/menu.constants";

  type SettingsDesktopPropsType = {
    draft: GlobalSettingsType;
    autostart: boolean;
  };

  let { draft = $bindable(), autostart = $bindable() }: SettingsDesktopPropsType = $props();

  const closeActions = $derived([
    { value: CloseAction.ASK, label: t.close_ask() },
    { value: CloseAction.QUIT, label: t.close_quit_title(), description: t.close_quit_text() },
    { value: CloseAction.TRAY, label: t.close_tray_title(), description: t.close_tray_text() },
  ]);

  const openData = async (): Promise<void> => {
    try {
      await openDataDirectory();
    } catch (e: any) {
      toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
    }
  };
</script>

<Panel title={t.settings_desktop()}>
  <FormField id="close-action" label={t.settings_close_action()}>
    <Select
      id="close-action"
      value={draft.close_action}
      options={closeActions}
      label={t.settings_close_action()}
      align={MenuAlign.LEFT}
      onchange={(value) => (draft.close_action = value)}
    />
  </FormField>
  <div>
    <Setting
      title={t.settings_autostart()}
      checked={autostart}
      disabled={false}
      onchange={(value) => (autostart = value)}
    />
    <Setting
      title={t.settings_start_minimized()}
      description={t.settings_start_minimized_hint()}
      checked={draft.start_minimized}
      disabled={false}
      onchange={(value) => (draft.start_minimized = value)}
    />
  </div>
  {#if app.info}
    {@const dataDir = app.info.data_dir}
    {#snippet path()}
      <Input id="data-dir" class="font-mono" value={dataDir} readonly={true} />
    {/snippet}
    {#snippet folderActions()}
      <CopyButton
        text={() => dataDir}
        label={t.common_copy()}
        copiedLabel={t.common_copied()}
        name={t.common_copy_named({ name: t.settings_data_folder() })}
        layout={CopyLayout.ICON}
        size={ButtonSize.MD}
      />
      <IconButton
        icon={IconName.FOLDER}
        label={t.settings_open_data()}
        busy={false}
        disabled={false}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        onclick={openData}
      />
    {/snippet}
    <FormField id="data-dir" label={t.settings_data_folder()} hint={t.settings_data_folder_hint()}>
      <FieldActions field={path} actions={folderActions} />
    </FormField>
  {/if}
</Panel>
