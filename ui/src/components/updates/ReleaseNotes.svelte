<script lang="ts">
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { availableUpdate } from "../../lib/updates/updates.svelte";
  import Badge from "../ui/Badge.svelte";
  import Markdown from "../ui/Markdown.svelte";

  type ReleaseNotesPropsType = { class?: string };

  let { class: className }: ReleaseNotesPropsType = $props();

  const update = $derived(availableUpdate());
  const releases = $derived(update?.notes ?? []);
  const titleId = $props.id();
</script>

{#if releases.length > 0}
  <div class={cn("flex min-w-0 flex-col gap-1.5", className)}>
    <h3 id={titleId} class="text-hint font-medium">{t.updates_notes_title()}</h3>
    <!-- ! Focusable so a keyboard can scroll long notes. -->
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div
      class="max-h-64 overflow-y-auto overscroll-contain rounded-lg border bg-muted/30 px-3 py-2"
      tabindex="0"
      role="region"
      aria-labelledby={titleId}
    >
      <ul class="flex flex-col">
        {#each releases as release (release.version)}
          <li
            class="flex flex-col gap-1.5 border-t pt-3 not-first:mt-3 first:border-t-0 first:pt-0"
          >
            <Badge
              text={`v${release.version}`}
              tone={release.version === update?.version ? BadgeTone.SUCCESS : BadgeTone.NEUTRAL}
              class="self-start"
            />
            <Markdown markdown={release.notes} />
          </li>
        {/each}
      </ul>
    </div>
  </div>
{/if}
