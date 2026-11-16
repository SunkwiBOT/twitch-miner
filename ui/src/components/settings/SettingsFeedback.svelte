<script lang="ts">
  import { openLogsDirectory } from "../../lib/api/client";
  import { isTauri } from "../../lib/api/tauri";
  import { describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { Page, ToastKind } from "../../lib/state/state.constants";
  import { navigate } from "../../lib/state/router";
  import { toast } from "../../lib/state/toasts";
  import FeedbackChoices from "../feedback/FeedbackChoices.svelte";
  import Panel from "../ui/Panel.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";

  const openLogs = async (): Promise<void> => {
    try {
      await openLogsDirectory();
    } catch (e: any) {
      toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
    }
  };
</script>

<Panel title={t.settings_feedback()} description={t.settings_feedback_text()}>
  <FeedbackChoices />
  <div class="flex flex-wrap gap-2">
    <TextButton
      label={t.nav_logs()}
      icon={IconName.FILE}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => navigate(Page.LOGS)}
    />
    {#if isTauri()}
      <TextButton
        label={t.settings_open_logs()}
        icon={IconName.FOLDER}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={openLogs}
      />
    {/if}
  </div>
</Panel>
