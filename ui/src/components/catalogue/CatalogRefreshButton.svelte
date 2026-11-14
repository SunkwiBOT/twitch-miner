<script lang="ts">
  import { mmss } from "../../lib/format.utils";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { app } from "../../lib/state/app.svelte";
  import { refreshCatalog } from "../../lib/state/session";
  import { secondClock } from "../../lib/time/clock.svelte";
  import ActionButton from "../ui/ActionButton.svelte";
  import { ButtonSize, ButtonType, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";

  type CatalogRefreshButtonPropsType = { label: string };

  let { label }: CatalogRefreshButtonPropsType = $props();

  let busy = $state(false);

  const next = $derived(new Date(app.snapshot?.catalog.next_refresh_at ?? 0).getTime());
  const clock = secondClock(() => next);
  const wait = $derived(Math.max(0, next - clock.now));

  const refresh = async (): Promise<void> => {
    busy = true;
    await refreshCatalog();
    clock.now = Date.now();
    busy = false;
  };
</script>

<ActionButton
  icon={IconName.REFRESH}
  label={wait > 0 ? t.catalogue_refresh_wait({ time: mmss(wait) }) : label}
  busyLabel={t.common_refreshing()}
  {busy}
  disabled={wait > 0}
  variant={ButtonVariant.DEFAULT}
  size={ButtonSize.MD}
  type={ButtonType.BUTTON}
  onclick={refresh}
/>
