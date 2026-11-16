<script lang="ts">
  // ! Edited after generation: `group-data-[disabled=true]:pointer-events-none` replaced by
  // `cursor-not-allowed`, to match the peer's disabled cursor.

  import { Label as LabelPrimitive } from "bits-ui";
  import { cn } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    ...restProps
  }: LabelPrimitive.RootProps = $props();
</script>

<LabelPrimitive.Root
  bind:ref
  data-slot="label"
  class={cn(
    "gap-2 text-sm leading-none font-medium group-data-[disabled=true]:opacity-50 peer-disabled:opacity-50 flex items-center select-none group-data-[disabled=true]:cursor-not-allowed peer-disabled:cursor-not-allowed",
    className,
  )}
  {...restProps}
/>
