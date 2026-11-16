<script lang="ts">
  import type { Snippet } from "svelte";

  type PageBodyPropsType = { children: Snippet };

  let { children }: PageBodyPropsType = $props();
</script>

<!-- ! A size container: a page's own content can fill the scroller's height with cqh units. -->
<main
  class="relative flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto p-6 @container-size *:shrink-0 max-md:gap-4 max-md:p-3"
>
  {@render children()}
</main>
