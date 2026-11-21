import type { CampaignType } from "../../lib/bindings/CampaignType";
import { t } from "../../lib/i18n/i18n.svelte";
import { IconName } from "../../lib/icons.constants";
import { CampaignAction, CampaignSort } from "./catalogue.constants";
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

const SORT_LABELS: Record<CampaignSort, () => string> = {
  [CampaignSort.QUEUE]: () => t.queue_title(),
  [CampaignSort.ENDING]: () => t.catalogue_sort_ending(),
  [CampaignSort.NEW]: () => t.catalogue_sort_new(),
  [CampaignSort.GAME]: () => t.catalogue_sort_game(),
};

export const sortOptions = (
  sorts: readonly CampaignSort[],
): { value: CampaignSort; label: string }[] =>
  sorts.map((sort) => ({ value: sort, label: SORT_LABELS[sort]() }));

// ! `QUEUE` keeps the order the campaigns come in: the caller's mining order.
export const sortCampaigns = (
  campaigns: readonly CampaignType[],
  sort: CampaignSort,
): CampaignType[] => {
  const list = [...campaigns];

  switch (sort) {
    case CampaignSort.QUEUE:
      return list;
    case CampaignSort.ENDING:
      return list.sort((a, b) => Date.parse(a.ends_at) - Date.parse(b.ends_at));
    case CampaignSort.NEW:
      return list.sort((a, b) => Date.parse(b.starts_at) - Date.parse(a.starts_at));
    case CampaignSort.GAME:
      return list.sort(
        (a, b) => a.game.name.localeCompare(b.game.name) || a.name.localeCompare(b.name),
      );
  }
};

const BUSY_LABELS: Record<CampaignAction, () => string> = {
  [CampaignAction.IGNORE]: t.campaign_ignoring,
  [CampaignAction.RESTORE]: t.ignored_restoring,
  [CampaignAction.SELECT]: t.campaign_selecting,
  [CampaignAction.UNSELECT]: t.campaign_unselecting,
  [CampaignAction.FIRST]: t.campaign_prioritizing,
};

export const busyLabel = (busy: CampaignAction): string => BUSY_LABELS[busy]();
