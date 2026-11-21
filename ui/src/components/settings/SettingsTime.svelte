<script lang="ts">
  import { ClockFormat } from "../../lib/bindings/ClockFormat";
  import { DateFormat } from "../../lib/bindings/DateFormat";
  import type { GlobalSettingsType } from "../../lib/bindings/GlobalSettingsType";
  import { t } from "../../lib/i18n/i18n.svelte";
  import { ticker } from "../../lib/state/ticker.svelte";
  import { strong } from "../../lib/text.utils";
  import { AUTO_TIME_ZONE } from "../../lib/time/time.constants";
  import { deviceTimeZone } from "../../lib/time/time.svelte";
  import {
    formatClockAs,
    formatDateAs,
    formatZoneOffset,
    listTimeZones,
  } from "../../lib/time/time.utils";
  import {
    clockInputValueToMinutes,
    minutesToClockInputValue,
  } from "../../lib/time/quiet-window.utils";
  import { Input } from "$lib/components/ui/input";
  import FormField from "../ui/FormField.svelte";
  import Panel from "../ui/Panel.svelte";
  import Select from "../ui/Select.svelte";
  import { MenuAlign } from "../ui/menu.constants";
  import Setting from "../ui/Setting.svelte";
  import { FIELD_GRID_CLASSES, SAMPLE_DATE } from "./settings.constants";

  type SettingsTimePropsType = {
    draft: GlobalSettingsType;
  };

  let { draft = $bindable() }: SettingsTimePropsType = $props();

  const sampleDate = (format: DateFormat): string => formatDateAs(SAMPLE_DATE, false, format);
  const sampleClock = (format: ClockFormat): string => formatClockAs(SAMPLE_DATE, format);

  const zones = $derived([
    {
      value: AUTO_TIME_ZONE,
      label: t.settings_time_zone_auto({ zone: deviceTimeZone() }),
    },
    ...listTimeZones().map((zone) => ({
      value: zone,
      label: zone.replaceAll("_", " "),
      meta: formatZoneOffset(zone, ticker.now),
    })),
  ]);

  const dateFormats = $derived([
    { value: DateFormat.AUTO, label: t.date_format_auto({ example: sampleDate(DateFormat.AUTO) }) },
    { value: DateFormat.DAY_MONTH_YEAR, label: sampleDate(DateFormat.DAY_MONTH_YEAR) },
    { value: DateFormat.MONTH_DAY_YEAR, label: sampleDate(DateFormat.MONTH_DAY_YEAR) },
    { value: DateFormat.YEAR_MONTH_DAY, label: sampleDate(DateFormat.YEAR_MONTH_DAY) },
  ]);

  const clockFormats = $derived([
    { value: ClockFormat.AUTO, label: t.clock_auto({ example: sampleClock(ClockFormat.AUTO) }) },
    { value: ClockFormat.H24, label: t.clock_24({ example: sampleClock(ClockFormat.H24) }) },
    { value: ClockFormat.H12, label: t.clock_12({ example: sampleClock(ClockFormat.H12) }) },
  ]);

  let startError = $state<string | undefined>();
  let endError = $state<string | undefined>();

  const setQuietWindowMinutes = (which: "start" | "end", raw: string): void => {
    const minutes = clockInputValueToMinutes(raw);
    const other =
      which === "start" ? draft.quiet_window.end_minutes : draft.quiet_window.start_minutes;
    const error =
      minutes === null
        ? t.quiet_window_time_required()
        : minutes === other
          ? t.quiet_window_same_time()
          : undefined;

    if (which === "start") {
      startError = error;
    } else {
      endError = error;
    }

    if (minutes !== null && error === undefined) {
      if (which === "start") {
        draft.quiet_window.start_minutes = minutes;
      } else {
        draft.quiet_window.end_minutes = minutes;
      }
    }
  };
</script>

<Panel title={t.settings_time()}>
  <div class="grid gap-3">
    <FormField id="set-zone" label={t.settings_time_zone()}>
      <Select
        id="set-zone"
        value={draft.time_zone}
        options={zones}
        label={t.settings_time_zone()}
        align={MenuAlign.LEFT}
        onchange={(value) => (draft.time_zone = value)}
      />
    </FormField>
    <FormField id="set-date" label={t.settings_date_format()}>
      <Select
        id="set-date"
        value={draft.date_format}
        options={dateFormats}
        label={t.settings_date_format()}
        align={MenuAlign.LEFT}
        onchange={(value) => (draft.date_format = value)}
      />
    </FormField>
    <FormField id="set-clock" label={t.settings_clock_format()}>
      <Select
        id="set-clock"
        value={draft.clock_format}
        options={clockFormats}
        label={t.settings_clock_format()}
        align={MenuAlign.LEFT}
        onchange={(value) => (draft.clock_format = value)}
      />
    </FormField>
  </div>
  <div class="border-t pt-3">
    <Setting
      title={t.quiet_window_title()}
      description={t.quiet_window_hint()}
      checked={draft.quiet_window.enabled}
      disabled={false}
      onchange={(enabled) => (draft.quiet_window.enabled = enabled)}
    />
    {#if draft.quiet_window.enabled}
      <div class={FIELD_GRID_CLASSES}>
        <FormField id="set-quiet-start" label={t.quiet_window_start()} error={startError}>
          <Input
            id="set-quiet-start"
            type="time"
            value={minutesToClockInputValue(draft.quiet_window.start_minutes)}
            oninput={(event) => setQuietWindowMinutes("start", event.currentTarget.value)}
          />
        </FormField>
        <FormField
          id="set-quiet-end"
          label={t.quiet_window_end()}
          hint={t.quiet_window_zone_hint({
            zone: strong(draft.time_zone === AUTO_TIME_ZONE ? deviceTimeZone() : draft.time_zone),
          })}
          error={endError}
        >
          <Input
            id="set-quiet-end"
            type="time"
            value={minutesToClockInputValue(draft.quiet_window.end_minutes)}
            oninput={(event) => setQuietWindowMinutes("end", event.currentTarget.value)}
          />
        </FormField>
      </div>
    {/if}
  </div>
</Panel>
