type TrialBannerProps = {
  successfulConversions: number;
  trialLimit: number;
  isPro: boolean;
  isLocked: boolean;
};

export function TrialBanner({ successfulConversions, trialLimit, isPro, isLocked }: TrialBannerProps) {
  return (
    <p className={`trial${isLocked ? " is-locked" : ""}`} aria-live="polite">
      {isPro ? "Pro license active" : isLocked ? "Trial complete" : `${successfulConversions} of ${trialLimit} free conversions used`}
    </p>
  );
}
