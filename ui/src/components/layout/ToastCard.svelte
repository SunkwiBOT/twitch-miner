<script lang="ts">
  import TextButton from "../ui/TextButton.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import type { ToastCardPropsType } from "../../lib/state/toasts";
  import Avatar from "../ui/Avatar.svelte";
  import ChannelAvatar from "../ui/ChannelAvatar.svelte";
  import RichText from "../ui/RichText.svelte";
  import Icon from "../ui/Icon.svelte";
  import IconButton from "../ui/IconButton.svelte";
  import { AvatarSize } from "../ui/picture.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";

  let {
    kind,
    title,
    description,
    people,
    categories,
    account,
    action,
    onclose,
  }: ToastCardPropsType = $props();
</script>

<div
  class="relative flex w-toast max-w-[calc(100vw-2rem)] items-start gap-3 rounded-lg border bg-popover p-3 pr-10 text-sm text-popover-foreground shadow-lg"
  role={kind === ToastKind.ERROR ? "alert" : "status"}
>
  {#if kind === ToastKind.ERROR}
    <Icon name={IconName.OCTAGON_X} size={16} class="mt-0.5 shrink-0 text-destructive" />
  {:else if kind === ToastKind.SUCCESS}
    <Icon name={IconName.CIRCLE_CHECK} size={16} class="mt-0.5 shrink-0 text-success-emphasis" />
  {:else}
    <Icon name={IconName.INFO} size={16} class="mt-0.5 shrink-0 text-info-emphasis" />
  {/if}
  <div class="flex min-w-0 flex-1 flex-col gap-2">
    <div class="flex flex-col gap-1 leading-snug break-words">
      <p class="m-0 font-semibold"><RichText text={title} /></p>
      {#if account}
        <p class="m-0 flex min-w-0 items-center gap-2">
          {#if account.channel}
            <ChannelAvatar
              url={account.url}
              name={account.name}
              size={AvatarSize.SM}
              live={false}
            />
          {:else}
            <Avatar url={account.url} name={account.name} size={AvatarSize.SM} />
          {/if}
          <strong class="min-w-0 truncate font-semibold">{account.name}</strong>
        </p>
      {/if}
      {#if description}<p class="m-0"><RichText text={description} {people} {categories} /></p>{/if}
    </div>
    {#if action}
      <TextButton
        label={action.label}
        icon={action.icon}
        variant={ButtonVariant.PRIMARY}
        size={ButtonSize.SM}
        type={ButtonType.BUTTON}
        disabled={false}
        class="w-full"
        onclick={() => {
          // ! The action first: what it opens decides whether the toast's end still has to clean up.
          action.run();
          onclose();
        }}
      />
    {/if}
  </div>
  <div class="absolute top-1.5 right-1.5">
    <IconButton
      icon={IconName.X}
      label={t.common_close()}
      busy={false}
      disabled={false}
      variant={ButtonVariant.GHOST}
      size={ButtonSize.SM}
      onclick={onclose}
    />
  </div>
</div>
