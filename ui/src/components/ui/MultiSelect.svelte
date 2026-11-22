<script lang="ts" generics="T extends string">
  import { Button } from "$lib/components/ui/button";
  import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
    CommandSeparator,
  } from "$lib/components/ui/command";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { overlay } from "../../lib/state/overlay.svelte";
  import Avatar from "./Avatar.svelte";
  import Icon from "./Icon.svelte";
  import { AvatarSize } from "./picture.constants";
  import { BUTTON_VARIANTS, ButtonVariant } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";
  import {
    MENU_EDGE_GAP,
    MULTI_ALL_VALUE,
    MULTI_CHECKED_OPTION,
    MULTI_ITEM_CLASSES,
    MULTI_PICTURES_MAX,
    MenuAlign,
    SELECT_SEARCH_FROM,
  } from "./menu.constants";
  import { centerOption, matchOption } from "./ui.utils";

  type OptionType = { value: T; label: string; image?: string; icon?: IconName; group?: string };

  type MultiSelectPropsType = {
    values: T[];
    options: OptionType[];
    label: string;
    allLabel: string;
    noneLabel: string;
    countLabel: (count: number) => string;
    align: MenuAlign;
    onchange: (values: T[]) => void;
  };

  let {
    values,
    options,
    label,
    allLabel,
    noneLabel,
    countLabel,
    align,
    onchange,
  }: MultiSelectPropsType = $props();

  let open = $state(false);

  let list = $state<HTMLElement | null>(null);

  $effect(() => {
    if (list) {
      return centerOption(list, MULTI_CHECKED_OPTION);
    }
  });

  const all = $derived(values.length === options.length);

  // ! Set as the list opens: toggling options afterwards keeps the highlight in place.
  let highlighted = $state("");

  const highlightFirstChecked = (opening: boolean): void => {
    if (opening) {
      highlighted = all
        ? MULTI_ALL_VALUE
        : (options.find((option) => values.includes(option.value))?.value ?? "");
    }
  };

  const summary = $derived.by(() => {
    if (values.length === 0) {
      return noneLabel;
    }

    if (all) {
      return allLabel;
    }

    const only = options.find((option) => option.value === values[0]);

    return values.length === 1 && only ? only.label : countLabel(values.length);
  });

  const pictures = $derived(
    options
      .filter((option) => option.image && values.includes(option.value))
      .slice(0, MULTI_PICTURES_MAX),
  );

  const groups = $derived(
    options.reduce<{ heading?: string; options: OptionType[] }[]>((list, option) => {
      const last = list.at(-1);

      if (last && last.heading === option.group) {
        last.options.push(option);
      } else {
        list.push({ heading: option.group, options: [option] });
      }

      return list;
    }, []),
  );

  const toggle = (value: T): void => {
    onchange(values.includes(value) ? values.filter((item) => item !== value) : [...values, value]);
  };

  const toggleAll = (): void => {
    onchange(all ? [] : options.map((option) => option.value));
  };
</script>

{#snippet box(on: boolean, some: boolean)}
  <span
    class={cn(
      "flex size-4 shrink-0 items-center justify-center rounded-[4px] border border-input",
      (on || some) && "border-primary bg-primary text-primary-foreground",
    )}
    aria-hidden="true"
  >
    <!-- ! The `size-` class keeps the command item's own svg sizing off this smaller mark. -->
    {#if on}<Icon name={IconName.CHECK} size={12} class="size-3" />{:else if some}<Icon
        name={IconName.MINUS}
        size={12}
        class="size-3"
      />{/if}
  </span>
{/snippet}

{#snippet item(option: OptionType)}
  {@const on = values.includes(option.value)}
  <CommandItem
    value={option.value}
    keywords={[option.label]}
    aria-checked={on}
    class={MULTI_ITEM_CLASSES}
    onSelect={() => toggle(option.value)}
  >
    {@render box(on, false)}
    {#if option.image}
      <Avatar url={option.image} name={option.label} size={AvatarSize.XS} />
    {:else if option.icon}
      <Icon name={option.icon} size={14} />
    {/if}
    <span class="truncate">{option.label}</span>
  </CommandItem>
{/snippet}

<Popover bind:open onOpenChange={highlightFirstChecked}>
  <PopoverTrigger>
    {#snippet child({ props })}
      <Button
        {...props}
        variant={BUTTON_VARIANTS[ButtonVariant.OUTLINE]}
        role="combobox"
        aria-expanded={open}
        aria-label={label}
        class="w-full min-w-0 justify-between font-normal"
      >
        <span class="flex min-w-0 items-center gap-2">
          {#if pictures.length}
            <span class="flex shrink-0 -space-x-1.5">
              {#each pictures as option (option.value)}
                <span class="rounded-full ring-2 ring-background">
                  <Avatar url={option.image} name={option.label} size={AvatarSize.XS} />
                </span>
              {/each}
            </span>
          {/if}
          <span class="truncate">{summary}</span>
        </span>
        <Icon name={IconName.CHEVRONS_UP_DOWN} size={16} class="opacity-50" />
      </Button>
    {/snippet}
  </PopoverTrigger>
  <PopoverContent
    class="max-h-(--bits-popover-content-available-height) w-(--bits-popover-anchor-width) min-w-56 p-0"
    align={align === MenuAlign.RIGHT ? "end" : "start"}
    collisionPadding={{
      top: MENU_EDGE_GAP,
      right: MENU_EDGE_GAP,
      left: MENU_EDGE_GAP,
      bottom: overlay.bottom + MENU_EDGE_GAP,
    }}
  >
    <!-- ! The list takes the popover's room, the screen's available height minus the search field. -->
    <Command
      class="min-h-0"
      bind:value={highlighted}
      filter={matchOption}
      label={t.select_search()}
      disableInitialScroll={true}
    >
      {#if options.length >= SELECT_SEARCH_FROM}
        <CommandInput placeholder={t.select_search()} />
      {/if}
      <CommandList bind:ref={list} class="max-h-none min-h-0 flex-1 p-1" aria-label={label}>
        <CommandEmpty>{t.select_empty()}</CommandEmpty>
        <CommandItem
          value={MULTI_ALL_VALUE}
          keywords={[allLabel]}
          aria-checked={all ? true : values.length > 0 ? "mixed" : false}
          class={MULTI_ITEM_CLASSES}
          onSelect={toggleAll}
        >
          {@render box(all, values.length > 0)}
          <span class="truncate">{allLabel}</span>
        </CommandItem>
        <CommandSeparator class="my-1" />
        {#each groups as group, index (index)}
          {#if group.heading}
            <CommandGroup heading={group.heading} class="p-0">
              {#each group.options as option (option.value)}
                {@render item(option)}
              {/each}
            </CommandGroup>
          {:else}
            {#each group.options as option (option.value)}
              {@render item(option)}
            {/each}
          {/if}
        {/each}
      </CommandList>
    </Command>
  </PopoverContent>
</Popover>
