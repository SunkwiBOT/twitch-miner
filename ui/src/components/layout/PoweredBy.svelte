<script lang="ts">
  import { onMount } from "svelte";

  import { cn } from "$lib/utils";
  import ExternalAnchor from "../ui/ExternalAnchor.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { PARTNERS, REPO_URL, ROTATION_FADE_MS, ROTATION_MS } from "../../lib/partners.constants";
  import Icon from "../ui/Icon.svelte";
  import LinkText from "../ui/LinkText.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { LinkMark, LinkSize } from "../ui/link.constants";

  type EntryType = { url: string; label: string; name: string; mark?: string };

  let index = $state(0);
  let fading = $state(false);
  let hovered = false;
  let focused = false;

  const entries = $derived<EntryType[]>([
    ...PARTNERS.map((partner) => ({
      url: partner.url,
      label: t.nav_powered_by(),
      name: partner.name,
      mark: partner.mark,
    })),
    { url: REPO_URL, label: t.nav_support_project(), name: t.nav_star_github() },
  ]);

  const entry = $derived(entries[index % entries.length]);

  onMount(() => {
    let fade: ReturnType<typeof setTimeout> | null = null;

    const timer = setInterval(() => {
      if (hovered || focused) {
        return;
      }

      fading = true;
      fade = setTimeout(() => {
        index = (index + 1) % entries.length;
        fading = false;
      }, ROTATION_FADE_MS);
    }, ROTATION_MS);

    return () => {
      clearInterval(timer);

      if (fade) {
        clearTimeout(fade);
      }
    };
  });
</script>

<ExternalAnchor
  url={entry.url}
  class={cn(
    "group/link flex items-center gap-2.5 rounded-md p-1.5 transition-opacity duration-300 hover:bg-sidebar-accent",
    fading && "opacity-0",
  )}
  onpointerenter={() => (hovered = true)}
  onpointerleave={() => (hovered = false)}
  onfocus={() => (focused = true)}
  onblur={() => (focused = false)}
>
  {#if entry.mark}
    <img class="size-7 shrink-0 rounded-md" src={entry.mark} alt="" />
  {:else}
    <span class="flex size-7 shrink-0 items-center justify-center text-foreground">
      <Icon name={IconName.GITHUB} size={24} />
    </span>
  {/if}
  <span class="flex min-w-0 flex-col leading-tight">
    <span class="text-tiny font-bold tracking-wider text-muted-foreground uppercase">
      {entry.label}
    </span>
    <span class="flex min-w-0 items-center gap-1.5 text-hint">
      <LinkText text={entry.name} mark={LinkMark.END} size={LinkSize.MD} fit={true} />
    </span>
  </span>
</ExternalAnchor>
