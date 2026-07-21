import {
  LICENSE_LOCK_QUEUE_MESSAGE,
  blockQueued,
  blockQueuedForLicenseLock,
  markFailed,
  markUnchanged,
  markSourceAction,
  queueDisplayItems,
  queueHeadline,
  queueSummary,
  type QueueEntry,
} from "./queue.js";
import { lockedMessage } from "./licenseLock.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

function licenseLockBlocksOnlyQueuedJobs() {
  const items = blockQueuedForLicenseLock([
    entry(1, "succeeded"),
    entry(2, "running"),
    entry(3, "queued"),
    entry(4, "queued"),
  ]);

  assert(items[0]?.status === "succeeded", "succeeded job changed");
  assert(items[1]?.status === "running", "running job changed");
  assert(items[2]?.status === "blocked", "first queued job not blocked");
  assert(items[3]?.status === "blocked", "second queued job not blocked");
  assert(items[2]?.error === LICENSE_LOCK_QUEUE_MESSAGE, "missing lock message");
  assert(items[3]?.error === LICENSE_LOCK_QUEUE_MESSAGE, "missing lock message");
}

function licenseLockSummaryCountsBlockedJobsAsFinished() {
  const items = blockQueuedForLicenseLock([
    entry(1, "succeeded", { savedBytes: 80 }),
    entry(2, "queued"),
    entry(3, "queued"),
  ]);
  const summary = queueSummary(items);

  assert(summary.finished === 3, "blocked jobs should be finished");
  assert(summary.blocked === 2, "blocked count should include locked jobs");
  assert(summary.queued === 0, "locked queue should have no queued jobs");
  assert(summary.savedBytes === 80, "blocked jobs should not add saved bytes");
}

function refreshLockKeepsReconnectMessage() {
  const error = lockedMessage("license-refresh-required");
  const items = blockQueued([
    entry(1, "running"),
    entry(2, "queued"),
  ], error);

  assert(items[0]?.status === "running", "running job changed");
  assert(items[1]?.status === "blocked", "queued job not blocked");
  assert(items[1]?.error === error, "refresh lock message was not preserved");
}

function sourceActionUpdatesOnlyMatchingOutput() {
  const items = markSourceAction([
    entry(1, "succeeded", { outputPath: "/tmp/a.mp4", sourceAction: "ask-user" }),
    entry(2, "succeeded", { outputPath: "/tmp/b.mp4", sourceAction: "ask-user" }),
  ], "/tmp/b.mp4", "move-original-to-trash");

  assert(items[0]?.result?.sourceAction === "ask-user", "unmatched row changed");
  assert(
    items[1]?.result?.sourceAction === "move-original-to-trash",
    "matched row was not updated",
  );
}

function failureRowsUseFriendlyMessages() {
  const items = markFailed([
    entry(1, "running"),
  ], 1, "failed", "encoder failed: native export failed output verification: output is not smaller (1177000 bytes -> 1234571 bytes)");

  assert(items[0]?.status === "unchanged", "not-smaller row should keep the original");
  assert(
    items[0]?.error === "この録画はこれ以上小さくできませんでした。すでに十分小さい可能性があるため、元ファイルを保持し、この試行は回数に数えません / This recording could not be made smaller. DropSquash kept the original and did not count the attempt. Try a smaller Size setting if needed.",
    "failed row should use the friendly not-smaller message",
  );
}

function friendlyNotSmallerRowsStillKeepOriginal() {
  const items = markFailed([
    { ...entry(1, "running"), progress: 50 },
  ], 1, "failed", "This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip.");

  assert(items[0]?.status === "unchanged", "friendly not-smaller row should keep the original");
  assert(items[0]?.progress === undefined, "finished not-smaller row should not keep stale progress");
}

function unchangedRowsUseFriendlyMessagesDirectly() {
  const items = markUnchanged([
    { ...entry(1, "running"), progress: 50 },
  ], 1, "encoder failed: native export failed output verification: output is not smaller (1177000 bytes -> 1234571 bytes)");

  assert(items[0]?.status === "unchanged", "unchanged row should keep the original");
  assert(items[0]?.progress === undefined, "unchanged row should not keep stale progress");
  assert(
    items[0]?.error === "この録画はこれ以上小さくできませんでした。すでに十分小さい可能性があるため、元ファイルを保持し、この試行は回数に数えません / This recording could not be made smaller. DropSquash kept the original and did not count the attempt. Try a smaller Size setting if needed.",
    "unchanged row should use the friendly not-smaller message",
  );
}

function queueHeadlineIncludesActiveAndFinishedCounts() {
  const headline = queueHeadline(queueSummary([
    entry(1, "running"),
    entry(2, "queued"),
    entry(3, "succeeded", { savedBytes: 80 }),
    entry(4, "unchanged"),
    entry(5, "blocked"),
  ]));

  assert(headline.includes("合計 5 件"), "headline should include total count");
  assert(headline.includes("進行中 1 件"), "headline should include active count");
  assert(headline.includes("待機 1 件"), "headline should include queued count");
  assert(headline.includes("完了 3 件"), "headline should include finished count");
  assert(headline.includes("80 bytes 節約"), "headline should include saved bytes");
  assert(headline.includes("元維持 1 件"), "headline should include unchanged count");
  assert(headline.includes("ロック 1 件"), "headline should include blocked count");
}

function activeAndQueuedRowsSortAboveFinishedRows() {
  const sorted = queueDisplayItems([
    entry(3, "succeeded"),
    entry(4, "blocked"),
    entry(2, "queued"),
    entry(1, "running"),
  ]);

  assert(sorted[0]?.status === "running", "running row should be first");
  assert(sorted[1]?.status === "queued", "queued row should stay near the top");
  assert(sorted[sorted.length - 1]?.status === "succeeded", "finished success should sink");
}

function entry(
  id: number,
  status: QueueEntry["status"],
  result?: Partial<NonNullable<QueueEntry["result"]>>,
): QueueEntry {
  return {
    id,
    inputPath: `/tmp/${id}.mov`,
    status,
    result: result as QueueEntry["result"],
  };
}

licenseLockBlocksOnlyQueuedJobs();
licenseLockSummaryCountsBlockedJobsAsFinished();
refreshLockKeepsReconnectMessage();
sourceActionUpdatesOnlyMatchingOutput();
failureRowsUseFriendlyMessages();
friendlyNotSmallerRowsStillKeepOriginal();
unchangedRowsUseFriendlyMessagesDirectly();
queueHeadlineIncludesActiveAndFinishedCounts();
activeAndQueuedRowsSortAboveFinishedRows();
