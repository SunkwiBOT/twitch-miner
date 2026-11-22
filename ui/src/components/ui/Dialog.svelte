<script lang="ts">
  import type { Snippet } from "svelte";
  import { on } from "svelte/events";

  import {
    Dialog as DialogRoot,
    DialogContent,
    DialogHeader,
    DialogTitle,
  } from "$lib/components/ui/dialog";
  import { cn } from "$lib/utils";
  import DialogActions from "./DialogActions.svelte";
  import IconButton from "./IconButton.svelte";
  import { ButtonSize, ButtonVariant } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";

  type DialogPropsType = {
    open: boolean;
    title: string;
    wide: boolean;
    closable: boolean;
    closeOnOutside: boolean;
    closeLabel: string;
    onclose: () => void;
    children: Snippet;
    lead?: Snippet;
    footer?: Snippet;
  };

  let {
    open,
    title,
    wide,
    closable,
    closeOnOutside,
    closeLabel,
    onclose,
    children,
    lead,
    footer,
  }: DialogPropsType = $props();

  const behavior = $derived(closable ? "close" : "ignore");
  const outsideBehavior = $derived(closable && closeOnOutside ? "close" : "ignore");

  let content = $state<HTMLElement | null>(null);
  let scrolled = $state(false);

  // ! Mounted only while open or closing: its portal then lands last, so a sheet opened over another one stacks on top.
  let mounted = $state(false);

  $effect.pre(() => {
    if (open) {
      mounted = true;
    }
  });

  // ! The close cross is never focused first: it would show selected, its tip open.
  const focusFirst = (event: Event): void => {
    event.preventDefault();
    scrolled = false;

    const field = content?.querySelector<HTMLElement>(
      "input:not([type=hidden]):not([disabled]), textarea:not([disabled])",
    );

    (field ?? content)?.focus();
  };

  const inToaster = (target: EventTarget | null): boolean =>
    target instanceof Element && target.closest("[data-sonner-toaster]") !== null;

  // ! A toast's action follows up on this sheet ("Apply to other accounts"): using it keeps the sheet open.
  const keepForToasts = (event: PointerEvent): void => {
    if (inToaster(event.target)) {
      event.preventDefault();
    }
  };

  // ! The sheet's focus trap pulls back any focus that leaves it, from a document capture listener: stopped
  // earlier, at the window, for a toast reached by keyboard (Alt+T, sonner's own shortcut).
  $effect(() => {
    if (!open) {
      return;
    }

    return on(
      window,
      "focusin",
      (event) => {
        if (inToaster(event.target)) {
          event.stopImmediatePropagation();
        }
      },
      { capture: true },
    );
  });
</script>

{#if mounted}
  <DialogRoot
    {open}
    onOpenChange={(next) => {
      if (!next && closable) {
        onclose();
      }
    }}
    onOpenChangeComplete={(next) => (mounted = next)}
  >
    <DialogContent
      bind:ref={content}
      tabindex={-1}
      onOpenAutoFocus={focusFirst}
      showCloseButton={false}
      interactOutsideBehavior={outsideBehavior}
      onInteractOutside={keepForToasts}
      escapeKeydownBehavior={behavior}
      class={cn(
        "flex max-h-[90dvh] flex-col gap-0 p-0 outline-none",
        wide ? "sm:max-w-3xl" : "sm:max-w-lg",
      )}
    >
      <DialogHeader
        class={cn("border-b px-5 pt-5 pb-3", scrolled ? "border-border" : "border-transparent")}
      >
        <DialogTitle class="flex items-center gap-2.5 pr-8 text-base font-semibold">
          {#if lead}{@render lead()}{/if}
          <span class="line-clamp-2 min-w-0 break-words">{title}</span>
        </DialogTitle>
      </DialogHeader>
      <div class="absolute top-[18px] right-4">
        <IconButton
          icon={IconName.X}
          label={closeLabel}
          busy={false}
          disabled={!closable}
          variant={ButtonVariant.GHOST}
          size={ButtonSize.SM}
          onclick={onclose}
        />
      </div>
      <div
        class={cn(
          // ! One column no wider than the dialog: a long unbroken line would widen the track past the edge.
          "grid min-h-0 grid-cols-[minmax(0,1fr)] gap-4 overflow-y-auto px-5 pt-1",
          footer ? "pb-4" : "pb-5",
        )}
        onscroll={(event) => (scrolled = event.currentTarget.scrollTop > 0)}
      >
        {@render children()}
      </div>
      {#if footer}
        <DialogActions>{@render footer()}</DialogActions>
      {/if}
    </DialogContent>
  </DialogRoot>
{/if}
