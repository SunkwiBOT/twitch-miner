<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { REPO_URL } from "../../lib/partners.constants";
  import { app, run } from "../../lib/state/state.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant, IconName } from "../ui/ui.constants";

  let closed = $state(false);

  // ! Never over another window (sign-in, setup): it waits for the next free moment.
  const open = $derived(
    !closed && app.snapshot?.star_prompt === true && !app.loginDialog && !app.feedbackDialog,
  );

  const close = (): void => {
    closed = true;
    void run({ command: CommandName.DISMISS_STAR_PROMPT });
  };
</script>

<Dialog
  {open}
  title={t.star_prompt_title()}
  wide={false}
  closable={true}
  closeLabel={t.common_close()}
  onclose={close}
>
  <div class="flex items-start gap-3">
    <img class="size-12 shrink-0 [image-rendering:pixelated]" src="/app-icon.svg" alt="" />
    <p class="text-sm text-muted-foreground">{t.star_prompt_text()}</p>
  </div>

  {#snippet footer()}
    <TextButton
      label={t.star_prompt_no()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={close}
    />
    <ExternalButton
      url={REPO_URL}
      label={t.star_prompt_yes()}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      icon={IconName.STAR}
      onfollow={close}
    />
  {/snippet}
</Dialog>
