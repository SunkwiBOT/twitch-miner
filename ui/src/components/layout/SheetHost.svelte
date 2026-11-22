<script lang="ts">
  import { badgeList } from "../../lib/badges/badges.svelte";
  import { badgeKey } from "../../lib/badges/badges.utils";
  import type { CampaignType } from "../../lib/bindings/CampaignType";
  import type { GlobalBadgeType } from "../../lib/bindings/GlobalBadgeType";
  import { campaignOf, knownCampaign } from "../../lib/catalogue/catalogue.svelte";
  import { account, app } from "../../lib/state/app.svelte";
  import { openCampaignSheet } from "../../lib/state/sheets";
  import BadgeDialog from "../badges/BadgeDialog.svelte";
  import CampaignDialog from "../catalogue/CampaignDialog.svelte";

  // ! A sheet keeps no frozen copy: a campaign or badge gone from the data closes the dialog on its own.
  const liveCampaign = $derived.by((): CampaignType | null => {
    const sheet = app.campaignSheet;

    if (!sheet) {
      return null;
    }

    const owner = account(sheet.accountId);

    return (
      (owner && campaignOf(owner, sheet.key)) ??
      knownCampaign(app.snapshot?.accounts ?? [], sheet.key) ??
      null
    );
  });

  const liveBadge = $derived.by((): GlobalBadgeType | null => {
    const sheet = app.badgeSheet;

    if (!sheet) {
      return null;
    }

    const match = (badge: GlobalBadgeType): boolean =>
      badgeKey(badge.set_id, badge.version_id) === sheet.key;

    return badgeList.badges.find(match) ?? null;
  });
</script>

<BadgeDialog
  badge={liveBadge}
  badges={badgeList.badges}
  sheet={app.badgeSheet}
  onopencampaign={(campaign) => openCampaignSheet(campaign, app.badgeSheet?.accountId ?? null)}
  onclose={() => (app.badgeSheet = null)}
/>
<CampaignDialog
  campaign={liveCampaign}
  sheet={app.campaignSheet}
  onclose={() => (app.campaignSheet = null)}
/>
