<script lang="ts">
  import { Badge as BadgeRoot } from "$lib/components/ui/badge";
  import { cn } from "$lib/utils";
  import Icon from "./Icon.svelte";
  import RichText from "./RichText.svelte";
  import Tip from "./Tip.svelte";
  import {
    BADGE_MARQUEE_PX_PER_SECOND,
    BADGE_MARQUEE_TRAVEL_SHARE,
    BADGE_TONE_CLASSES,
    BADGE_VARIANTS,
    BadgeVariant,
  } from "./badge.constants";
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

  // ! A label stays on one line: a badge squeezed below its text scrolls it instead of cutting it.
  const classes = $derived(cn(BADGE_TONE_CLASSES[tone], "max-w-full", className));

  let overflow = $state(0);

  const marqueeSeconds = $derived(
    overflow / BADGE_MARQUEE_PX_PER_SECOND / BADGE_MARQUEE_TRAVEL_SHARE,
  );

  const tip = $derived(overflow > 0 && hint ? `${text} · ${hint}` : overflow > 0 ? text : hint);

  // ! The text's own width against the room its clip leaves, again whenever either one resizes.
  const measure = (clip: HTMLElement): (() => void) => {
    const label = clip.firstElementChild as HTMLElement;

    // ! A badge left no room at all (undisplayed, or a screen-reader-only copy) is not laid out to be read.
    const update = (): void => {
      const style = getComputedStyle(clip);
      const room =
        clip.clientWidth - parseFloat(style.paddingLeft) - parseFloat(style.paddingRight);

      overflow = room > 0 ? Math.max(0, Math.ceil(label.offsetWidth - room)) : 0;
    };

    const observer = new ResizeObserver(update);

    observer.observe(clip);
    observer.observe(label);

    return () => observer.disconnect();
  };
</script>

{#snippet content()}
  {#if icon}<Icon name={icon} size={12} class="shrink-0" />{/if}
  <!-- ! The padding keeps ink past the line box (Thai marks) inside the clip. -->
  <span
    class="-m-0.5 min-w-0 overflow-hidden p-0.5 leading-3.5 [&_strong]:font-bold"
    {@attach measure}
  >
    <span
      class="inline-block"
      data-marquee={overflow > 0 ? "" : undefined}
      style:--marquee-shift={`${overflow}px`}
      style:--marquee-duration={`${marqueeSeconds}s`}
    >
      <RichText {text} />
    </span>
  </span>
{/snippet}

{#if tip}
  <Tip text={tip}>
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
