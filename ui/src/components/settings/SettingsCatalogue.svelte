<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DROPS_API_NAME, DROPS_WEB_URL } from "../../lib/partners.constants";
  import { app } from "../../lib/state/app.svelte";
  import { refreshCatalog } from "../../lib/state/session";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import Panel from "../ui/Panel.svelte";
  import RefreshButton from "../ui/RefreshButton.svelte";
  import { ButtonSize, ButtonVariant } from "../ui/button.constants";
</script>

<Panel title={t.settings_catalog()} description={t.settings_catalog_text()}>
  <div class="flex flex-wrap gap-2">
    <RefreshButton
      label={t.settings_catalog_refresh()}
      nextRefreshAt={app.snapshot?.catalog.next_manual_refresh_at ?? new Date(0).toISOString()}
      onrefresh={refreshCatalog}
    />
    <ExternalButton
      url={DROPS_WEB_URL}
      label={DROPS_API_NAME}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
    />
  </div>
</Panel>
