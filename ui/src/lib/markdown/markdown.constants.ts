export enum MarkdownBlockKind {
  HEADING = "HEADING",
  PARAGRAPH = "PARAGRAPH",
  BULLETS = "BULLETS",
  NUMBERS = "NUMBERS",
}

// ! Raw HTML is dropped, never shown or run: a script or style keeps none of its content.
export const MARKDOWN_HTML =
  /<(script|style)\b[\s\S]*?<\/\1\s*>|<!--[\s\S]*?-->|<\/?[A-Za-z][A-Za-z0-9-]*(?:\s[^<>]*)?\/?>/gi;

export const MARKDOWN_HEADING = /^\s{0,3}#{1,6}\s+(.*?)(?:\s+#+)?\s*$/;
export const MARKDOWN_BULLET = /^\s*[-*+]\s+(.*)$/;
export const MARKDOWN_NUMBER = /^\s*\d+[.)]\s+(.*)$/;
export const MARKDOWN_RULE = /^\s*([-*_])(?:\s*\1){2,}\s*$/;

const LINK_SOURCE = String.raw`\[[^\]]+\]\([^()\s]*(?:\([^()\s]*\)[^()\s]*)*\)`;

export const MARKDOWN_INLINE = new RegExp(
  String.raw`(\*\*[^*]+\*\*|__[^_]+__|\`[^\`]+\`|${LINK_SOURCE}|\*[^*\s][^*]*\*)`,
);

export const MARKDOWN_LINK = /^\[([^\]]+)\]\((.*)\)$/;

// ! Only web links open: `javascript:`, `data:` or a relative path stay plain text.
export const MARKDOWN_SAFE_URL = /^https?:\/\/\S+$/i;
