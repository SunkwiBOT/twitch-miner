<script lang="ts" generics="T">
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { SEARCH_SHOWN_MAX } from "../../lib/search/search.constants";
  import { TwitchSearch } from "../../lib/search/search.svelte";
  import {
    tagSuggestions,
    typedTag,
    type TagSuggestionType,
  } from "../../lib/search/tag-suggestions.utils";
  import Icon from "./Icon.svelte";
  import ChipRemoveButton from "./ChipRemoveButton.svelte";
  import ConfirmDialog from "./ConfirmDialog.svelte";
  import SearchField from "./SearchField.svelte";
  import SearchNoResult from "./SearchNoResult.svelte";
  import TextButton from "./TextButton.svelte";
  import { BADGE_CHIP_CLASSES, BADGE_VARIANTS, BadgeVariant } from "./badge.constants";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonType,
    ButtonVariant,
  } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { KeyboardKey } from "./field.constants";
  import { coreLimits } from "../../lib/state/limits";
  import type { TextCategoryType } from "../../lib/text.utils";
  import GameName from "./GameName.svelte";
  import NameList from "./NameList.svelte";
  import { NameListKind } from "./link.constants";
  import { ThumbSize } from "./picture.constants";

  type TagInputPropsType = {
    id: string;
    items: T[];
    key: (item: T) => string;
    label: (item: T) => string;
    category: ((item: T) => TextCategoryType) | null;
    make: (suggestion: TagSuggestionType) => T;
    placeholder: string;
    suggestions: TagSuggestionType[];
    addLabel: string;
    removeLabel: string;
    confirmTitle: string;
    confirmText: (item: T) => string;
    onchange: (items: T[]) => void;
    remote: boolean;
  };

  let {
    id,
    items,
    key,
    label,
    category,
    make,
    placeholder,
    suggestions,
    addLabel,
    removeLabel,
    confirmTitle,
    confirmText,
    onchange,
    remote,
  }: TagInputPropsType = $props();

  let draft = $state("");
  const twitch = new TwitchSearch();

  const found = $derived(
    twitch.results.games.map((game) => ({
      label: game.name,
      id: game.id,
      image: game.box_art_url,
    })),
  );

  const matches = $derived(
    tagSuggestions(
      draft,
      items.map(label),
      suggestions,
      found,
      SEARCH_SHOWN_MAX,
      coreLimits().search_min_chars,
    ),
  );

  // ! Typed out in full, a known category still carries its id and image.
  const typed = (): TagSuggestionType => typedTag(draft, [...suggestions, ...found]);

  const add = (suggestion: TagSuggestionType): void => {
    const text = suggestion.label.trim();

    draft = "";
    twitch.clear();

    if (!text || items.some((known) => label(known).toLowerCase() === text.toLowerCase())) {
      return;
    }

    onchange([...items, make({ ...suggestion, label: text })]);
  };

  // ! An IME's Enter only ends the composed word, and no suggestion is known while Twitch answers: both add nothing.
  const onkeydown = (event: KeyboardEvent): void => {
    if (event.key !== KeyboardKey.ENTER || event.isComposing) {
      return;
    }

    event.preventDefault();

    if (!twitch.busy) {
      add(matches.at(0) ?? typed());
    }
  };

  let removing = $state<T | null>(null);

  const remove = (): void => {
    const item = removing;
    removing = null;

    if (item !== null) {
      onchange(items.filter((known) => key(known) !== key(item)));
    }
  };
</script>

<div class="flex flex-col gap-2">
  {#if items.length}
    <NameList kind={NameListKind.WRAP} class="gap-1.5">
      {#each items as item (key(item))}
        {@const shown = category?.(item)}
        <li class="flex">
          <Badge
            variant={BADGE_VARIANTS[BadgeVariant.CHIP]}
            class="{BADGE_CHIP_CLASSES} h-6 gap-1 pr-0.5 text-hint font-medium"
          >
            {#if shown}
              <GameName name={shown.name} url={shown.url} art={shown.art} size={ThumbSize.XS} />
            {:else}
              {label(item)}
            {/if}
            <ChipRemoveButton
              label={removeLabel}
              name={label(item)}
              iconSize={12}
              onclick={() => (removing = item)}
            />
          </Badge>
        </li>
      {/each}
    </NameList>
  {/if}
  <div class="flex gap-2">
    {#if remote}
      <div class="min-w-0 flex-1">
        <SearchField
          {id}
          {placeholder}
          bind:value={draft}
          busy={twitch.busy}
          oninput={() => twitch.update(draft)}
          {onkeydown}
        />
      </div>
    {:else}
      <Input {id} class="min-w-0 flex-1" {placeholder} bind:value={draft} {onkeydown} />
    {/if}
    <TextButton
      label={addLabel}
      icon={IconName.PLUS}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={!draft.trim()}
      onclick={() => add(typed())}
    />
  </div>
  {#if matches.length}
    <div class="flex flex-wrap gap-1.5">
      {#each matches as match (match.label)}
        <Button
          variant={BUTTON_VARIANTS[ButtonVariant.OUTLINE]}
          size={BUTTON_SIZES[ButtonSize.SM]}
          type={ButtonType.BUTTON}
          onclick={() => add(match)}
        >
          {#if match.image}
            <img
              class="h-5 w-3.75 shrink-0 rounded-sm object-cover"
              src={boxArt(match.image, BoxArtSize.ROW)}
              alt=""
            />
          {:else}
            <Icon name={IconName.PLUS} size={13} />
          {/if}
          {match.label}
        </Button>
      {/each}
    </div>
  {/if}
  {#if remote}<SearchNoResult shown={matches.length === 0 && twitch.answered} query={draft} />{/if}
</div>

<ConfirmDialog
  open={removing !== null}
  title={confirmTitle}
  text={removing ? confirmText(removing) : ""}
  categories={removing && category ? [category(removing)] : []}
  confirmLabel={t.confirm_remove()}
  busyLabel={t.confirm_removing()}
  busy={false}
  onconfirm={remove}
  oncancel={() => (removing = null)}
/>
