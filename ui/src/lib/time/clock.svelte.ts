import { SECOND_MS } from "./time.constants";

export const secondClock = (
  until: () => number = () => Number.POSITIVE_INFINITY,
): { now: number } => {
  const clock = $state({ now: Date.now() });

  $effect(() => {
    const end = until();
    const current = Date.now();

    clock.now = current;

    if (end <= current) {
      return;
    }

    const timer = setInterval(() => {
      const tick = Date.now();

      clock.now = tick;

      if (tick >= end) {
        clearInterval(timer);
      }
    }, SECOND_MS);

    return () => clearInterval(timer);
  });

  return clock;
};
