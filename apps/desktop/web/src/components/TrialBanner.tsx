type TrialBannerProps = {
  successfulConversions: number;
  trialLimit: number;
  isPro: boolean;
  isLocked: boolean;
  lockedMessage?: string;
};

export function TrialBanner({ successfulConversions, trialLimit, isPro, isLocked, lockedMessage }: TrialBannerProps) {
  return (
    <p className={`trial${isLocked ? " is-locked" : ""}`} aria-live="polite">
      {isPro ? "Pro license active" : isLocked ? lockedMessage ?? "Trial complete" : `${successfulConversions} of ${trialLimit} free conversions used`}
    </p>
  );
}
