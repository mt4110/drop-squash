import type { LockedReason } from "./commands.js";
import { lockedTitle } from "./licenseLock.js";

type TrialBannerState = {
  successfulConversions: number;
  trialLimit: number;
  isPro: boolean;
  isLocked: boolean;
  lockedReason?: LockedReason;
};

export function trialBannerMessage({
  successfulConversions,
  trialLimit,
  isPro,
  isLocked,
  lockedReason,
}: TrialBannerState) {
  if (isPro) return "このMacではProが有効です / DropSquash Pro is active on this Mac.";
  if (isLocked) return lockedTitle(lockedReason);
  const remaining = Math.max(0, trialLimit - successfulConversions);
  return `無料変換 ${successfulConversions}/${trialLimit} 回を使用、残り ${remaining} 回 / ${successfulConversions} of ${trialLimit} free conversions used (${remaining} left)`;
}
