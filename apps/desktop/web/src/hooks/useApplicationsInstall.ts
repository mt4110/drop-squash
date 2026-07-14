import { useCallback, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { useInstallLocation } from "./useInstallLocation";
import type { ApplicationsInstall } from "../lib/commands";

type ApplicationsInstallState = {
  appPath?: string;
  installedPath?: string;
  isMoving: boolean;
  isOpeningInstalledApp: boolean;
  didOpenInstalledApp: boolean;
  didCopyToApplications: boolean;
  shouldShowNotice: boolean;
  dismissNotice: () => void;
  moveToApplications: () => Promise<void>;
  openInstalledApp: () => Promise<void>;
  quitCurrentApp: () => Promise<void>;
};

export function useApplicationsInstall(
  onError: (message: string) => void,
  onReady: () => void,
): ApplicationsInstallState {
  const [isDismissed, setIsDismissed] = useState(false);
  const [isMoving, setIsMoving] = useState(false);
  const [isOpeningInstalledApp, setIsOpeningInstalledApp] = useState(false);
  const [didOpenInstalledApp, setDidOpenInstalledApp] = useState(false);
  const [installedPath, setInstalledPath] = useState<string>();
  const installLocation = useInstallLocation(onError);
  const shouldShowNotice =
    (Boolean(installLocation?.shouldOfferApplicationsMove) || Boolean(installedPath)) &&
    !isDismissed;

  const moveToApplications = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    setIsMoving(true);
    try {
      const install = await invoke<ApplicationsInstall>("copy_to_applications");
      await revealItemInDir(install.targetPath);
      setInstalledPath(install.targetPath);
      setDidOpenInstalledApp(false);
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
    didOpenInstalledApp,
    didCopyToApplications: Boolean(installedPath),
    shouldShowNotice,
    dismissNotice: () => setIsDismissed(true),
    moveToApplications,
    openInstalledApp,
    quitCurrentApp,
  };
}
