<script lang="ts" generics="T extends string">
  import { Tabs, TabsList, TabsTrigger } from "$lib/components/ui/tabs";
  import { number } from "../../lib/format.utils";
  import Badge from "./Badge.svelte";
  import Icon from "./Icon.svelte";
  import Tip from "./Tip.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import type { IconName } from "../../lib/icons.constants";
  import { segmentId } from "./ui.utils";

  // ! `disabled` + `tip`: the trigger itself carries the native `disabled`
  // (bits-ui never activates it, and it leaves the tab order), while the
  // tooltip wrapper around it stays the sole focusable stop explaining why.
  type OptionType = {
    value: T;
    label: string;
    icon?: IconName;
    count?: number;
    disabled?: boolean;
    tip?: string;
  };

  type SegmentedPropsType = {
    value: T;
    options: OptionType[];
    label: string;
    panel?: string;
    onchange: (value: T) => void;
  };

  let { value, options, label, panel, onchange }: SegmentedPropsType = $props();
</script>

{#snippet trigger(option: OptionType)}
  <TabsTrigger
    value={option.value}
    id={panel ? segmentId(panel, option.value) : undefined}
    aria-controls={panel}
    disabled={option.disabled}
    class="h-[25px] px-2.5 disabled:cursor-help disabled:opacity-50 not-data-active:hover:text-foreground/60"
  >
    {#if option.icon}<Icon name={option.icon} size={14} />{/if}
    {option.label}
    {#if option.count}<Badge text={number(option.count)} tone={BadgeTone.NEUTRAL} />{/if}
  </TabsTrigger>
{/snippet}

<Tabs
  class="max-w-full min-w-0"
  {value}
  onValueChange={(next) => {
    const option = options.find((item) => item.value === next);

    if (option) {
      onchange(option.value);
    }
  }}
>
  <TabsList
    aria-label={label}
    class="min-h-8 max-w-full flex-wrap justify-start gap-y-0.5 group-data-horizontal/tabs:h-auto"
  >
    {#each options as option (option.value)}
      {#if option.disabled && option.tip}
        <Tip text={option.tip}>
          {#snippet children(tipProps)}
            <span {...tipProps} class="cursor-help">{@render trigger(option)}</span>
          {/snippet}
        </Tip>
      {:else}
        {@render trigger(option)}
      {/if}
    {/each}
  </TabsList>
</Tabs>
