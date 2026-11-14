<script lang="ts">
  import { cn } from "$lib/utils";

  type RewardImagePropsType = {
    url: string;
    color: string | undefined;
    size: number;
  };

  let { url, color, size }: RewardImagePropsType = $props();
</script>

<!-- ! Twitch's default reward image is white and only reads on the streamer's colour. -->
<span
  class={cn("grid shrink-0 place-items-center rounded-md", !color && "bg-muted")}
  style:width="{size}px"
  style:height="{size}px"
  style:background-color={color}
>
  <img class="size-[80%] object-contain" src={url} alt="" loading="lazy" />
</span>
