import type { DropZoneState, SavedConfig } from "./commands";

export function savedConfigFromState(state: DropZoneState): SavedConfig {
  return {
    outputDir: state.outputDir,
    profile: state.profile,
    outputSize: state.outputSize,
    sourcePolicy: state.sourcePolicy,
  };
}

export function savedConfigWith(
  state: DropZoneState,
  patch: Partial<SavedConfig>,
): SavedConfig {
  return { ...savedConfigFromState(state), ...patch };
}
