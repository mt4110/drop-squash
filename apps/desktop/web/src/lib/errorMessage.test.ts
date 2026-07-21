import { isNotSmallerMessage, userErrorMessage } from "./errorMessage.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

assert(
  userErrorMessage(
    "encoder failed: native export failed output verification: output is not smaller (1177311 bytes -> 1235958 bytes)",
  )
    === "この録画はこれ以上小さくできませんでした。すでに十分小さい可能性があるため、元ファイルを保持し、この試行は回数に数えません / This recording could not be made smaller. DropSquash kept the original and did not count the attempt. Try a smaller Size setting if needed.",
  "not-smaller failures should stay friendly in the UI",
);

assert(
  userErrorMessage(new Error("Conversion cancelled.")) === "Conversion cancelled.",
  "plain errors should drop the Error prefix",
);

assert(
  userErrorMessage("encoder failed: The operation could not be completed")
    === "この録画は変換できませんでした。DropSquash は元ファイルを保持し、この試行は回数に数えません / This recording could not be converted, so DropSquash kept the original and did not count the attempt.",
  "generic encoder failures should stay friendly in the UI",
);

assert(
  userErrorMessage("encoder failed: output is not smaller than original")
    === "この録画はこれ以上小さくできませんでした。すでに十分小さい可能性があるため、元ファイルを保持し、この試行は回数に数えません / This recording could not be made smaller. DropSquash kept the original and did not count the attempt. Try a smaller Size setting if needed.",
  "not-smaller-than-original failures should stay friendly in the UI",
);

assert(
  userErrorMessage("Error: encoder failed: output is not smaller than original")
    === "この録画はこれ以上小さくできませんでした。すでに十分小さい可能性があるため、元ファイルを保持し、この試行は回数に数えません / This recording could not be made smaller. DropSquash kept the original and did not count the attempt. Try a smaller Size setting if needed.",
  "prefixed not-smaller failures should stay friendly in the UI",
);

assert(
  isNotSmallerMessage("encoder failed: output is not smaller than original"),
  "not-smaller helper should recognize the original wording",
);

assert(
  isNotSmallerMessage(
    "この録画はこれ以上小さくできませんでした。すでに十分小さい可能性があるため、元ファイルを保持し、この試行は回数に数えません / This recording could not be made smaller. DropSquash kept the original and did not count the attempt. Try a smaller Size setting if needed.",
  ),
  "not-smaller helper should recognize the friendly wording too",
);
