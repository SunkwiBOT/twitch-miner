<script lang="ts">
  import type { Snippet } from "svelte";

  import TextButton from "./TextButton.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import type { TextPersonType } from "../../lib/text.utils";
  import ActionButton from "./ActionButton.svelte";
  import Dialog from "./Dialog.svelte";
  import RichText from "./RichText.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";

  type ConfirmDialogPropsType = {
    open: boolean;
    title: string;
    text: string;
    people?: TextPersonType[];
    confirmLabel: string;
    busyLabel: string;
    busy: boolean;
    lead?: Snippet;
    extra?: Snippet;
    onconfirm: () => unknown;
    oncancel: () => void;
  };

  let {
    open,
    title,
    text,
    people,
    confirmLabel,
    busyLabel,
    busy,
    lead,
    extra,
    onconfirm,
    oncancel,
  }: ConfirmDialogPropsType = $props();
</script>

<Dialog
  {open}
  {title}
  wide={false}
  closable={!busy}
  closeOnOutside={true}
  closeLabel={t.common_close()}
  onclose={oncancel}
  {lead}
>
  <p class="text-hint text-muted-foreground"><RichText {text} {people} /></p>
  {#if extra}{@render extra()}{/if}
  {#snippet footer()}
    <TextButton
      label={t.common_cancel()}
      icon={IconName.X}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      disabled={busy}
      onclick={oncancel}
    />
    <ActionButton
      icon={IconName.TRASH}
      label={confirmLabel}
      {busyLabel}
      {busy}
      disabled={false}
      variant={ButtonVariant.DANGER}
      size={ButtonSize.MD}
      type={ButtonType.BUTTON}
      onclick={onconfirm}
    />
  {/snippet}
</Dialog>
