<script lang="ts" generics="T">
  import { untrack, type Snippet } from "svelte";

  import { badgeVariants } from "$lib/components/ui/badge";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { cn } from "$lib/utils";
  import { number } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { matchesSearch, normalizeSearch } from "../../lib/search/search.utils";
  import { overlay } from "../../lib/state/overlay.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { IconName } from "../../lib/icons.constants";
  import Badge from "./Badge.svelte";
  import Icon from "./Icon.svelte";
  import NameList from "./NameList.svelte";
  import SearchField from "./SearchField.svelte";
  import SearchNoResult from "./SearchNoResult.svelte";
  import { BADGE_CHIP_CLASSES, BADGE_VARIANTS, BadgeVariant } from "./badge.constants";
  import { MENU_EDGE_GAP, SELECT_SEARCH_FROM } from "./menu.constants";

  type OverflowNamesPropsType = {
    items: T[];
    shown: number;
    title: string;
    key: (item: T) => string;
    words: (item: T) => string[];
    item: Snippet<[T]>;
    class?: string;
    itemClass?: string;
    onshow?: (items: T[]) => void;
  };

  let {
    items,
    shown,
    title,
    key,
    words,
    item,
    class: className,
    itemClass,
    onshow,
  }: OverflowNamesPropsType = $props();

  let open = $state(false);
  let query = $state("");

  const visible = $derived(items.slice(0, shown));
  const hidden = $derived(items.length - visible.length);
  const searchable = $derived(items.length >= SELECT_SEARCH_FROM);
  const found = $derived.by(() => {
    const needle = searchable ? normalizeSearch(query) : "";

    return needle
      ? items.filter((entry) => words(entry).some((word) => matchesSearch(word, needle)))
      : items;
  });

  // ! Callers read names (pictures, lookups) only for what is on screen: the first ones, then every one once opened.
  $effect(() => {
    const names = visible;

    untrack(() => onshow?.(names));
  });

  const toggled = (next: boolean): void => {
    if (next) {
      onshow?.(items);
    } else {
      query = "";
    }
  };
</script>

<NameList class={className}>
  {#each visible as entry (key(entry))}<li class={itemClass}>{@render item(entry)}</li>{/each}
  {#if hidden > 0}<li class={itemClass}>
      <Popover bind:open onOpenChange={toggled}>
        <PopoverTrigger>
          {#snippet child({ props })}
            <button
              {...props}
              type="button"
              class={cn(
                badgeVariants({ variant: BADGE_VARIANTS[BadgeVariant.CHIP] }),
                BADGE_CHIP_CLASSES,
                "cursor-pointer font-semibold tabular-nums hover:bg-secondary/80",
              )}
            >
              +{number(hidden)}<span class="sr-only">
                {t.name_list_show_all({ listTitle: title, nameCount: number(items.length) })}
              </span><Icon name={IconName.DOWN} size={12} />
            </button>
          {/snippet}
        </PopoverTrigger>
        <PopoverContent
          class="w-80 gap-2"
          align="start"
          aria-label={title}
          collisionPadding={{
            top: MENU_EDGE_GAP,
            right: MENU_EDGE_GAP,
            left: MENU_EDGE_GAP,
            bottom: overlay.bottom + MENU_EDGE_GAP,
          }}
        >
          <p class="flex items-center gap-2 text-xs font-semibold text-muted-foreground">
            {title}<Badge text={number(items.length)} tone={BadgeTone.NEUTRAL} />
          </p>
          {#if searchable}
            <SearchField
              placeholder={t.common_search()}
              aria-label={t.common_search()}
              bind:value={query}
              busy={false}
            />
            <SearchNoResult shown={found.length === 0} {query} />
          {/if}
          <NameList
            class="max-h-72 flex-col flex-nowrap gap-y-2 overflow-y-auto overscroll-contain p-0.5 text-hint"
          >
            {#each found as entry (key(entry))}<li class="min-w-0">{@render item(entry)}</li>{/each}
          </NameList>
        </PopoverContent>
      </Popover>
    </li>{/if}
</NameList>
