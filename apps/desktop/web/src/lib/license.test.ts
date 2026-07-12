import { canSubmitLicenseKey, normalizedLicenseKey } from "./license.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

function trimsLicenseKeyBeforeActivation() {
  assert(normalizedLicenseKey("  LS-KEY  ") === "LS-KEY", "license key should be trimmed");
}

function rejectsEmptyLicenseKey() {
  assert(!canSubmitLicenseKey("   "), "empty key should not submit");
}

function acceptsNonEmptyLicenseKey() {
  assert(canSubmitLicenseKey("LS-KEY"), "non-empty key should submit");
}

trimsLicenseKeyBeforeActivation();
rejectsEmptyLicenseKey();
acceptsNonEmptyLicenseKey();
