export type LinkTargetType = { url: string } | { open: () => void };

export type TextLinkType = LinkTargetType & { label: string };

const LINK_DELIMITER = "\uE000";

export const linkMarker = (index: number): string => `${LINK_DELIMITER}${index}${LINK_DELIMITER}`;

export type TextPersonType = {
  name: string;
  avatar: string | undefined;
  channel: string | null;
};

const PERSON_DELIMITER = "\uE001";

export const personMarker = (index: number): string =>
  `${PERSON_DELIMITER}${index}${PERSON_DELIMITER}`;

const CODE_DELIMITER = "\uE002";

export type TextCategoryType = {
  name: string;
  art: string | undefined;
  url: string | null;
};

const CATEGORY_DELIMITER = "\uE003";

export const categoryMarker = (index: number): string =>
  `${CATEGORY_DELIMITER}${index}${CATEGORY_DELIMITER}`;

const EMPHASIS = "**";

const RICH_TEXT_TOKENS =
  /(\*\*|\uE000\d+\uE000|\uE001\d+\uE001|\uE002[^\uE002]*\uE002|\uE003\d+\uE003)/;

const DELIMITERS = /[\uE000-\uE003]/g;

const SENTENCE_TAIL = /^[\p{P}\s]*$/u;

export const escapeEmphasis = (value: string): string =>
  value.replace(DELIMITERS, "").replaceAll("*", "∗");

export const strong = (value: string | number): string =>
  `${EMPHASIS}${escapeEmphasis(String(value))}${EMPHASIS}`;

export const code = (value: string): string =>
  `${CODE_DELIMITER}${value.replace(DELIMITERS, "")}${CODE_DELIMITER}`;

// ! For attributes (`aria-label`, `title`) that cannot show markup; links, people and categories never reach them.
export const plainText = (text: string): string =>
  text.replaceAll(EMPHASIS, "").replace(DELIMITERS, "");

export enum RichTextPartKind {
  TEXT = "TEXT",
  STRONG = "STRONG",
  CODE = "CODE",
  LINK = "LINK",
  PERSON = "PERSON",
  CATEGORY = "CATEGORY",
}

export type RichTextPartType =
  | { kind: RichTextPartKind.TEXT | RichTextPartKind.STRONG | RichTextPartKind.CODE; text: string }
  | { kind: RichTextPartKind.LINK; link: TextLinkType; endsSentence: boolean }
  | { kind: RichTextPartKind.PERSON; person: TextPersonType; endsSentence: boolean }
  | { kind: RichTextPartKind.CATEGORY; category: TextCategoryType; endsSentence: boolean };

export const parseRichText = (
  text: string,
  links: TextLinkType[],
  people: TextPersonType[],
  categories: TextCategoryType[],
): RichTextPartType[] => {
  const parts: RichTextPartType[] = [];
  let bold = false;

  for (const token of text.split(RICH_TEXT_TOKENS)) {
    if (token === EMPHASIS) {
      bold = !bold;
    } else if (token.startsWith(LINK_DELIMITER)) {
      const link = links.at(Number(token.slice(1, -1)));

      if (link) {
        parts.push({ kind: RichTextPartKind.LINK, link, endsSentence: false });
      }
    } else if (token.startsWith(PERSON_DELIMITER)) {
      const person = people.at(Number(token.slice(1, -1)));

      if (person) {
        parts.push({ kind: RichTextPartKind.PERSON, person, endsSentence: false });
      }
    } else if (token.startsWith(CATEGORY_DELIMITER)) {
      const category = categories.at(Number(token.slice(1, -1)));

      if (category) {
        parts.push({ kind: RichTextPartKind.CATEGORY, category, endsSentence: false });
      }
    } else if (token.startsWith(CODE_DELIMITER)) {
      parts.push({ kind: RichTextPartKind.CODE, text: token.slice(1, -1) });
    } else if (token) {
      parts.push({ kind: bold ? RichTextPartKind.STRONG : RichTextPartKind.TEXT, text: token });
    }
  }

  // ! A link ending its sentence can show its external icon without leaving a gap mid-sentence.
  for (let index = parts.length - 1; index >= 0; index -= 1) {
    const part = parts[index];

    if (part.kind === RichTextPartKind.LINK) {
      parts[index] = { ...part, endsSentence: true };
      break;
    }

    if (part.kind === RichTextPartKind.PERSON) {
      parts[index] = { ...part, endsSentence: Boolean(part.person.channel) };
      break;
    }

    if (part.kind === RichTextPartKind.CATEGORY) {
      parts[index] = { ...part, endsSentence: Boolean(part.category.url) };
      break;
    }

    if (part.kind !== RichTextPartKind.TEXT && part.kind !== RichTextPartKind.STRONG) {
      break;
    }

    if (!SENTENCE_TAIL.test(part.text)) {
      break;
    }
  }

  return parts;
};
