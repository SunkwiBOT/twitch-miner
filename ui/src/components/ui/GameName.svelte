<script lang="ts">
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt } from "../../lib/format.utils";
  import LinkPair from "./LinkPair.svelte";
  import LinkTwin from "./LinkTwin.svelte";
  import TextLink from "./TextLink.svelte";
  import Thumb from "./Thumb.svelte";
  import { LinkMark, LinkSize } from "./link.constants";
  import { ThumbShape, ThumbSize } from "./picture.constants";

  type GameNamePropsType = {
    name: string;
    url: string | null;
    art: string | undefined;
    size: ThumbSize;
  };

  let { name, url, art, size }: GameNamePropsType = $props();

  const picture = $derived(boxArt(art, BoxArtSize.ROW));
</script>

{#snippet thumb()}
  <Thumb
    src={picture}
    round={false}
    contain={false}
    {size}
    shape={ThumbShape.BOX_ART}
    fallback={null}
  />
{/snippet}

<span
  class="inline-flex max-w-full min-w-0 items-center align-middle"
  class:gap-2={size !== ThumbSize.XS}
  class:gap-1.5={size === ThumbSize.XS}
>
  {#if url}
    <LinkPair>
      <LinkTwin target={{ url }} live={false}>{@render thumb()}</LinkTwin>
      <TextLink text={name} target={{ url, mark: LinkMark.END }} size={LinkSize.MD} />
    </LinkPair>
  {:else}
    {@render thumb()}
    <span class="truncate">{name}</span>
  {/if}
</span>
