import { code, escapeEmphasis, linkMarker, strong, type TextLinkType } from "../text.utils";
import {
  MARKDOWN_BULLET,
  MARKDOWN_HEADING,
  MARKDOWN_HTML,
  MARKDOWN_INLINE,
  MARKDOWN_LINK,
  MARKDOWN_NUMBER,
  MARKDOWN_RULE,
  MARKDOWN_SAFE_URL,
  MarkdownBlockKind,
} from "./markdown.constants";

// ! What `RichText` shows: a link reaches it only as a `TextLinkType`, never through the text.
export type MarkdownTextType = { text: string; links: TextLinkType[] };

type ListKindType = MarkdownBlockKind.BULLETS | MarkdownBlockKind.NUMBERS;

export type MarkdownBlockType =
  | { kind: MarkdownBlockKind.HEADING; text: MarkdownTextType }
  | { kind: MarkdownBlockKind.PARAGRAPH; text: MarkdownTextType }
  | { kind: ListKindType; items: MarkdownTextType[] };

type PendingBlocksType = {
  paragraph: string[];
  list: { kind: ListKindType; items: string[] } | null;
};

const inlineToken = (token: string, links: TextLinkType[]): string => {
  const link = MARKDOWN_LINK.exec(token);

  if (link) {
    const [, label, url] = link;

    if (!MARKDOWN_SAFE_URL.test(url)) {
      return escapeEmphasis(label);
    }

    links.push({ label, url });

    return linkMarker(links.length - 1);
  }

  if (token.length > 4 && (token.startsWith("**") || token.startsWith("__"))) {
    return strong(token.slice(2, -2));
  }

  if (token.length > 2 && token.startsWith("`")) {
    return code(token.slice(1, -1));
  }

  if (token.length > 2 && token.startsWith("*")) {
    return escapeEmphasis(token.slice(1, -1));
  }

  return escapeEmphasis(token);
};

export const parseInlineMarkdown = (markdown: string): MarkdownTextType => {
  const links: TextLinkType[] = [];
  const text = markdown
    .split(MARKDOWN_INLINE)
    .map((token) => inlineToken(token, links))
    .join("");

  return { text, links };
};

const closePending = (blocks: MarkdownBlockType[], pending: PendingBlocksType): void => {
  if (pending.paragraph.length) {
    blocks.push({
      kind: MarkdownBlockKind.PARAGRAPH,
      text: parseInlineMarkdown(pending.paragraph.join(" ")),
    });
    pending.paragraph = [];
  }

  if (pending.list) {
    blocks.push({ kind: pending.list.kind, items: pending.list.items.map(parseInlineMarkdown) });
    pending.list = null;
  }
};

export const parseMarkdown = (markdown: string): MarkdownBlockType[] => {
  const blocks: MarkdownBlockType[] = [];
  const pending: PendingBlocksType = { paragraph: [], list: null };

  for (const line of markdown.replace(MARKDOWN_HTML, "").split(/\r?\n/)) {
    const heading = MARKDOWN_HEADING.exec(line);
    const bullet = MARKDOWN_BULLET.exec(line);
    const item = bullet ?? MARKDOWN_NUMBER.exec(line);

    if (!line.trim() || MARKDOWN_RULE.test(line)) {
      closePending(blocks, pending);
    } else if (heading) {
      closePending(blocks, pending);

      if (heading[1]) {
        blocks.push({ kind: MarkdownBlockKind.HEADING, text: parseInlineMarkdown(heading[1]) });
      }
    } else if (item) {
      const kind = bullet ? MarkdownBlockKind.BULLETS : MarkdownBlockKind.NUMBERS;

      if (pending.paragraph.length || pending.list?.kind !== kind) {
        closePending(blocks, pending);
        pending.list = { kind, items: [] };
      }

      pending.list.items.push(item[1].trim());
    } else if (pending.list) {
      pending.list.items[pending.list.items.length - 1] += ` ${line.trim()}`;
    } else {
      pending.paragraph.push(line.trim());
    }
  }

  closePending(blocks, pending);

  return blocks;
};
