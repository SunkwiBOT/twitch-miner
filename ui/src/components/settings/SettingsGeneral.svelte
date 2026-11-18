<script lang="ts">
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import { Theme } from "../../lib/bindings/Theme";
  import { languageOptions, t } from "../../lib/i18n/i18n.svelte";
  import FormField from "../ui/FormField.svelte";
  import Panel from "../ui/Panel.svelte";
  import Select from "../ui/Select.svelte";
  import { MenuAlign } from "../ui/menu.constants";
  import { RETENTION_LABELS, RETENTION_OPTIONS } from "./settings.constants";

  type SettingsGeneralPropsType = {
    draft: GlobalSettingsType;
  };

  let { draft = $bindable() }: SettingsGeneralPropsType = $props();

  const languages = $derived(languageOptions());

  const themes = $derived([
    { value: Theme.DARK, label: t.theme_dark() },
    { value: Theme.LIGHT, label: t.theme_light() },
  ]);

  const retentions = $derived(
    RETENTION_OPTIONS.map((value) => ({ value, label: RETENTION_LABELS[value]() })),
  );
</script>

<Panel title={t.settings_general()}>
  <div class="grid gap-3">
    <FormField id="set-lang" label={t.settings_language()}>
      <Select
        id="set-lang"
        value={draft.language}
        options={languages}
        label={t.settings_language()}
        align={MenuAlign.LEFT}
        onchange={(language) => (draft.language = language)}
      />
    </FormField>
    <FormField id="set-theme" label={t.settings_theme()}>
      <Select
        id="set-theme"
        value={draft.theme}
        options={themes}
        label={t.settings_theme()}
        align={MenuAlign.LEFT}
        onchange={(theme) => (draft.theme = theme)}
      />
    </FormField>
    <FormField id="set-retention" label={t.settings_history_retention()}>
      <Select
        id="set-retention"
        value={draft.history_retention}
        options={retentions}
        label={t.settings_history_retention()}
        align={MenuAlign.LEFT}
        onchange={(value) => (draft.history_retention = value)}
      />
    </FormField>
  </div>
</Panel>
