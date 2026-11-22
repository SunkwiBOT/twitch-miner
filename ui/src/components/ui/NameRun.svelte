<script lang="ts">
  import type { Snippet } from "svelte";

  import { NameListKind } from "./link.constants";
  import { provideNameList } from "./name-list.svelte";

  type NameRunPropsType = { children: Snippet };

  let { children }: NameRunPropsType = $props();

  provideNameList(NameListKind.RUN);
</script>

<!-- ! No element of its own: names run inline with separators, like a `NameList` without its list. -->
{@render children()}
