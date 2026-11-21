<script lang="ts">
  import type { Snippet } from "svelte";

  import { maskProxyCredentials } from "../../lib/accounts/accounts.utils";
  import {
    buildTokenLines,
    dedupeByAccount,
    hasClientWarning,
    ImportLineStatus,
    ImportStep,
    maskToken,
    parseTokenLines,
  } from "../../lib/accounts/token-import.utils";
  import { checkPastedToken, sendCommand, startImportTokens } from "../../lib/api/client";
  import type { AccountSettingsPatchType } from "../../lib/bindings/AccountSettingsPatchType";
  import { CommandName } from "../../lib/bindings/CommandName";
  import { ErrorApiMessage } from "../../lib/bindings/ErrorApiMessage";
  import type { ImportTokenOutcomeType } from "../../lib/bindings/ImportTokenOutcomeType";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { ImportTokensFlowStatus } from "../../lib/bindings/ImportTokensFlowStatus";
  import type { TokenCheckType } from "../../lib/bindings/TokenCheckType";
  import { describeCode, describeError, toApiError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { account as accountOf, app } from "../../lib/state/app.svelte";
  import { coreLimits } from "../../lib/state/limits";
  import { refresh, reportFailure } from "../../lib/state/session";
  import { strong } from "../../lib/text.utils";
  import { cn } from "$lib/utils";
  import AvatarImage from "../ui/AvatarImage.svelte";
  import ActionButton from "../ui/ActionButton.svelte";
  import Icon from "../ui/Icon.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import ProxyField from "../ui/ProxyField.svelte";
  import RichText from "../ui/RichText.svelte";
  import Setting from "../ui/Setting.svelte";
  import Spinner from "../ui/Spinner.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { Textarea } from "$lib/components/ui/textarea";
  import { BADGE_TONE_CLASSES } from "../ui/badge.constants";
  import { BadgeTone } from "../../lib/tones.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import SetupWizard from "./SetupWizard.svelte";
  import { PROXY_EXAMPLE } from "../account/account.constants";

  type ImportTokensDialogPropsType = {
    onclose: () => void;
    onusecode: () => void;
    finishing: boolean;
    actions: Snippet | undefined;
    flowId: string | null;
    wizardOpen: boolean;
  };

  let {
    onclose,
    onusecode,
    finishing = $bindable(),
    actions = $bindable(),
    flowId = $bindable(null),
    wizardOpen = $bindable(false),
  }: ImportTokensDialogPropsType = $props();

  type LineType = {
    token: string;
    status: ImportLineStatus;
    check: TokenCheckType | null;
    reason: string | null;
  };

  type SentLineType = {
    token: string;
    check: TokenCheckType | null;
    outcome: ImportTokenOutcomeType;
    // ! The flow that owns this row and this row's index within that
    // flow's lines: a retry re-points the row at the new flow in place
    // instead of appending a copy, so there is never more than one row
    // per imported token.
    flowId: string;
    flowIndex: number;
  };

  const OUTCOME_PENDING = "PENDING" as const;
  const OUTCOME_CHECKING = "CHECKING" as const;
  const OUTCOME_IMPORTED = "IMPORTED" as const;
  const OUTCOME_FAILED = "FAILED" as const;

  const PENDING_OUTCOME: ImportTokenOutcomeType = { status: OUTCOME_PENDING };

  // ! The vault-locked branch in `LoginDialog` renders in place of this
  // component until `app.snapshot` exists, so this throws only if that
  // invariant is ever broken, never in normal use.
  const requireDefaultPatch = (): AccountSettingsPatchType => {
    if (!app.snapshot) {
      throw new Error("ImportTokensDialog mounted before app.snapshot exists");
    }

    return app.snapshot.import_default_patch;
  };

  const STATUS_CLASSES: Record<ImportLineStatus, string> = {
    [ImportLineStatus.PENDING]: "border-dashed border-input text-muted-foreground",
    [ImportLineStatus.CHECKING]: "border-dashed border-input text-muted-foreground",
    [ImportLineStatus.VALID]: BADGE_TONE_CLASSES[BadgeTone.SUCCESS],
    [ImportLineStatus.ALREADY_ADDED]: BADGE_TONE_CLASSES[BadgeTone.MUTED],
    [ImportLineStatus.DUPLICATE]: BADGE_TONE_CLASSES[BadgeTone.WARNING],
    [ImportLineStatus.INVALID]: BADGE_TONE_CLASSES[BadgeTone.DANGER],
    [ImportLineStatus.OTHER_ERROR]: BADGE_TONE_CLASSES[BadgeTone.DANGER],
    [ImportLineStatus.NOT_IMPORTED]: BADGE_TONE_CLASSES[BadgeTone.NEUTRAL],
  };

  const STATUS_LABELS: Partial<Record<ImportLineStatus, () => string>> = {
    [ImportLineStatus.CHECKING]: () => t.import_tokens_status_pending(),
    [ImportLineStatus.VALID]: () => t.import_tokens_status_valid(),
    [ImportLineStatus.ALREADY_ADDED]: () => t.import_tokens_status_already_added(),
    [ImportLineStatus.DUPLICATE]: () => t.import_tokens_status_duplicate(),
    [ImportLineStatus.INVALID]: () => t.import_tokens_status_invalid(),
    [ImportLineStatus.NOT_IMPORTED]: () => t.import_tokens_status_not_imported(),
  };

  const lineLabel = (line: LineType): string =>
    line.status === ImportLineStatus.OTHER_ERROR
      ? (line.reason ?? "")
      : (STATUS_LABELS[line.status]?.() ?? "");

  let step = $state<ImportStep>(ImportStep.LIST);
  // ! The steps' buttons go up to the dialog's footer, same as every other
  // multi-step flow — except the wizard step, which owns `actions` itself
  // through its own `bind:actions`, forwarded straight through this one.
  $effect.pre(() => {
    if (step === ImportStep.WIZARD) {
      return;
    }

    actions = footer;

    return () => {
      actions = undefined;
    };
  });

  $effect(() => {
    wizardOpen = step === ImportStep.WIZARD;
  });

  let pasted = $state("");
  let pasteError = $state<string | null>(null);
  let pasteShown = $state(false);
  let lines = $state<LineType[]>([]);
  let checking = $state(false);
  let cancelRequested = false;
  let inflight: AbortController | null = null;

  // ! Defaults from the backend's own `AccountSettings::default()`, see
  // `AccountSettingsPatch::defaults()`. `$state.snapshot` takes the deep
  // plain copy: `structuredClone` chokes on the reactive proxy itself.
  let patch = $state<AccountSettingsPatchType>($state.snapshot(requireDefaultPatch()));
  let overwriteExisting = $state(false);
  let committing = $state(false);
  let commitError = $state<string | null>(null);

  let sentLines = $state<SentLineType[]>([]);
  let dismissed = $state<ReadonlySet<number>>(new Set());
  // ! An index is frozen exactly once: writing the same outcome back to
  // `sentLines` on every effect run (a fresh object each poll, never equal
  // by reference) reschedules this effect forever and trips Svelte's own
  // update-depth guard.
  let frozen = $state<ReadonlySet<number>>(new Set());

  const pendingCount = $derived(
    lines.filter((line) => line.status === ImportLineStatus.PENDING).length,
  );

  const checkedCount = $derived(
    lines.filter(
      (line) =>
        line.status !== ImportLineStatus.PENDING && line.status !== ImportLineStatus.CHECKING,
    ).length,
  );

  const importableLines = $derived(
    lines.filter(
      (line) =>
        line.status === ImportLineStatus.VALID || line.status === ImportLineStatus.ALREADY_ADDED,
    ),
  );

  const alreadyAddedCount = $derived(
    lines.filter((line) => line.status === ImportLineStatus.ALREADY_ADDED).length,
  );

  const hasAlreadyAdded = $derived(alreadyAddedCount > 0);

  const checkAgainCount = $derived(
    lines.filter((line) => line.status === ImportLineStatus.OTHER_ERROR).length,
  );

  const buildLines = (tokens: string[]): LineType[] =>
    buildTokenLines(tokens).map((line) => ({ ...line, check: null, reason: null }));

  const settleCheck = (
    index: number,
    check: TokenCheckType | null,
    status: ImportLineStatus,
    reason: string | null,
  ): void => {
    lines[index] = { ...lines[index], status, check, reason };
    lines = dedupeByAccount(lines);
  };

  const runChecks = async (indexes: number[]): Promise<void> => {
    checking = true;
    cancelRequested = false;

    try {
      for (const index of indexes) {
        if (cancelRequested) {
          break;
        }

        lines[index] = { ...lines[index], status: ImportLineStatus.CHECKING };
        inflight = new AbortController();

        try {
          const check = await checkPastedToken(lines[index].token, inflight.signal);

          settleCheck(
            index,
            check,
            check.already_added ? ImportLineStatus.ALREADY_ADDED : ImportLineStatus.VALID,
            null,
          );
        } catch (e: unknown) {
          // ! `cancelChecking` aborts the in-flight request: that is not a
          // network problem, the line just never got its answer and goes
          // back to pending so it can be checked again like any other.
          if (cancelRequested) {
            lines[index] = { ...lines[index], status: ImportLineStatus.PENDING };

            continue;
          }

          const code = toApiError(e).code;
          const status =
            code === ErrorApiMessage.INVALID_TOKEN
              ? ImportLineStatus.INVALID
              : ImportLineStatus.OTHER_ERROR;

          settleCheck(
            index,
            null,
            status,
            status === ImportLineStatus.OTHER_ERROR ? describeError(e) : null,
          );
        } finally {
          inflight = null;
        }
      }
    } finally {
      checking = false;
    }
  };

  const pendingIndexes = (): number[] =>
    lines.flatMap((line, index) => (line.status === ImportLineStatus.PENDING ? [index] : []));

  const advance = async (): Promise<void> => {
    if (lines.length === 0) {
      const tokens = parseTokenLines(pasted);

      if (tokens.length === 0) {
        return;
      }

      const { token_import_max: importMax, token_line_max_chars: lineMaxChars } = coreLimits();

      if (tokens.length > importMax) {
        pasteError = t.import_tokens_too_many({ max: strong(importMax) });

        return;
      }

      if (tokens.some((token) => token.length > lineMaxChars)) {
        pasteError = t.import_tokens_line_too_long({ max: strong(lineMaxChars) });

        return;
      }

      pasteError = null;
      pasted = "";
      lines = buildLines(tokens);
      await runChecks(pendingIndexes());

      return;
    }

    if (pendingCount > 0) {
      await runChecks(pendingIndexes());

      return;
    }

    step = ImportStep.SETTINGS;
  };

  const cancelChecking = (): void => {
    cancelRequested = true;
    inflight?.abort();
  };

  // ! Checked lines are never dropped on the way back: the paste field
  // gets its tokens back instead of leaving token mode outright.
  const backToPaste = (): void => {
    pasted = lines.map((line) => line.token).join("\n");
    lines = [];
  };

  const checkAgain = async (): Promise<void> => {
    const indexes = lines.flatMap((line, index) =>
      line.status === ImportLineStatus.OTHER_ERROR ? [index] : [],
    );

    for (const index of indexes) {
      lines[index] = { ...lines[index], status: ImportLineStatus.PENDING, reason: null };
    }

    await runChecks(indexes);
  };

  const canAdvance = $derived(
    lines.length === 0 ? pasted.trim() !== "" : pendingCount > 0 || importableLines.length > 0,
  );

  const importFlow = $derived(
    app.snapshot?.import && app.snapshot.import.flow_id === flowId ? app.snapshot.import : null,
  );

  const importRunning = $derived(importFlow?.status === ImportTokensFlowStatus.RUNNING);
  const importCancelled = $derived(importFlow?.status === ImportTokensFlowStatus.CANCELLED);

  // ! A frozen outcome (once past `PENDING`/`CHECKING`) belongs to the
  // line itself from then on: a retry re-points `flowId`/`flowIndex` at
  // the new flow and clears the freeze, so the same row picks up the new
  // flow's outcome instead of a copy being appended for it.
  const outcomeOf = (line: SentLineType): ImportTokenOutcomeType => {
    if (importFlow && line.flowId === importFlow.flow_id) {
      return importFlow.lines[line.flowIndex] ?? line.outcome;
    }

    return line.outcome;
  };

  const resultRows = $derived(
    sentLines
      .map((line, index) => ({ line, index, outcome: outcomeOf(line) }))
      .filter(({ index }) => !dismissed.has(index)),
  );

  const resultCheckedCount = $derived(
    resultRows.filter(
      ({ outcome }) => outcome.status !== OUTCOME_PENDING && outcome.status !== OUTCOME_CHECKING,
    ).length,
  );

  const importedCount = $derived(
    resultRows.filter(({ outcome }) => outcome.status === OUTCOME_IMPORTED).length,
  );

  // ! Every account this batch actually touched: a brand-new one always,
  // an already-known one only if the user asked to overwrite its stored
  // settings too — same scope the "overwrite existing settings" choice
  // already controls at commit time, carried over to who the setup
  // wizard configures once the import is done.
  const wizardAccounts = $derived(
    resultRows
      .filter(
        ({ line, outcome }) =>
          outcome.status === OUTCOME_IMPORTED && (!line.check?.already_added || overwriteExisting),
      )
      .map(({ outcome }) =>
        outcome.status === OUTCOME_IMPORTED ? accountOf(outcome.account_id) : null,
      )
      .filter((a): a is AccountSnapshotType => a !== null),
  );

  // ! Cancelling a running import leaves the lines it never settled as
  // `PENDING`/`CHECKING` forever (the backend has nothing to flip them
  // to): once the flow stops running, those are dead ends too, exactly
  // like a `FAILED` line — retryable and dismissible the same way.
  const failedRows = $derived(
    resultRows.filter(
      ({ outcome }) =>
        outcome.status === OUTCOME_FAILED ||
        (!importRunning &&
          (outcome.status === OUTCOME_PENDING || outcome.status === OUTCOME_CHECKING)),
    ),
  );

  const dismissFailed = (index: number): void => {
    dismissed = new Set([...dismissed, index]);
  };

  // ! Freezes each line's outcome onto `sentLines` once its own flow
  // settles it (and drops the raw token once imported): a row keeps this
  // frozen outcome past its flow being replaced by a retry's, until that
  // retry re-points it at the new flow and clears the freeze itself.
  $effect(() => {
    const flow = importFlow;

    if (!flow) {
      return;
    }

    sentLines.forEach((line, index) => {
      const outcome = flow.lines[line.flowIndex];

      if (
        line.flowId !== flow.flow_id ||
        !outcome ||
        frozen.has(index) ||
        (flow.status !== ImportTokensFlowStatus.CANCELLED &&
          (outcome.status === OUTCOME_PENDING || outcome.status === OUTCOME_CHECKING))
      ) {
        return;
      }

      sentLines[index] = {
        ...line,
        outcome,
        token: outcome.status === OUTCOME_IMPORTED ? "" : line.token,
      };
      frozen = new Set([...frozen, index]);
    });
  });

  const start = async (
    tokens: { token: string; check: TokenCheckType | null }[],
    retryIndexes: number[] | null,
  ): Promise<void> => {
    committing = true;
    finishing = true;
    commitError = null;

    try {
      const started = await startImportTokens(
        tokens.map((line) => line.token),
        $state.snapshot(patch),
        overwriteExisting,
      );

      if (retryIndexes) {
        retryIndexes.forEach((sentIndex, flowIndex) => {
          sentLines[sentIndex] = {
            ...sentLines[sentIndex],
            outcome: PENDING_OUTCOME,
            flowId: started.flow_id,
            flowIndex,
          };
        });
        frozen = new Set([...frozen].filter((index) => !retryIndexes.includes(index)));
      } else {
        sentLines = [
          ...sentLines,
          ...tokens.map((line, flowIndex) => ({
            ...line,
            outcome: PENDING_OUTCOME,
            flowId: started.flow_id,
            flowIndex,
          })),
        ];
      }

      flowId = started.flow_id;
      lines = [];
      await refresh();
      step = ImportStep.RESULT;
    } catch (e: unknown) {
      commitError = describeError(e);
    } finally {
      committing = false;
      finishing = false;
    }
  };

  const commit = (): Promise<void> =>
    start(
      importableLines.map((line) => ({ token: line.token, check: line.check })),
      null,
    );

  const retryFailed = (): Promise<void> =>
    start(
      failedRows.map(({ line }) => ({ token: line.token, check: line.check })),
      failedRows.map(({ index }) => index),
    );

  const cancelImport = async (): Promise<void> => {
    try {
      await sendCommand({ command: CommandName.CANCEL_IMPORT_TOKENS });
    } catch (e: unknown) {
      reportFailure(e);
    }
  };

  const done = (): void => {
    onclose();
  };

  // ! The dialog closing outright goes through `onclose`/`LoginDialog`'s
  // own cancel-or-dismiss; switching to the sign-in-code step instead
  // unmounts this component without going through `onclose`, so it
  // cancels a running import itself before handing off.
  const useCode = (): void => {
    if (importRunning) {
      void cancelImport();
    }

    onusecode();
  };

  // ! Stops the validation loop and drops whatever check is in flight on unmount.
  $effect(() => {
    return () => {
      cancelRequested = true;
      inflight?.abort();
    };
  });
</script>

{#snippet lineRow(line: LineType, onremove: (() => void) | undefined)}
  {@const clientWarning = line.check !== null && hasClientWarning(line.check)}
  <li
    class={cn(
      "flex flex-col gap-1 rounded-md border px-2 py-1.5 text-hint",
      STATUS_CLASSES[line.status],
    )}
  >
    <div class="flex items-center gap-2">
      {#if line.status === ImportLineStatus.CHECKING}
        <Spinner size={14} />
      {:else if line.status === ImportLineStatus.VALID || line.status === ImportLineStatus.ALREADY_ADDED}
        <Icon name={IconName.CHECK} size={14} />
      {:else if line.status === ImportLineStatus.DUPLICATE}
        <Icon name={IconName.COPY} size={14} />
      {:else if line.status === ImportLineStatus.NOT_IMPORTED}
        <Icon name={IconName.MINUS} size={14} />
      {:else}
        <Icon name={IconName.X} size={14} />
      {/if}
      {#if line.check}
        <AvatarImage
          url={line.check.avatar_url}
          name={line.check.display_name}
          class="size-5.5 shrink-0 rounded-md bg-muted text-initial-sm font-semibold text-muted-foreground uppercase"
        />
      {/if}
      <span class="min-w-0 flex-1 truncate font-medium">
        {line.check?.display_name ?? (line.token ? maskToken(line.token) : "")}
      </span>
      <span class="min-w-0 shrink truncate text-muted-foreground">{lineLabel(line)}</span>
      {#if onremove}
        <IconButton
          icon={IconName.X}
          label={t.common_remove()}
          busy={false}
          disabled={false}
          variant={ButtonVariant.GHOST}
          size={ButtonSize.SM}
          onclick={onremove}
        />
      {/if}
    </div>
    {#if clientWarning}
      <p class="flex items-start gap-1.5 pl-[22px] text-muted-foreground">
        <Icon name={IconName.ALERT} size={12} class="mt-0.5 shrink-0" />
        <span class="min-w-0">{t.import_tokens_client_warning()}</span>
      </p>
    {/if}
  </li>
{/snippet}

{#if step === ImportStep.LIST}
  {#if lines.length === 0}
    <div class="flex flex-col gap-2">
      <div class="relative">
        <Textarea
          id="import-tokens-paste"
          rows={6}
          placeholder={t.import_tokens_placeholder()}
          aria-label={t.login_tokens_label()}
          spellcheck={false}
          autocomplete="off"
          autocapitalize="off"
          autocorrect="off"
          class={cn("pr-9", !pasteShown && pasted !== "" && "[font-family:text-security-disc]")}
          bind:value={pasted}
          oninput={() => (pasteError = null)}
        />
        <button
          type="button"
          class="absolute top-1.5 right-1.5 flex size-7 items-center justify-center rounded-md text-muted-foreground hover:bg-muted hover:text-foreground"
          aria-controls="import-tokens-paste"
          aria-pressed={pasteShown}
          aria-label={pasteShown ? t.secret_hide() : t.secret_show()}
          onclick={() => (pasteShown = !pasteShown)}
        >
          <Icon name={pasteShown ? IconName.EYE_OFF : IconName.EYE} size={15} />
        </button>
      </div>
      <p class="text-hint text-muted-foreground">{t.login_tokens_hint()}</p>
      {#if pasteError}
        <p class="text-destructive"><RichText text={pasteError} /></p>
      {/if}
    </div>
  {:else}
    <ul
      class="m-0 flex max-h-72 flex-col gap-1.5 overflow-y-auto rounded-lg border border-input bg-input/30 p-2"
      aria-label={t.import_tokens_title()}
      aria-busy={checking}
    >
      {#each lines as line, index (index)}
        {@render lineRow(line, undefined)}
      {/each}
    </ul>
    <span class="text-hint text-muted-foreground tabular-nums" aria-live="polite">
      <RichText
        text={t.import_progress({ done: strong(checkedCount), total: strong(lines.length) })}
      />
    </span>
  {/if}
{:else if step === ImportStep.SETTINGS}
  <div class="flex flex-col gap-1">
    <h3 class="text-base font-semibold">
      {t.import_tokens_settings_step_title({ count: importableLines.length })}
    </h3>
    <p class="text-hint text-muted-foreground">
      {t.import_tokens_settings_step_hint({ count: importableLines.length })}
    </p>
  </div>
  <div class="flex flex-wrap gap-1.5">
    {#each importableLines as line, index (index)}
      <span
        class="inline-flex items-center gap-1.5 rounded-full border bg-card px-2 py-1 text-hint"
      >
        <AvatarImage
          url={line.check?.avatar_url}
          name={line.check?.display_name ?? maskToken(line.token)}
          class="size-5 shrink-0 rounded-full bg-muted text-initial-xs font-semibold text-muted-foreground uppercase"
        />
        {line.check?.display_name ?? maskToken(line.token)}
      </span>
    {/each}
  </div>
  <ProxyField
    id="import-tokens-proxy"
    label={t.account_proxy({ count: importableLines.length })}
    hint={t.account_proxy_hint({ count: importableLines.length })}
    proxy={patch.proxy}
    onProxyChange={(proxy) => (patch = { ...patch, proxy })}
    proxyPlaceholder={maskProxyCredentials(app.snapshot?.settings.default_proxy ?? "") ||
      PROXY_EXAMPLE}
  />
  {#if hasAlreadyAdded}
    <Setting
      title={t.import_tokens_overwrite_existing_label({ count: alreadyAddedCount })}
      checked={overwriteExisting}
      disabled={false}
      onchange={(value) => (overwriteExisting = value)}
    />
  {/if}
  {#if commitError}
    <p class="text-destructive">{commitError}</p>
  {/if}
{:else if step === ImportStep.RESULT}
  {#if importRunning}
    <span class="text-hint text-muted-foreground tabular-nums" aria-live="polite">
      <RichText
        text={t.import_tokens_importing_progress({
          done: strong(resultCheckedCount),
          total: strong(resultRows.length),
        })}
      />
    </span>
  {:else if importCancelled}
    <p class="flex items-center gap-2 text-muted-foreground">
      <Icon name={IconName.X} size={18} />
      <RichText
        text={t.import_tokens_result_cancelled({
          count: importedCount,
          formattedCount: strong(importedCount),
        })}
      />
    </p>
  {:else if importedCount > 0}
    <p class="flex items-center gap-2 text-success-foreground">
      <Icon name={IconName.CHECK} size={18} />
      <RichText
        text={t.import_tokens_result_count({
          count: importedCount,
          formattedCount: strong(importedCount),
        })}
      />
    </p>
  {:else}
    <p class="flex items-center gap-2 text-muted-foreground">
      <Icon name={IconName.X} size={18} />
      <RichText
        text={t.import_tokens_result_count({
          count: importedCount,
          formattedCount: strong(importedCount),
        })}
      />
    </p>
  {/if}
  {#if commitError}
    <p class="text-destructive">{commitError}</p>
  {/if}
  {#if resultRows.length > 0}
    <ul class="m-0 flex flex-col gap-1.5">
      {#each resultRows as { line, index, outcome } (index)}
        {@const unsettled =
          outcome.status === OUTCOME_PENDING || outcome.status === OUTCOME_CHECKING}
        {@const dead = unsettled && !importRunning}
        {@const status =
          outcome.status === OUTCOME_IMPORTED
            ? ImportLineStatus.VALID
            : outcome.status === OUTCOME_FAILED
              ? ImportLineStatus.OTHER_ERROR
              : dead
                ? ImportLineStatus.NOT_IMPORTED
                : ImportLineStatus.CHECKING}
        {@const reason = outcome.status === OUTCOME_FAILED ? describeCode(outcome.problem) : null}
        {@render lineRow(
          { token: line.token, status, check: line.check, reason },
          outcome.status === OUTCOME_FAILED || dead ? () => dismissFailed(index) : undefined,
        )}
      {/each}
    </ul>
  {/if}
{:else if step === ImportStep.WIZARD}
  <SetupWizard accounts={wizardAccounts} onclose={done} bind:finishing bind:actions />
{/if}

{#snippet footer()}
  {#if step === ImportStep.LIST}
    {#if checking}
      <TextButton
        label={t.common_cancel()}
        icon={IconName.X}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={cancelChecking}
      />
    {:else}
      <TextButton
        label={t.common_back()}
        icon={IconName.ARROW_LEFT}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={lines.length > 0 ? backToPaste : useCode}
      />
    {/if}
    {#if !checking && checkAgainCount > 0}
      <TextButton
        label={t.import_check_again({ count: checkAgainCount })}
        icon={IconName.REFRESH}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => void checkAgain()}
      />
    {/if}
    <ActionButton
      icon={IconName.CHECK}
      label={t.common_next()}
      busyLabel={t.login_tokens_checking()}
      busy={checking}
      disabled={!canAdvance}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={advance}
    />
  {:else if step === ImportStep.SETTINGS}
    <TextButton
      label={t.common_back()}
      icon={IconName.ARROW_LEFT}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={committing}
      onclick={() => (step = ImportStep.LIST)}
    />
    <ActionButton
      icon={IconName.CHECK}
      label={t.login_tokens_submit({ count: importableLines.length })}
      busyLabel={t.common_saving()}
      busy={committing}
      disabled={committing || importableLines.length === 0}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={commit}
    />
  {:else if importRunning}
    <TextButton
      label={t.import_tokens_cancel()}
      icon={IconName.X}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => void cancelImport()}
    />
  {:else}
    {#if failedRows.length > 0}
      <TextButton
        label={t.import_tokens_retry_failed()}
        icon={IconName.REFRESH}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={committing}
        onclick={() => void retryFailed()}
      />
    {/if}
    {#if wizardAccounts.length > 0}
      <TextButton
        label={t.common_close()}
        icon={IconName.X}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={done}
      />
      <ActionButton
        icon={IconName.CHECK}
        label={t.import_tokens_configure({ count: wizardAccounts.length })}
        busyLabel={t.common_saving()}
        busy={false}
        disabled={false}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        onclick={() => (step = ImportStep.WIZARD)}
      />
    {:else}
      <TextButton
        label={t.common_close()}
        icon={IconName.X}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={done}
      />
    {/if}
  {/if}
{/snippet}
