<script lang="ts">
  import type { Snippet } from "svelte";

  import { provideNameList } from "./name-list.svelte";

  type NameRunPropsType = { children: Snippet };

  let { children }: NameRunPropsType = $props();

  provideNameList();
</script>

<!-- ! No element of its own: names run inline with separators, like a `NameList` without its list. -->
{@render children()}
