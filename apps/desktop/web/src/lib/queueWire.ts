import type { ConvertRequest, Profile } from "./commands.js";
import type { QueueEntry, QueueStatus } from "./queue.js";

export type RustQueueStatus =
  | "Queued"
  | "Running"
  | "Succeeded"
  | "Failed"
  | "Cancelled"
  | "Blocked";

export type RustQueueItem = {
  id: number;
  job: {
    input_path: string;
    output_dir: string;
    profile: ConvertRequest["profile"];
    output_size: ConvertRequest["outputSize"];
    source_policy: ConvertRequest["sourcePolicy"];
  };
  status: RustQueueStatus;
  error?: string | null;
};

export type RustEncodeResult = {
  input_path: string;
  output_path: string;
  profile: Profile;
  original_bytes: number;
  output_bytes: number;
  success: boolean;
  error_message?: string | null;
};

export type RustQueueEvent =
  | { Enqueued: RustQueueItem }
  | { Started: RustQueueItem }
  | { Finished: { id: number; result: RustEncodeResult } }
  | { Cancelled: number }
  | { Blocked: { id: number; error: string } }
  | { Failed: { id: number; error: string } };

export function queueEntryFromRustItem(item: RustQueueItem): QueueEntry {
  return {
    id: item.id,
    inputPath: item.job.input_path,
    status: queueStatusFromRust(item.status),
    error: item.error ?? undefined,
  };
}

export function requestFromRustItem(
  item: RustQueueItem,
  writePrivacyReceipt: boolean,
): ConvertRequest {
  return {
    inputPath: item.job.input_path,
    outputDir: item.job.output_dir,
    profile: item.job.profile,
    outputSize: item.job.output_size,
    sourcePolicy: item.job.source_policy,
    writePrivacyReceipt,
  };
}

export function queueStatusFromRust(status: RustQueueStatus): QueueStatus {
  switch (status) {
    case "Queued":
      return "queued";
    case "Running":
      return "running";
    case "Succeeded":
      return "succeeded";
    case "Failed":
      return "failed";
    case "Cancelled":
      return "cancelled";
    case "Blocked":
      return "blocked";
  }
}
