import type { ConversionSummary, SourceActionDecision } from "./commands.js";

export function applySourceActionDecision(
  result: ConversionSummary | undefined,
  outputPath: string,
  decision: SourceActionDecision,
) {
  if (!result || result.outputPath !== outputPath) return result;
  return { ...result, sourceAction: decision.action };
}

export function sourceActionError(decision: SourceActionDecision) {
  return decision.action === "move-original-to-trash" ? undefined : decision.reason;
}
