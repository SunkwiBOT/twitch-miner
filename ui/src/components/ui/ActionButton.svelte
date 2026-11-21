<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import RichText from "./RichText.svelte";
  import Spinner from "./Spinner.svelte";
  import Tip from "./Tip.svelte";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANT_CLASSES,
    BUTTON_VARIANTS,
    ButtonSize,
    ButtonType,
    ButtonVariant,
  } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";

  type ActionButtonPropsType = {
    icon: IconName;
    label: string;
    ariaLabel?: string;
    busyLabel: string;
    busy: boolean;
    disabled: boolean;
    reason?: string;
    variant: ButtonVariant;
    size: ButtonSize;
    type: ButtonType;
    class?: string;
    describedby?: string;
    onclick?: () => unknown;
  };

  let {
    icon,
    label,
    ariaLabel,
    busyLabel,
    busy,
    disabled,
    reason,
    variant,
    size,
    type,
    class: className,
    describedby,
    onclick,
  }: ActionButtonPropsType = $props();

  const iconSize = $derived(size === ButtonSize.SM ? 13 : 14);
</script>

{#snippet button(props: Record<string, unknown>)}
  <Button
    variant={BUTTON_VARIANTS[variant]}
    size={BUTTON_SIZES[size]}
    class={cn(BUTTON_VARIANT_CLASSES[variant], reason && "cursor-help", className)}
    type={type as "button" | "submit"}
    aria-label={ariaLabel}
    aria-busy={busy}
    aria-describedby={describedby}
    onclick={reason ? undefined : onclick}
    {...props}
    disabled={reason ? undefined : disabled || busy}
    aria-disabled={reason ? true : undefined}
  >
    {#if busy}<Spinner size={iconSize} />{:else}<Icon name={icon} size={iconSize} />{/if}
    <span class="truncate"><RichText text={busy ? busyLabel : label} /></span>
  </Button>
{/snippet}

<!-- ! A reason keeps the button focusable (`aria-disabled`, not `disabled`): its tip must open on hover and focus. -->
{#if reason}
  <Tip text={reason}>
    {#snippet children(props)}{@render button(props)}{/snippet}
  </Tip>
{:else}
  {@render button({})}
{/if}
