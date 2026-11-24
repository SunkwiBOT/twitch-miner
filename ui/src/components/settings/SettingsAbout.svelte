<script lang="ts">
  import { tick } from "svelte";

  import { isTauri } from "../../lib/api/tauri";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import type { SnapshotType } from "../../lib/bindings/SnapshotType";
  import { UpdateInterval } from "../../lib/bindings/UpdateInterval";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { REPO_URL } from "../../lib/partners.constants";
  import { updates } from "../../lib/updates/updates.svelte";
  import UpdatesPanel from "../updates/UpdatesPanel.svelte";
  import VersionBadge from "../updates/VersionBadge.svelte";
  import ExternalButton from "../ui/ExternalButton.svelte";
  import FormField from "../ui/FormField.svelte";
  import Panel from "../ui/Panel.svelte";
  import Select from "../ui/Select.svelte";
  import { ButtonSize, ButtonVariant } from "../ui/button.constants";
  import { IconName } from "../../lib/icons.constants";
  import { MenuAlign } from "../ui/menu.constants";

  type SettingsAboutPropsType = { snapshot: SnapshotType; draft: GlobalSettingsType };

  let { snapshot, draft = $bindable() }: SettingsAboutPropsType = $props();

  const intervals = $derived([
    { value: UpdateInterval.AT_START, label: t.updates_interval_start() },
    { value: UpdateInterval.MINUTES30, label: t.updates_interval_minutes({ minutesCount: 30 }) },
    { value: UpdateInterval.HOURS1, label: t.updates_interval_hour() },
    { value: UpdateInterval.HOURS3, label: t.updates_interval_hours({ hoursCount: 3 }) },
    { value: UpdateInterval.HOURS6, label: t.updates_interval_hours({ hoursCount: 6 }) },
    { value: UpdateInterval.HOURS12, label: t.updates_interval_hours({ hoursCount: 12 }) },
    { value: UpdateInterval.HOURS24, label: t.updates_interval_hours({ hoursCount: 24 }) },
  ]);

  let about = $state<HTMLElement>();

  $effect(() => {
    if (!updates.reveal || !about) {
      return;
    }

    const target = about;

    updates.reveal = false;
    void tick().then(() => {
      target.scrollIntoView({ block: "center", behavior: "smooth" });
      target.focus({ preventScroll: true });
    });
  });
</script>

{#snippet repository()}
  <ExternalButton
    url={REPO_URL}
    label={t.settings_about_repo()}
    variant={ButtonVariant.OUTLINE}
    size={ButtonSize.MD}
    icon={IconName.GITHUB}
  />
{/snippet}

<div bind:this={about} tabindex="-1" class="flex min-w-0 flex-col outline-none *:flex-1">
  <Panel title={t.settings_about()} description={t.settings_about_text()}>
    {#snippet actions()}
      <VersionBadge current={snapshot.version} />
    {/snippet}
    <FormField
      id="set-update-interval"
      label={t.updates_interval()}
      hint={isTauri() ? t.updates_interval_hint() : t.updates_interval_hint_server()}
    >
      <Select
        id="set-update-interval"
        value={draft.update_interval}
        options={intervals}
        label={t.updates_interval()}
        align={MenuAlign.LEFT}
        onchange={(value) => (draft.update_interval = value)}
      />
    </FormField>
    <UpdatesPanel checkedAt={snapshot.update_checked_at ?? null} actions={repository} />
  </Panel>
</div>
