import type { SparkleTone } from "./sparkles.constants";

export type SparkleRangeType = { min: number; max: number };

export type RandomSourceType = () => number;

export type SparklePositionType = { x: number; y: number };

export type SparkleType = SparklePositionType & {
  id: number;
  size: number;
  tone: SparkleTone;
  burst: boolean;
};
