import type { DropZoneState, SavedConfig } from "./commands";

export function savedConfigFromState(state: DropZoneState): SavedConfig {
  return {
    outputDir: state.outputDir,
    profile: state.profile,
    outputSize: state.outputSize,
    sourcePolicy: state.sourcePolicy,
    writePrivacyReceipt: state.writePrivacyReceipt,
  };
}

export function stateWithSavedConfigPatch(
  state: DropZoneState,
  patch: Partial<SavedConfig>,
): DropZoneState {
  return { ...state, ...patch };
}
