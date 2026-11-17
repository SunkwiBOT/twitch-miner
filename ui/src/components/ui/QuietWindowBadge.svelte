<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { minutesToClock } from "../../lib/time/quiet-window.utils";
  import Badge from "./Badge.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { IconName } from "../../lib/icons.constants";

  type QuietWindowBadgePropsType = { class?: string };

  let { class: className }: QuietWindowBadgePropsType = $props();

  const quietWindow = $derived(app.snapshot?.settings.quiet_window);
  const active = $derived(app.snapshot?.quiet_window_active ?? false);
</script>

{#if active && quietWindow}
  <Badge
    class={className}
    text={t.quiet_window_active_badge()}
    tone={BadgeTone.INFO}
    dot={true}
    icon={IconName.MOON}
    hint={t.quiet_window_active_hint({
      start: minutesToClock(quietWindow.start_minutes),
      end: minutesToClock(quietWindow.end_minutes),
    })}
  />
{/if}
