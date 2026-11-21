import type { CoreLimitsType } from "../bindings/CoreLimitsType";
import { app } from "./app.svelte";

// ! Every screen and request that needs a core limit only exists once the first snapshot arrived.
export const coreLimits = (): CoreLimitsType => {
  const limits = app.snapshot?.limits;

  if (!limits) {
    throw new Error("Core limits read before the first snapshot");
  }

  return limits;
};
