<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  import { Input } from "$lib/components/ui/input";
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import Tip from "./Tip.svelte";
  import {
    CLEAR_SEARCH_KEY,
    FIELD_BUTTON_CLASSES,
    FIELD_ICON_CLASSES,
    IconName,
    SEARCH_FIELD_PADDING,
  } from "./ui.constants";

  type SearchFieldPropsType = Omit<HTMLInputAttributes, "type" | "value" | "files"> & {
    value: string;
    busy: boolean;
  };

  let {
    value = $bindable(),
    busy,
    class: className,
    onkeydown,
    ...rest
  }: SearchFieldPropsType = $props();

  let input = $state<HTMLInputElement | null>(null);

  const clear = (): void => {
    if (!input) {
      return;
    }

    input.value = "";
    input.dispatchEvent(new Event("input", { bubbles: true }));
    input.focus();
  };
</script>

<div class="relative w-full min-w-0">
  <Input
    bind:ref={input}
    type="search"
    class={cn(SEARCH_FIELD_PADDING, "[&::-webkit-search-cancel-button]:hidden", className)}
    spellcheck={false}
    aria-busy={busy}
    bind:value
    {...rest}
    onkeydown={(event) => {
      // ! A filled field empties first; in an empty one Escape keeps its meaning
      // (closing the menu or the dialog).
      if (event.key === CLEAR_SEARCH_KEY && value !== "" && !event.isComposing) {
        event.preventDefault();
        event.stopPropagation();
        clear();
      }

      onkeydown?.(event);
    }}
  />

  <!-- ! One place on the right: the spinner while a search waits or runs, else
  the clear cross once there is text, else the magnifier. -->
  {#if busy}
    <span class={FIELD_ICON_CLASSES} aria-hidden="true"><Spinner size={15} /></span>
  {:else if value !== ""}
    <Tip text={t.common_clear_search()}>
      {#snippet children(props)}
        <button
          {...props}
          type="button"
          class={FIELD_BUTTON_CLASSES}
          aria-label={t.common_clear_search()}
          onclick={clear}
        >
          <Icon name={IconName.X} size={15} />
        </button>
      {/snippet}
    </Tip>
  {:else}
    <span class={FIELD_ICON_CLASSES} aria-hidden="true">
      <Icon name={IconName.SEARCH} size={15} />
    </span>
  {/if}
</div>
