import type { DropZoneState } from "./commands.js";

export const TRIAL_COMPLETE_MESSAGE =
  "有効な縮小変換を 20 回使いました。このMacで続けるには Pro を有効化してください / You used 20 successful smaller conversions. Unlock Pro to keep squashing on this Mac.";
export const LICENSE_REFRESH_MESSAGE =
  "このMacの Pro を更新するため、ライセンスキーで一度再認証してください / Reconnect once with your license key to refresh Pro on this Mac.";

export function lockedMessage(reason: DropZoneState["lockedReason"]) {
  return reason === "license-refresh-required" ? LICENSE_REFRESH_MESSAGE : TRIAL_COMPLETE_MESSAGE;
}

export function lockedTitle(reason: DropZoneState["lockedReason"]) {
  return reason === "license-refresh-required"
    ? "再認証が必要です / License refresh required"
    : "トライアル完了 / Trial complete";
}

export function isLockedMessage(message: string) {
  return message === TRIAL_COMPLETE_MESSAGE || message === LICENSE_REFRESH_MESSAGE;
}
