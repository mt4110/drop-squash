import type { Locale } from "../components/LocaleSwitch.js";

export function secureShareError(reason: unknown, locale: Locale): string {
  const error = String(reason);
  if (needsAccessibilityPermission(error)) {
    return locale === "ja"
      ? "アクセシビリティの許可が必要です。システム設定 > プライバシーとセキュリティ > アクセシビリティ で、この DropSquash をオンにしてください。"
      : "Accessibility permission is required. Enable this copy of DropSquash in System Settings > Privacy & Security > Accessibility.";
  }
  if (displayConfigurationFailed(error)) {
    return locale === "ja"
      ? "ディスプレイ構成が変化しました。保存せずに停止しました。画面の接続・解像度・ミラーリングを安定させてから、対象を選び直してください。"
      : "The display configuration changed. Recording stopped without saving; stabilize displays, then choose the target again.";
  }
  if (targetContinuityFailed(error)) {
    return locale === "ja"
      ? "選択したウィンドウ、または入力欄の所属を確認できませんでした。保存せずに停止しました。もう一度「対象を選ぶ」から選択してください。"
      : "The selected window or its focused input could not be verified. Recording stopped without saving; choose the target again.";
  }
  if (frameContinuityFailed(error)) {
    return locale === "ja"
      ? "連続フレームを検証できませんでした。保存せずに停止しました。もう一度収録してください。"
      : "Frame continuity could not be verified. Recording stopped without saving; record again.";
  }
  if (writerVerificationFailed(error)) {
    return locale === "ja"
      ? "動画書き込みを検証できませんでした。保存せずに停止しました。もう一度収録してください。"
      : "Video writing could not be verified. Recording stopped without saving; record again.";
  }
  if (foregroundChanged(error)) {
    return locale === "ja"
      ? "別のアプリが前面になりました。保存せずに停止しました。収録中は対象ウィンドウとDropSquashだけを操作してください。"
      : "Another app became active. Recording stopped without saving; use only the target window and DropSquash while recording.";
  }
  if (lifecycleChanged(error)) {
    return locale === "ja"
      ? "Macの画面状態が変わりました。保存せずに停止しました。スリープ、画面ロック、操作スペースの変更を避けてもう一度収録してください。"
      : "The Mac screen/session state changed. Recording stopped without saving; avoid sleep, lock, or Space changes and record again.";
  }
  if (nativeEnvironmentFailed(error)) {
    return locale === "ja"
      ? "収録中の対象ウィンドウ、画面状態、または前面アプリの監視で変化を検出しました。保存せずに停止しました。対象を選び直し、画面構成を固定してからもう一度収録してください。"
      : "A target-window, display, session, or foreground-app watch changed. Recording stopped without saving; choose the target again and record with a stable Mac state.";
  }
  if (!needsScreenPermission(error)) {
    return error;
  }
  return locale === "ja"
    ? "画面収録の許可が必要です。システム設定 > プライバシーとセキュリティ > 画面収録 で、この DropSquash をオンにしてください。"
    : "Screen Recording permission is required. Enable this copy of DropSquash in System Settings > Privacy & Security > Screen Recording.";
}

function displayConfigurationFailed(error: string): boolean {
  return error.includes("display configuration changed")
    || error.includes("could not watch macOS display configuration");
}

function targetContinuityFailed(error: string): boolean {
  return error.includes("selected window changed")
    || error.includes("selected window changed or could not be verified")
    || error.includes("recording target changed")
    || error.includes("requested window candidate was not found");
}

function frameContinuityFailed(error: string): boolean {
  return error.includes("stream output rejected frame metadata")
    || error.includes("presentation time gap")
    || error.includes("non-complete ScreenCaptureKit frame")
    || error.includes("Secure Share frame continuity could not be verified");
}

function writerVerificationFailed(error: string): boolean {
  return error.includes("Secure Share video writing could not be verified");
}

function foregroundChanged(error: string): boolean {
  return error.includes("foreground application changed");
}

function lifecycleChanged(error: string): boolean {
  return error.includes("macOS lifecycle changed")
    || error.includes("user session resigned active")
    || error.includes("workspace will sleep")
    || error.includes("screens slept")
    || error.includes("active Space changed");
}

function nativeEnvironmentFailed(error: string): boolean {
  return error.includes("The display, session, or active app changed during recording");
}

export function needsScreenPermission(reason: unknown): boolean {
  const error = String(reason);
  return error.includes("requires Screen Recording") || error.includes("Screen Recording permission");
}

export function needsAccessibilityPermission(reason: unknown): boolean {
  return String(reason).includes("Accessibility permission");
}
