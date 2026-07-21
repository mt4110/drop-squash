import type { ConversionOutcome } from "./commands.js";
import type { ConversionSummary } from "./commands.js";
import { normalizeConversionSummary } from "./conversionSummaryWire.js";

type OutcomeSummary = ConversionSummary | {
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

type ConversionOutcomeWire = {
  Converted?: OutcomeSummary;
  converted?: OutcomeSummary;
  KeptOriginal?: { message: string };
  keptOriginal?: { message: string };
};

export function keptOriginalMessage(outcome: unknown) {
  const wire = outcome as ConversionOutcomeWire;
  if (wire.KeptOriginal) {
    return wire.KeptOriginal.message;
  }
  return wire.keptOriginal?.message;
}

export function convertedSummary(outcome: unknown) {
  const wire = outcome as ConversionOutcomeWire;
  if (wire.Converted) {
    return normalizeConversionSummary(wire.Converted);
  }
  if (wire.converted) {
    return normalizeConversionSummary(wire.converted);
  }
  throw new Error("conversion outcome did not include a converted summary");
}
