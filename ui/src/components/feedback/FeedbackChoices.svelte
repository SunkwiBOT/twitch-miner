<script lang="ts">
  import { cn } from "$lib/utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DISCORD_NAME, DISCORD_URL, GITHUB_NAME, ISSUES_URL } from "../../lib/partners.constants";
  import Icon from "../ui/Icon.svelte";
  import ExternalAnchor from "../ui/ExternalAnchor.svelte";
  import LinkText from "../ui/LinkText.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { LinkMark, LinkSize } from "../ui/link.constants";

  type FeedbackChoicesPropsType = { onchosen?: () => void };

  type ChoiceType = { url: string; icon: IconName; name: string; text: string; tone: string };

  let { onchosen }: FeedbackChoicesPropsType = $props();

  const choices = $derived<ChoiceType[]>([
    {
      url: DISCORD_URL,
      icon: IconName.DISCORD,
      name: DISCORD_NAME,
      text: t.feedback_discord_text(),
      tone: "text-brand-discord",
    },
    {
      url: ISSUES_URL,
      icon: IconName.GITHUB,
      name: GITHUB_NAME,
      text: t.feedback_github_text(),
      tone: "text-foreground",
    },
  ]);
</script>

<div class="grid gap-2 @xl:grid-cols-2">
  {#each choices as choice (choice.url)}
    <ExternalAnchor
      url={choice.url}
      class="group/link grid grid-cols-[auto_minmax(0,1fr)] items-center gap-3 rounded-lg border bg-card p-3 no-underline transition-colors hover:border-primary/50 hover:bg-muted/40"
      onfollow={onchosen}
    >
      <span class={cn("flex size-8 shrink-0 items-center justify-center", choice.tone)}>
        <Icon name={choice.icon} size={26} />
      </span>
      <span class="flex min-w-0 flex-col gap-0.5">
        <span class="flex min-w-0 items-center gap-1.5">
          <LinkText text={choice.name} mark={LinkMark.END} size={LinkSize.MD} fit={true} />
        </span>
        <span class="text-hint leading-snug text-muted-foreground">{choice.text}</span>
      </span>
    </ExternalAnchor>
  {/each}
</div>
