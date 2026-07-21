import { normalizeConversionSummary } from "./conversionSummaryWire.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

function keepsCamelCaseSummary() {
  const summary = normalizeConversionSummary({
    outputPath: "/tmp/out.mp4",
    originalBytes: 100,
    outputBytes: 40,
    savedBytes: 60,
    reductionPercent: 60,
    sourceAction: "ask-user",
    sourcePath: "/tmp/in.mov",
    receiptPath: "/tmp/out.secure-share.json",
    receiptKind: "secure-share",
    privacyReceiptPath: "/tmp/out.privacy.json",
  });

  assert(summary.sourcePath === "/tmp/in.mov", "camel sourcePath should stay");
  assert(summary.outputPath === "/tmp/out.mp4", "camel outputPath should stay");
  assert(summary.receiptKind === "secure-share", "camel receipt kind should stay");
}

function rewritesSnakeCaseSummary() {
  const summary = normalizeConversionSummary({
    output_path: "/tmp/out.mp4",
    original_bytes: 100,
    output_bytes: 40,
    saved_bytes: 60,
    reduction_percent: 60,
    source_action: "ask-user",
    source_path: "/tmp/in.mov",
    receipt_path: "/tmp/out.secure-share.json",
    receipt_kind: "secure-share",
    privacy_receipt_path: "/tmp/out.privacy.json",
  });

  assert(summary.sourcePath === "/tmp/in.mov", "snake source_path should map");
  assert(summary.outputPath === "/tmp/out.mp4", "snake output_path should map");
  assert(summary.receiptPath === "/tmp/out.secure-share.json", "snake receipt path should map");
  assert(summary.privacyReceiptPath === "/tmp/out.privacy.json", "snake receipt should map");
}

function defaultsMissingSourceFields() {
  const summary = normalizeConversionSummary({
    output_path: "/tmp/out.mp4",
    saved_bytes: 60,
  });

  assert(summary.sourcePath === "", "missing source path should stay empty");
  assert(summary.sourceAction === "keep-original", "missing source action should default");
}

keepsCamelCaseSummary();
rewritesSnakeCaseSummary();
defaultsMissingSourceFields();
