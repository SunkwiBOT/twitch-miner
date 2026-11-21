<script lang="ts">
  import { isTauri } from "../../lib/api/tauri";
  import { testNotificationTarget } from "../../lib/api/client";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import type { NotificationTestRequestType } from "../../lib/bindings/NotificationTestRequestType";
  import { NotificationTarget } from "../../lib/bindings/NotificationTarget";
  import { describeCode, describeError } from "../../lib/errors/errors.utils";
  import { languageOptions, t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { toast } from "../../lib/state/toasts";
  import { escapeEmphasis, strong } from "../../lib/text.utils";
  import { Input } from "$lib/components/ui/input";
  import BrowserNotificationsAccess from "./BrowserNotificationsAccess.svelte";
  import ActionButton from "../ui/ActionButton.svelte";
  import FieldActions from "../ui/FieldActions.svelte";
  import FormField from "../ui/FormField.svelte";
  import Icon from "../ui/Icon.svelte";
  import Panel from "../ui/Panel.svelte";
  import SecretInput from "../ui/SecretInput.svelte";
  import Select from "../ui/Select.svelte";
  import Setting from "../ui/Setting.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MenuAlign } from "../ui/menu.constants";
  import { FIELD_GRID_CLASSES } from "./settings.constants";
  import { NOTIFY_KEYS, NOTIFY_LABELS, TARGET_NAMES } from "./settings.utils";

  type SettingsNotificationsPropsType = {
    draft: GlobalSettingsType;
  };

  let { draft = $bindable() }: SettingsNotificationsPropsType = $props();

  const languages = $derived(languageOptions());

  let testing = $state<Record<NotificationTarget, boolean>>({
    [NotificationTarget.DISCORD]: false,
    [NotificationTarget.TELEGRAM]: false,
    [NotificationTarget.WEBHOOK]: false,
  });

  const testTarget = async (request: NotificationTestRequestType): Promise<void> => {
    testing[request.target] = true;

    try {
      const result = await testNotificationTarget(request);
      const target = TARGET_NAMES[result.target]();

      if (result.problem) {
        toast(ToastKind.ERROR, t.settings_test_failed_title(), {
          description: t.settings_test_failed({
            service: strong(target),
            error: escapeEmphasis(describeCode(result.problem)),
          }),
        });
      } else {
        toast(ToastKind.SUCCESS, t.settings_test_ok_title(), {
          description: t.settings_test_ok({ service: strong(target) }),
        });
      }
    } catch (e: any) {
      toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
    } finally {
      testing[request.target] = false;
    }
  };
</script>

{#snippet settings_discordMark()}<span class="text-brand-discord">
    <Icon name={IconName.DISCORD} size={16} />
  </span>{/snippet}

{#snippet settings_telegramMark()}<span class="text-brand-telegram">
    <Icon name={IconName.TELEGRAM} size={16} />
  </span>{/snippet}

{#snippet settings_webhookMark()}<span class="text-muted-foreground">
    <Icon name={IconName.WEBHOOK} size={16} />
  </span>{/snippet}

{#snippet settings_system_notificationsMark()}<span class="text-muted-foreground">
    <Icon name={IconName.BELL} size={16} />
  </span>{/snippet}

{#snippet discordField()}
  <SecretInput
    id="n-discord"
    class="font-mono"
    autocomplete="off"
    bind:value={draft.notifications.discord.webhook_url}
    placeholder="https://discord.com/api/webhooks/…"
  />
{/snippet}

{#snippet discordTest()}
  <ActionButton
    icon={IconName.DISCORD}
    label={t.settings_discord_test()}
    ariaLabel={`${t.settings_discord_test()} ${t.settings_discord()}`}
    busyLabel={t.common_testing()}
    busy={testing[NotificationTarget.DISCORD]}
    disabled={!draft.notifications.discord.webhook_url.trim()}
    variant={ButtonVariant.DEFAULT}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    onclick={() =>
      void testTarget({
        target: "DISCORD",
        webhook_url: draft.notifications.discord.webhook_url,
      })}
  />
{/snippet}

<Panel title={t.settings_notifications()}>
  <div class="grid gap-x-8 gap-y-5 @3xl:grid-cols-[minmax(0,2fr)_minmax(0,3fr)]">
    <section class="flex min-w-0 flex-col">
      <h3 class="pb-2 text-sm font-semibold">{t.settings_notifications_targets()}</h3>
      <div class="flex flex-col *:border-t">
        <div class="flex flex-col">
          <Setting
            title={t.settings_discord()}
            lead={settings_discordMark}
            checked={draft.notifications.discord.enabled}
            disabled={false}
            onchange={(enabled) => (draft.notifications.discord.enabled = enabled)}
          />
          {#if draft.notifications.discord.enabled}
            <div class="pb-3">
              <FormField id="n-discord" label={t.settings_discord_url()}>
                <FieldActions field={discordField} actions={discordTest} />
              </FormField>
            </div>
          {/if}
        </div>
        <div class="flex flex-col">
          <Setting
            title={t.settings_telegram()}
            lead={settings_telegramMark}
            checked={draft.notifications.telegram.enabled}
            disabled={false}
            onchange={(enabled) => (draft.notifications.telegram.enabled = enabled)}
          />
          {#if draft.notifications.telegram.enabled}
            <div class="pb-3">
              <div class={FIELD_GRID_CLASSES}>
                <FormField id="n-tg-token" label={t.settings_telegram_token()}>
                  <SecretInput
                    id="n-tg-token"
                    class="font-mono"
                    autocomplete="off"
                    bind:value={draft.notifications.telegram.bot_token}
                  />
                </FormField>
                <FormField id="n-tg-chat" label={t.settings_telegram_chat()}>
                  <Input
                    id="n-tg-chat"
                    class="font-mono"
                    bind:value={draft.notifications.telegram.chat_id}
                  />
                </FormField>
              </div>
              <div class="flex flex-wrap justify-end gap-2 pt-2">
                <ActionButton
                  icon={IconName.TELEGRAM}
                  label={t.settings_telegram_test()}
                  ariaLabel={`${t.settings_telegram_test()} ${t.settings_telegram()}`}
                  busyLabel={t.common_testing()}
                  busy={testing[NotificationTarget.TELEGRAM]}
                  disabled={!draft.notifications.telegram.bot_token.trim() ||
                    !draft.notifications.telegram.chat_id.trim()}
                  variant={ButtonVariant.DEFAULT}
                  size={ButtonSize.MD}
                  type={ButtonType.BUTTON}
                  onclick={() =>
                    void testTarget({
                      target: "TELEGRAM",
                      bot_token: draft.notifications.telegram.bot_token,
                      chat_id: draft.notifications.telegram.chat_id,
                    })}
                />
              </div>
            </div>
          {/if}
        </div>
        <div class="flex flex-col">
          <Setting
            title={t.settings_webhook()}
            lead={settings_webhookMark}
            checked={draft.notifications.webhook.enabled}
            disabled={false}
            onchange={(enabled) => (draft.notifications.webhook.enabled = enabled)}
          />
          {#if draft.notifications.webhook.enabled}
            <div class="pb-3">
              <div class={FIELD_GRID_CLASSES}>
                <FormField id="n-wh" label={t.settings_webhook_url()}>
                  <SecretInput
                    id="n-wh"
                    class="font-mono"
                    autocomplete="off"
                    bind:value={draft.notifications.webhook.url}
                  />
                </FormField>
                <FormField id="n-wh-auth" label={t.settings_webhook_auth()}>
                  <SecretInput
                    id="n-wh-auth"
                    class="font-mono"
                    autocomplete="off"
                    bind:value={draft.notifications.webhook.authorization}
                  />
                </FormField>
              </div>
              <div class="flex flex-wrap justify-end gap-2 pt-2">
                <ActionButton
                  icon={IconName.WEBHOOK}
                  label={t.settings_webhook_test()}
                  ariaLabel={`${t.settings_webhook_test()} ${t.settings_webhook()}`}
                  busyLabel={t.common_testing()}
                  busy={testing[NotificationTarget.WEBHOOK]}
                  disabled={!draft.notifications.webhook.url.trim()}
                  variant={ButtonVariant.DEFAULT}
                  size={ButtonSize.MD}
                  type={ButtonType.BUTTON}
                  onclick={() =>
                    void testTarget({
                      target: "WEBHOOK",
                      url: draft.notifications.webhook.url,
                      authorization: draft.notifications.webhook.authorization,
                    })}
                />
              </div>
            </div>
          {/if}
        </div>
        {#if isTauri()}
          <Setting
            title={t.settings_system_notifications()}
            lead={settings_system_notificationsMark}
            checked={draft.notifications.system}
            disabled={false}
            onchange={(enabled) => (draft.notifications.system = enabled)}
          />
        {/if}
        <div class="flex flex-col">
          <Setting
            title={t.settings_update_notifications()}
            lead={settings_system_notificationsMark}
            checked={draft.notifications.system_update}
            disabled={false}
            onchange={(enabled) => (draft.notifications.system_update = enabled)}
          />
          {#if !isTauri() && draft.notifications.system_update}
            <BrowserNotificationsAccess />
          {/if}
        </div>
        {#if !isTauri()}
          <div class="pt-3">
            <FormField id="n-language" label={t.settings_notifications_language()}>
              <Select
                id="n-language"
                value={draft.notifications.language}
                options={languages}
                label={t.settings_notifications_language()}
                align={MenuAlign.LEFT}
                onchange={(language) => (draft.notifications.language = language)}
              />
            </FormField>
          </div>
        {/if}
      </div>
    </section>
    <section class="flex min-w-0 flex-col">
      <h3 class="pb-2 text-sm font-semibold">{t.settings_notifications_events()}</h3>
      <div class="grid grid-cols-[repeat(auto-fit,minmax(min(220px,100%),1fr))] gap-x-8 *:border-t">
        {#each NOTIFY_KEYS as key (key)}
          <Setting
            title={NOTIFY_LABELS[key]()}
            checked={draft.notifications.events[key]}
            disabled={false}
            onchange={(enabled) => (draft.notifications.events[key] = enabled)}
          />
        {/each}
      </div>
    </section>
  </div>
</Panel>
