<script lang="ts">
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt } from "../../lib/format.utils";
  import TextLink from "./TextLink.svelte";
  import Thumb from "./Thumb.svelte";
  import { LinkMark, LinkSize } from "./link.constants";
  import { ThumbShape, ThumbSize } from "./picture.constants";

  type CategoryLinkPropsType = {
    name: string;
    art: string | undefined;
    url: string;
    mark: LinkMark;
  };

  let { name, art, url, mark }: CategoryLinkPropsType = $props();
</script>

<!-- ! The running-sentence twin of `GameName`: the picture sits inside the link, as a channel's photo does. -->
{#snippet picture()}
  <Thumb
    src={boxArt(art, BoxArtSize.ROW)}
    round={false}
    contain={false}
    size={ThumbSize.XS}
    shape={ThumbShape.BOX_ART}
    fallback={null}
  />
{/snippet}

<TextLink text={name} target={{ url, mark }} size={LinkSize.MD} lead={picture} />
