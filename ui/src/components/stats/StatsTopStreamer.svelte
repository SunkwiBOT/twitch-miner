<script lang="ts">
  import { knownChannels } from "../../lib/channels/channels.svelte";
  import { t } from "../../lib/i18n/i18n.svelte";
  import ChannelLink from "../ui/ChannelLink.svelte";

  type StatsTopStreamerPropsType = { login: string | undefined };

  let { login }: StatsTopStreamerPropsType = $props();

  const channel = $derived(login ? knownChannels[login] : undefined);
</script>

{#if login}
  <ChannelLink {login} name={channel?.display_name ?? login} avatar={channel?.avatar_url} />
{:else}
  <span class="text-muted-foreground">{t.stats_tile_top_empty()}</span>
{/if}
