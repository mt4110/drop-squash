import { fileName, formatBytes } from "../lib/format";
import type { QueueEntry } from "../lib/queue";
import { hasFinished, statusLabel } from "../lib/queue";

type QueuePanelProps = {
  items: QueueEntry[];
  onClearFinished: () => void;
  onRevealOutput: (outputPath: string) => void;
};

export function QueuePanel({ items, onClearFinished, onRevealOutput }: QueuePanelProps) {
  if (items.length === 0) {
    return null;
  }

  return (
    <section className="queue" aria-label="Queue">
      {hasFinished(items) && (
        <div className="queue-actions">
          <button type="button" onClick={onClearFinished}>Clear finished</button>
        </div>
      )}
      {items.map((item) => (
        <div className="queue-row" key={item.id}>
          <span>{fileName(item.inputPath)}</span>
          {item.result ? (
            <button type="button" onClick={() => onRevealOutput(item.result!.outputPath)}>
              {formatBytes(item.result.savedBytes)}
            </button>
          ) : (
            <strong title={item.error}>
              {item.progress ? `${item.progress}%` : statusLabel(item.status)}
            </strong>
          )}
        </div>
      ))}
    </section>
  );
}
