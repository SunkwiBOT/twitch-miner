<script lang="ts">
  import type { CampaignType } from "../../../lib/bindings/CampaignType";
  import type { TimedDropType } from "../../../lib/bindings/TimedDropType";
  import {
    campaignEnd,
    dropTotals,
    periodsText,
    subsMissing,
  } from "../../../lib/catalogue/campaign.utils";
  import { BoxArtSize } from "../../../lib/format.constants";
  import { boxArt } from "../../../lib/format.utils";
  import { t } from "../../../lib/i18n/i18n.svelte";
  import { ticker } from "../../../lib/state/ticker.svelte";
  import { escapeEmphasis } from "../../../lib/text.utils";
  import Badge from "../../ui/Badge.svelte";
  import EmptyState from "../../ui/EmptyState.svelte";
  import List from "../../ui/List.svelte";
  import ListRow from "../../ui/ListRow.svelte";
  import Panel from "../../ui/Panel.svelte";
  import ProgressLine from "../../ui/ProgressLine.svelte";
  import RewardThumb from "../../ui/RewardThumb.svelte";
  import Thumb from "../../ui/Thumb.svelte";
  import { BadgeTone } from "../../../lib/tones.constants";
  import { IconName } from "../../../lib/icons.constants";
  import { ThumbShape, ThumbSize } from "../../ui/picture.constants";

  type InProgressGroupType = { campaign: CampaignType; drops: TimedDropType[] };

  type InventoryInProgressTabPropsType = {
    inProgress: InProgressGroupType[];
    onopen: (campaign: CampaignType) => void;
  };

  let { inProgress, onopen }: InventoryInProgressTabPropsType = $props();
</script>

<Panel title={t.inventory_in_progress()} count={inProgress.length}>
  {#if inProgress.length === 0}
    <EmptyState text={t.inventory_empty_progress()} />
  {:else}
    <List>
      {#each inProgress as group (group.campaign.id)}
        <ListRow
          title={group.campaign.game.name}
          details={[
            escapeEmphasis(group.campaign.name),
            campaignEnd(group.campaign.ends_at, ticker.now),
          ]}
          onopen={() => onopen(group.campaign)}
          muted={false}
        >
          {#snippet lead()}
            <Thumb
              src={boxArt(group.campaign.game.box_art_url, BoxArtSize.ROW)}
              round={false}
              contain={false}
              size={ThumbSize.MD}
              shape={ThumbShape.BOX_ART}
              fallback={IconName.GIFT}
            />
          {/snippet}
          {#snippet below()}
            <span class="flex flex-col gap-2">
              {#each group.drops as drop (drop.id)}
                <span class="flex items-center gap-2.5">
                  <RewardThumb item={{ campaign: group.campaign, drop }} size={ThumbSize.SM} />
                  <span class="flex min-w-0 flex-1 flex-col gap-1">
                    <span class="flex min-w-0 items-center gap-2">
                      <span class="truncate text-xs font-semibold">{drop.name}</span>
                      {#if drop.repeat}<span class="shrink-0 text-hint text-muted-foreground"
                          >{periodsText(drop.periods_done, drop.repeat)}</span
                        >{/if}
                      {#if subsMissing(drop)}<Badge
                          text={t.reward_sub_only()}
                          tone={BadgeTone.WARNING}
                          dot={false}
                          hint={t.badge_hint_sub_only()}
                        />{/if}
                    </span>
                    {#if !subsMissing(drop)}
                      {@const totals = dropTotals(drop)}
                      <ProgressLine
                        current={totals.current}
                        required={totals.required}
                        stacked={false}
                      />
                    {/if}
                  </span>
                </span>
              {/each}
            </span>
          {/snippet}
        </ListRow>
      {/each}
    </List>
  {/if}
</Panel>
