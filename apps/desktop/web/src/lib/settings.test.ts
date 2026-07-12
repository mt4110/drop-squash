import { savedConfigFromState, stateWithSavedConfigPatch } from "./settings.js";
import { initialState } from "./initialState.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

function savedConfigIncludesSafetyPreferences() {
  const config = savedConfigFromState({
    ...initialState,
    sourcePolicy: "trash",
    writePrivacyReceipt: false,
  });

  assert(config.sourcePolicy === "trash", "source policy was not persisted");
  assert(config.writePrivacyReceipt === false, "receipt preference was not persisted");
}

function patchKeepsUnchangedSettingsStable() {
  const patched = stateWithSavedConfigPatch(initialState, {
    sourcePolicy: "keep",
  });

  assert(patched.sourcePolicy === "keep", "source policy patch failed");
  assert(patched.writePrivacyReceipt === true, "receipt preference changed");
  assert(patched.outputDir === initialState.outputDir, "output dir changed");
}

savedConfigIncludesSafetyPreferences();
patchKeepsUnchangedSettingsStable();
