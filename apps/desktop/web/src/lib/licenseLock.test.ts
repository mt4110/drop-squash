import { lockedMessage, lockedTitle } from "./licenseLock.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

assert(
  lockedMessage("trial-complete") === "Enter a license key to continue.",
  "trial lock should ask for a license key",
);

assert(
  lockedMessage("license-refresh-required") === "Reconnect once with your license key to refresh Pro.",
  "expired Pro cache should ask for a reconnect",
);

assert(
  lockedTitle(lockedMessage("license-refresh-required")) === "License refresh required",
  "refresh lock should not look like trial completion",
);
