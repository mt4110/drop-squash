import { useCallback, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";

type WindowTarget = { windowId: number; title: string; width: number; height: number };
type CaptureResult = {
  visionObservationCount: number;
  liveMaskedFrameCount: number;
  liveMaskedRectCount: number;
  liveVerifiedPixelCount: number;
  maskPlanPreview: { policy: string; frames: { regions: unknown[] }[] };
};

export function SecureShareAlpha({ disabled }: { disabled: boolean }) {
  const [targets, setTargets] = useState<WindowTarget[]>([]);
  const [selected, setSelected] = useState<number>();
  const [result, setResult] = useState<CaptureResult>();
  const [error, setError] = useState<string>();
  const [isLoading, setIsLoading] = useState(false);

  const loadTargets = useCallback(async () => {
    if (!isTauri()) return;
    setIsLoading(true); setError(undefined); setResult(undefined);
    try {
      const next = await invoke<WindowTarget[]>("secure_share_alpha_windows");
      setTargets(next); setSelected(next[0]?.windowId);
    } catch (reason) {
      setError(String(reason));
    } finally { setIsLoading(false); }
  }, []);

  const capture = useCallback(async () => {
    if (!selected || !isTauri()) return;
    setIsLoading(true); setError(undefined); setResult(undefined);
    try {
      setResult(await invoke<CaptureResult>("secure_share_alpha_capture", { windowId: selected }));
    } catch (reason) {
      setError(String(reason));
    } finally { setIsLoading(false); }
  }, [selected]);

  return (
    <section className="secure-share-alpha" aria-label="Secure Share alpha">
      <div className="secure-share-heading">
        <span><strong>安全共有 alpha</strong><small>Secure Share research</small></span>
        <button disabled={disabled || isLoading} onClick={() => void loadTargets()} type="button">
          対象を選ぶ <small>Choose</small>
        </button>
      </div>
      {targets.length > 0 && <div className="secure-share-controls">
        <select aria-label="収録対象 / Capture target" onChange={(event) => setSelected(Number(event.target.value))} value={selected}>
          {targets.map((target) => <option key={target.windowId} value={target.windowId}>
            {target.title} ({target.width} x {target.height})
          </option>)}
        </select>
        <button disabled={disabled || isLoading || !selected} onClick={() => void capture()} type="button">
          {isLoading ? "確認中..." : "1秒テスト"} <small>Test</small>
        </button>
      </div>}
      {result && <p className="secure-share-result">
        {result.visionObservationCount} 件の文字候補を検出し、実フレーム {result.liveMaskedFrameCount} 枚の
        {result.liveMaskedRectCount} 領域を黒塗り、{result.liveVerifiedPixelCount} 点を読み戻し確認しました。
        <small>
          共有前の破壊マスク計画: Strict Reveal / {result.maskPlanPreview.frames[0]?.regions.length ?? 0} 領域
        </small>
        <small>Local-only frame test. No video is saved yet.</small>
      </p>}
      {error && <p className="secure-share-error">{error}</p>}
      <p className="secure-share-note">
        日英の文字領域を検出して、その場の画素を黒で破壊する研究用テストです。録画ファイルの保存・再検証は次段階です。
        <small>Detected text is not stored or uploaded.</small>
      </p>
    </section>
  );
}
