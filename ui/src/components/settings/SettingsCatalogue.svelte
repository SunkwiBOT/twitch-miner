<script lang="ts">
  import { t } from "../../lib/i18n/i18n.svelte";
  import { DROPS_API_NAME, DROPS_WEB_HOST, DROPS_WEB_URL } from "../../lib/partners.constants";
  import { app } from "../../lib/state/app.svelte";
  import { strong } from "../../lib/text.utils";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import Panel from "../ui/Panel.svelte";
  import { ButtonSize, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";

  const minutes = $derived(Math.round((app.snapshot?.catalog.interval_seconds ?? 0) / 60));
</script>

<Panel
  title={t.settings_catalog()}
  description={t.settings_catalog_text({
    domain: strong(DROPS_WEB_HOST),
    minutes: strong(minutes),
  })}
>
  <div class="flex flex-wrap gap-2">
    <ExternalButton
      url={DROPS_WEB_URL}
      label={DROPS_API_NAME}
      variant={ButtonVariant.OUTLINE}
      size={ButtonSize.MD}
      icon={IconName.GLOBE}
    />
  </div>
</Panel>
