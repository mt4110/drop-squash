import type { ConversionSummary } from "./commands.js";
import { TRIAL_COMPLETE_MESSAGE } from "./licenseLock.js";

export type QueueStatus =
  | "queued"
  | "running"
  | "succeeded"
  | "failed"
  | "cancelled"
  | "blocked";

export type QueueEntry = {
  id: number;
  inputPath: string;
  status: QueueStatus;
  progress?: number;
  result?: ConversionSummary;
  error?: string;
  writePrivacyReceipt?: boolean;
};

export type QueueSummary = {
  total: number;
  finished: number;
  queued: number;
  running: number;
  succeeded: number;
  failed: number;
  cancelled: number;
  blocked: number;
  savedBytes: number;
};

export const LICENSE_LOCK_QUEUE_MESSAGE = TRIAL_COMPLETE_MESSAGE;

export function nextQueued(items: QueueEntry[]) {
  return items.find((item) => item.status === "queued");
}

export function hasFinished(items: QueueEntry[]) {
  return items.some((item) => isFinishedStatus(item.status));
}

export function clearFinished(items: QueueEntry[]) {
  return items.filter((item) => !isFinishedStatus(item.status));
}

export function queueSummary(items: QueueEntry[]): QueueSummary {
  return items.reduce<QueueSummary>((summary, item) => {
    summary.total += 1;
    summary[item.status] += 1;
    if (isFinishedStatus(item.status)) summary.finished += 1;
    if (item.status === "succeeded") {
      summary.savedBytes += Math.max(0, item.result?.savedBytes ?? 0);
    }
    return summary;
  }, emptySummary());
}

export function cancelQueued(items: QueueEntry[], id: number) {
  return items.map((item) => (
    item.id === id && item.status === "queued"
      ? { ...item, status: "cancelled" as const, error: "Cancelled before starting" }
      : item
  ));
}

export function blockQueued(items: QueueEntry[], error: string) {
  return items.map((item) => (
    item.status === "queued" ? { ...item, status: "blocked" as const, error } : item
  ));
}

export function blockQueuedForLicenseLock(items: QueueEntry[]) {
  return blockQueued(items, LICENSE_LOCK_QUEUE_MESSAGE);
}

export function markRunning(items: QueueEntry[], id: number) {
  return items.map((item) => (
    item.id === id ? { ...item, status: "running" as const, progress: 0 } : item
  ));
}

export function markSucceeded(items: QueueEntry[], id: number, result: ConversionSummary) {
  return items.map((item) => (
    item.id === id ? { ...item, status: "succeeded" as const, progress: 100, result } : item
  ));
}

export function markFailed(
  items: QueueEntry[],
  id: number,
  status: "failed" | "cancelled",
  error: string,
) {
  return items.map((item) => (
    item.id === id ? { ...item, status, error } : item
  ));
}

export function updateProgress(items: QueueEntry[], id: number, progress: number) {
  return items.map((item) => item.id === id ? { ...item, progress } : item);
}

export function markSourceAction(
  items: QueueEntry[],
  outputPath: string,
  sourceAction: ConversionSummary["sourceAction"],
) {
  return items.map((item) => (
    item.result?.outputPath === outputPath
      ? { ...item, result: { ...item.result, sourceAction } }
      : item
  ));
}

export function statusLabel(status: QueueStatus) {
  switch (status) {
    case "queued":
      return "Queued";
    case "running":
      return "Compressing";
    case "succeeded":
      return "Saved";
    case "failed":
      return "Failed";
    case "cancelled":
      return "Cancelled";
    case "blocked":
      return "Blocked";
  }
}

export function isCancelReason(reason: unknown) {
  return String(reason).toLowerCase().includes("cancelled");
}

function isFinishedStatus(status: QueueStatus) {
  return (
    status === "succeeded"
    || status === "failed"
    || status === "cancelled"
    || status === "blocked"
  );
}

function emptySummary(): QueueSummary {
  return {
    total: 0,
    finished: 0,
    queued: 0,
    running: 0,
    succeeded: 0,
    failed: 0,
    cancelled: 0,
    blocked: 0,
    savedBytes: 0,
  };
}
