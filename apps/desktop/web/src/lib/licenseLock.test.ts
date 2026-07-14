import { lockedMessage, lockedTitle } from "./licenseLock.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

assert(
  lockedMessage("trial-complete")
    === "You used 10 successful conversions. Upgrade once to keep squashing locally.",
  "trial lock should explain the successful-conversion limit",
);

assert(
  lockedMessage("license-refresh-required") === "Reconnect once with your license key to refresh Pro.",
  "expired Pro cache should ask for a reconnect",
);

assert(
  lockedTitle("license-refresh-required") === "License refresh required",
  "refresh lock should not look like trial completion",
);

assert(
  lockedTitle("trial-complete") === "Trial complete",
  "trial lock title should stay distinct",
);
