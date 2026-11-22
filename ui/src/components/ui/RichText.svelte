<script lang="ts">
  // ! Links come from code, never the text: every dynamic value goes through `strong`, `code` or `escapeEmphasis`.
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt } from "../../lib/format.utils";
  import {
    parseRichText,
    RichTextPartKind,
    type TextCategoryType,
    type TextLinkType,
    type TextPersonType,
  } from "../../lib/text.utils";
  import Avatar from "./Avatar.svelte";
  import CategoryLink from "./CategoryLink.svelte";
  import ChannelLink from "./ChannelLink.svelte";
  import InlineCode from "./InlineCode.svelte";
  import TextLink from "./TextLink.svelte";
  import Thumb from "./Thumb.svelte";
  import { AvatarSize, ThumbShape, ThumbSize } from "./picture.constants";
  import { LinkMark, LinkSize } from "./link.constants";
  import { INLINE_PICTURE_CLASSES, RAISED_CLASSES } from "./ui.constants";
  import { provideRunningText } from "./running-text.svelte";

  type RichTextPropsType = {
    text: string;
    links?: TextLinkType[];
    people?: TextPersonType[];
    categories?: TextCategoryType[];
  };

  let { text, links = [], people = [], categories = [] }: RichTextPropsType = $props();

  provideRunningText();

  const parts = $derived(parseRichText(text, links, people, categories));
</script>

{#snippet boxArtThumb(category: TextCategoryType)}
  <Thumb
    src={boxArt(category.art, BoxArtSize.ROW)}
    round={false}
    contain={false}
    size={ThumbSize.XS}
    shape={ThumbShape.BOX_ART}
    fallback={null}
  />
{/snippet}

<!-- ! One inline box: a flex parent (a tooltip, a row with an icon) keeps the sentence as one paragraph. -->
<span>
  {#each parts as part, index (index)}{#if part.kind === RichTextPartKind.PERSON && part.person.channel}{@const channel =
        part.person.channel}<ChannelLink
        login={channel}
        name={part.person.name}
        avatar={part.person.avatar}
        size={AvatarSize.SM}
        mark={part.endsSentence ? LinkMark.OVERLAY : LinkMark.NONE}
      />{:else if part.kind === RichTextPartKind.PERSON}<span
        class="inline-flex items-baseline gap-1 align-baseline"
      >
        <span class={INLINE_PICTURE_CLASSES}>
          <Avatar url={part.person.avatar} name={part.person.name} size={AvatarSize.XS} />
        </span><strong class="font-semibold in-[.text-muted-foreground]:text-foreground">
          {part.person.name}
        </strong>
      </span>{:else if part.kind === RichTextPartKind.CATEGORY && part.category.url}<span
        class={RAISED_CLASSES}
      >
        <CategoryLink
          name={part.category.name}
          art={part.category.art}
          url={part.category.url}
          mark={part.endsSentence ? LinkMark.OVERLAY : LinkMark.NONE}
        />
      </span>{:else if part.kind === RichTextPartKind.CATEGORY}<span
        class="inline-flex items-baseline gap-1 align-baseline"
      >
        <span class={INLINE_PICTURE_CLASSES}>{@render boxArtThumb(part.category)}</span><strong
          class="font-semibold in-[.text-muted-foreground]:text-foreground"
        >
          {part.category.name}
        </strong>
      </span>{:else if part.kind === RichTextPartKind.LINK}<TextLink
        text={part.link.label}
        target={"url" in part.link
          ? {
              url: part.link.url,
              mark: part.endsSentence ? LinkMark.OVERLAY : LinkMark.NONE,
            }
          : part.link}
        size={LinkSize.MD}
      />{:else if part.kind === RichTextPartKind.CODE}<InlineCode
        text={part.text}
      />{:else if part.kind === RichTextPartKind.STRONG}<strong
        class="font-semibold in-[.text-muted-foreground]:text-foreground"
      >
        {part.text}
      </strong>{:else}{part.text}{/if}{/each}
</span>
