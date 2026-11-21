<script lang="ts">
  import type { Snippet } from "svelte";

  import { Alert, AlertDescription } from "$lib/components/ui/alert";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import RichText from "./RichText.svelte";
  import { NOTICE_TONE_CLASSES, NOTICE_TONE_ICONS, NoticeTone } from "./ui.constants";

  type NoticePropsType = { tone: NoticeTone; text: string; actions?: Snippet };

  let { tone, text, actions }: NoticePropsType = $props();
</script>

<Alert
  class={cn(
    "flex gap-2.5",
    actions ? "flex-wrap items-center" : "items-start",
    NOTICE_TONE_CLASSES[tone],
  )}
>
  <span class={cn("shrink-0", !actions && "mt-0.5")}>
    <Icon name={NOTICE_TONE_ICONS[tone]} size={16} />
  </span>
  <AlertDescription class="text-hint text-current"><RichText {text} /></AlertDescription>
  {#if actions}
    <div class="ml-auto flex shrink-0 items-center gap-2">{@render actions()}</div>
  {/if}
</Alert>
