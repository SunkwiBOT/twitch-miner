<script lang="ts">
  // ! Links come from code, never the text: every dynamic value goes through `strong` or `escapeEmphasis`.
  import { channelUrl } from "../../lib/links/twitch.utils";
  import {
    LINK_DELIMITER,
    PERSON_DELIMITER,
    RICH_TEXT_TOKENS,
    type TextLinkType,
    type TextPersonType,
  } from "../../lib/text.utils";
  import Avatar from "./Avatar.svelte";
  import ChannelAvatar from "./ChannelAvatar.svelte";
  import ExternalLink from "./ExternalLink.svelte";
  import { AvatarSize } from "./picture.constants";
  import { LinkLook, LinkMark, LinkSize } from "./link.constants";

  type RichTextPropsType = {
    text: string;
    links?: TextLinkType[];
    people?: TextPersonType[];
  };

  type PartType = {
    text: string;
    strong: boolean;
    link: TextLinkType | null;
    person: TextPersonType | null;
  };

  let { text, links, people }: RichTextPropsType = $props();

  const parts = $derived.by(() => {
    const out: PartType[] = [];
    let bold = false;

    for (const token of text.split(RICH_TEXT_TOKENS)) {
      if (token === "**") {
        bold = !bold;
      } else if (token.startsWith(LINK_DELIMITER)) {
        const link = links?.[Number(token.slice(1, -1))];

        if (link) {
          out.push({ text: link.label, strong: bold, link, person: null });
        }
      } else if (token.startsWith(PERSON_DELIMITER)) {
        const person = people?.[Number(token.slice(1, -1))];

        if (person) {
          out.push({ text: person.name, strong: true, link: null, person });
        }
      } else if (token) {
        out.push({ text: token, strong: bold, link: null, person: null });
      }
    }

    return out;
  });
</script>

{#each parts as part, index (index)}{#if part.person?.channel}<span class="whitespace-nowrap"
      ><span class="mr-1.5 inline-flex align-middle"
        ><ChannelAvatar
          url={part.person.avatar}
          name={part.person.name}
          size={AvatarSize.SM}
          live={false}
        /></span
      ><ExternalLink
        url={channelUrl(part.person.channel)}
        text={part.text}
        mark={LinkMark.NONE}
        look={LinkLook.SENTENCE}
        size={LinkSize.MD}
      /></span
    >{:else if part.person}<span class="inline-flex items-center gap-1 align-[-0.3em]"
      ><Avatar url={part.person.avatar} name={part.person.name} size={AvatarSize.XS} /><strong
        class="font-semibold in-[.text-muted-foreground]:text-foreground">{part.text}</strong
      ></span
    >{:else if part.link}<ExternalLink
      url={part.link.url}
      text={part.text}
      mark={LinkMark.NONE}
      look={LinkLook.SENTENCE}
      size={LinkSize.MD}
    />{:else if part.strong}<strong
      class="font-semibold in-[.text-muted-foreground]:text-foreground">{part.text}</strong
    >{:else}{part.text}{/if}{/each}
