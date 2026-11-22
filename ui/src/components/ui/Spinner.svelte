<script lang="ts">
  import Icon from "./Icon.svelte";
  import { IconName } from "../../lib/icons.constants";

  type SpinnerPropsType = { size: number };

  let { size }: SpinnerPropsType = $props();
</script>

<Icon name={IconName.LOADER} {size} class="shrink-0" />
