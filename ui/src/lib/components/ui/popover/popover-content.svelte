<script lang="ts">
  import { Popover as PopoverPrimitive } from "bits-ui";
  import { cn, type WithoutChildrenOrChild } from "$lib/utils.js";
  import PopoverPortal from "./popover-portal.svelte";
  import type { ComponentProps } from "svelte";

  let {
    ref = $bindable(null),
    class: className,
    sideOffset = 4,
    align = "center",
    portalProps,
    onOpenAutoFocus,
    ...restProps
  }: PopoverPrimitive.ContentProps & {
    portalProps?: WithoutChildrenOrChild<ComponentProps<typeof PopoverPortal>>;
  } = $props();

  // ! A popover holding a Command list (the pause menu, the selects)
  // opens with focus on its own container: arrow keys and Enter never
  // reach the list. Its search field takes the focus when it has one, so
  // typing filters at once; else the Command root (`tabindex=-1`) does.
  const focusCommand = (event: Event): void => {
    onOpenAutoFocus?.(event);

    if (event.defaultPrevented) {
      return;
    }

    const command =
      ref?.querySelector<HTMLElement>("[data-slot=command-input]") ??
      ref?.querySelector<HTMLElement>("[data-slot=command]");

    if (command) {
      event.preventDefault();
      command.focus();
    }
  };
</script>

<PopoverPortal {...portalProps}>
  <PopoverPrimitive.Content
    bind:ref
    data-slot="popover-content"
    {sideOffset}
    {align}
    onOpenAutoFocus={focusCommand}
    class={cn(
      "bg-popover text-popover-foreground data-open:animate-in data-closed:animate-out data-closed:fade-out-0 data-open:fade-in-0 data-closed:zoom-out-95 data-open:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 ring-foreground/10 flex flex-col gap-2.5 rounded-lg p-2.5 text-sm shadow-md ring-1 duration-100 z-50 w-72 origin-(--bits-popover-content-transform-origin) outline-hidden",
      className,
    )}
    {...restProps}
  />
</PopoverPortal>
