import type { OutputSize, Profile, SelectOption } from "../lib/commands";
import { displayPath } from "../lib/format";

type SettingsDrawerProps = {
  outputDir: string;
  profile: Profile;
  outputSize: OutputSize;
  profiles: SelectOption<Profile>[];
  outputSizes: SelectOption<OutputSize>[];
  onChooseOutput: () => void;
  onProfileChange: (profile: Profile) => void;
  onOutputSizeChange: (outputSize: OutputSize) => void;
};

export function SettingsDrawer({
  outputDir,
  profile,
  outputSize,
  profiles,
  outputSizes,
  onChooseOutput,
  onProfileChange,
  onOutputSizeChange,
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
      <div><span>Privacy</span><strong>Local only &#x2713;</strong></div>
    </section>
  );
}
