import type { ConversionSummary } from "./commands";

export type QueueStatus =
  | "queued"
  | "running"
  | "succeeded"
  | "failed"
  | "cancelled";

export type QueueEntry = {
  id: number;
  inputPath: string;
  status: QueueStatus;
  progress?: number;
  result?: ConversionSummary;
  error?: string;
};

export function entriesForInputPaths(inputPaths: string[], nextId: number) {
  return {
    entries: inputPaths.map((inputPath, index) => ({
      id: nextId + index,
      inputPath,
      status: "queued" as const,
    })),
    nextId: nextId + inputPaths.length,
  };
}

export function nextQueued(items: QueueEntry[]) {
  return items.find((item) => item.status === "queued");
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
  }
}

export function isCancelReason(reason: unknown) {
  return String(reason).toLowerCase().includes("cancelled");
}
