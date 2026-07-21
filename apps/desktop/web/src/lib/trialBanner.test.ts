import { trialBannerMessage } from "./trialBanner.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

assert(
  trialBannerMessage({
    successfulConversions: 5,
    trialLimit: 20,
    isPro: false,
    isLocked: false,
  }) === "無料変換 5/20 回を使用、残り 15 回 / 5 of 20 free conversions used (15 left)",
  "trial banner should show usage and remaining conversions",
);

assert(
  trialBannerMessage({
    successfulConversions: 20,
    trialLimit: 20,
    isPro: false,
    isLocked: true,
    lockedReason: "trial-complete",
  }) === "トライアル完了 / Trial complete",
  "locked banner should stay concise",
);

assert(
  trialBannerMessage({
    successfulConversions: 20,
    trialLimit: 20,
    isPro: false,
    isLocked: true,
    lockedReason: "license-refresh-required",
  }) === "再認証が必要です / License refresh required",
  "refresh-required banner should use the lock title",
);

assert(
  trialBannerMessage({
    successfulConversions: 0,
    trialLimit: 20,
    isPro: true,
    isLocked: false,
  }) === "このMacではProが有効です / DropSquash Pro is active on this Mac.",
  "pro banner should stay simple",
);
