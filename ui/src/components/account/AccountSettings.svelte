<script lang="ts">
  import PanelColumns from "../ui/PanelColumns.svelte";
  import { AccountSource } from "../../lib/bindings/AccountSource";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import { CommandName } from "../../lib/bindings/CommandName";
  import { untrack } from "svelte";

  import { PauseScope } from "../../lib/accounts/account-state.constants";
  import { maskProxyCredentials } from "../../lib/accounts/accounts.utils";
  import {
    copyOptions,
    differingOptions,
    sameOptions,
    STREAMER_OPTIONS,
  } from "../../lib/accounts/options.utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { maskId } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { personMarker } from "../../lib/text.utils";
  import { AccountField, Page } from "../../lib/state/state.constants";
  import { editsOf } from "../../lib/state/edits.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { navigate } from "../../lib/state/router";
  import { run } from "../../lib/state/session";
  import TextButton from "../ui/TextButton.svelte";
  import CopyButton from "../ui/CopyButton.svelte";
  import ConfirmDialog from "../ui/ConfirmDialog.svelte";
  import Meta from "../ui/Meta.svelte";
  import AccountHeading from "../ui/AccountHeading.svelte";
  import PageBody from "../ui/PageBody.svelte";
  import PauseControl from "../ui/PauseControl.svelte";
  import Panel from "../ui/Panel.svelte";
  import Setting from "../ui/Setting.svelte";
  import AccountSettingsForm from "./AccountSettingsForm.svelte";
  import AccountSaveBar from "./AccountSaveBar.svelte";
  import StreamerOptions from "./StreamerOptions.svelte";
  import { PROXY_EXAMPLE } from "./account.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { MenuAlign } from "../ui/menu.constants";
  import { CopyLayout, MetaLayout, PanelWidth } from "../ui/ui.constants";
  import { IconName } from "../../lib/icons.constants";

  type AccountSettingsPropsType = { account: AccountSnapshotType; snapshot: SnapshotType };

  let { account, snapshot }: AccountSettingsPropsType = $props();

  const edits = editsOf(untrack(() => account.id));
  const shown = $derived(edits.applied(account.settings));

  let defaults = $state(structuredClone(untrack(() => shown.points.streamer_defaults)));

  $effect(() => {
    const next = shown.points.streamer_defaults;

    untrack(() => {
      if (!sameOptions(next, defaults)) {
        defaults = structuredClone(next);
      }
    });
  });

  const setDefaults = (): void => {
    const next = $state.snapshot(defaults);
    const changes = differingOptions(
      next,
      account.settings.points.streamer_defaults,
      STREAMER_OPTIONS,
    );

    edits.set(AccountField.STREAMER_DEFAULTS, (settings) =>
      copyOptions(next, settings.points.streamer_defaults, changes),
    );
  };

  let confirming = $state(false);
  let removing = $state(false);
  let revokeToken = $state(true);

  const remove = async (): Promise<void> => {
    removing = true;

    const removed = await run({
      command: CommandName.REMOVE_ACCOUNT,
      account_id: account.id,
      revoke_token: revokeToken,
    });

    removing = false;

    if (removed) {
      confirming = false;
      navigate(Page.HOME);
    }
  };
</script>

