<script lang="ts">
  import { isTauri } from "../../lib/api/tauri";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { app } from "../../lib/state/app.svelte";
  import {
    allowBrowserNotifications,
    browserNotifications,
    dismissUpdateBanner,
  } from "../../lib/updates/browser-notifications.svelte";
  import { updateBannerShown } from "../../lib/updates/updates.utils";
  import IconButton from "../ui/IconButton.svelte";
  import Notice from "../ui/Notice.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { NoticeTone } from "../ui/ui.constants";

  const shown = $derived(
    updateBannerShown({
      desktop: isTauri(),
      permission: browserNotifications.permission,
      enabled: app.snapshot?.settings.notifications.system_update ?? false,
      dismissed: browserNotifications.bannerDismissed,
    }),
  );
</script>

{#if shown}
  <Notice tone={NoticeTone.INFO} text={t.home_update_notifications()}>
    {#snippet actions()}
      <TextButton
        label={t.home_update_notifications_allow()}
        icon={IconName.BELL}
        variant={ButtonVariant.OUTLINE}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => void allowBrowserNotifications()}
      />
      <IconButton
        icon={IconName.X}
        label={t.home_update_notifications_dismiss()}
        busy={false}
        disabled={false}
        variant={ButtonVariant.GHOST}
        size={ButtonSize.SM}
        onclick={dismissUpdateBanner}
      />
    {/snippet}
  </Notice>
{/if}
