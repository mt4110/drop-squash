import { useCallback, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { useInstallLocation } from "./useInstallLocation";
import type { ApplicationsInstall, InstallerCleanup } from "../lib/commands";

type ApplicationsInstallState = {
  appPath?: string;
  installedPath?: string;
  isMoving: boolean;
  isOpeningInstalledApp: boolean;
  isEjectingInstallerVolume: boolean;
  didOpenInstalledApp: boolean;
  didEjectInstallerVolume: boolean;
  didCopyToApplications: boolean;
  shouldOfferInstallerVolumeEject: boolean;
  shouldShowNotice: boolean;
  dismissNotice: () => void;
  moveToApplications: () => Promise<void>;
  openInstalledApp: () => Promise<void>;
  ejectInstallerVolume: () => Promise<void>;
  quitCurrentApp: () => Promise<void>;
};

export function useApplicationsInstall(
  onError: (message: string) => void,
  onReady: () => void,
): ApplicationsInstallState {
  const [isDismissed, setIsDismissed] = useState(false);
  const [isMoving, setIsMoving] = useState(false);
  const [isOpeningInstalledApp, setIsOpeningInstalledApp] = useState(false);
  const [isEjectingInstallerVolume, setIsEjectingInstallerVolume] = useState(false);
  const [didOpenInstalledApp, setDidOpenInstalledApp] = useState(false);
  const [didEjectInstallerVolume, setDidEjectInstallerVolume] = useState(false);
  const [installedPath, setInstalledPath] = useState<string>();
  const [cleanup, setCleanup] = useState<InstallerCleanup>();
  const installLocation = useInstallLocation(onError);
  const shouldShowNotice =
    (Boolean(installLocation?.shouldOfferApplicationsMove) || Boolean(installedPath)) &&
    !isDismissed;
  const shouldOfferInstallerVolumeEject =
    Boolean(cleanup?.shouldOfferMountedVolumeEject) &&
    Boolean(cleanup?.mountedVolumePath) &&
    !didEjectInstallerVolume;

  const moveToApplications = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    setIsMoving(true);
    try {
      const install = await invoke<ApplicationsInstall>("copy_to_applications");
      await revealItemInDir(install.targetPath);
      setInstalledPath(install.targetPath);
      setCleanup(install.cleanup);
      setDidOpenInstalledApp(false);
      setDidEjectInstallerVolume(false);
      setIsDismissed(false);
      onReady();
    } catch (reason) {
      onError(String(reason));
    } finally {
      setIsMoving(false);
    }
  }, [onError, onReady]);

  const openInstalledApp = useCallback(async () => {
    if (!isTauri() || !installedPath) {
      return;
    }

    setIsOpeningInstalledApp(true);
    try {
      await invoke("open_installed_application", { installedAppPath: installedPath });
      setDidOpenInstalledApp(true);
      onReady();
    } catch (reason) {
      onError(String(reason));
    } finally {
      setIsOpeningInstalledApp(false);
    }
  }, [installedPath, onError, onReady]);

  const ejectInstallerVolume = useCallback(async () => {
    if (!isTauri() || !cleanup?.mountedVolumePath) {
      return;
    }

    setIsEjectingInstallerVolume(true);
    try {
      await invoke("eject_installer_volume", {
        mountedVolumePath: cleanup.mountedVolumePath,
      });
      setDidEjectInstallerVolume(true);
      onReady();
    } catch (reason) {
      onError(String(reason));
    } finally {
      setIsEjectingInstallerVolume(false);
    }
  }, [cleanup?.mountedVolumePath, onError, onReady]);

  const quitCurrentApp = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    try {
      await invoke("quit_current_app");
    } catch (reason) {
      onError(String(reason));
    }
  }, [onError]);

  return {
    appPath: installLocation?.appPath,
    installedPath,
    isMoving,
    isOpeningInstalledApp,
    isEjectingInstallerVolume,
    didOpenInstalledApp,
    didEjectInstallerVolume,
    didCopyToApplications: Boolean(installedPath),
    shouldOfferInstallerVolumeEject,
    shouldShowNotice,
    dismissNotice: () => setIsDismissed(true),
    moveToApplications,
    openInstalledApp,
    ejectInstallerVolume,
    quitCurrentApp,
  };
}
