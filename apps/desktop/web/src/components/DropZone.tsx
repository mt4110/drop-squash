import { useEffect, useRef } from "react";
import type { ConversionSummary } from "../lib/commands";
import type { LockedReason } from "../lib/commands";
import { displayPath, fileName, formatBytes, parentPath } from "../lib/format";
import { lockedMessage as defaultLockedMessage, lockedTitle } from "../lib/licenseLock";
import { recordManualQaEvent } from "../lib/manualQa";

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
  const loggedReceiptPathRef = useRef<string | undefined>(undefined);
  const canTrashOriginal =
    result?.sourceAction === "ask-user" &&
    typeof result.sourcePath === "string" &&
    result.sourcePath.length > 0;
  const receiptPath = result?.receiptPath ?? result?.privacyReceiptPath;
  const receiptLabel = result?.receiptKind === "secure-share"
    ? { ja: "共有記録", en: "Share receipt", title: "Finderで共有記録を表示 / Show share receipt in Finder" }
    : { ja: "プライバシー記録", en: "Privacy receipt", title: "Finderで記録を表示 / Show privacy receipt in Finder" };
  const lockTitle = lockedTitle(lockedReason);
  const lockMessage = lockedMessage ?? defaultLockedMessage(lockedReason);
  const titleMain = isLocked
    ? lockedReason === "license-refresh-required"
      ? "ライセンスの再認証が必要です"
      : "トライアル完了"
    : isBusy
      ? "圧縮しています"
      : result
        ? "保存しました"
        : "録画をドロップ";
  const titleSub = isLocked
    ? lockTitle
    : isBusy
      ? "Compressing recording"
      : result
        ? "Saved"
        : "Drop Recording";
  const metaMain = error
    ? error
    : isBusy
      ? progress && progress > 0 ? `進行状況 ${progress}%` : "録画を準備しています"
      : result
        ? fileName(result.outputPath)
        : isLocked
          ? "ライセンスを更新すると、このMacで続けて使えます"
          : `${inputExtensions.map((extension) => extension.toUpperCase()).join(" / ")} をここへ`;
  const metaSub = error
    ? undefined
    : isBusy
      ? progress && progress > 0 ? `${progress}% complete` : "Preparing recording"
      : result
        ? "より小さい MP4 をローカルに保存しました / Smaller MP4 saved locally"
        : isLocked
          ? lockMessage
          : `${inputExtensions.map((extension) => extension.toUpperCase()).join(" / ")} here`;

  useEffect(() => {
    if (!result?.receiptPath || result.receiptKind !== "secure-share") {
      return;
    }
    if (loggedReceiptPathRef.current === result.receiptPath) {
      return;
    }
    loggedReceiptPathRef.current = result.receiptPath;
    recordManualQaEvent("secure-share-result", fileName(result.receiptPath));
  }, [result]);

  return (
    <section
      className={`drop-zone${isDragging ? " is-dragging" : ""}`}
      aria-label="録画をドロップ / Drop recording"
    >
      <p className="drop-title">
        <span className="ui-copy ui-copy-center">
          <span className="ui-main ui-main-title">{titleMain}</span>
          <span className="ui-sub ui-sub-title">{titleSub}</span>
        </span>
      </p>
      <p className={`drop-meta${error ? " is-error" : ""}`} role={error ? "alert" : undefined}>
        <span className="ui-copy ui-copy-center">
          <span className="ui-main">{metaMain}</span>
          {metaSub && <span className="ui-sub">{metaSub}</span>}
        </span>
      </p>
      {result && <button className="saved-destination" title="Finderで保存先を表示 / Show output in Finder" type="button" onClick={() => onRevealOutput(result.outputPath)}><span className="ui-copy ui-copy-center"><span className="ui-main">{`${formatBytes(result.savedBytes)} 節約して保存`}</span><span className="ui-sub">{`${formatBytes(result.savedBytes)} smaller to ${displayPath(parentPath(result.outputPath))}`}</span></span></button>}
      {receiptPath && <button className="receipt-link" title={receiptLabel.title} type="button" onClick={() => {
        recordManualQaEvent("receipt-open", fileName(receiptPath));
        onRevealReceipt(receiptPath);
      }}><span className="ui-copy ui-copy-center"><span className="ui-main">{receiptLabel.ja}</span><span className="ui-sub">{receiptLabel.en}</span></span></button>}
      {canTrashOriginal && (
        <button
          className="trash-original"
          disabled={isTrashingOriginal}
          type="button"
          onClick={() => onTrashOriginal(result.sourcePath, result.outputPath)}
        >
          <span className="ui-copy ui-copy-center">
            <span className="ui-main">{isTrashingOriginal ? "元ファイルを移動中" : "元ファイルをゴミ箱へ移動"}</span>
            <span className="ui-sub">{isTrashingOriginal ? "Moving original..." : "Move original to Trash"}</span>
          </span>
        </button>
      )}
      {!result && inputPath && <p className="source-path" title={inputPath}>{inputPath}</p>}
      {isBusy && <div className="progress" aria-label="変換進行状況 / Conversion progress" aria-valuemax={100} aria-valuemin={0} aria-valuenow={progress ?? 0} role="progressbar"><span style={{ width: `${Math.max(3, progress ?? 0)}%` }} /></div>}
      {isBusy && <button className="cancel-button" type="button" onClick={onCancel}><span className="ui-copy ui-copy-center"><span className="ui-main">停止</span><span className="ui-sub">Cancel</span></span></button>}
      {!isBusy && !isLocked && <button className="drop-picker" type="button" onClick={onPick}><span className="ui-copy ui-copy-center"><span className="ui-main">録画を選ぶ</span><span className="ui-sub">Choose recording</span></span></button>}
    </section>
  );
}
