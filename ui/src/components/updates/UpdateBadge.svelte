<script lang="ts">
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { isTauri } from "../../lib/api/tauri";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { overlay } from "../../lib/state/overlay.svelte";
  import { strong } from "../../lib/text.utils";
  import { BadgeTone } from "../../lib/tones.constants";
  import { availableVersion, showUpdate } from "../../lib/updates/updates.svelte";
  import Badge from "../ui/Badge.svelte";
  import RichText from "../ui/RichText.svelte";
  import { ButtonType } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MENU_EDGE_GAP } from "../ui/menu.constants";
  import ServerUpdateCommands from "./ServerUpdateCommands.svelte";
  import { UPDATE_TRIGGER_CLASSES } from "./updates.constants";

  const version = $derived(availableVersion());

  let open = $state(false);
  let content = $state<HTMLElement | null>(null);

  // ! The popover takes the focus, not its copy button: that button's tip would open and eat the first Escape.
  const focusContent = (event: Event): void => {
    event.preventDefault();
    content?.focus();
  };
</script>

{#snippet pill(text: string)}
  <Badge
    {text}
    tone={BadgeTone.INFO}
    icon={IconName.IMPORT}
    class="cursor-pointer group-hover:bg-info/80"
  />
{/snippet}

{#if version}
  {#if isTauri()}
    <button class={UPDATE_TRIGGER_CLASSES} type={ButtonType.BUTTON} onclick={showUpdate}>
      {@render pill(t.updates_badge({ version }))}
    </button>
  {:else}
    <Popover bind:open>
      <PopoverTrigger>
        {#snippet child({ props })}
          <button
            {...props}
            class={UPDATE_TRIGGER_CLASSES}
            type={ButtonType.BUTTON}
            aria-expanded={open}
          >
            {@render pill(t.updates_badge({ version }))}
          </button>
        {/snippet}
      </PopoverTrigger>
      <PopoverContent
        bind:ref={content}
        onOpenAutoFocus={focusContent}
        class="w-88 gap-3 p-4"
        align="end"
        collisionPadding={{
          top: MENU_EDGE_GAP,
          right: MENU_EDGE_GAP,
          left: MENU_EDGE_GAP,
          bottom: overlay.bottom + MENU_EDGE_GAP,
        }}
      >
        <p class="text-hint font-medium">
          <RichText text={t.updates_available({ version: strong(version) })} />
        </p>
        <ServerUpdateCommands {version} />
      </PopoverContent>
    </Popover>
  {/if}
{/if}
