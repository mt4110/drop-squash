import {
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
