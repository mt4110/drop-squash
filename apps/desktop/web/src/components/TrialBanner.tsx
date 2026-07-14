import { TRIAL_COMPLETE_MESSAGE } from "../lib/licenseLock";

type TrialBannerProps = {
  successfulConversions: number;
  trialLimit: number;
  isPro: boolean;
  isLocked: boolean;
  lockedMessage?: string;
};

export function TrialBanner({
  successfulConversions,
  trialLimit,
  isPro,
  isLocked,
  lockedMessage,
}: TrialBannerProps) {
  const message = isPro
    ? "DropSquash Pro active"
    : isLocked
      ? lockedMessage ?? TRIAL_COMPLETE_MESSAGE
      : `${successfulConversions} of ${trialLimit} free conversions used`;

  return (
    <p className={`trial${isLocked ? " is-locked" : ""}`} aria-live="polite">
      {message}
    </p>
  );
}
