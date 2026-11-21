<script lang="ts">
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import Tip from "./Tip.svelte";
  import { BUTTON_VARIANT_CLASSES, ButtonVariant } from "./button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { escapeEmphasis } from "../../lib/text.utils";

  type ChipRemoveButtonPropsType = {
    label: string;
    name: string;
    iconSize: number;
    onclick: () => void;
  };

  let { label, name, iconSize, onclick }: ChipRemoveButtonPropsType = $props();
</script>

<Tip text={escapeEmphasis(label)}>
  {#snippet children(props)}
    <button
      {...props}
      class={cn(
        "flex size-5 items-center justify-center rounded-full",
        BUTTON_VARIANT_CLASSES[ButtonVariant.DANGER_GHOST],
      )}
      type="button"
      aria-label={`${label} ${name}`}
      {onclick}
    >
      <Icon name={IconName.X} size={iconSize} />
    </button>
  {/snippet}
</Tip>
