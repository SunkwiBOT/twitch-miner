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

  // ! A badge narrower than its text wraps it rather than cutting it.
  const classes = $derived(
    cn(BADGE_TONE_CLASSES[tone], "h-auto min-h-5 max-w-full whitespace-normal", className),
  );
</script>

{#snippet content()}
  {#if icon}<Icon name={icon} size={12} class="shrink-0" />{/if}
  <!-- ! The padding leaves room for ink past the line box (Thai marks): only the badge clips it. -->
  <span class="-m-0.5 min-w-0 p-0.5 leading-3.5 break-words">{text}</span>
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
