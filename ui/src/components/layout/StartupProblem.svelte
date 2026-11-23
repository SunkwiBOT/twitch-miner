<script lang="ts">
  import { Input } from "$lib/components/ui/input";
  import { cn } from "$lib/utils";
  import { openDataDirectory, quitApp } from "../../lib/api/client";
  import { ErrorApiMessage } from "../../lib/bindings/ErrorApiMessage";
  import type { StartupProblemType } from "../../lib/bindings/StartupProblemType";
  import { describeCode, describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { toast } from "../../lib/state/toasts";
  import { LATEST_RELEASE_URL } from "../../lib/partners.constants";
  import CopyButton from "../ui/CopyButton.svelte";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import FormField from "../ui/FormField.svelte";
  import AppLogo from "../ui/AppLogo.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { CopyLayout, NOTICE_TONE_CLASSES, NoticeTone } from "../ui/ui.constants";
  import { IconName } from "../../lib/icons.constants";

  type StartupProblemPropsType = { problem: StartupProblemType };

  let { problem }: StartupProblemPropsType = $props();

  const newerData = $derived(problem.code === ErrorApiMessage.NEWER_DATA);

  const act = async (action: () => Promise<void>): Promise<void> => {
    try {
      await action();
    } catch (e: any) {
      toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
    }
  };
</script>

<div class="grid h-full w-full place-items-center bg-background p-6">
  <div class="flex w-full max-w-150 flex-col gap-4 rounded-xl border bg-card p-7 shadow-lg">
    <AppLogo class="size-16 self-center" />
    <h1 class="text-center text-xl font-semibold">{t.startup_title()}</h1>
    <p class="text-center text-hint text-muted-foreground">
      {newerData ? describeCode(problem.code) : t.startup_text()}
    </p>
    {#if newerData}
      <ExternalButton
        url={LATEST_RELEASE_URL}
        label={t.startup_download_latest()}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.MD}
        icon={IconName.IMPORT}
        class="self-center"
      />
    {/if}
    <p
      class={cn(
        "rounded-lg border px-3 py-2 font-mono text-xs break-words",
        NOTICE_TONE_CLASSES[NoticeTone.DANGER],
      )}
    >
      {problem.detail}
    </p>
    <FormField id="startup-data-dir" label={t.settings_data_folder()}>
      <div class="flex gap-1.5">
        <Input id="startup-data-dir" class="font-mono" value={problem.data_dir} readonly={true} />
        <CopyButton
          text={() => problem.data_dir}
          label={t.common_copy()}
          copiedLabel={t.common_copied()}
          name={t.common_copy_named({ copiedItem: t.settings_data_folder() })}
          layout={CopyLayout.ICON}
          size={ButtonSize.MD}
        />
      </div>
    </FormField>
    <div class="flex flex-wrap justify-end gap-2">
      <TextButton
        label={t.settings_open_data()}
        icon={IconName.FILE}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => act(openDataDirectory)}
      />
      <TextButton
        label={t.startup_quit()}
        icon={IconName.X}
        variant={newerData ? ButtonVariant.OUTLINE : ButtonVariant.PRIMARY}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => act(quitApp)}
      />
    </div>
  </div>
</div>
