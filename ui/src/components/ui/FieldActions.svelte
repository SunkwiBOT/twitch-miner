<script lang="ts">
  import type { Snippet } from "svelte";

  type FieldActionsPropsType = { field: Snippet; actions: Snippet };

  let { field, actions }: FieldActionsPropsType = $props();
</script>

<div class="flex flex-wrap gap-2">
  <div class="min-w-40 flex-1">{@render field()}</div>
  <div class="flex shrink-0 gap-2">{@render actions()}</div>
</div>
