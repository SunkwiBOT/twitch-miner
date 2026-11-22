<script lang="ts">
  // ! Links come from code, never the text: every dynamic value goes through `strong`, `code` or `escapeEmphasis`.
  import {
    parseRichText,
    RichTextPartKind,
    type TextLinkType,
    type TextPersonType,
  } from "../../lib/text.utils";
  import Avatar from "./Avatar.svelte";
  import ChannelLink from "./ChannelLink.svelte";
  import InlineCode from "./InlineCode.svelte";
  import TextLink from "./TextLink.svelte";
  import { AvatarSize } from "./picture.constants";
  import { LinkMark, LinkSize } from "./link.constants";

  type RichTextPropsType = {
    text: string;
    links?: TextLinkType[];
    people?: TextPersonType[];
  };

  let { text, links = [], people = [] }: RichTextPropsType = $props();

  const parts = $derived(parseRichText(text, links, people));
</script>

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
        class="inline-flex items-center gap-1 align-[-0.3em]"
      >
        <Avatar url={part.person.avatar} name={part.person.name} size={AvatarSize.XS} /><strong
          class="font-semibold in-[.text-muted-foreground]:text-foreground"
        >
          {part.person.name}
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
