import type { DropZoneState } from "./commands.js";

export const initialState: DropZoneState = {
  productName: "DropSquash",
  outputDir: "~/Movies/DropSquash",
  profile: "auto",
  outputSize: "auto",
  profiles: [
    { value: "auto", label: "Auto" },
    { value: "slack", label: "Slack" },
    { value: "teams", label: "Microsoft Teams" },
    { value: "discord", label: "Discord" },
    { value: "chatwork", label: "Chatwork" },
    { value: "line", label: "LINE" },
    { value: "whatsapp", label: "WhatsApp" },
    { value: "docs", label: "Docs" },
  ],
  outputSizes: [
    { value: "auto", label: "Auto" },
    { value: "1080p", label: "1920 x 1080" },
    { value: "720p", label: "1280 x 720" },
    { value: "480p", label: "640 x 480" },
  ],
  inputExtensions: ["mov", "mp4", "m4v"],
  sourcePolicy: "ask",
  sourcePolicies: [
    { value: "ask", label: "Ask after saving" },
    { value: "keep", label: "Keep original" },
    { value: "trash", label: "Move to Trash" },
  ],
  privacyMode: "local-only",
  writePrivacyReceipt: true,
  successfulConversions: 0,
  trialLimit: 10,
  isPro: false,
  isLocked: false,
  lockedReason: undefined,
};
