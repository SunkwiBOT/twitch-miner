<script lang="ts">
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";

  import { Button } from "$lib/components/ui/button";
  import { Command, CommandItem, CommandList, CommandSeparator } from "$lib/components/ui/command";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { cn } from "$lib/utils";
  import {
    fromLocalInputValue,
    PAUSE_PRESET_MINUTES,
    PAUSE_TOMORROW_HOUR,
    toLocalInputValue,
    untilInMinutes,
    untilTomorrowAt,
  } from "../../lib/accounts/pause.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { formatClock } from "../../lib/time/time.utils";
  import { timePreferences } from "../../lib/time/time.svelte";
  import ActionButton from "./ActionButton.svelte";
  import Dialog from "./Dialog.svelte";
  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import { Input } from "$lib/components/ui/input";
  import FormField from "./FormField.svelte";
  import TextButton from "./TextButton.svelte";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonType,
    ButtonVariant,
  } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MENU_EDGE_GAP, MENU_ITEM_CLASSES, MenuAlign } from "./menu.constants";
  import { overlay } from "../../lib/state/overlay.svelte";

  type PauseMenuPropsType = {
    pausedUntil?: string;
    label: string;
    align: MenuAlign;
    size: ButtonSize;
    compact?: boolean;
    onpause: (until: string) => Promise<unknown>;
    onresume: () => Promise<unknown>;
  };

  let {
    pausedUntil,
    label,
    align,
    size,
    compact = false,
    onpause,
    onresume,
  }: PauseMenuPropsType = $props();

  let open = $state(false);
  let acting = $state(false);
  let customOpen = $state(false);
  let customValue = $state("");

  const tomorrowClock = $derived.by(() => {
    void ticker.now;

    return formatClock(
      untilTomorrowAt(PAUSE_TOMORROW_HOUR, Date.now(), timePreferences.zone).toISOString(),
    );
  });

  const resumable = $derived(pausedUntil !== undefined && new Date(pausedUntil) > new Date());
  const customUntil = $derived(fromLocalInputValue(customValue, timePreferences.zone));
  const customValid = $derived(customUntil !== null && customUntil > new Date());

  const iconSize = $derived(size === ButtonSize.SM ? 13 : 14);

  const choose = async (until: Date): Promise<void> => {
    open = false;
    acting = true;
    await onpause(until.toISOString());
    acting = false;
  };

  const resume = async (): Promise<void> => {
    open = false;
    acting = true;
    await onresume();
    acting = false;
  };

  const openCustom = (): void => {
    open = false;
    customValue = toLocalInputValue(
      untilInMinutes(PAUSE_PRESET_MINUTES.ONE_HOUR, Date.now()),
      timePreferences.zone,
    );
    customOpen = true;
  };

  const confirmCustom = async (): Promise<void> => {
    if (!customValid || !customUntil) {
      return;
    }

    customOpen = false;
    acting = true;
    await onpause(customUntil.toISOString());
    acting = false;
  };
</script>

<Popover bind:open>
  <PopoverTrigger>
    {#snippet child({ props })}
      <Button
        {...props}
        variant={BUTTON_VARIANTS[ButtonVariant.OUTLINE]}
        size={BUTTON_SIZES[size]}
        role="combobox"
        aria-expanded={open}
        aria-label={label}
        disabled={acting}
        class="min-w-0 justify-between gap-2 font-normal"
      >
        <span class="flex min-w-0 items-center gap-2">
          {#if acting}
            <Spinner size={iconSize} />
          {:else}
            <Icon name={IconName.CLOCK} size={iconSize} />
          {/if}
          <span class={cn("truncate @max-[60rem]:sr-only", compact && "sr-only")}
            >{acting ? t.pause_menu_working() : label}</span
          >
        </span>
        <ChevronsUpDownIcon class={cn("opacity-50", acting && "hidden")} />
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
        <CommandItem
          value="30m"
          class={MENU_ITEM_CLASSES}
          onSelect={() => void choose(untilInMinutes(PAUSE_PRESET_MINUTES.THIRTY_MIN, Date.now()))}
        >
          {t.pause_preset_30m()}
        </CommandItem>
        <CommandItem
          value="1h"
          class={MENU_ITEM_CLASSES}
          onSelect={() => void choose(untilInMinutes(PAUSE_PRESET_MINUTES.ONE_HOUR, Date.now()))}
        >
          {t.pause_preset_1h()}
        </CommandItem>
        <CommandItem
          value="4h"
          class={MENU_ITEM_CLASSES}
          onSelect={() => void choose(untilInMinutes(PAUSE_PRESET_MINUTES.FOUR_HOURS, Date.now()))}
        >
          {t.pause_preset_4h()}
        </CommandItem>
        <CommandItem
          value="tomorrow"
          class={MENU_ITEM_CLASSES}
          onSelect={() =>
            void choose(untilTomorrowAt(PAUSE_TOMORROW_HOUR, Date.now(), timePreferences.zone))}
        >
          {t.pause_preset_tomorrow({ time: tomorrowClock })}
        </CommandItem>
        <CommandItem value="custom" class={MENU_ITEM_CLASSES} onSelect={openCustom}>
          {t.pause_preset_custom()}
        </CommandItem>
        {#if resumable}
          <CommandSeparator class="my-1" />
          <CommandItem value="resume" class={MENU_ITEM_CLASSES} onSelect={() => void resume()}>
            {t.pause_resume_now()}
          </CommandItem>
        {/if}
      </CommandList>
    </Command>
  </PopoverContent>
</Popover>

<Dialog
  open={customOpen}
  title={t.pause_dialog_title()}
  wide={false}
  closable={!acting}
  closeLabel={t.common_close()}
  onclose={() => (customOpen = false)}
>
  <FormField id="pause-custom" label={t.pause_dialog_datetime()}>
    <Input id="pause-custom" type="datetime-local" bind:value={customValue} />
  </FormField>

  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={acting}
      onclick={() => (customOpen = false)}
    />
    <ActionButton
      icon={IconName.CLOCK}
      label={t.pause_dialog_confirm()}
      busyLabel={t.pause_menu_working()}
      busy={acting}
      disabled={!customValid}
      variant={ButtonVariant.PRIMARY}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={confirmCustom}
    />
  {/snippet}
</Dialog>
