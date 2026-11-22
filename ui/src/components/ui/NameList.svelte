<script lang="ts">
  import { untrack, type Snippet } from "svelte";

  import { cn } from "$lib/utils";
  import { NameListKind } from "./link.constants";
  import { provideNameList } from "./name-list.svelte";

  type NameListPropsType = {
    kind: NameListKind.WRAP | NameListKind.COLUMN;
    class?: string;
    children: Snippet;
  };

  let { kind, class: className, children }: NameListPropsType = $props();

  provideNameList(untrack(() => kind));
</script>

<ul class={cn("flex", kind === NameListKind.WRAP ? "flex-wrap" : "flex-col", className)}>
  {@render children()}
</ul>
