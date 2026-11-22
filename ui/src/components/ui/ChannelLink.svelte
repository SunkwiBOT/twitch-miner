<script lang="ts">
  import { channelUrl } from "../../lib/links/twitch.utils";
  import ChannelAvatar from "./ChannelAvatar.svelte";
  import TextLink from "./TextLink.svelte";
  import { AvatarSize } from "./picture.constants";
  import { LinkMark, LinkSize } from "./link.constants";

  type ChannelLinkPropsType = {
    login: string;
    name: string;
    avatar: string | undefined;
    size?: AvatarSize;
    mark?: LinkMark;
  };

  let {
    login,
    name,
    avatar,
    size = AvatarSize.XS,
    mark = LinkMark.END,
  }: ChannelLinkPropsType = $props();
</script>

{#snippet photo()}
  <ChannelAvatar url={avatar} {name} {size} live={false} />
{/snippet}

<TextLink text={name} target={{ url: channelUrl(login), mark }} size={LinkSize.MD} lead={photo} />
