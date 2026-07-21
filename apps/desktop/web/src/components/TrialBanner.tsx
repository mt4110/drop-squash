import type { LockedReason } from "../lib/commands";
import { trialBannerMessage } from "../lib/trialBanner";
import type { Locale } from "./LocaleSwitch";

type TrialBannerProps = {
  successfulConversions: number;
  trialLimit: number;
  isPro: boolean;
  isLocked: boolean;
  lockedReason?: LockedReason;
  locale: Locale;
};

export function TrialBanner({
  successfulConversions,
  trialLimit,
  isPro,
  isLocked,
  lockedReason,
  locale,
}: TrialBannerProps) {
  const message = trialBannerMessage({
    successfulConversions,
    trialLimit,
    isPro,
    isLocked,
    lockedReason,
  });
  const remaining = Math.max(0, trialLimit - successfulConversions);
  const main = isPro
    ? "Pro版が有効です"
    : isLocked
      ? lockedReason === "license-refresh-required"
        ? "ライセンスの再認証が必要です"
        : "無料トライアルが終了しました"
      : `無料変換 ${successfulConversions}/${trialLimit}、残り ${remaining} 回`;
  const english = isPro
    ? "DropSquash Pro is active on this Mac"
    : isLocked
      ? lockedReason === "license-refresh-required"
        ? "Reconnect your Pro license on this Mac"
        : "Your free trial is complete"
      : `${successfulConversions} of ${trialLimit} free conversions used (${remaining} left)`;

  return (
    <p className={`trial${isLocked ? " is-locked" : ""}`} aria-live="polite">
      <span className="ui-copy">
        <span className="ui-main">{main}</span>
        <span className="ui-sub">{locale === "en" ? english : message}</span>
      </span>
    </p>
  );
}
