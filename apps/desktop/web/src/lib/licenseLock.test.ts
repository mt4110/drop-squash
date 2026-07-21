import { isLockedMessage, lockedMessage, lockedTitle } from "./licenseLock.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

assert(
  lockedMessage("trial-complete")
    === "有効な縮小変換を 20 回使いました。このMacで続けるには Pro を有効化してください / You used 20 successful smaller conversions. Unlock Pro to keep squashing on this Mac.",
  "trial lock should explain the successful-conversion limit",
);

assert(
  lockedMessage("license-refresh-required") === "このMacの Pro を更新するため、ライセンスキーで一度再認証してください / Reconnect once with your license key to refresh Pro on this Mac.",
  "expired Pro cache should ask for a reconnect",
);

assert(
  lockedTitle("license-refresh-required") === "再認証が必要です / License refresh required",
  "refresh lock should not look like trial completion",
);

assert(
  lockedTitle("trial-complete") === "トライアル完了 / Trial complete",
  "trial lock title should stay distinct",
);

assert(
  isLockedMessage("有効な縮小変換を 20 回使いました。このMacで続けるには Pro を有効化してください / You used 20 successful smaller conversions. Unlock Pro to keep squashing on this Mac."),
  "trial lock message should be recognized",
);

assert(
  isLockedMessage("このMacの Pro を更新するため、ライセンスキーで一度再認証してください / Reconnect once with your license key to refresh Pro on this Mac."),
  "refresh lock message should be recognized",
);

assert(
  !isLockedMessage("This recording could not be converted."),
  "ordinary failures should not look like license locks",
);
