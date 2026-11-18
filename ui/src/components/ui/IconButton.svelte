<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import Tip from "./Tip.svelte";
  import {
    BUTTON_VARIANTS,
    BUTTON_VARIANT_CLASSES,
    ButtonSize,
    ButtonVariant,
    ICON_BUTTON_SIZES,
  } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";

  type IconButtonPropsType = {
    icon: IconName;
    label: string;
    tooltip?: string;
    busy: boolean;
    disabled: boolean;
    variant: ButtonVariant;
    size: ButtonSize;
    class?: string;
    onclick: () => unknown;
  };

  let {
    icon,
    label,
    tooltip,
    busy,
    disabled,
    variant,
    size,
    class: className,
    onclick,
  }: IconButtonPropsType = $props();

  const iconSize = $derived(size === ButtonSize.SM ? 14 : 16);
</script>

<Tip text={tooltip ?? label}>
  {#snippet children(props)}
    <Button
      {...props}
      variant={BUTTON_VARIANTS[variant]}
      size={ICON_BUTTON_SIZES[size]}
      class={cn(BUTTON_VARIANT_CLASSES[variant], className)}
      type="button"
      aria-label={label}
      disabled={disabled || busy}
      aria-busy={busy}
      {onclick}
    >
      {#if busy}<Spinner size={iconSize} />{:else}<Icon name={icon} size={iconSize} />{/if}
    </Button>
  {/snippet}
</Tip>
