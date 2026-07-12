import { useState } from "react";
import type { FormEvent } from "react";
import { canSubmitLicenseKey, normalizedLicenseKey } from "../lib/license";

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
    const key = normalizedLicenseKey(licenseKey);
    if (!canSubmitLicenseKey(key)) {
      return;
    }
    setIsSubmitting(true);
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
    <section className="license" aria-busy={isSubmitting} aria-label="License">
      {isPro ? (
        <button
          disabled={isSubmitting}
          title="Forgets the local cache only; server-side activation is unchanged"
          type="button"
          onClick={() => void forget()}
        >
          {isSubmitting ? "Forgetting..." : "Forget license on this Mac"}
        </button>
      ) : (
        <form onSubmit={(event) => void submit(event)}>
          <input
            aria-label="License key"
            autoCapitalize="off"
            autoComplete="off"
            autoCorrect="off"
            disabled={isSubmitting}
            name="license-key"
            placeholder="License key"
            spellCheck={false}
            type="password"
            value={licenseKey}
            onChange={(event) => setLicenseKey(event.target.value)}
          />
          <button disabled={isSubmitting || !canSubmitLicenseKey(licenseKey)} type="submit">
            {isSubmitting ? "Activating..." : "Activate"}
          </button>
        </form>
      )}
    </section>
  );
}
