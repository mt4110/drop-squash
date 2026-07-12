import { invoke } from "@tauri-apps/api/core";
import type { ConvertRequest } from "./commands.js";
import type { QueueEntry } from "./queue.js";
import {
  queueEntryFromRustItem,
  type RustQueueEvent,
  type RustQueueItem,
} from "./queueWire.js";

export function enqueueQueueJob(request: ConvertRequest) {
  return invoke<RustQueueEvent>("enqueue_queue_job", { request });
}

export function startNextQueueJob() {
  return invoke<RustQueueEvent | null>("start_next_queue_job");
}

export function cancelQueuedJob(id: number) {
  return invoke<RustQueueEvent | null>("cancel_queued_job", { id });
}

export function blockQueuedJobs(error: string) {
  return invoke<RustQueueEvent[]>("block_queued_jobs", { error });
}

export async function clearCompletedQueueJobs(): Promise<QueueEntry[]> {
  const items = await invoke<RustQueueItem[]>("clear_completed_queue_jobs");
  return items.map(queueEntryFromRustItem);
}
