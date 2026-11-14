<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DROPS_API_NAME, DROPS_WEB_URL } from "../../lib/partners.constants";
  import CatalogRefreshButton from "../catalogue/CatalogRefreshButton.svelte";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import Panel from "../ui/Panel.svelte";
  import { ButtonSize, ButtonVariant } from "../ui/button.constants";
</script>

<Panel title={t.settings_catalog()} description={t.settings_catalog_text()}>
  <div class="flex flex-wrap gap-2">
    <CatalogRefreshButton label={t.settings_catalog_refresh()} />
    <ExternalButton
      url={DROPS_WEB_URL}
      label={DROPS_API_NAME}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
    />
  </div>
</Panel>
