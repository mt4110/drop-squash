import { useState } from "react";
import type { FormEvent } from "react";

type LicensePanelProps = {
  isPro: boolean;
  onActivate: (licenseKey: string) => Promise<void>;
  onForget: () => Promise<void>;
};

export function LicensePanel({ isPro, onActivate, onForget }: LicensePanelProps) {
  const [licenseKey, setLicenseKey] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  async function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
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
      await onForget();
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <section className="license" aria-label="License">
      {isPro ? (
        <button
          disabled={isSubmitting}
          title="Forgets the local cache only; server-side activation is unchanged"
          type="button"
          onClick={() => void forget()}
        >
          Forget license on this Mac
        </button>
      ) : (
        <form onSubmit={(event) => void submit(event)}>
          <input
            aria-label="License key"
            autoComplete="off"
            placeholder="License key"
            type="password"
            value={licenseKey}
            onChange={(event) => setLicenseKey(event.target.value)}
          />
          <button disabled={isSubmitting || licenseKey.trim().length === 0} type="submit">
            Activate
          </button>
        </form>
      )}
    </section>
  );
}
