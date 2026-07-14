import type { DropZoneState } from "./commands.js";

export const TRIAL_COMPLETE_MESSAGE =
  "You used 10 successful conversions. Upgrade once to keep squashing locally.";

export function lockedMessage(reason: DropZoneState["lockedReason"]) {
  return reason === "license-refresh-required"
    ? "Reconnect once with your license key to refresh Pro."
    : TRIAL_COMPLETE_MESSAGE;
}

export function lockedTitle(reason: DropZoneState["lockedReason"]) {
  return reason === "license-refresh-required" ? "License refresh required" : "Trial complete";
}
