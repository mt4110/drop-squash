import type { ConversionSummary } from "./commands.js";
import { isNotSmallerMessage, userErrorMessage } from "./errorMessage.js";
import { TRIAL_COMPLETE_MESSAGE } from "./licenseLock.js";

export type QueueStatus =
  | "queued"
  | "running"
  | "succeeded"
  | "unchanged"
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
  unchanged: number;
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

export function queueHeadline(summary: QueueSummary) {
  const parts = [
    `合計 ${summary.total} 件`,
    `進行中 ${summary.running} 件`,
    `待機 ${summary.queued} 件`,
  ];
  if (summary.finished > 0) parts.push(`完了 ${summary.finished} 件`);
  if (summary.savedBytes > 0) parts.push(`${summary.savedBytes} bytes 節約`);
  if (summary.unchanged > 0) parts.push(`元維持 ${summary.unchanged} 件`);
  if (summary.failed > 0) parts.push(`失敗 ${summary.failed} 件`);
  if (summary.cancelled > 0) parts.push(`停止 ${summary.cancelled} 件`);
  if (summary.blocked > 0) parts.push(`ロック ${summary.blocked} 件`);
  return parts.join(" - ");
}

export function queueDisplayItems(items: QueueEntry[]) {
  return [...items].sort((left, right) => (
    statusPriority(left.status) - statusPriority(right.status) || left.id - right.id
  ));
}

export function cancelQueued(items: QueueEntry[], id: number) {
  return items.map((item) => (
    item.id === id && item.status === "queued"
      ? { ...item, status: "cancelled" as const, error: "開始前に停止しました / Cancelled before starting" }
      : item
  ));
}

export function blockQueued(items: QueueEntry[], error: string) {
  const message = userErrorMessage(error);
  return items.map((item) => (
    item.status === "queued" ? { ...item, status: "blocked" as const, error: message } : item
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

export function markUnchanged(items: QueueEntry[], id: number, error: string) {
  const message = userErrorMessage(error);
  return items.map((item) => (
    item.id === id
      ? { ...item, status: "unchanged" as const, error: message, progress: undefined }
      : item
  ));
}

export function markFailed(
  items: QueueEntry[],
  id: number,
  status: "failed" | "cancelled",
  error: string,
) {
  const nextStatus: QueueStatus = status === "failed" && isNotSmallerMessage(error)
    ? "unchanged"
    : status;
  const message = status === "failed" ? userErrorMessage(error) : error;
  return items.map((item) => (
    item.id === id
      ? { ...item, status: nextStatus, error: message, progress: undefined }
      : item
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
      return "待機中 / Queued";
    case "running":
      return "圧縮中 / Compressing";
    case "succeeded":
      return "保存済み / Saved";
    case "unchanged":
      return "元ファイル維持 / Kept original";
    case "failed":
      return "失敗 / Failed";
    case "cancelled":
      return "停止 / Cancelled";
    case "blocked":
      return "ロック中 / Blocked";
  }
}

export function isCancelReason(reason: unknown) {
  return String(reason).toLowerCase().includes("cancelled");
}

function isFinishedStatus(status: QueueStatus) {
  return (
    status === "succeeded"
    || status === "unchanged"
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
    unchanged: 0,
    failed: 0,
    cancelled: 0,
    blocked: 0,
    savedBytes: 0,
  };
}

function statusPriority(status: QueueStatus) {
  switch (status) {
    case "running":
      return 0;
    case "queued":
      return 1;
    case "failed":
      return 2;
    case "unchanged":
      return 3;
    case "cancelled":
      return 4;
    case "blocked":
      return 5;
    case "succeeded":
      return 6;
  }
}
