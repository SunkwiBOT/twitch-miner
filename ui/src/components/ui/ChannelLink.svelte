<script lang="ts">
  import { channelUrl } from "../../lib/links/twitch.utils";
  import ChannelAvatar from "./ChannelAvatar.svelte";
  import ExternalLink from "./ExternalLink.svelte";
  import { AvatarSize } from "./picture.constants";
  import { LinkLook, LinkMark, LinkSize } from "./link.constants";

  type ChannelLinkPropsType = { login: string; name: string; avatar: string | undefined };

  let { login, name, avatar }: ChannelLinkPropsType = $props();
</script>

{#snippet photo()}
  <ChannelAvatar url={avatar} {name} size={AvatarSize.XS} live={false} />
{/snippet}

<ExternalLink
  url={channelUrl(login)}
  text={name}
  mark={LinkMark.NONE}
  look={LinkLook.SENTENCE}
  size={LinkSize.MD}
  lead={photo}
  twin={false}
/>
