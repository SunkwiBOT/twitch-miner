<script lang="ts">
  import type { Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import { provideNameList } from "./name-list.svelte";

  type NameListPropsType = { class?: string; children: Snippet };

  let { class: className, children }: NameListPropsType = $props();

  provideNameList();
</script>

<ul class={cn("flex flex-wrap", className)}>
  {@render children()}
</ul>
