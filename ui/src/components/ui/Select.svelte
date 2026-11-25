<script lang="ts" generics="T extends string | number">
  import { Button } from "$lib/components/ui/button";
  import {
    Command,
    CommandEmpty,
    CommandInput,
    CommandItem,
    CommandList,
  } from "$lib/components/ui/command";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { overlay } from "../../lib/state/overlay.svelte";
  import Avatar from "./Avatar.svelte";
  import ChannelAvatar from "./ChannelAvatar.svelte";
  import Icon from "./Icon.svelte";
  import { AvatarSize } from "./picture.constants";
  import { BUTTON_VARIANTS, ButtonVariant } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";
  import {
    CHECKED_OPTION,
    MENU_DESCRIBED_WIDTH_CLASSES,
    MENU_EDGE_GAP,
    MENU_FIT_WIDTH_CLASSES,
    MENU_ITEM_CLASSES,
    MenuAlign,
    SEARCH_FROM,
  } from "./menu.constants";
  import { centerOption, matchOption } from "./ui.utils";

  type OptionType = {
    value: T;
    label: string;
    description?: string;
    meta?: string;
    image?: string;
  };

  type SelectPropsType = {
    value: T | null;
    options: OptionType[];
    label: string;
    id?: string;
    placeholder?: string;
    align: MenuAlign;
    // ! The app's channel avatar shape (rounded square, no live tag here):
    // used for streamer pickers instead of the default round account avatar.
    channelShape?: boolean;
    onchange: (value: T) => void;
  };

  let {
    value,
    options,
    label,
    id,
    placeholder,
    align,
    channelShape = false,
    onchange,
  }: SelectPropsType = $props();

  let open = $state(false);

  let list = $state<HTMLElement | null>(null);

  $effect(() => {
    if (list) {
      return centerOption(list, CHECKED_OPTION);
    }
  });

  const current = $derived(options.find((option) => option.value === value));
</script>

<Popover bind:open>
  <PopoverTrigger>
    {#snippet child({ props })}
      <Button
        {...props}
        {id}
        variant={BUTTON_VARIANTS[ButtonVariant.OUTLINE]}
        role="combobox"
        aria-expanded={open}
        aria-label={label}
        class="w-full min-w-0 justify-between font-normal"
      >
        <span class="flex min-w-0 items-center gap-2">
          {#if current?.image}
            {#if channelShape}
              <ChannelAvatar
                url={current.image}
                name={current.label}
                size={AvatarSize.XS}
                live={false}
              />
            {:else}
              <Avatar url={current.image} name={current.label} size={AvatarSize.XS} />
            {/if}
          {/if}
          {#if current}
            <span class="truncate">{current.label}</span>
          {:else}
            <span class="truncate text-muted-foreground">{placeholder ?? ""}</span>
          {/if}
        </span>
        <Icon name={IconName.CHEVRONS_UP_DOWN} size={16} class="opacity-50" />
      </Button>
    {/snippet}
  </PopoverTrigger>
  <PopoverContent
    class={options.some((option) => option.description || option.meta)
      ? MENU_DESCRIBED_WIDTH_CLASSES
      : MENU_FIT_WIDTH_CLASSES}
    align={align === MenuAlign.RIGHT ? "end" : "start"}
    collisionPadding={{
      top: MENU_EDGE_GAP,
      right: MENU_EDGE_GAP,
      left: MENU_EDGE_GAP,
      bottom: overlay.bottom + MENU_EDGE_GAP,
    }}
  >
    <Command
      class="min-h-0"
      value={value === null ? "" : String(value)}
      loop={true}
      filter={matchOption}
      label={t.select_search()}
      disableInitialScroll={true}
    >
      {#if options.length >= SEARCH_FROM}
        <CommandInput placeholder={t.select_search()} />
      {/if}
      <CommandList bind:ref={list} class="max-h-none min-h-0 flex-1 p-1" aria-label={label}>
        <CommandEmpty>{t.select_empty()}</CommandEmpty>
        {#each options as option (option.value)}
          <CommandItem
            value={String(option.value)}
            keywords={[option.label]}
            data-checked={option.value === value ? "true" : undefined}
            class={MENU_ITEM_CLASSES}
            onSelect={() => {
              onchange(option.value);
              open = false;
            }}
          >
            {#if option.image}
              {#if channelShape}
                <ChannelAvatar
                  url={option.image}
                  name={option.label}
                  size={AvatarSize.XS}
                  live={false}
                />
              {:else}
                <Avatar url={option.image} name={option.label} size={AvatarSize.XS} />
              {/if}
            {/if}
            <span class="flex min-w-0 flex-1 flex-col gap-0.5">
              <span class="truncate">{option.label}</span>
              {#if option.description}
                <span class="text-xs leading-snug font-normal text-muted-foreground">
                  {option.description}
                </span>
              {/if}
            </span>
            {#if option.meta}
              <span class="shrink-0 text-xs font-normal text-muted-foreground tabular-nums">
                {option.meta}
              </span>
            {/if}
          </CommandItem>
        {/each}
      </CommandList>
    </Command>
  </PopoverContent>
</Popover>
