const NOT_SMALLER_MESSAGE =
  "この録画はこれ以上小さくできませんでした。すでに十分小さい可能性があるため、元ファイルを保持し、この試行は回数に数えません / This recording could not be made smaller. DropSquash kept the original and did not count the attempt. Try a smaller Size setting if needed.";
const CONVERSION_FAILED_MESSAGE =
  "この録画は変換できませんでした。DropSquash は元ファイルを保持し、この試行は回数に数えません / This recording could not be converted, so DropSquash kept the original and did not count the attempt.";

function stripErrorPrefixes(reason: unknown): string {
  let message = String(reason).trim();
  while (true) {
    if (message.startsWith("Error: ")) {
      message = message.slice(7).trimStart();
      continue;
    }
    if (message.startsWith("encoder failed: ")) {
      message = message.slice(16).trimStart();
      continue;
    }
    return message;
  }
}

export function isNotSmallerMessage(reason: unknown): boolean {
  const lower = stripErrorPrefixes(reason).toLowerCase();
  return lower.includes("output is not smaller")
    || lower.includes("not smaller than original")
    || lower.includes("could not be made smaller");
}

export function userErrorMessage(reason: unknown): string {
  const message = stripErrorPrefixes(reason);
  if (isNotSmallerMessage(message)) return NOT_SMALLER_MESSAGE;
  if (message.includes("The operation could not be completed")) return CONVERSION_FAILED_MESSAGE;
  return message;
}
