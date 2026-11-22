<script lang="ts">
  import type { ChannelType } from "../../lib/bindings/ChannelType";
  import { gameArt, gameUrl } from "../../lib/catalogue/games.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import ChannelAvatar from "../ui/ChannelAvatar.svelte";
  import GameName from "../ui/GameName.svelte";
  import TextButton from "../ui/TextButton.svelte";
  import ViewerCount from "../ui/ViewerCount.svelte";
  import { AvatarSize, ThumbSize } from "../ui/picture.constants";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";

  type StreamerChannelCardPropsType = {
    channel: ChannelType | null;
    name: string;
    onchange: (() => void) | null;
  };

  let { channel, name, onchange }: StreamerChannelCardPropsType = $props();
</script>

<div class="flex items-center gap-3 rounded-lg border bg-muted/40 p-3">
  <ChannelAvatar
    url={channel?.avatar_url}
    {name}
    size={AvatarSize.LG}
    live={channel?.live === true}
  />
  <div class="flex min-w-0 flex-1 flex-col gap-0.5">
    <span class="flex min-w-0 items-center gap-1.5">
      <span class="truncate font-semibold">{name}</span>
    </span>
    {#if channel?.live}
      <span class="flex min-w-0 items-center gap-2 text-xs text-muted-foreground">
        {#if channel.game}<GameName
            name={channel.game.name}
            url={gameUrl(channel.game.name, channel.game.slug)}
            art={gameArt(channel.game.name, channel.game.box_art_url)}
            size={ThumbSize.XS}
          />{/if}
        {#if channel.viewers !== undefined}<ViewerCount count={channel.viewers} />{/if}
      </span>
    {/if}
  </div>
  {#if onchange}
    <TextButton
      label={t.streamer_change()}
      icon={IconName.EDIT}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.SM}
      type={ButtonType.BUTTON}
      disabled={false}
      onclick={onchange}
    />
  {/if}
</div>
