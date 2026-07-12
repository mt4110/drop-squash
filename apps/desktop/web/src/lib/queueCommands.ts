import { invoke } from "@tauri-apps/api/core";
import type { ConvertRequest } from "./commands.js";
import type { QueueEntry } from "./queue.js";
import {
  queueEntryFromRustItem,
  type RustEncodeResult,
  type RustQueueEvent,
  type RustQueueItem,
} from "./queueWire.js";

export function enqueueQueueJob(request: ConvertRequest) {
  return invoke<RustQueueEvent>("enqueue_queue_job", { request });
}

export function enqueueFiles(requests: ConvertRequest[]) {
  return invoke<RustQueueEvent[]>("enqueue_files", { requests });
}

export function startNextQueueJob() {
  return invoke<RustQueueEvent | null>("start_next_queue_job");
}

export function finishActiveQueueJob(result: RustEncodeResult) {
  return invoke<RustQueueEvent | null>("finish_active_queue_job", { result });
}

export function failActiveQueueJob(error: string) {
  return invoke<RustQueueEvent | null>("fail_active_queue_job", { error });
}

export function cancelActiveQueueJob() {
  return invoke<RustQueueEvent | null>("cancel_active_queue_job");
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
