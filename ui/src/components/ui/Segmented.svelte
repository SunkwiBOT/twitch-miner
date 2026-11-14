<script lang="ts" generics="T extends string">
  import { Tabs, TabsList, TabsTrigger } from "$lib/components/ui/tabs";
  import { number } from "../../lib/format.utils";
  import Badge from "./Badge.svelte";
  import Icon from "./Icon.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import type { IconName } from "../../lib/icons.constants";
  import { segmentId } from "./ui.utils";

  type OptionType = { value: T; label: string; icon?: IconName; count?: number };

  type SegmentedPropsType = {
    value: T;
    options: OptionType[];
    label: string;
    panel?: string;
    onchange: (value: T) => void;
  };

  let { value, options, label, panel, onchange }: SegmentedPropsType = $props();
</script>

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
      <TabsTrigger
        value={option.value}
        id={panel ? segmentId(panel, option.value) : undefined}
        aria-controls={panel}
        class="h-[25px] px-2.5"
      >
        {#if option.icon}<Icon name={option.icon} size={14} />{/if}
        {option.label}
        {#if option.count}<Badge
            text={number(option.count)}
            tone={BadgeTone.NEUTRAL}
            dot={false}
          />{/if}
      </TabsTrigger>
    {/each}
  </TabsList>
</Tabs>
