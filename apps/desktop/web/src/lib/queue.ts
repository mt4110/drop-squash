import type { ConversionSummary } from "./commands";

export type QueueStatus = "queued" | "running" | "succeeded" | "failed" | "cancelled";

export type QueueEntry = {
  id: number;
  inputPath: string;
  status: QueueStatus;
  progress?: number;
  result?: ConversionSummary;
  error?: string;
};

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
