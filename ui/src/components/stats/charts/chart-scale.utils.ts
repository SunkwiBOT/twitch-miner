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
