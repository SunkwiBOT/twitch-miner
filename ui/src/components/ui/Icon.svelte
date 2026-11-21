<script lang="ts">
  import { FILLED_ICONS, ICONS } from "./icons.constants";
  import type { IconName } from "../../lib/icons.constants";

  type IconPropsType = { name: IconName; size: number; class?: string };

  let { name, size, class: className }: IconPropsType = $props();

  const icon = $derived(ICONS[name]);
</script>

{#if "path" in icon}
  <svg
    class={className}
    width={size}
    height={size}
    viewBox={icon.viewBox}
    fill="currentColor"
    aria-hidden="true"
  >
    <path d={icon.path} fill-rule="evenodd" />
  </svg>
{:else}
  {@const Lucide = icon}
  <Lucide
    class={className}
    {size}
    fill={FILLED_ICONS.has(name) ? "currentColor" : "none"}
    aria-hidden="true"
  />
{/if}
