import {
  encodeResultFromSummary,
  isFinishedQueueEvent,
  queueEntryFromRustItem,
  queueStatusFromRust,
  requestFromRustItem,
  type RustQueueItem,
} from "./queueWire.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

function mapsRustQueueItemToUiEntry() {
  const entry = queueEntryFromRustItem(item("Blocked", "Trial complete"));

  assert(entry.id === 7, "id should be preserved");
  assert(entry.inputPath === "/tmp/input.mov", "input path should be mapped");
  assert(entry.status === "blocked", "status should be mapped");
  assert(entry.error === "Trial complete", "error should be mapped");
}

function rewritesNotSmallerRustQueueErrorForPeople() {
  const entry = queueEntryFromRustItem(
    item(
      "Failed",
      "encoder failed: native export failed output verification: output is not smaller (1177311 bytes -> 1235958 bytes)",
    ),
  );

  assert(
    entry.status === "unchanged",
    "not-smaller rows should keep the original instead of showing a generic failure",
  );
  assert(
    entry.error === "この録画はこれ以上小さくできませんでした。すでに十分小さい可能性があるため、元ファイルを保持し、この試行は回数に数えません / This recording could not be made smaller. DropSquash kept the original and did not count the attempt. Try a smaller Size setting if needed.",
    "not-smaller error should be rewritten",
  );
}

function mapsRustStatusesToUiStatuses() {
  assert(queueStatusFromRust("Queued") === "queued", "queued status");
  assert(queueStatusFromRust("Running") === "running", "running status");
  assert(queueStatusFromRust("Succeeded") === "succeeded", "succeeded status");
  assert(queueStatusFromRust("Unchanged") === "unchanged", "unchanged status");
  assert(queueStatusFromRust("Failed") === "failed", "failed status");
  assert(queueStatusFromRust("Cancelled") === "cancelled", "cancelled status");
  assert(queueStatusFromRust("Blocked") === "blocked", "blocked status");
}

function mapsRustQueueItemToConversionRequest() {
  const request = requestFromRustItem(item("Queued"), false);

  assert(request.inputPath === "/tmp/input.mov", "input path should be mapped");
  assert(request.outputDir === "/tmp/out", "output dir should be mapped");
  assert(request.profile === "auto", "profile should be mapped");
  assert(request.outputSize === "auto", "output size should be mapped");
  assert(request.sourcePolicy === "ask", "source policy should be mapped");
  assert(!request.writePrivacyReceipt, "privacy receipt flag should be preserved");
}

function preservesSecureShareInConversionRequest() {
  const request = requestFromRustItem(item("Queued", undefined, {
    maskMode: "solid_black",
    maskRects: [{ x: 1, y: 2, width: 3, height: 4 }],
  }), true);

  assert(request.writePrivacyReceipt, "privacy receipt flag should be preserved");
  assert(request.secureShare?.maskMode === "solid_black", "mask mode should survive queue round-trip");
  assert(request.secureShare?.maskRects[0]?.width === 3, "mask rect should survive queue round-trip");
}

function preservesMultipleSecureShareRectsInConversionRequest() {
  const request = requestFromRustItem(item("Queued", undefined, {
    maskMode: "black_noise",
    maskRects: [
      { x: 1, y: 2, width: 3, height: 4 },
      { x: 10, y: 20, width: 30, height: 40 },
    ],
  }), true);

  assert(request.secureShare?.maskMode === "black_noise", "second mask mode should survive queue round-trip");
  assert(request.secureShare?.maskRects.length === 2, "multiple mask rects should survive queue round-trip");
  assert(request.secureShare?.maskRects[1]?.height === 40, "second mask rect should survive queue round-trip");
}

function mapsConversionSummaryToRustEncodeResult() {
  const result = encodeResultFromSummary({
    outputPath: "/tmp/out.mp4",
    originalBytes: 100,
    outputBytes: 40,
    savedBytes: 60,
    reductionPercent: 60,
    sourceAction: "keep-original",
    sourcePath: "/tmp/input.mov",
  }, "docs");

  assert(result.input_path === "/tmp/input.mov", "source path should be mapped");
  assert(result.output_path === "/tmp/out.mp4", "output path should be mapped");
  assert(result.profile === "docs", "profile should be mapped");
  assert(result.original_bytes === 100, "original bytes should be mapped");
  assert(result.output_bytes === 40, "output bytes should be mapped");
  assert(result.success, "successful summary should finish active queue job");
}

function recognizesFinishedQueueEventForJob() {
  const result = encodeResultFromSummary({
    outputPath: "/tmp/out.mp4",
    originalBytes: 100,
    outputBytes: 40,
    savedBytes: 60,
    reductionPercent: 60,
    sourceAction: "keep-original",
    sourcePath: "/tmp/input.mov",
  }, "auto");

  assert(isFinishedQueueEvent({ Finished: { id: 7, result } }, 7), "matching finished event");
  assert(!isFinishedQueueEvent({ Finished: { id: 8, result } }, 7), "wrong job id");
  assert(!isFinishedQueueEvent(null, 7), "missing event");
}

function item(
  status: RustQueueItem["status"],
  error?: string,
  secureShare?: NonNullable<RustQueueItem["job"]["secure_share"]>,
): RustQueueItem {
  return {
    id: 7,
    job: {
      input_path: "/tmp/input.mov",
      output_dir: "/tmp/out",
      profile: "auto",
      output_size: "auto",
      source_policy: "ask",
      secure_share: secureShare,
    },
    status,
    error,
  };
}

mapsRustQueueItemToUiEntry();
rewritesNotSmallerRustQueueErrorForPeople();
mapsRustStatusesToUiStatuses();
mapsRustQueueItemToConversionRequest();
preservesSecureShareInConversionRequest();
preservesMultipleSecureShareRectsInConversionRequest();
mapsConversionSummaryToRustEncodeResult();
recognizesFinishedQueueEventForJob();
