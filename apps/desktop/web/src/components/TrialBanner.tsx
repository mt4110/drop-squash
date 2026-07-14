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
      ? lockedMessage ?? "Trial complete"
      : `${successfulConversions} of ${trialLimit} free conversions used`;

  return (
    <p className={`trial${isLocked ? " is-locked" : ""}`} aria-live="polite">
      {message}
    </p>
  );
}
