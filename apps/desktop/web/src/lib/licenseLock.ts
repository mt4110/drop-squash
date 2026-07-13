import type { DropZoneState } from "./commands.js";

export function lockedMessage(reason: DropZoneState["lockedReason"]) {
  return reason === "license-refresh-required"
    ? "Reconnect once with your license key to refresh Pro."
    : "Enter a license key to continue.";
}

export function lockedTitle(message?: string) {
  return message?.includes("refresh") ? "License refresh required" : "Trial complete";
}
