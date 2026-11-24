<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { BadgeTone } from "../../lib/tones.constants";
  import { availableVersion } from "../../lib/updates/updates.svelte";
  import Badge from "../ui/Badge.svelte";
  import Icon from "../ui/Icon.svelte";

  type VersionBadgePropsType = { current: string };

  let { current }: VersionBadgePropsType = $props();

  const next = $derived(availableVersion());
</script>

{#snippet running()}
  <Badge text={`v${current}`} tone={BadgeTone.NEUTRAL} />
{/snippet}

{#if next && next !== current}
  <span
    role="group"
    aria-label={t.updates_version_change({ currentVersion: current, newVersion: next })}
    class="inline-flex shrink-0 items-center gap-1.5 whitespace-nowrap"
  >
    {@render running()}
    <Icon name={IconName.ARROW_RIGHT} size={12} class="shrink-0 text-muted-foreground" />
    <Badge text={`v${next}`} tone={BadgeTone.SUCCESS} />
  </span>
{:else}
  {@render running()}
{/if}
