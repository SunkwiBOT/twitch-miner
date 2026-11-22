<script lang="ts">
  import type { Snippet } from "svelte";

  import { queueReasonView } from "../../lib/accounts/campaign-states.utils";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import { QueueReason } from "../../lib/bindings/QueueReason";
  import { BoxArtSize } from "../../lib/format.constants";
  import { boxArt, number, shortDuration } from "../../lib/format.utils";
  import { strong } from "../../lib/text.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { NEW_MS, URGENT_MS } from "./catalogue.constants";
  import Badge from "../ui/Badge.svelte";
  import { BadgeTone } from "../../lib/tones.constants";
  import { CARD_LIFT_CLASSES } from "../ui/ui.constants";
  import LinkBadge from "./LinkBadge.svelte";
  import TimeLeftTag from "./TimeLeftTag.svelte";

  type CampaignCardPropsType = {
    campaign: CampaignType;
    ordered: boolean;
    cover?: Snippet;
    tags?: Snippet;
    onopen: (campaign: CampaignType) => void;
  };

  let { campaign, ordered, cover, tags, onopen }: CampaignCardPropsType = $props();

  const left = $derived(new Date(campaign.ends_at).getTime() - ticker.now);
  const isNew = $derived(ticker.now - new Date(campaign.starts_at).getTime() < NEW_MS);
  const art = $derived(boxArt(campaign.game.box_art_url, BoxArtSize.CARD));
</script>

<button
  class="group flex min-w-0 flex-col gap-1.5 rounded-lg text-left outline-none"
  type="button"
  onclick={() => onopen(campaign)}
>
  <span class="relative block aspect-[3/4] overflow-hidden rounded-lg bg-muted {CARD_LIFT_CLASSES}">
    {#if art}<img class="block size-full object-cover" src={art} alt="" loading="lazy" />{/if}
    {@render cover?.()}
    {#if left > 0}
      <span class="absolute bottom-1.5 left-1.5">
        <TimeLeftTag text={strong(shortDuration(left))} urgent={left < URGENT_MS} />
      </span>
    {:else}
      <span class="absolute bottom-1.5 left-1.5 rounded-4xl bg-background">
        <Badge text={t.campaign_ended()} tone={BadgeTone.MUTED} />
      </span>
    {/if}
    {#if ordered || isNew}
      <span class="absolute top-1.5 right-1.5 rounded-4xl bg-background">
        {#if ordered}
          <Badge {...queueReasonView(QueueReason.ORDERED)} />
        {:else}
          <Badge text={t.catalogue_new()} tone={BadgeTone.INFO} />
        {/if}
      </span>
    {/if}
  </span>
  <span
    class="line-clamp-2 min-h-[2lh] text-hint leading-tight font-semibold"
    title={campaign.name}
  >
    {campaign.name}
  </span>
  <span class="-mt-1 truncate text-xs text-muted-foreground">{campaign.game.name}</span>
  <span class="flex flex-wrap gap-1">
    <Badge
      text={t.catalogue_rewards({
        rewardCount: campaign.drops.length,
        formattedRewardCount: strong(number(campaign.drops.length)),
      })}
      tone={BadgeTone.NEUTRAL}
    />
    <LinkBadge linked={campaign.linked} />
    {@render tags?.()}
  </span>
</button>
