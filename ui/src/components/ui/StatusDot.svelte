<script lang="ts">
  import { cn } from "$lib/utils";
  import { BadgeTone } from "../../lib/tones.constants";
  import { DOT_TONE_CLASSES } from "./badge.constants";

  type StatusDotPropsType = { tone: BadgeTone };

  let { tone }: StatusDotPropsType = $props();
</script>

<span class={cn("inline-block size-2 shrink-0 rounded-full", DOT_TONE_CLASSES[tone])}></span>
