<script lang="ts">
  import type { IconName } from "../../lib/icons.constants";
  import type { BadgeTone } from "../../lib/tones.constants";
  import Badge from "../ui/Badge.svelte";

  type CampaignOverlayPropsType = { text: string; tone: BadgeTone; icon: IconName; hint: string };

  let { text, tone, icon, hint }: CampaignOverlayPropsType = $props();
</script>

<span
  class="absolute inset-0 flex items-center justify-center bg-background/55 p-2 backdrop-blur-[3px]"
>
  <span class="max-w-full rounded-4xl bg-background shadow-sm">
    <Badge
      {text}
      {tone}
      dot={false}
      {icon}
      {hint}
      class="h-auto py-1 text-center whitespace-normal"
    />
  </span>
</span>
