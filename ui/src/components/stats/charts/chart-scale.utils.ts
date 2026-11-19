import { LABEL_CHARACTER_PX } from "../../../lib/text-metrics.utils";

// ! The step is never below 1: points are integers, and a smaller step would
// round two neighbouring ticks to the same value.
const niceStep = (rawStep: number): number => {
  const step = Math.max(1, rawStep);
  const magnitude = 10 ** Math.floor(Math.log10(step));
  const residual = step / magnitude;
  const niceResidual = residual <= 1 ? 1 : residual <= 2 ? 2 : residual <= 5 ? 5 : 10;

  return Math.max(1, niceResidual * magnitude);
};

export const niceTicks = (maxValue: number, tickCount = 4): number[] => {
  if (maxValue <= 0) {
    return [0];
  }

  const step = niceStep(maxValue / tickCount);
  const top = Math.ceil(maxValue / step) * step;
  const ticks: number[] = [];

  for (let value = 0; value <= top + step / 2; value += step) {
    ticks.push(value);
  }

  return ticks;
};

// ! The charts draw at their real pixel width: a phone has no room for every axis label.
const AXIS_LABEL_GAP_PX = 8;

const neededAxisLabelWidth = (labelLength: number): number =>
  labelLength * LABEL_CHARACTER_PX + AXIS_LABEL_GAP_PX;

export const axisLabelStep = (labelLength: number, spacing: number): number =>
  spacing > 0 ? Math.max(1, Math.ceil(neededAxisLabelWidth(labelLength) / spacing)) : 1;

export const axisLabelCount = (
  labelLength: number,
  plotWidth: number,
  maxCount: number,
): number => {
  const needed = neededAxisLabelWidth(labelLength);
  const fit = needed > 0 ? Math.floor(plotWidth / needed) + 1 : maxCount;

  return Math.max(2, Math.min(maxCount, fit));
};

// ! The gap between the widest Y-axis label's right edge and the axis line,
// plus a small margin so that label's own leftmost glyph never touches the SVG edge.
export const Y_AXIS_LABEL_GAP_PX = 6;
const Y_AXIS_EDGE_MARGIN_PX = 2;

export const yAxisLeftPad = (labels: string[]): number =>
  Math.max(0, ...labels.map((label) => label.length)) * LABEL_CHARACTER_PX +
  Y_AXIS_LABEL_GAP_PX +
  Y_AXIS_EDGE_MARGIN_PX;

const CURVE_PADDING_RATIO = 0.1;

export const curveYTicks = (values: number[], tickCount = 4): number[] => {
  if (values.length === 0) {
    return [0, 1];
  }

  const rawMin = Math.min(...values);
  const rawMax = Math.max(...values);

  if (rawMin === rawMax) {
    return niceTicks(Math.max(rawMax, 1), tickCount);
  }

  const padding = (rawMax - rawMin) * CURVE_PADDING_RATIO;
  const step = niceStep((rawMax - rawMin + padding * 2) / tickCount);
  const flooredMin = Math.floor((rawMin - padding) / step) * step;

  // ! Zero stays the baseline when the padded minimum sits within one step of it,
  // so a near-zero balance never floats above an empty gap.
  const niceMin = flooredMin <= step ? 0 : flooredMin;
  const niceMax = Math.ceil((rawMax + padding) / step) * step;

  const ticks: number[] = [];

  for (let value = niceMin; value <= niceMax + step / 2; value += step) {
    ticks.push(value);
  }

  return ticks;
};

// ! The first and last shown label would overflow the SVG if centred on their
// tick like every other one: anchoring them from their own inner edge keeps
// every label inside the plot without touching its neighbour.
export const xTickAnchor = (isFirst: boolean, isLast: boolean): "start" | "middle" | "end" =>
  isFirst ? "start" : isLast ? "end" : "middle";
