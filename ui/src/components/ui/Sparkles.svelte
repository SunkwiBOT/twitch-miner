<script lang="ts">
  // ! Four-point stars around the host (this layer's parent, `relative`): one now and then, more under the pointer, a burst on a press. Paused off screen and in a hidden tab; decorative.
  import { cn } from "$lib/utils";
  import {
    MAX_SPARKLES,
    SPARKLE_BURST_COUNT,
    SPARKLE_BURST_INSTANT_COUNT,
    SPARKLE_BURST_STEP_MS,
    SPARKLE_LIFETIME_MS,
    SPARKLE_PATH,
    SPARKLE_PRIMARY_BUTTON,
    SPARKLE_REMOVAL_MARGIN_MS,
    SPARKLE_TONE_CLASSES,
    SPARKLE_VIEW_BOX,
  } from "./sparkles.constants";
  import { createSparkle, pickSparkleDelay, type SparkleType } from "./sparkles.utils";

  let layer = $state<HTMLSpanElement | null>(null);
  let sparkles = $state<SparkleType[]>([]);

  $effect(() => {
    const host = layer?.parentElement;

    if (!host) {
      return;
    }

    const timers = new Set<number>();
    let nextId = 0;
    let hovered = false;
    let visible = false;
    let tickTimer: number | null = null;

    const later = (callback: () => void, delay: number): number => {
      const timer = window.setTimeout(() => {
        timers.delete(timer);
        callback();
      }, delay);

      timers.add(timer);

      return timer;
    };

    const spawn = (burst: boolean): void => {
      const sparkle = createSparkle(nextId, burst, Math.random);

      nextId += 1;
      sparkles = [...sparkles, sparkle].slice(-MAX_SPARKLES);
      later(() => {
        sparkles = sparkles.filter((item) => item.id !== sparkle.id);
      }, SPARKLE_LIFETIME_MS + SPARKLE_REMOVAL_MARGIN_MS);
    };

    const canPlay = (): boolean => visible && !document.hidden;
    const disabled = (): boolean => host.matches(':disabled, [aria-disabled="true"]');

    const stop = (): void => {
      if (tickTimer !== null) {
        window.clearTimeout(tickTimer);
        timers.delete(tickTimer);
        tickTimer = null;
      }
    };

    const schedule = (): void => {
      if (tickTimer !== null || !canPlay()) {
        return;
      }

      tickTimer = later(
        () => {
          tickTimer = null;

          if (!canPlay()) {
            return;
          }

          if (!disabled()) {
            spawn(false);
          }

          schedule();
        },
        pickSparkleDelay(hovered, Math.random),
      );
    };

    const burst = (): void => {
      if (!canPlay() || disabled()) {
        return;
      }

      for (let index = 0; index < SPARKLE_BURST_COUNT; index += 1) {
        if (index < SPARKLE_BURST_INSTANT_COUNT) {
          spawn(true);
        } else {
          later(
            () => spawn(true),
            (index - SPARKLE_BURST_INSTANT_COUNT + 1) * SPARKLE_BURST_STEP_MS,
          );
        }
      }
    };

    const onPointerEnter = (): void => {
      hovered = true;
      // ! The quicker pace starts now, not after the slow wait.
      stop();
      schedule();
    };
    const onPointerLeave = (): void => {
      hovered = false;
    };
    // ! The burst starts on the press, not the release; a keyboard click (`detail` 0) still throws it.
    const onPointerDown = (event: PointerEvent): void => {
      if (event.button === SPARKLE_PRIMARY_BUTTON) {
        burst();
      }
    };
    const onClick = (event: MouseEvent): void => {
      if (event.detail === 0) {
        burst();
      }
    };
    const onVisibilityChange = (): void => {
      if (document.hidden) {
        stop();
      } else {
        schedule();
      }
    };

    const observer = new IntersectionObserver(([entry]) => {
      visible = entry?.isIntersecting ?? false;

      if (visible) {
        schedule();
      } else {
        stop();
      }
    });

    observer.observe(host);
    host.addEventListener("pointerenter", onPointerEnter);
    host.addEventListener("pointerleave", onPointerLeave);
    host.addEventListener("pointerdown", onPointerDown);
    host.addEventListener("click", onClick);
    document.addEventListener("visibilitychange", onVisibilityChange);

    return () => {
      observer.disconnect();
      host.removeEventListener("pointerenter", onPointerEnter);
      host.removeEventListener("pointerleave", onPointerLeave);
      host.removeEventListener("pointerdown", onPointerDown);
      host.removeEventListener("click", onClick);
      document.removeEventListener("visibilitychange", onVisibilityChange);

      for (const timer of timers) {
        window.clearTimeout(timer);
      }
    };
  });
</script>

<span
  bind:this={layer}
  class="pointer-events-none absolute inset-0"
  style:--sparkle-lifetime="{SPARKLE_LIFETIME_MS}ms"
  data-slot="sparkles"
  aria-hidden="true"
>
  {#each sparkles as sparkle (sparkle.id)}
    <span
      class={cn(
        "absolute -translate-1/2",
        sparkle.burst ? "animate-sparkle-pop" : "animate-sparkle-life",
        SPARKLE_TONE_CLASSES[sparkle.tone],
      )}
      style:left="{sparkle.x}%"
      style:top="{sparkle.y}%"
      style:width="{sparkle.size}px"
      style:height="{sparkle.size}px"
      data-slot="sparkle"
    >
      <svg class="block size-full animate-sparkle-spin fill-current" viewBox={SPARKLE_VIEW_BOX}>
        <path d={SPARKLE_PATH} />
      </svg>
    </span>
  {/each}
</span>
