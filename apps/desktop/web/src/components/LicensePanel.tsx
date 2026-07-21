import { useState } from "react";
import type { FormEvent } from "react";
import type { LockedReason } from "../lib/commands";
import { canSubmitLicenseKey, normalizedLicenseKey } from "../lib/license";
import { licensePanelCopy } from "../lib/licensePanelCopy";
import type { Locale } from "./LocaleSwitch";

type LicensePanelProps = {
  isPro: boolean;
  lockedReason?: LockedReason;
  locale: Locale;
  onActivate: (licenseKey: string) => Promise<void>;
  onForget: () => Promise<void>;
};

export function LicensePanel({
  isPro,
  lockedReason,
  locale,
  onActivate,
  onForget,
}: LicensePanelProps) {
  const [licenseKey, setLicenseKey] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const copy = licensePanelCopy(isPro, lockedReason);
  const messageMain = isPro
    ? "このMacではProで使えます"
    : lockedReason === "license-refresh-required"
      ? "このMacのProを再認証してください"
      : "このMacでProを有効化できます";
  const submitMain = isPro
    ? "このMacのライセンスを削除"
    : lockedReason === "license-refresh-required"
      ? "Proを再認証"
      : "Proを有効化";
  const submitBusy = isPro
    ? "削除中"
    : lockedReason === "license-refresh-required"
      ? "再認証中"
      : "認証中";
  const submitSub = isPro
    ? "Forget local license"
    : lockedReason === "license-refresh-required"
      ? "Refresh Pro"
      : "Unlock Pro";
  const busySub = isPro
    ? "Forgetting..."
    : lockedReason === "license-refresh-required"
      ? "Refreshing..."
      : "Activating...";
  const englishMessage = isPro
    ? "Pro stays unlocked on this Mac. Your recordings still stay local."
    : lockedReason === "license-refresh-required"
      ? "Reconnect once to refresh Pro. Your recordings still stay local."
      : "Unlock Pro to keep squashing on this Mac. Your recordings still stay local.";

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
    <section className={`license${isPro ? " is-pro" : ""}`} aria-busy={isSubmitting} aria-label="ライセンス / License">
      <p className="license-copy">
        <span className="ui-copy">
          <span className="ui-main">{messageMain}</span>
          <span className="ui-sub">{locale === "en" ? englishMessage : copy.message}</span>
        </span>
      </p>
      {isPro ? (
        <button
          disabled={isSubmitting}
          title="このMacの保存情報だけを削除 / Only forgets the local cache on this Mac"
          type="button"
          onClick={() => void forget()}
        >
          <span className="ui-copy">
            <span className="ui-main">{isSubmitting ? submitBusy : submitMain}</span>
            <span className="ui-sub">{isSubmitting ? busySub : submitSub}</span>
          </span>
        </button>
      ) : (
        <form onSubmit={(event) => void submit(event)}>
          <input
            aria-label={locale === "en" ? "License key" : "ライセンスキー"}
            autoCapitalize="off"
            autoComplete="off"
            autoCorrect="off"
            disabled={isSubmitting}
            name="license-key"
            placeholder={locale === "en" ? "License key" : "ライセンスキー"}
            spellCheck={false}
            type="password"
            value={licenseKey}
            onChange={(event) => setLicenseKey(event.target.value)}
          />
          <button disabled={isSubmitting || !canSubmitLicenseKey(licenseKey)} type="submit">
            <span className="ui-copy">
              <span className="ui-main">{isSubmitting ? submitBusy : submitMain}</span>
              <span className="ui-sub">{isSubmitting ? busySub : submitSub}</span>
            </span>
          </button>
        </form>
      )}
    </section>
  );
}