{#snippet copy(field: string, value: string)}
  <CopyButton
    text={() => value}
    label={t.common_copy()}
    copiedLabel={t.common_copied()}
    name={t.common_copy_named({ name: field })}
    layout={CopyLayout.ICON}
    size={ButtonSize.SM}
  />
{/snippet}

<PageBody>
  <Panel title={t.account_twitch_title()}>
    <AccountHeading
      {account}
      subtitle={account.status === AccountStatus.PAUSED
        ? t.account_state_paused_hint()
        : t.account_state_running_hint()}
    >
      {#snippet actions()}
        <PauseControl
          accounts={[account]}
          scope={PauseScope.ACCOUNT}
          align={MenuAlign.LEFT}
          size={ButtonSize.MD}
        />
        <TextButton
          label={t.accounts_login_again()}
          icon={IconName.REFRESH}
          variant={ButtonVariant.OUTLINE}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => (app.loginDialog = true)}
        />
        <TextButton
          label={t.account_remove()}
          icon={IconName.TRASH}
          variant={ButtonVariant.DANGER}
          size={ButtonSize.MD}
          type={ButtonType.BUTTON}
          disabled={false}
          onclick={() => {
            revokeToken = true;
            confirming = true;
          }}
        />
      {/snippet}
    </AccountHeading>
    <dl
      class="m-0 grid grid-cols-[repeat(auto-fit,minmax(min(200px,100%),1fr))] gap-x-6 gap-y-3 border-t pt-4"
    >
      <Meta
        label={t.account_twitch_name()}
        value={account.display_name}
        layout={MetaLayout.STACKED}
        mono={false}
      >
        {#snippet after()}{@render copy(t.account_twitch_name(), account.display_name)}{/snippet}
      </Meta>
      <Meta
        label={t.account_twitch_login()}
        value={account.login}
        layout={MetaLayout.STACKED}
        mono={true}
      >
        {#snippet after()}{@render copy(t.account_twitch_login(), account.login)}{/snippet}
      </Meta>
      <Meta
        label={t.account_twitch_id()}
        value={maskId(account.id)}
        layout={MetaLayout.STACKED}
        mono={true}
      >
        {#snippet after()}{@render copy(t.account_twitch_id(), account.id)}{/snippet}
      </Meta>
    </dl>
    <p class="text-hint text-muted-foreground">
      {account.source === AccountSource.IMPORTED_TOKEN
        ? t.account_twitch_hint_imported_token()
        : t.account_twitch_hint()}
    </p>
  </Panel>

  <PanelColumns width={PanelWidth.MD}>
    <AccountSettingsForm
      mode={shown.mode}
      onModeChange={(mode) => edits.set(AccountField.MODE, (settings) => (settings.mode = mode))}
      miningOrder={shown.drops.mining_order}
      onMiningOrderChange={(order) =>
        edits.set(AccountField.MINING_ORDER, (settings) => (settings.drops.mining_order = order))}
      slots={shown.slots}
      onSlotsChange={(slots) =>
        edits.set(AccountField.SLOTS, (settings) => (settings.slots = slots))}
      proxy={shown.proxy}
      onProxyChange={(value) =>
        edits.set(AccountField.PROXY, (settings) => (settings.proxy = value))}
      proxyPlaceholder={maskProxyCredentials(snapshot.settings.default_proxy) || PROXY_EXAMPLE}
    />
    <Panel title={t.account_claims_priorities_title()}>
      <div>
        <Setting
          title={t.account_drops_auto_claim()}
          description={t.drops_auto_claim_hint()}
          checked={shown.drops.auto_claim}
          disabled={false}
          onchange={(value) =>
            edits.set(AccountField.AUTO_CLAIM, (settings) => (settings.drops.auto_claim = value))}
        />
        <Setting
          title={t.account_points_streak_priority()}
          description={t.points_streak_priority_hint()}
          checked={shown.points.streak_priority}
          disabled={false}
          onchange={(value) =>
            edits.set(
              AccountField.STREAK_PRIORITY,
              (settings) => (settings.points.streak_priority = value),
            )}
        />
        <Setting
          title={t.points_subscriber_priority()}
          description={t.points_subscriber_priority_hint()}
          checked={shown.points.subscriber_priority}
          disabled={false}
          onchange={(value) =>
            edits.set(
              AccountField.SUBSCRIBER_PRIORITY,
              (settings) => (settings.points.subscriber_priority = value),
            )}
        />
      </div>
    </Panel>
  </PanelColumns>
  <Panel title={t.points_streamer_defaults()} description={t.points_streamer_defaults_hint()}>
    <StreamerOptions bind:options={defaults} mixed={[]} onchange={setDefaults} />
  </Panel>
  <AccountSaveBar {account} />
</PageBody>

{#snippet revokeLead()}
  {#if account.source === AccountSource.IMPORTED_TOKEN}
    <Setting
      title={t.account_remove_revoke_label()}
      checked={revokeToken}
      disabled={removing}
      onchange={(value) => (revokeToken = value)}
    />
  {/if}
{/snippet}

<ConfirmDialog
  open={confirming}
  title={t.accounts_remove_title()}
  text={account.source === AccountSource.IMPORTED_TOKEN
    ? t.accounts_remove_text_imported_token({ name: personMarker(0) })
    : t.accounts_remove_text({ name: personMarker(0) })}
  people={[{ name: account.display_name, avatar: account.avatar_url, channel: null }]}
  confirmLabel={t.common_remove()}
  busyLabel={t.common_removing()}
  busy={removing}
  extra={revokeLead}
  onconfirm={remove}
  oncancel={() => (confirming = false)}
/>
