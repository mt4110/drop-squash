import { useEffect, useRef } from "react";

type HelpPopoverProps = {
  onClose: () => void;
};

export function HelpPopover({ onClose }: HelpPopoverProps) {
  const closeButtonRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    closeButtonRef.current?.focus();
  }, []);

  return (
    <div className="help-backdrop" role="presentation" onClick={onClose}>
      <aside
        aria-label="使い方 / Quick tips"
        aria-modal="true"
        className="help-popover"
        onClick={(event) => event.stopPropagation()}
        onKeyDown={(event) => {
          if (event.key === "Escape") {
            onClose();
          }
        }}
        role="dialog"
      >
        <div className="ui-copy">
          <strong className="ui-main">使い方</strong>
          <span className="ui-sub">Quick tips</span>
        </div>
        <p className="help-copy">
          <span className="ui-copy">
            <span className="ui-main">録画をドロップするか選ぶだけです。元ファイルはそのまま残ります。</span>
            <span className="ui-sub">Drop or choose a recording. The original stays untouched.</span>
          </span>
        </p>
        <p className="help-copy">
          <span className="ui-copy">
            <span className="ui-main">通常は自動で十分です。容量の上限があるときだけサイズを選んでください。</span>
            <span className="ui-sub">Auto is usually enough. Pick a size only when you have a hard limit.</span>
          </span>
        </p>
        <p className="help-copy">
          <span className="ui-copy">
            <span className="ui-main">変換記録をオンにすると確認用の記録ファイルも保存します。オフなら動画だけです。</span>
            <span className="ui-sub">Turn on the receipt to save a sidecar record file. Off saves only the video.</span>
          </span>
        </p>
        <button ref={closeButtonRef} type="button" onClick={onClose}>
          <span className="ui-copy ui-copy-center">
            <span className="ui-main">閉じる</span>
            <span className="ui-sub">Done</span>
          </span>
        </button>
      </aside>
    </div>
  );
}
