import type { MinerEventType } from "../../lib/bindings/MinerEventType";
import type { BetResult } from "../../lib/bindings/BetResult";
import { EventKind } from "../../lib/bindings/EventKind";
import {
  ANNOTATION_KINDS,
  STATS_ANNOTATION_STYLES,
  type StatsAnnotationKindType,
  type StatsAnnotationStyleType,
} from "./stats-annotations.constants";

const isSimpleAnnotationKind = (
  kind: EventKind,
): kind is Exclude<StatsAnnotationKindType, `${EventKind.PREDICTION_RESULT}_${BetResult}`> =>
  ANNOTATION_KINDS.has(kind);

export const annotationStyle = (event: MinerEventType): StatsAnnotationStyleType | null => {
  if (event.kind === EventKind.PREDICTION_RESULT) {
    return event.context.bet_result
      ? STATS_ANNOTATION_STYLES[`${EventKind.PREDICTION_RESULT}_${event.context.bet_result}`]
      : null;
  }

  if (!isSimpleAnnotationKind(event.kind)) {
    return null;
  }

  return STATS_ANNOTATION_STYLES[event.kind];
};
