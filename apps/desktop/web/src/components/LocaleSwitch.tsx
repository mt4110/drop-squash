export type Locale = "ja" | "en";

type LocaleSwitchProps = {
  locale: Locale;
  onChange: (locale: Locale) => void;
};

export function LocaleSwitch({ locale, onChange }: LocaleSwitchProps) {
  return (
    <div aria-label="表示言語 / Display language" className="locale-switch">
      <button
        aria-pressed={locale === "ja"}
        type="button"
        onClick={() => onChange("ja")}
      >
        JA
      </button>
      <button
        aria-pressed={locale === "en"}
        type="button"
        onClick={() => onChange("en")}
      >
        EN
      </button>
    </div>
  );
}
