import type { ConversionSummary } from "./commands.js";

type ConversionSummaryWire = Partial<ConversionSummary> & {
  output_path?: string;
  original_bytes?: number;
  output_bytes?: number;
  saved_bytes?: number;
  reduction_percent?: number;
  source_action?: ConversionSummary["sourceAction"];
  source_path?: string;
  receipt_path?: string;
  receipt_kind?: ConversionSummary["receiptKind"];
  privacy_receipt_path?: string;
};

export function normalizeConversionSummary(
  summary: ConversionSummaryWire,
): ConversionSummary {
  return {
    outputPath: summary.outputPath ?? summary.output_path ?? "",
    originalBytes: summary.originalBytes ?? summary.original_bytes ?? 0,
    outputBytes: summary.outputBytes ?? summary.output_bytes ?? 0,
    savedBytes: summary.savedBytes ?? summary.saved_bytes ?? 0,
    reductionPercent: summary.reductionPercent ?? summary.reduction_percent ?? 0,
    sourceAction: summary.sourceAction ?? summary.source_action ?? "keep-original",
    sourcePath: summary.sourcePath ?? summary.source_path ?? "",
    receiptPath: summary.receiptPath ?? summary.receipt_path,
    receiptKind: summary.receiptKind ?? summary.receipt_kind,
    privacyReceiptPath:
      summary.privacyReceiptPath ?? summary.privacy_receipt_path,
  };
}
