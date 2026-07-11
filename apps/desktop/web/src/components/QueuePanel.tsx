import { fileName, formatBytes } from "../lib/format";
import type { QueueEntry } from "../lib/queue";
import { hasFinished, queueSummary, statusLabel } from "../lib/queue";

type QueuePanelProps = {
  items: QueueEntry[];
  onCancelQueued: (id: number) => void;
  onClearFinished: () => void;
  onRevealOutput: (outputPath: string) => void;
  onRevealReceipt: (receiptPath: string) => void;
};

export function QueuePanel({ items, onCancelQueued, onClearFinished, onRevealOutput, onRevealReceipt }: QueuePanelProps) {
  if (items.length === 0) {
    return null;
  }

  const summary = queueSummary(items);

  return (
    <section className="queue" aria-label="Queue">
      {hasFinished(items) && (
        <div className="queue-actions">
          <span>{summary.finished} of {summary.total} finished - {formatBytes(summary.savedBytes)} saved</span>
          <button type="button" onClick={onClearFinished}>Clear finished</button>
        </div>
      )}
      {items.map((item) => {
        const result = item.result;
        const receiptPath = result?.privacyReceiptPath;

        return (
          <div className="queue-row" key={item.id}>
            <span>{fileName(item.inputPath)}</span>
            {result ? (
              <span className="queue-result">
                <button type="button" onClick={() => onRevealOutput(result.outputPath)}>
                  {formatBytes(result.savedBytes)}
                </button>
                {receiptPath && (
                  <button type="button" onClick={() => onRevealReceipt(receiptPath)}>
                    Receipt
                  </button>
                )}
              </span>
            ) : (
              <span className="queue-result">
                <strong title={item.error}>
                  {item.progress ? `${item.progress}%` : statusLabel(item.status)}
                </strong>
                {item.status === "queued" && (
                  <button type="button" onClick={() => onCancelQueued(item.id)}>
                    Cancel
                  </button>
                )}
              </span>
            )}
          </div>
        );
      })}
    </section>
  );
}
