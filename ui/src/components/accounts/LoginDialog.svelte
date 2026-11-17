<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { untrack, type Snippet } from "svelte";

  import { startLogin } from "../../lib/api/client";
  import { ImportTokensFlowStatus } from "../../lib/bindings/ImportTokensFlowStatus";
  import { LoginFlowStatus } from "../../lib/bindings/LoginFlowStatus";
  import { describeCode, describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { account, app } from "../../lib/state/app.svelte";
  import { refresh, run } from "../../lib/state/session";
  import { strong } from "../../lib/text.utils";
  import ActionButton from "../ui/ActionButton.svelte";
  import Dialog from "../ui/Dialog.svelte";
  import Icon from "../ui/Icon.svelte";
  import RichText from "../ui/RichText.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { LoginMode } from "../../lib/accounts/login.constants";
  import ImportTokensDialog from "./ImportTokensDialog.svelte";
  import LoginCode from "./LoginCode.svelte";
  import SetupWizard from "./SetupWizard.svelte";

  type KnownAccountsType = { flowId: string; ids: string[] };
  const flow = $derived(app.snapshot?.login ?? null);
  // ! Kept while the dialog fades out: closing cancels the flow, and the clearing snapshot can land mid-animation.
  let shown = $state<typeof flow>(null);
  let starting = $state(false);
  let startError = $state<string | null>(null);
  let autoStarted = false;

  let mode = $state<LoginMode>(LoginMode.CODE);

  let wasOpen = false;

  $effect.pre(() => {
    if (app.loginDialog) {
      shown = flow;
    }
  });

  $effect(() => {
    if (app.loginDialog && !wasOpen) {
      mode = LoginMode.CODE;
    }

    wasOpen = app.loginDialog;
  });

  const start = async (): Promise<void> => {
    starting = true;

    try {
      await startLogin();
      await refresh();
      startError = null;
    } catch (e: any) {
      startError = describeError(e);
    } finally {
      starting = false;
    }
  };

  const failure = $derived(shown?.status === LoginFlowStatus.PENDING ? null : startError);

  let knownAccounts = $state<KnownAccountsType | null>(null);

  $effect(() => {
    if (
      flow?.status === LoginFlowStatus.PENDING &&
      untrack(() => knownAccounts?.flowId) !== flow.flow_id
    ) {
      knownAccounts = {
        flowId: flow.flow_id,
        ids: untrack(() => app.snapshot?.accounts.map((a) => a.id) ?? []),
      };
    }
  });

  const newAccount = $derived(
    shown?.status === LoginFlowStatus.COMPLETED &&
      shown.account_id !== undefined &&
      knownAccounts?.flowId === shown.flow_id &&
      !knownAccounts.ids.includes(shown.account_id)
      ? account(shown.account_id)
      : null,
  );

  let finishing = $state(false);
  let setupActions = $state<Snippet>();
  let importFlowId = $state<string | null>(null);

  const close = (): void => {
    app.loginDialog = false;

    if (flow && flow.status !== LoginFlowStatus.PENDING) {
      void run({ command: CommandName.CANCEL_LOGIN });
    }

    // ! A running flow is cancelled, a settled one dismissed: one command
    // per close, never both for the same flow.
    if (importFlowId !== null && app.snapshot?.import?.flow_id === importFlowId) {
      const command =
        app.snapshot.import.status === ImportTokensFlowStatus.RUNNING
          ? CommandName.CANCEL_IMPORT_TOKENS
          : CommandName.DISMISS_IMPORT_TOKENS;

      void run({ command });
    }
  };

  // ! One automatic start per opening: a failed start keeps its Restart button instead of looping.
  $effect(() => {
    if (!app.loginDialog) {
      autoStarted = false;
    } else if (mode === LoginMode.CODE && !autoStarted && !flow && app.snapshot?.vault.unlocked) {
      autoStarted = true;
      startError = null;
      void start();
    }
  });
</script>

{#snippet actions()}
  {#if shown?.status === LoginFlowStatus.COMPLETED}
    <TextButton
      label={t.common_close()}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={close}
    />
  {:else if failure !== null || (shown && shown.status !== LoginFlowStatus.PENDING)}
    <TextButton
      label={t.common_close()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={starting}
      onclick={close}
    />
    <ActionButton
      icon={IconName.REFRESH}
      label={t.login_restart()}
      busyLabel={t.login_getting_code()}
      busy={starting}
      disabled={false}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={start}
    />
  {/if}
{/snippet}

{#snippet modeToggle()}
  <div class="flex justify-end">
    <TextButton
      label={t.login_use_tokens()}
      variant={ButtonVariant.GHOST}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => (mode = LoginMode.TOKEN)}
    />
  </div>
{/snippet}

<Dialog
  open={app.loginDialog}
  title={newAccount
    ? t.login_setup_title()
    : mode === LoginMode.TOKEN
      ? t.import_tokens_title()
      : t.login_title()}
  wide={newAccount !== null || mode === LoginMode.TOKEN}
  closable={!finishing}
  closeLabel={t.common_close()}
  onclose={close}
  footer={newAccount
    ? setupActions
    : mode === LoginMode.TOKEN
      ? setupActions
      : failure !== null || (shown && shown.status !== LoginFlowStatus.PENDING)
        ? actions
        : undefined}
>
  {#if !app.snapshot?.vault.unlocked}
    <p class="text-hint text-muted-foreground">{t.login_vault_locked()}</p>
  {:else if mode === LoginMode.TOKEN}
    <ImportTokensDialog
      onclose={close}
      onusecode={() => (mode = LoginMode.CODE)}
      bind:finishing
      bind:actions={setupActions}
      bind:flowId={importFlowId}
    />
  {:else if failure !== null}
    <p class="text-destructive">
      <RichText text={t.login_failed({ error: strong(failure) })} />
    </p>
  {:else if !shown}
    <p class="text-hint text-muted-foreground">{t.common_loading()}</p>
  {:else if shown.status === LoginFlowStatus.PENDING}
    <LoginCode flow={shown} />
    {@render modeToggle()}
  {:else if shown.status === LoginFlowStatus.COMPLETED && newAccount}
    <SetupWizard account={newAccount} onclose={close} bind:finishing bind:actions={setupActions} />
  {:else if shown.status === LoginFlowStatus.COMPLETED}
    <p class="flex items-center gap-2 text-success-foreground">
      <Icon name={IconName.CHECK} size={18} />
      {t.login_completed()}
    </p>
  {:else}
    <p class="text-destructive">
      <RichText
        text={shown.status === LoginFlowStatus.EXPIRED
          ? t.login_expired()
          : t.login_failed({ error: shown.problem ? strong(describeCode(shown.problem)) : "" })}
      />
    </p>
  {/if}
</Dialog>
