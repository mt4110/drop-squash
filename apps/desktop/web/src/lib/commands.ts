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
export type SourcePolicy = "keep" | "trash" | "ask";
export type SourceAction = "keep-original" | "ask-user" | "move-original-to-trash";

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
  sourcePolicy: SourcePolicy;
  sourcePolicies: SelectOption<SourcePolicy>[];
  privacyMode: "local-only";
  successfulConversions: number;
  trialLimit: number;
  isPro: boolean;
  isLocked: boolean;
};

export type ConversionSummary = {
  outputPath: string;
  originalBytes: number;
  outputBytes: number;
  savedBytes: number;
  reductionPercent: number;
  sourceAction: SourceAction;
  sourcePath: string;
};

export type SourceActionDecision = {
  action: SourceAction;
  reason: string;
  sourcePath: string;
};

export type SavedConfig = {
  outputDir: string;
  profile: Profile;
  outputSize: OutputSize;
  sourcePolicy: SourcePolicy;
};
