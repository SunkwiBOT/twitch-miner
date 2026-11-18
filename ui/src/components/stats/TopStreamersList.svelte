<script lang="ts">
  import type { StreamerPointsType } from "../../lib/bindings/StreamerPointsType";
  import { knownChannels } from "../../lib/channels/channels.svelte";
  import { number } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { IconName } from "../../lib/icons.constants";
  import { ButtonSize, ButtonVariant } from "../ui/button.constants";
  import ChannelLink from "../ui/ChannelLink.svelte";
  import Icon from "../ui/Icon.svelte";
  import IconButton from "../ui/IconButton.svelte";

  type TopStreamersListPropsType = {
    rows: StreamerPointsType[];
    ariaLabel: string;
    emptyText: string;
    onopencurve: (login: string) => void;
  };

  let { rows, ariaLabel, emptyText, onopencurve }: TopStreamersListPropsType = $props();
</script>

{#if rows.length === 0}
  <p class="text-hint text-muted-foreground">{emptyText}</p>
{:else}
  <ul class="flex flex-col divide-y" aria-label={ariaLabel}>
    {#each rows as row (row.login)}
      {@const channel = knownChannels[row.login]}
      <li class="flex items-center justify-between gap-3 py-2.5">
        <ChannelLink
          login={row.login}
          name={channel?.display_name ?? row.login}
          avatar={channel?.avatar_url}
        />
        <span class="flex shrink-0 items-center gap-1.5">
          <span class="inline-flex items-center gap-1 font-semibold tabular-nums">
            <span class="text-link"><Icon name={IconName.CHANNEL_POINTS} size={14} /></span>{number(
              row.points,
            )}
          </span>
          <IconButton
            icon={IconName.CHART}
            label={t.stats_curve_open({ streamer: channel?.display_name ?? row.login })}
            tooltip={t.stats_curve_open_tip()}
            busy={false}
            disabled={false}
            variant={ButtonVariant.GHOST}
            size={ButtonSize.SM}
            onclick={() => onopencurve(row.login)}
          />
        </span>
      </li>
    {/each}
  </ul>
{/if}
