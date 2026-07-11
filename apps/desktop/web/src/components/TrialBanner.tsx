type TrialBannerProps = {
  successfulConversions: number;
  trialLimit: number;
  isLocked: boolean;
};

export function TrialBanner({ successfulConversions, trialLimit, isLocked }: TrialBannerProps) {
  return (
    <p className={`trial${isLocked ? " is-locked" : ""}`} aria-live="polite">
      {isLocked ? "Trial complete" : `${successfulConversions} of ${trialLimit} free conversions used`}
    </p>
  );
}
