export type Profile =
  | "auto"
  | "slack"
  | "teams"
  | "discord"
  | "chatwork"
  | "line"
  | "whatsapp"
  | "docs"
  | "archive"
  | "privacy";
export type OutputSize = "auto" | "1080p" | "720p" | "480p";

export type SelectOption<T> = {
  value: T;
  label: string;
};

export type DropZoneState = {
  productName: string;
  outputDir: string;
  profile: Profile;
  outputSize: OutputSize;
  profiles: SelectOption<Profile>[];
  outputSizes: SelectOption<OutputSize>[];
  inputExtensions: string[];
  sourcePolicy: "keep" | "trash" | "ask";
  privacyMode: "local-only";
  successfulConversions: number;
  trialLimit: number;
  isLocked: boolean;
};

export type ConversionSummary = {
  outputPath: string;
  originalBytes: number;
  outputBytes: number;
  savedBytes: number;
  reductionPercent: number;
};

export type SavedConfig = {
  outputDir: string;
  profile: Profile;
  outputSize: OutputSize;
};
