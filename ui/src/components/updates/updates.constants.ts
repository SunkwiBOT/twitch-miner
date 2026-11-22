import { buttonVariants } from "$lib/components/ui/button";
import { cn } from "$lib/utils";

// ! The shared status pill, in a button as tall as the top bar's other buttons for the click.
export const UPDATE_TRIGGER_CLASSES = cn(
  buttonVariants.variants.size.default,
  "group inline-flex shrink-0 items-center rounded-md px-0 outline-hidden focus-visible:outline-2 focus-visible:outline-ring focus-visible:outline-solid",
);
