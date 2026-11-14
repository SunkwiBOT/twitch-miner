import { t } from "../../lib/i18n/i18n.svelte";
import { IconName } from "../../lib/icons.constants";
import { TIMELINE_ZOOMS, TimelineView, TimelineZoom } from "../../lib/catalogue/timeline.constants";

const ZOOM_LABELS: Record<TimelineZoom, () => string> = {
  [TimelineZoom.DAY]: () => t.timeline_zoom_day(),
  [TimelineZoom.THREE_DAYS]: () => t.timeline_zoom_three_days(),
  [TimelineZoom.TWO_WEEKS]: () => t.timeline_zoom_two_weeks(),
};

export const zoomOptions = (): { value: TimelineZoom; label: string }[] =>
  TIMELINE_ZOOMS.map((zoom) => ({ value: zoom, label: ZOOM_LABELS[zoom]() }));

export const viewOptions = (): { value: TimelineView; label: string; icon: IconName }[] => [
  { value: TimelineView.TIMELINE, label: t.catalogue_view_timeline(), icon: IconName.CALENDAR },
  { value: TimelineView.CARDS, label: t.catalogue_view_cards(), icon: IconName.GRID },
];
