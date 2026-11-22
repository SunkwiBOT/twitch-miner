<script lang="ts">
  import { cn } from "$lib/utils";
  import { FILLED_ICONS, ICONS, SPINNING_ICONS } from "./icon-registry.constants";
  import type { IconName } from "../../lib/icons.constants";

  type IconPropsType = { name: IconName; size: number; class?: string };

  let { name, size, class: className }: IconPropsType = $props();

  const icon = $derived(ICONS[name]);
  const classes = $derived(cn(className, SPINNING_ICONS.has(name) && "animate-spin"));
</script>

{#if "path" in icon}
  <svg
    class={classes}
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
    class={classes}
    {size}
    fill={FILLED_ICONS.has(name) ? "currentColor" : "none"}
    aria-hidden="true"
  />
{/if}
