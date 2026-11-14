// ! Lowering ignores the language (Turkish "INSIDE"); accents go after NFD and NFC rejoins, so Korean and Japanese stay whole.
export const normalizeSearch = (text: string): string =>
  text
    .trim()
    .normalize("NFD")
    .replace(/[̀-ͯ]/g, "")
    .normalize("NFC")
    .toLowerCase()
    .replace(/ı/g, "i");

export const matchesSearch = (text: string, needle: string): boolean =>
  normalizeSearch(text).includes(needle);
