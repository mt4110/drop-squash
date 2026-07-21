import { useCallback, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import type { Locale } from "./LocaleSwitch";
import type { OutputSize } from "../lib/commands";
import { needsAccessibilityPermission, needsScreenPermission, secureShareError } from "../lib/secureShareError";

type WindowTarget = { windowId: number; ownerPid: number; x: number; y: number; title: string; width: number; height: number };
type RecordingResult = {
  outputPath: string;
  maskPlanPath: string;
  frameCount: number;
  accessibilityObservationCount: number;
  focusedTextObservationCount: number;
  visionObservationCount: number;
  temporalObservationCount: number;
  maskedRegionCount: number;
  fullyMaskedFrameCount: number;
  continuityAttested: boolean;
};

type SecureShareAlphaProps = {
  disabled: boolean;
  locale: Locale;
  outputDir: string;
  outputSize: OutputSize;
  onRevealOutput: (path: string) => void;
};

export function SecureShareAlpha({ disabled, locale, outputDir, outputSize, onRevealOutput }: SecureShareAlphaProps) {
  const [targets, setTargets] = useState<WindowTarget[]>([]);
  const [selected, setSelected] = useState<WindowTarget>();
  const [recording, setRecording] = useState<RecordingResult>();
  const [error, setError] = useState<string>();
  const [permissionNeeded, setPermissionNeeded] = useState(false);
  const [accessibilityNeeded, setAccessibilityNeeded] = useState(false);
  const [phase, setPhase] = useState<"idle" | "starting" | "recording" | "saving">("idle");
  const busy = disabled || phase === "starting" || phase === "saving";
  const startDisabled = busy || !selected;
  const copy = locale === "ja" ? japanese : english;

  const loadTargets = useCallback(async () => {
    if (!isTauri()) return;
      setPhase("starting"); setError(undefined); setPermissionNeeded(false); setAccessibilityNeeded(false); setRecording(undefined);
    try {
      const next = await invoke<WindowTarget[]>("secure_share_recording_targets");
      setTargets(next); setSelected(next.length === 1 ? next[0] : undefined);
    } catch (reason) { setError(secureShareError(reason, locale)); setPermissionNeeded(needsScreenPermission(reason)); setAccessibilityNeeded(needsAccessibilityPermission(reason)); }
    finally { setPhase("idle"); }
  }, [locale]);

  const start = useCallback(async () => {
    if (!selected || !isTauri()) return;
    setPhase("starting"); setError(undefined); setPermissionNeeded(false); setAccessibilityNeeded(false); setRecording(undefined);
    try {
      await invoke("secure_share_recording_start", { selection: selected, outputDir, outputSize });
      setPhase("recording");
    } catch (reason) { setError(secureShareError(reason, locale)); setPermissionNeeded(needsScreenPermission(reason)); setAccessibilityNeeded(needsAccessibilityPermission(reason)); setPhase("idle"); }
  }, [locale, outputDir, outputSize, selected]);

  const stop = useCallback(async () => {
    if (!isTauri()) return;
    setPhase("saving"); setError(undefined); setPermissionNeeded(false); setAccessibilityNeeded(false);
    try {
      setRecording(await invoke<RecordingResult>("secure_share_recording_stop"));
    } catch (reason) { setError(secureShareError(reason, locale)); setPermissionNeeded(needsScreenPermission(reason)); setAccessibilityNeeded(needsAccessibilityPermission(reason)); }
    finally { setPhase("idle"); }
  }, [locale]);

  return (
    <section className="secure-share-alpha" aria-label={copy.title}>
      <div className="secure-share-heading">
        <span><strong>{copy.title}</strong><small>{copy.subtitle}</small></span>
        <button disabled={busy || phase === "recording"} onClick={() => void loadTargets()} type="button">{copy.choose}</button>
      </div>
      {targets.length === 0 && <p className="secure-share-prompt">{copy.choosePrompt}</p>}
      {targets.length > 0 && <div className="secure-share-controls">
        <select aria-label={copy.target} disabled={busy || phase === "recording"} onChange={(event) => setSelected(targets.find((target) => target.windowId === Number(event.target.value)))} value={selected?.windowId ?? ""}>
          {!selected && <option disabled value="">{copy.pickTarget}</option>}
          {targets.map((target) => <option key={target.windowId} value={target.windowId}>{target.title} ({target.width} x {target.height})</option>)}
        </select>
        {phase === "recording" ? (
          <button className="secure-share-stop" onClick={() => void stop()} type="button">{copy.stop}</button>
        ) : (
          <button className="secure-share-record" disabled={startDisabled} onClick={() => void start()} type="button">{phase === "starting" ? copy.starting : copy.start}</button>
        )}
      </div>}
      {targets.length > 0 && !selected && <p className="secure-share-prompt">{copy.selectPrompt}</p>}
      {targets.length > 0 && selected && disabled && <p className="secure-share-prompt">{copy.conversionBusy}</p>}
      {targets.length > 0 && selected && phase === "idle" && !recording && <p className="secure-share-prompt">{copy.ready}</p>}
      {phase === "recording" && <div className="secure-share-status is-recording"><p><span aria-hidden="true" /><strong>{copy.recordingTitle}</strong></p><small>{copy.recording}</small><small>{copy.monitoring}</small></div>}
      {phase === "saving" && <div className="secure-share-status"><strong>{copy.savingTitle}</strong><small>{copy.saving}</small></div>}
      {recording && <div className="secure-share-result">
        <strong>{copy.savedTitle}</strong><small>{copy.saved(recording)}</small><small>{copy.structure(recording)}</small><small>{copy.observed(recording)}</small><small>{recording.continuityAttested ? copy.attested : copy.unattested}</small>
        <span><button className="secure-share-output" onClick={() => onRevealOutput(recording.outputPath)} type="button">{copy.showVideo}</button><button className="secure-share-output" onClick={() => onRevealOutput(recording.maskPlanPath)} type="button">{copy.showReceipt}</button></span>
        <small>{copy.failClosed}</small>
      </div>}
      {error && <div className="secure-share-error"><strong>{copy.errorTitle}</strong><span>{error}</span>{permissionNeeded && <button onClick={() => void invoke("open_screen_capture_settings")} type="button">{copy.openSettings}</button>}{accessibilityNeeded && <button onClick={() => void invoke("open_accessibility_settings")} type="button">{copy.openAccessibilitySettings}</button>}<small>{copy.errorAction}</small></div>}
      <p className="secure-share-note">{copy.note}<small>{copy.detectionNotice}</small></p>
    </section>
  );
}

const japanese = {
  title: "黒塗り検証の研究", subtitle: "全フレーム黒塗り / Secure Share research", choose: "対象を選ぶ", target: "検証対象", pickTarget: "検証するウィンドウを選択", selectPrompt: "候補から、検証したい画面を選んでください。",
  choosePrompt: "黒塗り検証を行うウィンドウを選んでください。", ready: "このウィンドウで、全フレーム黒塗りの検証収録を開始できます。",
  conversionBusy: "変換処理が終わるまで、検証収録は開始できません。変換の停止または完了後に開始してください。",
  start: "黒塗り検証を開始", starting: "収録を準備中...", stop: "停止して検証結果を保存", recordingTitle: "検証収録中", recording: "対象の画面を操作してからDropSquashへ戻り、「停止して検証結果を保存」を押してください。このalphaの保存動画は黒一色です。", monitoring: "選んだ対象とDropSquashの間だけを切り替えてください。対象の移動・サイズ変更や、ほかのアプリの表示があると保存しません。", savingTitle: "黒塗りと検証中", saving: "各フレームを黒塗りし、独立検証と証跡作成を順に行っています。",
  savedTitle: "黒塗り検証動画を保存しました", saved: (value: RecordingResult) => `${value.frameCount} フレームを黒塗り・検証しました。画面内容を残す共有動画ではありません。`, structure: (value: RecordingResult) => `アクセシビリティ構造: 文字要素の矩形を ${value.accessibilityObservationCount} 件、入力欄のフォーカス矩形を ${value.focusedTextObservationCount} 件、端末内で観測しました。内容は保存していません。`, observed: (value: RecordingResult) => `Vision: 文字領域 ${value.visionObservationCount} 件。画面変化候補: ${value.temporalObservationCount} 件。どちらも端末内で観測し、文字列は保存していません。`, attested: "連続性監視を含む証跡を確認しました。", unattested: "連続性の証跡を確認できなかったため、共有しないでください。",
  showVideo: "検証動画をFinderで表示", showReceipt: "証跡を表示", failClosed: "検証または署名に失敗した場合、動画は保存されません。",
  errorTitle: "保存を中止しました", errorAction: "収録対象または検証を確認してから、もう一度試してください。",
  openSettings: "システム設定を開く / Open System Settings",
  openAccessibilitySettings: "アクセシビリティ設定を開く / Open Accessibility Settings",
  note: "無音の研究用ウィンドウ収録です。このalphaは画面内容を残す共有動画を作りません。検知結果にかかわらず各フレーム全体を黒塗りしてから独立デコード検証します。",
  detectionNotice: "検知した文字は保存・アップロードしません。",
};

const english = {
  title: "Secure Share research", subtitle: "Full-frame blackening", choose: "Choose target", target: "Research target", pickTarget: "Choose a window to verify", selectPrompt: "Choose the screen you want to verify.",
  choosePrompt: "Choose a window for blackening verification.", ready: "This window is ready for a full-frame blackening verification recording.",
  conversionBusy: "Verification recording starts after the current conversion stops or finishes.",
  start: "Start blackening verification", starting: "Preparing recording...", stop: "Stop and save verification", recordingTitle: "Verification recording", recording: "Use the chosen window, then return to DropSquash and save the verification. This alpha saves an all-black video.", monitoring: "Switch only between the chosen window and DropSquash. Moving or resizing the target, or bringing another app forward, stops saving.", savingTitle: "Blackening and verifying", saving: "Blackening every frame, independently verifying, and creating the receipt.",
  savedTitle: "Blackened verification video saved", saved: (value: RecordingResult) => `Blackened and verified ${value.frameCount} frames. This is not a shareable video that preserves screen content.`, structure: (value: RecordingResult) => `Accessibility structure: observed ${value.accessibilityObservationCount} text-element geometries and ${value.focusedTextObservationCount} focused-input geometries locally. Values were not saved.`, observed: (value: RecordingResult) => `Vision: ${value.visionObservationCount} local text regions. Change candidates: ${value.temporalObservationCount}. Both stay local; recognized text was not saved.`, attested: "The continuity-attested evidence was verified.", unattested: "Continuity evidence was not confirmed. Do not share this video.",
  showVideo: "Show verification video", showReceipt: "Show receipt", failClosed: "No video is saved when verification or signing fails.",
  errorTitle: "Saving was stopped", errorAction: "Check the target or verification message before trying again.",
  openSettings: "Open System Settings",
  openAccessibilitySettings: "Open Accessibility Settings",
  note: "This is a silent research recorder. It does not create a shareable video that preserves screen content. Strict Shield blackens every captured frame before independent decode verification.",
  detectionNotice: "Detected text is not stored or uploaded.",
};
