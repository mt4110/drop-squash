import { visibleInstallerCleanupActions } from "./installCleanup.js";
import type { InstallerCleanup } from "./commands.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

const cleanup: InstallerCleanup = {
  mountedVolumePath: "/Volumes/DropSquash",
  downloadedDmgPath: "/Users/me/Downloads/DropSquash.dmg",
  shouldOfferMountedVolumeEject: true,
  shouldOfferDownloadedDmgTrash: true,
};

function offersMountedVolumeEjectOnlyBeforeEject() {
  const actions = visibleInstallerCleanupActions(cleanup, false);

  assert(actions.length === 1, "unexpected cleanup action count");
  assert(actions[0] === "eject-mounted-volume", "mounted volume eject was not offered");
}

function hidesCleanupAfterMountedVolumeEject() {
  const actions = visibleInstallerCleanupActions(cleanup, true);

  assert(actions.length === 0, "cleanup action remained after eject");
}

function ignoresDownloadedDmgTrashUntilPathProofExists() {
  const actions = visibleInstallerCleanupActions(
    {
      ...cleanup,
      mountedVolumePath: null,
      shouldOfferMountedVolumeEject: false,
    },
    false,
  );

  assert(actions.length === 0, "downloaded DMG Trash action was offered too early");
}

offersMountedVolumeEjectOnlyBeforeEject();
hidesCleanupAfterMountedVolumeEject();
ignoresDownloadedDmgTrashUntilPathProofExists();
