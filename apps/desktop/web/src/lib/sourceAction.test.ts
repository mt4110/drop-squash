import {
  applySourceActionDecision,
  sourceActionError,
} from "./sourceAction.js";
import type { ConversionSummary, SourceActionDecision } from "./commands.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

const result: ConversionSummary = {
  outputPath: "/tmp/recording.squashed.mp4",
  originalBytes: 100,
  outputBytes: 40,
  savedBytes: 60,
  reductionPercent: 60,
  sourceAction: "ask-user",
  sourcePath: "/tmp/recording.mov",
};

function refusedDecision(): SourceActionDecision {
  return {
    action: "keep-original",
    reason: "output verification failed",
    sourcePath: "/tmp/recording.mov",
  };
}

function refusedTrashUpdatesMatchingResult() {
  const next = applySourceActionDecision(
    result,
    result.outputPath,
    refusedDecision(),
  );

  assert(next?.sourceAction === "keep-original", "refused action was not applied");
}

function sourceActionDecisionLeavesOtherOutputsAlone() {
  const next = applySourceActionDecision(
    result,
    "/tmp/other.squashed.mp4",
    refusedDecision(),
  );

  assert(next?.sourceAction === "ask-user", "unmatched output was changed");
}

function sourceActionErrorOnlyForRefusedDecisions() {
  assert(sourceActionError(refusedDecision()) === "output verification failed", "missing error");
  assert(sourceActionError({
    action: "move-original-to-trash",
    reason: "moved",
    sourcePath: "/tmp/recording.mov",
  }) === undefined, "successful trash decision should not be an error");
}

refusedTrashUpdatesMatchingResult();
sourceActionDecisionLeavesOtherOutputsAlone();
sourceActionErrorOnlyForRefusedDecisions();
