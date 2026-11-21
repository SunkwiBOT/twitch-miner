<script lang="ts">
  import { CommandName } from "../../lib/bindings/CommandName";
  import { PauseScope } from "../../lib/accounts/account-state.constants";
  import { AccountStatus } from "../../lib/bindings/AccountStatus";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { BETA_PAGES, Page } from "../../lib/state/state.constants";
  import { account, app } from "../../lib/state/app.svelte";
  import { isAccountPage } from "../../lib/state/router";
  import { run } from "../../lib/state/session";
  import { wideLayout } from "../../lib/state/viewport.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import Avatar from "../ui/Avatar.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import AccountStatusBadges from "../ui/AccountStatusBadges.svelte";
  import Badge from "../ui/Badge.svelte";
  import PauseBadge from "../ui/PauseBadge.svelte";
  import PauseControl from "../ui/PauseControl.svelte";
  import QuietWindowBadge from "../ui/QuietWindowBadge.svelte";
  import { AvatarSize } from "../ui/picture.constants";
  import { BadgeTone } from "../../lib/tones.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MenuAlign } from "../ui/menu.constants";
  import UpdateBadge from "../updates/UpdateBadge.svelte";
  import UpdateConfirmDialog from "../updates/UpdateConfirmDialog.svelte";
  import ShellControls from "./ShellControls.svelte";

  type TopbarPropsType = { title: string; section: string | null };

  let { title, section }: TopbarPropsType = $props();

  const current = $derived(isAccountPage(app.page) ? account(app.selectedAccount) : null);

  let refreshing = $state(false);

  const refreshAccount = async (id: string): Promise<void> => {
    refreshing = true;
    await run({ command: CommandName.REFRESH_ACCOUNT, account_id: id });
    refreshing = false;
  };
</script>

<header
  class="@container flex min-h-14 items-center gap-2.5 border-b bg-card px-6 py-2.5 max-md:gap-2 max-md:px-3"
>
  {#if !wideLayout.current}
    <IconButton
      icon={IconName.MENU}
      label={t.nav_open_menu()}
      busy={false}
      disabled={false}
      variant={ButtonVariant.GHOST}
      size={ButtonSize.MD}
      onclick={() => (app.navDrawer = true)}
    />
  {/if}
  {#if current}
    <Avatar url={current.avatar_url} name={current.display_name} size={AvatarSize.SM} />
  {/if}
  <h1 class="flex min-w-0 text-xl font-semibold max-md:text-lg">
    <span class="min-w-0 truncate">{title}</span>
    {#if section}<span class="min-w-0 shrink-[100] truncate ps-1.5 @max-[45rem]:hidden">
        · {section}
      </span>{/if}
  </h1>
  {#if BETA_PAGES.has(app.page)}<Badge
      text={t.common_beta()}
      tone={BadgeTone.ACCENT}
      dot={false}
    />{/if}
  <span class="flex-1"></span>
  <UpdateBadge />
  {#if app.page !== Page.HOME}<QuietWindowBadge class="@max-[50rem]:hidden" />{/if}
  {#if current}
    {#if wideLayout.current}<span class="contents @max-[65rem]:hidden">
        <AccountStatusBadges account={current} />
      </span>{/if}
    {#if current.status === AccountStatus.LOGIN_REQUIRED}
      <TextButton
        label={t.accounts_login_again()}
        icon={IconName.LOG_IN}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.MD}
        type={ButtonType.BUTTON}
        disabled={false}
        onclick={() => (app.loginDialog = true)}
      />
    {:else}
      <PauseControl
        accounts={[current]}
        scope={PauseScope.ACCOUNT}
        align={MenuAlign.RIGHT}
        size={ButtonSize.MD}
      />
      <PauseBadge account={current} class="@max-[50rem]:hidden" />
      <IconButton
        icon={IconName.REFRESH}
        label={t.account_refresh()}
        busy={refreshing}
        disabled={current.status === AccountStatus.PAUSED}
        variant={ButtonVariant.DEFAULT}
        size={ButtonSize.MD}
        onclick={() => refreshAccount(current.id)}
      />
    {/if}
  {/if}
  {#if app.snapshot && wideLayout.current}
    <ShellControls snapshot={app.snapshot} align={MenuAlign.RIGHT} />
  {/if}
</header>

<UpdateConfirmDialog />
