import type { ConversionSummary } from "../lib/commands";
import type { LockedReason } from "../lib/commands";
import { displayPath, fileName, formatBytes, parentPath } from "../lib/format";
import { lockedMessage as defaultLockedMessage, lockedTitle } from "../lib/licenseLock";

type DropZoneProps = {
  isBusy: boolean;
  isDragging: boolean;
  isLocked: boolean;
  lockedReason?: LockedReason;
  lockedMessage?: string;
  progress?: number;
  inputPath?: string;
  result?: ConversionSummary;
  error?: string;
  inputExtensions: string[];
  isTrashingOriginal: boolean;
  onPick: () => void;
  onCancel: () => void;
  onRevealOutput: (outputPath: string) => void;
  onRevealReceipt: (receiptPath: string) => void;
  onTrashOriginal: (sourcePath: string, outputPath: string) => void;
};

export function DropZone({
  isBusy,
  isDragging,
  isLocked,
  lockedReason,
  lockedMessage,
  progress,
  inputPath,
  result,
  error,
  inputExtensions,
  isTrashingOriginal,
  onPick,
  onCancel,
  onRevealOutput,
  onRevealReceipt,
  onTrashOriginal,
}: DropZoneProps) {
  const canTrashOriginal = result?.sourceAction === "ask-user";
  const receiptPath = result?.privacyReceiptPath;
  const lockTitle = lockedTitle(lockedReason);
  const lockMessage = lockedMessage ?? defaultLockedMessage(lockedReason);

  return (
    <section
      className={`drop-zone${isDragging ? " is-dragging" : ""}`}
      aria-label="Drop recording"
    >
      <p className="drop-title">
        {isLocked ? lockTitle : isBusy ? "Compressing recording" : result ? "Saved" : "Drop Recording"}
      </p>
      <p className={`drop-meta${error ? " is-error" : ""}`} role={error ? "alert" : undefined}>
        {error
          ? error
          : isBusy
            ? progress && progress > 0 ? `${progress}% complete` : "Preparing recording"
          : result
            ? fileName(result.outputPath)
          : isLocked
            ? lockMessage
            : `${inputExtensions.map((extension) => extension.toUpperCase()).join(" / ")} here`}
      </p>
      {result && <button className="saved-destination" title="Show output in Finder" type="button" onClick={() => onRevealOutput(result.outputPath)}>Saved {formatBytes(result.savedBytes)} to {displayPath(parentPath(result.outputPath))}</button>}
      {receiptPath && <button className="receipt-link" title="Show privacy receipt in Finder" type="button" onClick={() => onRevealReceipt(receiptPath)}>Privacy receipt</button>}
      {canTrashOriginal && (
        <button
          className="trash-original"
          disabled={isTrashingOriginal}
          type="button"
          onClick={() => onTrashOriginal(result.sourcePath, result.outputPath)}
        >
          {isTrashingOriginal ? "Moving original..." : "Move original to Trash"}
        </button>
      )}
      {!result && inputPath && <p className="source-path" title={inputPath}>{inputPath}</p>}
      {isBusy && <div className="progress" aria-label="Conversion progress" aria-valuemax={100} aria-valuemin={0} aria-valuenow={progress ?? 0} role="progressbar"><span style={{ width: `${Math.max(3, progress ?? 0)}%` }} /></div>}
      {isBusy && <button className="cancel-button" type="button" onClick={onCancel}>Cancel</button>}
      {!isBusy && !isLocked && <button className="drop-picker" type="button" onClick={onPick}>Choose recording</button>}
    </section>
  );
}
