<script lang="ts">
  import type { Snippet } from "svelte";

  import { provideLinkPair } from "./link-pair.svelte";

  type LinkPairPropsType = { children: Snippet };

  let { children }: LinkPairPropsType = $props();

  provideLinkPair();
</script>

<!-- ! No element of its own: a picture and its name often sit in different cells of a grid. -->
{@render children()}
