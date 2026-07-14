import { useCallback, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { useInstallLocation } from "./useInstallLocation";
import type { ApplicationsInstall } from "../lib/commands";

type ApplicationsInstallState = {
  appPath?: string;
  isMoving: boolean;
  shouldShowNotice: boolean;
  dismissNotice: () => void;
  moveToApplications: () => Promise<void>;
};

export function useApplicationsInstall(
  onError: (message: string) => void,
  onReady: () => void,
): ApplicationsInstallState {
  const [isDismissed, setIsDismissed] = useState(false);
  const [isMoving, setIsMoving] = useState(false);
  const installLocation = useInstallLocation(onError);
  const shouldShowNotice =
    Boolean(installLocation?.shouldOfferApplicationsMove) && !isDismissed;

  const moveToApplications = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    setIsMoving(true);
    try {
      const install = await invoke<ApplicationsInstall>("copy_to_applications");
      await revealItemInDir(install.targetPath);
      setIsDismissed(true);
      onReady();
    } catch (reason) {
      onError(String(reason));
    } finally {
      setIsMoving(false);
    }
  }, [onError, onReady]);

  return {
    appPath: installLocation?.appPath,
    isMoving,
    shouldShowNotice,
    dismissNotice: () => setIsDismissed(true),
    moveToApplications,
  };
}
