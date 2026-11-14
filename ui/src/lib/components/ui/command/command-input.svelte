<script lang="ts">
  // ! Edited after generation: the app's SearchField replaces the input group, so a menu search behaves like any other.
  import { Command as CommandPrimitive } from "bits-ui";
  import SearchField from "../../../../components/ui/SearchField.svelte";

  let {
    ref = $bindable(null),
    class: className,
    value = $bindable(""),
    ...restProps
  }: CommandPrimitive.InputProps = $props();
</script>

<div data-slot="command-input-wrapper" class="p-1 pb-0">
  <CommandPrimitive.Input
    {value}
    bind:ref
    data-slot="command-input"
    class={className}
    {...restProps}
  >
    {#snippet child({ props })}
      <SearchField {...props} bind:value busy={false} />
    {/snippet}
  </CommandPrimitive.Input>
</div>
