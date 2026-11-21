<script lang="ts">
  import { InstallKind } from "../../lib/bindings/InstallKind";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { IMAGE_UPDATE_COMMANDS } from "../../lib/updates/updates.constants";
  import { binaryUpdateCommands } from "../../lib/updates/updates.utils";
  import CopyButton from "../ui/CopyButton.svelte";
  import { ButtonSize } from "../ui/button.constants";
  import { CopyLayout } from "../ui/ui.constants";

  type ServerUpdateCommandsPropsType = { version: string };

  let { version }: ServerUpdateCommandsPropsType = $props();

  const steps = $derived.by((): { hint: string; commands: string } | null => {
    switch (app.snapshot?.install_kind) {
      case InstallKind.DOCKER:
        return { hint: t.updates_pull_image(), commands: IMAGE_UPDATE_COMMANDS.join("\n") };
      case InstallKind.BINARY:
        return {
          hint: t.updates_replace_binary(),
          commands: binaryUpdateCommands(version).join("\n"),
        };
      default:
        return null;
    }
  });
</script>

{#if steps}
  {@const { hint, commands } = steps}
  <div class="flex flex-col gap-1.5">
    <p class="text-hint text-muted-foreground">{hint}</p>
    <div class="flex items-start gap-1 rounded-lg border bg-muted/50 py-1 ps-3 pe-1">
      <pre class="min-w-0 flex-1 py-1 font-mono text-xs whitespace-pre-wrap">{commands}</pre>
      <CopyButton
        text={() => commands}
        label={t.common_copy()}
        copiedLabel={t.common_copied()}
        layout={CopyLayout.ICON}
        size={ButtonSize.SM}
      />
    </div>
  </div>
{/if}
