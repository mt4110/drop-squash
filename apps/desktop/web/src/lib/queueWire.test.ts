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

function mapsRustStatusesToUiStatuses() {
  assert(queueStatusFromRust("Queued") === "queued", "queued status");
  assert(queueStatusFromRust("Running") === "running", "running status");
  assert(queueStatusFromRust("Succeeded") === "succeeded", "succeeded status");
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

function item(status: RustQueueItem["status"], error?: string): RustQueueItem {
  return {
    id: 7,
    job: {
      input_path: "/tmp/input.mov",
      output_dir: "/tmp/out",
      profile: "auto",
      output_size: "auto",
      source_policy: "ask",
    },
    status,
    error,
  };
}

mapsRustQueueItemToUiEntry();
mapsRustStatusesToUiStatuses();
mapsRustQueueItemToConversionRequest();
mapsConversionSummaryToRustEncodeResult();
recognizesFinishedQueueEventForJob();
