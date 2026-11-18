<script lang="ts">
  import type { Snippet } from "svelte";

  import { SetupEnd, SetupSource, SetupStep } from "../../lib/accounts/accounts.constants";
  import { newStreamer } from "../../lib/accounts/import.svelte";
  import { sameMiningOrder } from "../../lib/accounts/mining-order.utils";
  import { dropsScopeChoices } from "../../lib/accounts/scope.utils";
  import type { AccountSettingsType } from "../../lib/bindings/AccountSettingsType";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { ChannelType } from "../../lib/bindings/ChannelType";
  import type { DropsScope } from "../../lib/bindings/DropsScope";
  import type { MiningLaneType } from "../../lib/bindings/MiningLaneType";
  import { Mode } from "../../lib/bindings/Mode";
  import type { StreamerSettingsType } from "../../lib/bindings/StreamerSettingsType";
  import {
    ensureChannels,
    knownChannels,
    rememberChannels,
  } from "../../lib/channels/channels.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { saveAccountSettings } from "../../lib/state/account-settings.svelte";
  import { account as accountOf, app } from "../../lib/state/app.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { toast } from "../../lib/state/toasts";
  import { personMarker } from "../../lib/text.utils";
  import AvatarImage from "../ui/AvatarImage.svelte";
  import ChoiceCards from "../ui/ChoiceCards.svelte";
  import FormField from "../ui/FormField.svelte";
  import RichText from "../ui/RichText.svelte";
  import Select from "../ui/Select.svelte";
  import Setting from "../ui/Setting.svelte";
  import { MenuAlign } from "../ui/menu.constants";
  import SetupActions from "./SetupActions.svelte";
  import MiningOrderList from "../account/drops/MiningOrderList.svelte";
  import SetupStepper from "./SetupStepper.svelte";
  import SetupStreamersStep from "./SetupStreamersStep.svelte";
  import SetupSummary from "./SetupSummary.svelte";
  import StreamerChip from "./StreamerChip.svelte";

  // ! One account after a device-code login or several after a token
  // import's accounts all exist by the time this mounts: every step below
  // reasons about `accounts` as a batch, a single login being the
  // batch-of-one case.
  type SetupWizardPropsType = {
    accounts: AccountSnapshotType[];
    onclose: () => void;
    finishing: boolean;
    actions: Snippet | undefined;
  };

  let {
    accounts,
    onclose,
    finishing = $bindable(),
    actions = $bindable(),
  }: SetupWizardPropsType = $props();

  // ! The steps' buttons go up to the dialog's footer, which stays in view while the step scrolls.
  $effect.pre(() => {
    actions = footer;

    return () => {
      actions = undefined;
    };
  });

  // ! An account list can empty out from under this step (e.g. the accounts
  // it configures get removed elsewhere while it is open): nothing left to
  // configure, so the wizard closes itself instead of indexing `accounts[0]`.
  $effect(() => {
    if (accounts.length === 0) {
      onclose();
    }
  });

  const others = $derived(
    (app.snapshot?.accounts ?? []).filter((a) => !accounts.some((subject) => subject.id === a.id)),
  );

  let picked = $state<SetupStep | null>(null);
  let source = $state<SetupSource | string>(SetupSource.DEFAULTS);

  const sourceAccount = $derived(
    source === SetupSource.DEFAULTS || accounts.some((subject) => subject.id === source)
      ? null
      : accountOf(source),
  );

  const base = $derived(sourceAccount?.settings ?? accounts[0].settings);

  let pickedMode = $state<Mode | null>(null);
  let pickedScope = $state<DropsScope | null>(null);
  let pickedOrder = $state<MiningLaneType[] | null>(null);
  let pickedAutoClaim = $state<boolean | null>(null);
  let pickedStreamers = $state<StreamerSettingsType[] | null>(null);
  let streamersStep = $state<ReturnType<typeof SetupStreamersStep>>();

  const mode = $derived(pickedMode ?? base.mode);
  const scope = $derived(pickedScope ?? base.drops.scope);
  const miningOrder = $derived(pickedOrder ?? base.drops.mining_order);
  const autoClaim = $derived(pickedAutoClaim ?? base.drops.auto_claim);
  const streamers = $derived(pickedStreamers ?? base.points.streamers);
  const importAction = $derived(streamersStep?.importAction() ?? null);

  const sameLogins = (a: StreamerSettingsType[], b: StreamerSettingsType[]): boolean =>
    a.length === b.length && a.every((item, index) => item.login === b[index].login);

  const customized = $derived(
    mode !== base.mode ||
      scope !== base.drops.scope ||
      !sameMiningOrder(miningOrder, base.drops.mining_order) ||
      autoClaim !== base.drops.auto_claim ||
      !sameLogins(streamers, base.points.streamers),
  );

  const sourceOptions = $derived([
    { value: SetupSource.DEFAULTS, label: t.login_setup_defaults() },
    ...others.map((a) => ({
      value: a.id,
      label: t.login_setup_copy_from({ name: a.display_name }),
      image: a.avatar_url,
    })),
    ...(customized ? [{ value: SetupSource.CUSTOM, label: t.login_setup_custom() }] : []),
  ]);

  const pickSource = (value: SetupSource | string): void => {
    if (value === SetupSource.CUSTOM) {
      return;
    }

    source = value;
    pickedMode = null;
    pickedScope = null;
    pickedOrder = null;
    pickedAutoClaim = null;
    pickedStreamers = null;
  };

  const scopeChoices = $derived(dropsScopeChoices());

  const channelFor = (login: string): ChannelType | undefined =>
    sourceAccount?.streamers.find((view) => view.login === login)?.channel ?? knownChannels[login];

  $effect(() => {
    ensureChannels(streamers.map((s) => s.login).filter((login) => !channelFor(login)));
  });

  const addStreamers = (logins: string[], channels: ChannelType[]): void => {
    rememberChannels(channels);
    pickedStreamers = [
      ...$state.snapshot(streamers),
      ...logins
        .filter((login) => !streamers.some((s) => s.login === login))
        .map((login) => newStreamer(base.points.streamer_defaults, login)),
    ];
  };

  const removeStreamer = (login: string): void => {
    pickedStreamers = $state.snapshot(streamers).filter((item) => item.login !== login);
  };

  const steps = $derived([
    ...(others.length > 0 ? [SetupStep.START] : []),
    SetupStep.MODE,
    ...(mode !== Mode.POINTS ? [SetupStep.DROPS] : []),
    ...(mode !== Mode.DROPS ? [SetupStep.STREAMERS] : []),
    SetupStep.SUMMARY,
  ]);

  const step = $derived(picked !== null && steps.includes(picked) ? picked : steps[0]);
  const stepIndex = $derived(steps.indexOf(step));

  const goTo = (index: number): void => {
    picked = steps[Math.max(0, Math.min(steps.length - 1, index))];
  };

  let ending = $state<SetupEnd | null>(null);

  const finish = async (end: SetupEnd): Promise<void> => {
    const picked = {
      mode,
      scope,
      miningOrder: $state.snapshot(miningOrder),
      autoClaim,
      streamers: $state.snapshot(streamers),
    };
    // ! Each account keeps its own proxy (already its batch's, for one
    // just imported; its own, for one re-logged-in or re-imported): the
    // wizard never has a proxy step of its own, see `ImportTokensDialog`
    // for where a token import's batch proxy is chosen instead.
    const settingsFor = (subject: AccountSnapshotType): AccountSettingsType => {
      const subjectBase = $state.snapshot(sourceAccount?.settings ?? subject.settings);

      return {
        ...subjectBase,
        mode: picked.mode,
        drops: {
          ...subjectBase.drops,
          scope: picked.scope,
          mining_order: picked.miningOrder,
          auto_claim: picked.autoClaim,
        },
        points: { ...subjectBase.points, streamers: picked.streamers },
        proxy: subject.settings.proxy,
        enabled: end === SetupEnd.START,
      };
    };

    ending = end;
    finishing = true;
    const saved = await Promise.all(
      accounts.map((subject) => saveAccountSettings(subject.id, settingsFor(subject), undefined)),
    );
    finishing = false;
    ending = null;

    if (saved.every(Boolean)) {
      toast(ToastKind.SUCCESS, t.common_saved());
      onclose();
    }
  };
