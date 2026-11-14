<script lang="ts">
  import Icon from "./Icon.svelte";
  import { IconName } from "../../lib/icons.constants";

  type SpinnerPropsType = { size: number };

  let { size }: SpinnerPropsType = $props();
</script>

<span
  class="inline-flex shrink-0 animate-spin"
  style:width="{size}px"
  style:height="{size}px"
  aria-hidden="true"
>
  <Icon name={IconName.LOADER} {size} />
</span>
