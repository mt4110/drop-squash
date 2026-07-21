import {
  needsAccessibilityPermission,
  needsScreenPermission,
  secureShareError,
} from "./secureShareError.js";

function assert(value: boolean, message: string) {
  if (!value) throw new Error(message);
}

assert(
  secureShareError("Screen Recording permission is required", "ja").includes("システム設定"),
  "Japanese Screen Recording recovery text should be clear",
);
assert(
  secureShareError("requires Screen Recording", "en").startsWith("Screen Recording permission"),
  "English Screen Recording recovery text should be clear",
);
assert(
  secureShareError("native capture failed", "en") === "native capture failed",
  "unrelated failures should remain visible",
);
assert(
  secureShareError("selected window changed after capture", "ja").includes("保存せず"),
  "Japanese target-change recovery should say the output was not saved",
);
assert(
  secureShareError("recording target changed or could not be revalidated", "en").startsWith("The selected window or its focused input"),
  "target disappearance should not expose an internal capture error",
);
assert(
  secureShareError("The selected window changed or could not be verified", "ja").includes("対象を選ぶ"),
  "native target verification failures should explain how to restart safely",
);
assert(
  secureShareError("stream output rejected frame metadata: presentation time gap", "ja").includes("連続フレーム"),
  "frame discontinuity should explain that no output was saved",
);
assert(
  secureShareError("Secure Share frame continuity could not be verified", "ja").includes("保存せず"),
  "native frame continuity should be localized",
);
assert(
  secureShareError("Secure Share video writing could not be verified", "ja").includes("動画書き込み"),
  "native writer verification should be localized",
);
assert(
  secureShareError("recording target changed: macOS display configuration changed", "ja").includes("ディスプレイ構成"),
  "display changes should not be presented as a generic target change",
);
assert(
  secureShareError("Secure Share foreground application changed during capture: third-party application became active", "ja").includes("別のアプリ"),
  "foreground app changes should explain the new fail-closed rule",
);
assert(
  secureShareError("Secure Share macOS lifecycle changed during capture: macOS screens slept", "en").startsWith("The Mac screen/session state changed"),
  "lifecycle changes should explain the Mac state rule",
);
assert(
  secureShareError("The display, session, or active app changed during recording", "ja").includes("保存せず"),
  "native environment failures should be localized",
);
assert(needsScreenPermission("Screen Recording permission is required"), "permission errors should be actionable");
assert(
  secureShareError("Accessibility permission is required", "ja").includes("アクセシビリティ"),
  "Accessibility failures should be localized",
);
assert(
  needsAccessibilityPermission("Accessibility permission is required"),
  "Accessibility failures should open the right settings page",
);
