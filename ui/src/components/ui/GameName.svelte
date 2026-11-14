<script lang="ts">
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt } from "../../lib/format.utils";
  import ExternalLink from "./ExternalLink.svelte";
  import Thumb from "./Thumb.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { LinkLook, LinkMark, LinkSize } from "./link.constants";
  import { ThumbShape, ThumbSize } from "./picture.constants";

  type GameNamePropsType = {
    name: string;
    url: string | null;
    art: string | undefined;
    size: ThumbSize;
    inSentence: boolean;
  };

  let { name, url, art, size, inSentence }: GameNamePropsType = $props();

  const picture = $derived(boxArt(art, BoxArtSize.ROW));
</script>

<span
  class="inline-flex max-w-full min-w-0 items-center align-middle"
  class:gap-2={size !== ThumbSize.XS}
  class:gap-1.5={size === ThumbSize.XS}
>
  <Thumb
    src={picture}
    round={false}
    contain={false}
    {size}
    shape={ThumbShape.BOX_ART}
    fallback={IconName.GIFT}
  />
  {#if url}
    <ExternalLink
      {url}
      text={name}
      mark={inSentence ? LinkMark.NONE : LinkMark.END}
      look={inSentence ? LinkLook.SENTENCE : LinkLook.HOVER}
      size={LinkSize.MD}
      twin={false}
    />
  {:else}
    <span class="truncate">{name}</span>
  {/if}
</span>
