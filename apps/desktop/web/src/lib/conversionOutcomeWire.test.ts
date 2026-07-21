import { convertedSummary, keptOriginalMessage } from "./conversionOutcomeWire.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

function readsCamelCaseOutcome() {
  const summary = convertedSummary({
    converted: {
      output_path: "/tmp/out.mp4",
      original_bytes: 100,
      output_bytes: 40,
      saved_bytes: 60,
      reduction_percent: 60,
      source_action: "ask-user",
      source_path: "/tmp/in.mov",
    },
  });

  assert(summary.outputPath === "/tmp/out.mp4", "camelCase outcome should map");
}

function readsLegacyOutcome() {
  const summary = convertedSummary({
    Converted: {
      outputPath: "/tmp/out.mp4",
      originalBytes: 100,
      outputBytes: 40,
      savedBytes: 60,
      reductionPercent: 60,
      sourceAction: "ask-user",
      sourcePath: "/tmp/in.mov",
    },
  });

  assert(summary.sourcePath === "/tmp/in.mov", "legacy outcome should still map");
}

function defaultsMissingOutcomeSourceFields() {
  const summary = convertedSummary({
    converted: {
      output_path: "/tmp/out.mp4",
      saved_bytes: 60,
    },
  });

  assert(summary.sourcePath === "", "missing outcome source path should stay empty");
  assert(summary.sourceAction === "keep-original", "missing outcome source action should default");
}

function readsKeptOriginalMessage() {
  assert(
    keptOriginalMessage({ keptOriginal: { message: "kept" } }) === "kept",
    "camel keptOriginal should map",
  );
  assert(
    keptOriginalMessage({ KeptOriginal: { message: "legacy" } }) === "legacy",
    "legacy KeptOriginal should map",
  );
}

readsCamelCaseOutcome();
readsLegacyOutcome();
defaultsMissingOutcomeSourceFields();
readsKeptOriginalMessage();
