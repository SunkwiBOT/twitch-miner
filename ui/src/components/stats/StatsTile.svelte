<script lang="ts">
  import type { Snippet } from "svelte";

  import { Card } from "$lib/components/ui/card";
  import type { IconName } from "../../lib/icons.constants";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Icon from "../ui/Icon.svelte";

  type StatsTilePropsType = {
    icon: IconName;
    value: string;
    label: string;
    topItem: Snippet;
  };

  let { icon, value, label, topItem }: StatsTilePropsType = $props();
</script>

<Card class="gap-2.5 p-4">
  <span class="inline-flex items-center gap-1.5 text-2xl font-bold tabular-nums">
    <Icon name={icon} size={18} class="text-link" />{value}
  </span>
  <span class="text-hint text-muted-foreground">{label}</span>
  <div class="mt-1 flex min-w-0 items-center gap-2 border-t pt-2.5">
    <span class="shrink-0 text-xs text-muted-foreground">{t.stats_tile_top_label()}</span>
    <span class="min-w-0 truncate text-hint">{@render topItem()}</span>
  </div>
</Card>
