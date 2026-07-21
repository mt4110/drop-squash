import type { OutputSize, Profile, SelectOption, SourcePolicy } from "../lib/commands";
import { displayPath } from "../lib/format";
import type { Locale } from "./LocaleSwitch";

type SettingsDrawerProps = {
  outputDir: string;
  profile: Profile;
  outputSize: OutputSize;
  sourcePolicy: SourcePolicy;
  writePrivacyReceipt: boolean;
  profiles: SelectOption<Profile>[];
  outputSizes: SelectOption<OutputSize>[];
  sourcePolicies: SelectOption<SourcePolicy>[];
  onChooseOutput: () => void;
  onProfileChange: (profile: Profile) => void;
  onOutputSizeChange: (outputSize: OutputSize) => void;
  onSourcePolicyChange: (policy: SourcePolicy) => void;
  onWritePrivacyReceiptChange: (enabled: boolean) => void;
  locale: Locale;
};

function optionLabel<T>(option: SelectOption<T>, locale: Locale) {
  if (locale === "ja") return option.label;
  return option.label.split(" / ").pop() ?? option.label;
}

export function SettingsDrawer({
  outputDir,
  profile,
  outputSize,
  sourcePolicy,
  writePrivacyReceipt,
  profiles,
  outputSizes,
  sourcePolicies,
  onChooseOutput,
  onProfileChange,
  onOutputSizeChange,
  onSourcePolicyChange,
  onWritePrivacyReceiptChange,
  locale,
}: SettingsDrawerProps) {
  const labels = {
    output: ["保存先", "Output"],
    profile: ["プロファイル", "Profile"],
    size: ["サイズ", "Size"],
    original: ["元ファイル", "Original"],
    receipt: ["変換記録", "Privacy receipt"],
    privacy: ["外部送信", "Network"],
  } as const;

  return (
    <section className="settings" aria-label="設定 / Settings">
      <div className="settings-row">
        <span className="setting-copy"><strong>{labels.output[0]}</strong><small>{labels.output[1]}</small></span>
        <button className="output-picker" title={outputDir} type="button" onClick={onChooseOutput}>
          <span>{displayPath(outputDir)}</span>
          <strong>{locale === "ja" ? "選ぶ" : "Choose"}</strong>
        </button>
      </div>
      <label>
        <span className="setting-copy"><strong>{labels.profile[0]}</strong><small>{labels.profile[1]}</small></span>
        <select value={profile} onChange={(event) => onProfileChange(event.target.value as Profile)}>
          {profiles.map((option) => <option key={option.value} value={option.value}>{optionLabel(option, locale)}</option>)}
        </select>
      </label>
      <label>
        <span className="setting-copy"><strong>{labels.size[0]}</strong><small>{labels.size[1]}</small></span>
        <select value={outputSize} onChange={(event) => onOutputSizeChange(event.target.value as OutputSize)}>
          {outputSizes.map((option) => <option key={option.value} value={option.value}>{optionLabel(option, locale)}</option>)}
        </select>
      </label>
      <label>
        <span className="setting-copy"><strong>{labels.original[0]}</strong><small>{labels.original[1]}</small></span>
        <select value={sourcePolicy} onChange={(event) => onSourcePolicyChange(event.target.value as SourcePolicy)}>
          {sourcePolicies.map((option) => <option key={option.value} value={option.value}>{optionLabel(option, locale)}</option>)}
        </select>
      </label>
      <label>
        <span className="setting-copy"><strong>{labels.receipt[0]}</strong><small>{labels.receipt[1]}</small></span>
        <input
          checked={writePrivacyReceipt}
          type="checkbox"
          onChange={(event) => onWritePrivacyReceiptChange(event.target.checked)}
        />
      </label>
      <p className="settings-help">
        {locale === "ja" ? "オンだと動画の横に確認用の記録ファイルを保存します。オフだと動画だけを保存します。" : "On saves a sidecar receipt file. Off saves only the video."}
      </p>
      <div className="privacy-note">
        <span className="setting-copy"><strong>{labels.privacy[0]}</strong><small>{labels.privacy[1]}</small></span>
        <span className="setting-value">
          <strong>{locale === "ja" ? "なし" : "No upload"}</strong>
        </span>
      </div>
      <p className="settings-help">
        {locale === "ja" ? "録画や記録はこのMacの中だけで処理します。自動アップロードはしません。" : "Everything stays on this Mac. There is no automatic upload."}
      </p>
    </section>
  );
}
