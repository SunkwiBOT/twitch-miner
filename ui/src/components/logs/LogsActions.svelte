<script lang="ts">
  import type { Snippet } from "svelte";

  import { t } from "../../lib/i18n/i18n.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { app } from "../../lib/state/app.svelte";
  import CopyButton from "../ui/CopyButton.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { CopyLayout } from "../ui/ui.constants";

  type LogsActionsPropsType = {
    report: () => Promise<string>;
    copiedLabel: string;
    children?: Snippet;
  };

  let { report, copiedLabel, children }: LogsActionsPropsType = $props();
</script>

<CopyButton
  text={report}
  label={t.logs_copy()}
  {copiedLabel}
  layout={CopyLayout.LABELLED}
  size={ButtonSize.MD}
/>
{@render children?.()}
<TextButton
  label={t.logs_report()}
  icon={IconName.ALERT}
  variant={ButtonVariant.OUTLINE}
  size={ButtonSize.MD}
  type={ButtonType.BUTTON}
  disabled={false}
  onclick={() => (app.feedbackDialog = true)}
/>
