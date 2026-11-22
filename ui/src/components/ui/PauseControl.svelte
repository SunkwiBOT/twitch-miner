<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Command, CommandItem, CommandList, CommandSeparator } from "$lib/components/ui/command";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { Input } from "$lib/components/ui/input";
  import { cn } from "$lib/utils";
  import { PauseScope, PauseState } from "../../lib/accounts/account-state.constants";
  import { pauseState } from "../../lib/accounts/account-state.utils";
  import { PAUSE_LABELS, RESUME_LABELS } from "../../lib/accounts/accounts.utils";
  import {
    fromLocalInputValue,
    PAUSE_CUSTOM_DEFAULT_MINUTES,
    PAUSE_PRESET_MINUTES,
    PAUSE_TOMORROW_HOUR,
    toLocalInputValue,
    untilInMinutes,
    untilTomorrowAt,
  } from "../../lib/accounts/pause.utils";
  import type { AccountSnapshotType } from "../../lib/bindings/AccountSnapshotType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { pauseAccounts, resumeAccounts } from "../../lib/state/pause";
  import { overlay } from "../../lib/state/overlay.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { formatClock } from "../../lib/time/time.utils";
  import { longDuration } from "../../lib/format.utils";
  import { timePreferences } from "../../lib/time/time.svelte";
  import ActionButton from "./ActionButton.svelte";
  import Dialog from "./Dialog.svelte";
  import FormField from "./FormField.svelte";
  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import TextButton from "./TextButton.svelte";
  import {
    BUTTON_ICON_SIZES,
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonType,
    ButtonVariant,
  } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MENU_EDGE_GAP, MENU_ITEM_CLASSES, MenuAlign } from "./menu.constants";

  type PauseControlPropsType = {
    accounts: AccountSnapshotType[];
    scope: PauseScope;
    align: MenuAlign;
  };

  let { accounts, scope, align }: PauseControlPropsType = $props();

  const size = ButtonSize.MD;
  const iconSize = BUTTON_ICON_SIZES[size];

  let open = $state(false);
  let pausing = $state(false);
  let resuming = $state(false);
  let customOpen = $state(false);
  let customValue = $state("");

  const current = $derived(
    pauseState(
      accounts.map((account) => account.settings),
      ticker.now,
    ),
  );

  const tomorrowClock = $derived(
    formatClock(
      untilTomorrowAt(PAUSE_TOMORROW_HOUR, ticker.now, timePreferences.zone).toISOString(),
    ),
  );

  const customUntil = $derived(fromLocalInputValue(customValue, timePreferences.zone));
  const customValid = $derived(customUntil !== null && customUntil.getTime() > ticker.now);
  const busy = $derived(pausing || resuming);
  const busyLabel = $derived(resuming ? t.action_resuming() : t.action_pausing());

  const pause = async (until: Date | null): Promise<boolean> => {
    open = false;
    pausing = true;

    const done = await pauseAccounts(scope, accounts, until?.toISOString() ?? null);

    pausing = false;

    return done;
  };

  const resume = async (): Promise<void> => {
    open = false;
    resuming = true;
    await resumeAccounts(scope, accounts, Date.now());
    resuming = false;
  };

  const openCustom = (): void => {
    open = false;
    customValue = toLocalInputValue(
      untilInMinutes(PAUSE_CUSTOM_DEFAULT_MINUTES, Date.now()),
      timePreferences.zone,
    );
    customOpen = true;
  };

  const confirmCustom = async (): Promise<void> => {
    if (customValid && customUntil && (await pause(customUntil))) {
      customOpen = false;
    }
  };
</script>

{#if current === PauseState.PAUSED}
  <ActionButton
    icon={IconName.PLAY}
    label={RESUME_LABELS[scope]()}
    {busyLabel}
    {busy}
    disabled={false}
    variant={ButtonVariant.PRIMARY}
    {size}
    type={ButtonType.BUTTON}
    onclick={resume}
  />
{:else}
  <Popover bind:open>
    <PopoverTrigger>
      {#snippet child({ props })}
        <Button
          {...props}
          variant={BUTTON_VARIANTS[ButtonVariant.DEFAULT]}
          size={BUTTON_SIZES[size]}
          type={ButtonType.BUTTON}
          aria-expanded={open}
          aria-busy={busy}
          disabled={busy}
          class="min-w-0"
        >
          {#if busy}
            <Spinner size={iconSize} />
          {:else}
            <Icon name={IconName.PAUSE} size={iconSize} />
          {/if}
          <span class="truncate">{busy ? busyLabel : PAUSE_LABELS[scope]()}</span>
          <Icon name={IconName.DOWN} size={iconSize} class={cn("opacity-50", busy && "hidden")} />
        </Button>
      {/snippet}
    </PopoverTrigger>
    <PopoverContent
      class="w-72 p-1"
      align={align === MenuAlign.RIGHT ? "end" : "start"}
      collisionPadding={{
        top: MENU_EDGE_GAP,
        right: MENU_EDGE_GAP,
        left: MENU_EDGE_GAP,
        bottom: overlay.bottom + MENU_EDGE_GAP,
      }}
    >
      <Command label={t.pause_menu_title()}>
        <CommandList class="p-1" aria-label={t.pause_menu_title()}>
          <CommandItem value="resumed" class={MENU_ITEM_CLASSES} onSelect={() => void pause(null)}>
            {t.pause_preset_until_resumed()}
          </CommandItem>
          <CommandSeparator class="my-1" />
          {#each PAUSE_PRESET_MINUTES as minutes (minutes)}
            <CommandItem
              value={`preset-${minutes}`}
              class={MENU_ITEM_CLASSES}
              onSelect={() => void pause(untilInMinutes(minutes, Date.now()))}
            >
              {longDuration(minutes)}
            </CommandItem>
          {/each}
          <CommandItem
            value="tomorrow"
            class={MENU_ITEM_CLASSES}
            onSelect={() =>
              void pause(untilTomorrowAt(PAUSE_TOMORROW_HOUR, Date.now(), timePreferences.zone))}
          >
            {t.pause_preset_tomorrow({ resumeTime: tomorrowClock })}
          </CommandItem>
          <CommandItem value="custom" class={MENU_ITEM_CLASSES} onSelect={openCustom}>
            {t.pause_preset_custom()}
          </CommandItem>
          {#if current === PauseState.MIXED}
            <CommandSeparator class="my-1" />
            <CommandItem value="resume" class={MENU_ITEM_CLASSES} onSelect={() => void resume()}>
              {RESUME_LABELS[scope]()}
            </CommandItem>
          {/if}
        </CommandList>
      </Command>
    </PopoverContent>
  </Popover>
{/if}

<Dialog
  open={customOpen}
  title={t.pause_dialog_title()}
  wide={false}
  closable={!pausing}
  closeOnOutside={true}
  closeLabel={t.common_close()}
  onclose={() => (customOpen = false)}
>
  <FormField id="pause-custom" label={t.pause_dialog_datetime()}>
    <Input id="pause-custom" type="datetime-local" bind:value={customValue} />
  </FormField>

  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      icon={IconName.X}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={pausing}
      onclick={() => (customOpen = false)}
    />
    <ActionButton
      icon={IconName.PAUSE}
      label={t.pause_dialog_confirm()}
      busyLabel={t.action_pausing()}
      busy={pausing}
      disabled={!customValid}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={confirmCustom}
    />
  {/snippet}
</Dialog>
