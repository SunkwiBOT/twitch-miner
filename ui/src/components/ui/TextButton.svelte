<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    BUTTON_VARIANT_CLASSES,
    ButtonSize,
    ButtonType,
    ButtonVariant,
  } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";

  type TextButtonPropsType = {
    label: string;
    name?: string;
    icon: IconName;
    variant: ButtonVariant;
    size: ButtonSize;
    type: ButtonType;
    disabled: boolean;
    class?: string;
    onclick?: () => unknown;
  };

  let {
    label,
    name,
    icon,
    variant,
    size,
    type,
    disabled,
    class: className,
    onclick,
  }: TextButtonPropsType = $props();

  const iconSize = $derived(size === ButtonSize.SM ? 13 : 14);
</script>

<Button
  variant={BUTTON_VARIANTS[variant]}
  size={BUTTON_SIZES[size]}
  class={cn(BUTTON_VARIANT_CLASSES[variant], className)}
  {type}
  aria-label={name}
  {disabled}
  {onclick}
>
  <Icon name={icon} size={iconSize} />
  {label}
</Button>
