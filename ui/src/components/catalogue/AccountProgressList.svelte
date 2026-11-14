<script lang="ts" generics="T extends AccountProgressType">
  import type { Snippet } from "svelte";

  import {
    NO_BAR_STATES,
    ProgressState,
    type AccountProgressType,
  } from "../../lib/catalogue/progress.constants";
  import { t } from "../../lib/i18n/i18n.svelte";
  import Avatar from "../ui/Avatar.svelte";
  import Badge from "../ui/Badge.svelte";
  import ProgressLine from "../ui/ProgressLine.svelte";
  import { AvatarSize } from "../ui/picture.constants";
  import { PROGRESS_BADGES } from "./progress.utils";

  type AccountProgressListPropsType = {
    entries: T[];
    tags?: Snippet<[T]>;
    action?: Snippet<[T]>;
  };

  let { entries, tags, action }: AccountProgressListPropsType = $props();
</script>

<section class="flex flex-col gap-2">
  <h3 class="text-xs font-semibold text-muted-foreground">{t.badge_dialog_accounts()}</h3>
  <ul class="flex flex-col gap-2">
    {#each entries as entry (entry.account.id)}
      {@const badge = PROGRESS_BADGES[entry.state]}
      <li class="flex flex-wrap items-center gap-3 rounded-lg border p-2.5">
        <Avatar
          url={entry.account.avatar_url}
          name={entry.account.display_name}
          size={AvatarSize.SM}
        />
        <span class="flex min-w-0 flex-1 basis-48 flex-col gap-1.5">
          <span class="flex min-w-0 flex-wrap items-center gap-x-2 gap-y-1">
            <span class="truncate text-sm font-semibold">{entry.account.display_name}</span>
            <Badge
              text={badge.text()}
              tone={badge.tone}
              dot={entry.state === ProgressState.MINING}
              icon={badge.icon}
              hint={badge.hint?.()}
            />
            {@render tags?.(entry)}
          </span>
          {#if entry.required > 0 && !NO_BAR_STATES.has(entry.state)}
            <ProgressLine current={entry.current} required={entry.required} stacked={false} />
          {/if}
        </span>
        {@render action?.(entry)}
      </li>
    {/each}
  </ul>
</section>