</script>

{#snippet stepTitle(title: string, hint: string)}
  <div class="flex flex-col gap-1">
    <h3 class="text-base font-semibold">{title}</h3>
    <p class="text-hint text-muted-foreground"><RichText text={hint} /></p>
  </div>
{/snippet}

{#if accounts.length > 0}
  <div class="flex flex-col gap-5">
    <p class="flex items-center gap-2 text-hint text-muted-foreground">
      {#if accounts.length === 1}
        <RichText
          text={t.login_setup_for({ name: personMarker(0) })}
          people={[
            { name: accounts[0].display_name, avatar: accounts[0].avatar_url, channel: null },
          ]}
        />
      {:else}
        <span class="flex -space-x-2">
          {#each accounts.slice(0, 5) as subject (subject.id)}
            <AvatarImage
              url={subject.avatar_url}
              name={subject.display_name}
              class="size-6 shrink-0 rounded-full bg-muted text-tiny font-semibold text-muted-foreground uppercase ring-2 ring-background"
            />
          {/each}
        </span>
        <span>{t.login_setup_for_many({ count: accounts.length })}</span>
      {/if}
    </p>
    <SetupStepper {steps} current={stepIndex} disabled={finishing} ongo={goTo} />

    {#if step === SetupStep.START}
      {@render stepTitle(t.login_setup_step_start(), t.login_setup_start_hint())}
      <FormField
        id="login-setup-from"
        label={t.login_setup_from()}
        hint={t.login_setup_copy_hint()}
      >
        <Select
          id="login-setup-from"
          value={customized ? SetupSource.CUSTOM : sourceAccount ? source : SetupSource.DEFAULTS}
          options={sourceOptions}
          label={t.login_setup_from()}
          align={MenuAlign.LEFT}
          onchange={pickSource}
        />
      </FormField>
    {:else if step === SetupStep.MODE}
      {@render stepTitle(t.account_mode_title(), t.account_mode_desc({ count: accounts.length }))}
      <ChoiceCards
        vertical={true}
        value={mode}
        label={t.account_mode_title()}
        onchange={(value) => (pickedMode = value)}
        choices={[
          { value: Mode.AUTO, title: t.mode_auto_title(), description: t.mode_auto_hint() },
          { value: Mode.DROPS, title: t.mode_drops_title(), description: t.mode_drops_hint() },
          { value: Mode.POINTS, title: t.mode_points_title(), description: t.mode_points_hint() },
        ]}
      />
    {:else if step === SetupStep.DROPS}
      {@render stepTitle(t.drops_scope(), t.login_setup_drops_hint({ count: accounts.length }))}
      <div class="flex flex-col gap-2">
        <h4 class="text-sm font-semibold">{t.mining_order_title()}</h4>
        <p class="text-hint text-muted-foreground">{t.mining_order_hint()}</p>
        <MiningOrderList order={miningOrder} onchange={(order) => (pickedOrder = order)} />
      </div>
      <ChoiceCards
        vertical={true}
        value={scope}
        label={t.drops_scope()}
        onchange={(value) => (pickedScope = value)}
        choices={scopeChoices}
      />
      <Setting
        title={t.account_drops_auto_claim()}
        description={t.drops_auto_claim_hint()}
        checked={autoClaim}
        disabled={false}
        onchange={(value) => (pickedAutoClaim = value)}
      />
    {:else if step === SetupStep.STREAMERS}
      {@render stepTitle(
        t.login_setup_streamers_title(),
        t.login_setup_streamers_hint({ count: accounts.length }),
      )}
      <SetupStreamersStep
        bind:this={streamersStep}
        account={accounts[0]}
        {streamers}
        {channelFor}
        onadd={addStreamers}
        onremove={removeStreamer}
      />
    {:else}
      {@render stepTitle(
        t.login_setup_step_summary(),
        t.login_setup_summary_hint({ count: accounts.length }),
      )}
      <SetupSummary source={sourceAccount} {mode} {scope} {miningOrder} {autoClaim} {streamers} />
      {#if mode !== Mode.DROPS && streamers.length > 0}
        <div class="flex flex-wrap gap-1.5">
          {#each streamers as streamer (streamer.login)}<StreamerChip
              login={streamer.login}
              channel={channelFor(streamer.login)}
            />{/each}
        </div>
      {/if}
      <p class="text-hint text-muted-foreground">
        {t.login_setup_later_hint({ count: accounts.length })}
      </p>
    {/if}
  </div>
{/if}

{#snippet footer()}
  {#if accounts.length > 0}
    <SetupActions
      {step}
      {stepIndex}
      total={steps.length}
      {importAction}
      {ending}
      ongo={goTo}
      onfinish={finish}
    />
  {/if}
{/snippet}
