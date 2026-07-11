import { fileName, formatBytes } from "../lib/format";
import type { QueueEntry } from "../lib/queue";
import { statusLabel } from "../lib/queue";

type QueuePanelProps = {
  items: QueueEntry[];
  onRevealOutput: (outputPath: string) => void;
};

export function QueuePanel({ items, onRevealOutput }: QueuePanelProps) {
  if (items.length === 0) {
    return null;
  }

  return (
    <section className="queue" aria-label="Queue">
      {items.map((item) => (
        <div className="queue-row" key={item.id}>
          <span>{fileName(item.inputPath)}</span>
          {item.result ? (
            <button type="button" onClick={() => onRevealOutput(item.result!.outputPath)}>
              {formatBytes(item.result.savedBytes)}
            </button>
          ) : (
            <strong>{item.progress ? `${item.progress}%` : statusLabel(item.status)}</strong>
          )}
        </div>
      ))}
    </section>
  );
}
