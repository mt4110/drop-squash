import type { DropZoneState } from "./commands.js";

export const initialState: DropZoneState = {
  productName: "DropSquash",
  outputDir: "~/Movies/DropSquash",
  profile: "auto",
  outputSize: "auto",
  profiles: [
    { value: "auto", label: "自動 / Auto" },
    { value: "slack", label: "共有 / Slack" },
    { value: "docs", label: "文書 / Docs" },
    { value: "archive", label: "保管 / Archive" },
  ],
  outputSizes: [
    { value: "auto", label: "自動 / Auto" },
    { value: "1080p", label: "大 / 1920 x 1080" },
    { value: "720p", label: "中 / 1280 x 720" },
    { value: "480p", label: "小 / 640 x 480" },
  ],
  inputExtensions: ["mov", "mp4", "m4v"],
  sourcePolicy: "ask",
  sourcePolicies: [
    { value: "ask", label: "保存後に確認 / Ask after save" },
    { value: "keep", label: "元を残す / Keep original" },
    { value: "trash", label: "ゴミ箱へ移動 / Move to Trash" },
  ],
  privacyMode: "local-only",
  writePrivacyReceipt: true,
  successfulConversions: 0,
  trialLimit: 20,
  isPro: false,
  isLocked: false,
  lockedReason: undefined,
};
