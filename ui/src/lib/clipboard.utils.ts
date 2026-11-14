import { writeText } from "@tauri-apps/plugin-clipboard-manager";

import { isTauri } from "./api/tauri";

// ! Safari writes only during the click: a text still being fetched goes in as a promise.
const writeLater = (text: Promise<string>): Promise<void> | null => {
  if (!window.isSecureContext || typeof ClipboardItem === "undefined") {
    return null;
  }

  const blob = text.then((value) => new Blob([value], { type: "text/plain" }));

  return navigator.clipboard.write([new ClipboardItem({ "text/plain": blob })]);
};

const writeOldWay = (text: string): boolean => {
  // ! Over plain HTTP the browser has no clipboard API; inside a dialog, its focus trap keeps the field.
  const host = document.activeElement?.closest('[role="dialog"]') ?? document.body;
  const area = document.createElement("textarea");

  area.value = text;
  area.readOnly = true;
  area.style.position = "fixed";
  area.style.opacity = "0";
  host.appendChild(area);
  area.focus();
  area.select();

  let copied = false;

  try {
    copied = document.execCommand("copy");
  } catch (e: any) {
    console.warn("the browser refused to copy", e);
  }

  area.remove();

  return copied;
};

export const copyText = async (source: Promise<string> | string): Promise<boolean> => {
  const text = Promise.resolve(source);

  if (isTauri()) {
    try {
      await writeText(await text);

      return true;
    } catch (e: any) {
      console.warn("Tauri clipboard refused the text, trying the browser's", e);
    }
  } else {
    const written = writeLater(text);

    if (written) {
      try {
        await written;

        return true;
      } catch (e: any) {
        console.warn("the browser refused the clipboard item, trying plain text", e);
      }
    }
  }

  const value = await text;

  try {
    await navigator.clipboard.writeText(value);

    return true;
  } catch (e: any) {
    return writeOldWay(value);
  }
};
