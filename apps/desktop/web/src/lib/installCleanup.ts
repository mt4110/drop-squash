import type { InstallerCleanup } from "./commands.js";

export type InstallerCleanupAction = "eject-mounted-volume";

export function visibleInstallerCleanupActions(
  cleanup: InstallerCleanup | undefined,
  didEjectInstallerVolume: boolean,
): InstallerCleanupAction[] {
  if (
    cleanup?.shouldOfferMountedVolumeEject &&
    cleanup.mountedVolumePath &&
    !didEjectInstallerVolume
  ) {
    return ["eject-mounted-volume"];
  }
  return [];
}
