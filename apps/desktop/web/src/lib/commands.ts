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
export type LockedReason = "trial-complete" | "license-refresh-required";
export type MaskMode = "solid_black" | "black_noise";
export type ReceiptKind = "privacy" | "secure-share";

export type MaskRect = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export type SecureShareOptions = {
  maskMode: MaskMode;
  maskRects: MaskRect[];
};

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
  writePrivacyReceipt: boolean;
  successfulConversions: number;
  trialLimit: number;
  isPro: boolean;
  isLocked: boolean;
  lockedReason?: LockedReason;
};

export type InstallLocation = {
  appPath: string;
  runningFromDiskImage: boolean;
  installedInApplications: boolean;
  shouldOfferApplicationsMove: boolean;
};

export type ApplicationsInstall = {
  sourcePath: string;
  targetPath: string;
  cleanup: InstallerCleanup;
};

export type InstallerCleanup = {
  mountedVolumePath: string | null;
  downloadedDmgPath: string | null;
  shouldOfferMountedVolumeEject: boolean;
  shouldOfferDownloadedDmgTrash: boolean;
};

export type ConversionSummary = {
  outputPath: string;
  originalBytes: number;
  outputBytes: number;
  savedBytes: number;
  reductionPercent: number;
  sourceAction: SourceAction;
  sourcePath: string;
  receiptPath?: string;
  receiptKind?: ReceiptKind;
  privacyReceiptPath?: string;
};

export type ConversionOutcome =
  | { Converted: ConversionSummary }
  | { converted: ConversionSummary }
  | { KeptOriginal: { message: string } }
  | { keptOriginal: { message: string } };

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
  writePrivacyReceipt: boolean;
};

export type ConvertRequest = SavedConfig & {
  inputPath: string;
  secureShare?: SecureShareOptions;
};
