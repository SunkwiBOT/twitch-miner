<script lang="ts">
  import { Badge as BadgeRoot } from "$lib/components/ui/badge";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import Tip from "./Tip.svelte";
  import { BADGE_TONE_CLASSES, BADGE_VARIANTS, BadgeVariant } from "./badge.constants";
  import { BadgeTone } from "../../lib/tones.constants";
  import { IconName } from "../../lib/icons.constants";

  type BadgePropsType = {
    text: string;
    tone: BadgeTone;
    icon?: IconName;
    hint?: string;
    class?: string;
  };

  let { text, tone, icon, hint, class: className }: BadgePropsType = $props();

  // ! A label stays on one line: only a badge squeezed below its text ends it with an ellipsis.
  const classes = $derived(cn(BADGE_TONE_CLASSES[tone], "max-w-full", className));
</script>

{#snippet content()}
  {#if icon}<Icon name={icon} size={12} class="shrink-0" />{/if}
  <!-- ! The padding keeps ink past the line box (Thai marks) inside the clip. -->
  <span class="-m-0.5 min-w-0 overflow-hidden p-0.5 leading-3.5 text-ellipsis">{text}</span>
{/snippet}

{#if hint}
  <Tip text={hint}>
    {#snippet children(props)}
      <BadgeRoot
        {...props}
        variant={BADGE_VARIANTS[BadgeVariant.TONE]}
        class={cn(classes, "cursor-help")}
      >
        {@render content()}
      </BadgeRoot>
    {/snippet}
  </Tip>
{:else}
  <BadgeRoot variant={BADGE_VARIANTS[BadgeVariant.TONE]} class={classes}>
    {@render content()}
  </BadgeRoot>
{/if}
