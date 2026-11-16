<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import { copyText } from "../../lib/clipboard.utils";
  import { describeError } from "../../lib/errors/errors.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ToastKind } from "../../lib/state/state.constants";
  import { toast } from "../../lib/state/toasts";
  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import Tip from "./Tip.svelte";
  import {
    BUTTON_SIZES,
    BUTTON_VARIANTS,
    BUTTON_VARIANT_CLASSES,
    ButtonSize,
    ButtonType,
    ButtonVariant,
    ICON_BUTTON_SIZES,
  } from "./button.constants";
  import { COPIED_FEEDBACK_MS, CopyLayout } from "./ui.constants";
  import { IconName } from "../../lib/icons.constants";

  type CopyButtonPropsType = {
    text: () => Promise<string> | string;
    label: string;
    copiedLabel: string;
    name?: string;
    layout: CopyLayout;
    size: ButtonSize;
  };

  let { text, label, copiedLabel, name, layout, size }: CopyButtonPropsType = $props();

  let copied = $state(false);
  let busy = $state(false);

  const iconSize = $derived(size === ButtonSize.SM ? 14 : 16);

  const copy = async (): Promise<void> => {
    busy = true;

    try {
      if (await copyText(text())) {
        copied = true;
        setTimeout(() => (copied = false), COPIED_FEEDBACK_MS);
      } else {
        toast(ToastKind.ERROR, t.action_failed(), { description: t.copy_failed() });
      }
    } catch (e: any) {
      toast(ToastKind.ERROR, t.action_failed(), { description: describeError(e) });
    } finally {
      busy = false;
    }
  };
</script>

{#snippet mark()}
  {#if busy}<Spinner size={iconSize} />{:else}<span
      class={cn("inline-flex", copied && "text-success-emphasis")}
      ><Icon name={copied ? IconName.CHECK : IconName.COPY} size={iconSize} /></span
    >{/if}
{/snippet}

{#if layout === CopyLayout.ICON}
  <Tip text={copied ? copiedLabel : label}>
    {#snippet children(props)}
      <Button
        {...props}
        variant={BUTTON_VARIANTS[ButtonVariant.GHOST]}
        size={ICON_BUTTON_SIZES[size]}
        class={cn(BUTTON_VARIANT_CLASSES[ButtonVariant.GHOST], "not-aria-busy:cursor-copy")}
        type={ButtonType.BUTTON}
        aria-label={name ?? label}
        disabled={busy}
        aria-busy={busy}
        onclick={copy}>{@render mark()}</Button
      >
    {/snippet}
  </Tip>
{:else}
  <Button
    variant={BUTTON_VARIANTS[ButtonVariant.OUTLINE]}
    size={BUTTON_SIZES[size]}
    type={ButtonType.BUTTON}
    class="not-aria-busy:cursor-copy"
    disabled={busy}
    aria-busy={busy}
    onclick={copy}>{@render mark()}{copied ? copiedLabel : label}</Button
  >
{/if}

<span class="sr-only" aria-live="polite">{copied ? copiedLabel : ""}</span>
