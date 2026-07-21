import { fileName, formatBytes } from "../lib/format";
import type { QueueEntry } from "../lib/queue";
import { hasFinished, queueDisplayItems, queueSummary, statusLabel } from "../lib/queue";

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
  const headlineJa = [
    `合計 ${summary.total} 件`,
    `進行中 ${summary.running} 件`,
    `待機 ${summary.queued} 件`,
    summary.finished > 0 ? `完了 ${summary.finished} 件` : undefined,
    summary.savedBytes > 0 ? `節約 ${formatBytes(summary.savedBytes)}` : undefined,
    summary.failed > 0 ? `失敗 ${summary.failed} 件` : undefined,
    summary.blocked > 0 ? `停止 ${summary.blocked} 件` : undefined,
  ].filter(Boolean).join(" ・ ");
  const statusMain = (status: QueueEntry["status"]) => {
    switch (status) {
      case "queued":
        return "待機中";
      case "running":
        return "圧縮中";
      case "succeeded":
        return "保存済み";
      case "unchanged":
        return "元ファイル維持";
      case "failed":
        return "失敗";
      case "cancelled":
        return "停止";
      case "blocked":
        return "ロック中";
    }
  };
  const headlineEn = [
    `${summary.total} total`,
    `${summary.running} active`,
    `${summary.queued} queued`,
    summary.finished > 0 ? `${summary.finished} finished` : undefined,
    summary.savedBytes > 0 ? `${formatBytes(summary.savedBytes)} saved` : undefined,
    summary.failed > 0 ? `${summary.failed} failed` : undefined,
    summary.blocked > 0 ? `${summary.blocked} blocked` : undefined,
  ].filter(Boolean).join(" ・ ");

  return (
    <section className="queue" aria-label="変換キュー / Queue">
      <div className="queue-actions">
        <span className="queue-summary">
          <strong>{headlineJa}</strong>
          <small>{headlineEn}</small>
        </span>
        {hasFinished(items) && (
          <button type="button" onClick={onClearFinished}>
            <span className="ui-copy ui-copy-center">
              <span className="ui-main">完了分を消去</span>
              <span className="ui-sub">Clear finished</span>
            </span>
          </button>
        )}
      </div>
      {queueDisplayItems(items).map((item) => {
        const result = item.result;
        const receiptPath = result?.receiptPath ?? result?.privacyReceiptPath;
        const receiptMain = result?.receiptKind === "secure-share" ? "共有記録" : "記録";
        const receiptSub = result?.receiptKind === "secure-share" ? "Share receipt" : "Receipt";

        return (
          <div className="queue-row" key={item.id}>
            <span className="queue-source">{fileName(item.inputPath)}</span>
            {result ? (
              <span className="queue-result">
                <button type="button" onClick={() => onRevealOutput(result.outputPath)}>
                  <span className="queue-result-copy">
                    <strong>{formatBytes(result.savedBytes)}</strong>
                    <small>Saved</small>
                  </span>
                </button>
                {receiptPath && (
                  <button type="button" onClick={() => onRevealReceipt(receiptPath)}>
                    <span className="queue-result-copy">
                      <strong>{receiptMain}</strong>
                      <small>{receiptSub}</small>
                    </span>
                  </button>
                )}
              </span>
            ) : (
              <span className="queue-result">
                <span
                  className={item.status === "unchanged" ? "queue-note" : undefined}
                  title={item.error}
                >
                  <span className="queue-result-copy">
                    <strong>{item.progress ? `${item.progress}%` : statusMain(item.status)}</strong>
                    <small>{item.progress ? "Compressing" : statusLabel(item.status)}</small>
                  </span>
                </span>
                {item.status === "queued" && (
                  <button type="button" onClick={() => onCancelQueued(item.id)}>
                    <span className="ui-copy ui-copy-center">
                      <span className="ui-main">停止</span>
                      <span className="ui-sub">Cancel</span>
                    </span>
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
