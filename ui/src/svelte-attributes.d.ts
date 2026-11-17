// ! Non-standard WebKit/Blink attribute, not declared by Svelte's own
// `HTMLAttributes`: typed here once so it is a normal attribute everywhere,
// instead of a spread that dodges the type checker at each call site.
declare module "svelte/elements" {
  // ! Declaration merging with Svelte's own `HTMLAttributes` requires an
  // `interface`, not a `type` alias.
  // oxlint-disable-next-line typescript/consistent-type-definitions
  interface HTMLAttributes<T> {
    autocorrect?: "on" | "off";
  }
}

export {};
