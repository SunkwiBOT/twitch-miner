import {
  SPARKLE_AREA,
  SPARKLE_HOVER_DELAY_MS,
  SPARKLE_LABEL_BOX,
  SPARKLE_POSITION_DRAWS,
  SPARKLE_REST_DELAY_MS,
  SPARKLE_SIZE_PX,
  SPARKLE_SIZE_STEPS_PER_PX,
  SPARKLE_TONES,
  SparkleTone,
  type SparkleRangeType,
} from "./sparkles.constants";

export type RandomSourceType = () => number;

export type SparklePositionType = { x: number; y: number };

export type SparkleType = SparklePositionType & {
  id: number;
  size: number;
  tone: SparkleTone;
  burst: boolean;
};

const between = (range: SparkleRangeType, random: RandomSourceType): number =>
  range.min + random() * (range.max - range.min);

export const isInLabelBox = ({ x, y }: SparklePositionType): boolean =>
  x > SPARKLE_LABEL_BOX.x.min &&
  x < SPARKLE_LABEL_BOX.x.max &&
  y > SPARKLE_LABEL_BOX.y.min &&
  y < SPARKLE_LABEL_BOX.y.max;

// ! Drawn again while it falls on the label; a source that keeps landing there ends on the top-left corner.
export const pickSparklePosition = (random: RandomSourceType): SparklePositionType => {
  for (let draw = 0; draw < SPARKLE_POSITION_DRAWS; draw += 1) {
    const position = { x: between(SPARKLE_AREA.x, random), y: between(SPARKLE_AREA.y, random) };

    if (!isInLabelBox(position)) {
      return position;
    }
  }

  return { x: SPARKLE_AREA.x.min, y: SPARKLE_AREA.y.min };
};

export const pickSparkleDelay = (hovered: boolean, random: RandomSourceType): number =>
  Math.round(between(hovered ? SPARKLE_HOVER_DELAY_MS : SPARKLE_REST_DELAY_MS, random));

export const createSparkle = (
  id: number,
  burst: boolean,
  random: RandomSourceType,
): SparkleType => ({
  id,
  ...pickSparklePosition(random),
  size:
    Math.round(between(SPARKLE_SIZE_PX, random) * SPARKLE_SIZE_STEPS_PER_PX) /
    SPARKLE_SIZE_STEPS_PER_PX,
  tone: SPARKLE_TONES[Math.floor(random() * SPARKLE_TONES.length)] ?? SparkleTone.SPARKLE,
  burst,
});
