import {
  LICENSE_LOCK_QUEUE_MESSAGE,
  blockQueuedForLicenseLock,
  queueSummary,
  type QueueEntry,
} from "./queue.js";

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
