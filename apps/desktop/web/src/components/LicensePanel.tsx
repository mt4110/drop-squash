import { useState } from "react";

type LicensePanelProps = {
  isPro: boolean;
  isLocked: boolean;
  onActivate: (licenseKey: string) => Promise<void>;
  onDeactivate: () => Promise<void>;
};

export function LicensePanel({ isPro, isLocked, onActivate, onDeactivate }: LicensePanelProps) {
  const [licenseKey, setLicenseKey] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const showForm = isLocked || isPro;

  if (!showForm) {
    return null;
  }

  async function submit() {
    setIsSubmitting(true);
    const key = licenseKey;
    setLicenseKey("");
    try {
      await onActivate(key);
    } finally {
      setIsSubmitting(false);
    }
  }

  async function forget() {
    setIsSubmitting(true);
    try {
      await onDeactivate();
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <section className="license" aria-label="License">
      {isPro ? (
        <button disabled={isSubmitting} type="button" onClick={() => void forget()}>Forget license on this Mac</button>
      ) : (
        <>
          <input
            aria-label="License key"
            autoComplete="off"
            placeholder="License key"
            type="password"
            value={licenseKey}
            onChange={(event) => setLicenseKey(event.target.value)}
          />
          <button disabled={isSubmitting || licenseKey.trim().length === 0} type="button" onClick={() => void submit()}>
            Activate
          </button>
        </>
      )}
    </section>
  );
}
