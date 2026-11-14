export const boundedInteger = (value: number, min: number, max: number): number =>
  Number.isNaN(value) ? min : Math.min(max, Math.max(min, Math.round(value)));

// ! Bounded back to the current value the setting does not change, so the typed number is written back by hand.
export const boundNumberInput = (input: HTMLInputElement, min: number, max: number): number => {
  const value = boundedInteger(Number(input.value), min, max);

  input.value = String(value);

  return value;
};
