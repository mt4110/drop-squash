import type { OutputSize, Profile, SelectOption, SourcePolicy } from "../lib/commands";
import { displayPath } from "../lib/format";

type SettingsDrawerProps = {
  outputDir: string;
  profile: Profile;
  outputSize: OutputSize;
  sourcePolicy: SourcePolicy;
  profiles: SelectOption<Profile>[];
  outputSizes: SelectOption<OutputSize>[];
  sourcePolicies: SelectOption<SourcePolicy>[];
  onChooseOutput: () => void;
  onProfileChange: (profile: Profile) => void;
  onOutputSizeChange: (outputSize: OutputSize) => void;
  onSourcePolicyChange: (policy: SourcePolicy) => void;
};

export function SettingsDrawer({
  outputDir,
  profile,
  outputSize,
  sourcePolicy,
  profiles,
  outputSizes,
  sourcePolicies,
  onChooseOutput,
  onProfileChange,
  onOutputSizeChange,
  onSourcePolicyChange,
}: SettingsDrawerProps) {
  return (
    <section className="settings" aria-label="Settings">
      <div className="settings-row">
        <span>Output</span>
        <button className="output-picker" title={outputDir} type="button" onClick={onChooseOutput}>
          <span>{displayPath(outputDir)}</span>
          <strong>Choose</strong>
        </button>
      </div>
      <label>
        <span>Profile</span>
        <select value={profile} onChange={(event) => onProfileChange(event.target.value as Profile)}>
          {profiles.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
        </select>
      </label>
      <label>
        <span>Size</span>
        <select value={outputSize} onChange={(event) => onOutputSizeChange(event.target.value as OutputSize)}>
          {outputSizes.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
        </select>
      </label>
      <label>
        <span>Original</span>
        <select value={sourcePolicy} onChange={(event) => onSourcePolicyChange(event.target.value as SourcePolicy)}>
          {sourcePolicies.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
        </select>
      </label>
      <div><span>Privacy</span><strong>Local only &#x2713;</strong></div>
    </section>
  );
}
