<script lang="ts">
  import { badgeList } from "../../lib/badges/badges.svelte";
  import { app } from "../../lib/state/app.svelte";
  import BadgeDialog from "../badges/BadgeDialog.svelte";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";
</script>

<BadgeDialog
  badge={app.badgeSheet?.item ?? null}
  badges={badgeList.badges}
  contextAccountId={app.badgeSheet?.accountId ?? null}
  onopencampaign={(campaign) => {
    app.campaignSheet = { item: campaign, accountId: app.badgeSheet?.accountId ?? null };
    app.badgeSheet = null;
  }}
  onclose={() => (app.badgeSheet = null)}
/>
<CampaignDialog
  campaign={app.campaignSheet?.item ?? null}
  contextAccountId={app.campaignSheet?.accountId ?? null}
  onclose={() => (app.campaignSheet = null)}
/>
