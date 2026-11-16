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

  const uid = $props.id();
  const waitId = `${uid}-wait`;

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

<span class="inline-flex items-center gap-2">
  <ActionButton
    icon={IconName.REFRESH}
    {label}
    busyLabel={t.common_refreshing()}
    {busy}
    disabled={wait > 0}
    describedby={wait > 0 ? waitId : undefined}
    variant={ButtonVariant.DEFAULT}
    size={ButtonSize.MD}
    type={ButtonType.BUTTON}
    onclick={refresh}
  />
  {#if wait > 0}
    <span id={waitId} class="text-hint text-muted-foreground"
      >{t.catalogue_refresh_wait({ time: mmss(wait) })}</span
    >
  {/if}
</span>
