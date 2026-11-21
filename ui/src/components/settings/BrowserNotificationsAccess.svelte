<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { IconName } from "../../lib/icons.constants";
  import {
    askBrowserNotifications,
    browserNotificationPermission,
  } from "../../lib/updates/update-notification.utils";
  import { BrowserPermission } from "../../lib/updates/updates.constants";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";

  let permission = $state(browserNotificationPermission());

  const allow = async (): Promise<void> => {
    permission = await askBrowserNotifications();
  };
</script>

{#if permission === null}
  <p class="pb-3 text-hint text-muted-foreground">
    {t.settings_update_notifications_unsupported()}
  </p>
{:else if permission === BrowserPermission.DENIED}
  <p class="pb-3 text-hint text-muted-foreground">{t.settings_update_notifications_blocked()}</p>
{:else if permission === BrowserPermission.DEFAULT}
  <div class="pb-3">
    <TextButton
      label={t.settings_update_notifications_allow()}
      icon={IconName.BELL}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={() => void allow()}
    />
  </div>
{/if}
