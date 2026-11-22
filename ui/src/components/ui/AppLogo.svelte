<script lang="ts">
  import { cn } from "$lib/utils";
  import { APP_LOGO_URL } from "../../lib/partners.constants";

  type AppLogoPropsType = { class: string };

  let { class: className }: AppLogoPropsType = $props();
</script>

<img class={cn("shrink-0 [image-rendering:pixelated]", className)} src={APP_LOGO_URL} alt="" />
