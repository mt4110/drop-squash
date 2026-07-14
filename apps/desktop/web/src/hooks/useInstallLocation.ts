import { useEffect, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import type { InstallLocation } from "../lib/commands";

export function useInstallLocation(onError: (message: string) => void) {
  const [installLocation, setInstallLocation] = useState<InstallLocation>();

  useEffect(() => {
    if (!isTauri()) {
      return;
    }

    void invoke<InstallLocation>("load_install_location")
      .then(setInstallLocation)
      .catch((reason) => onError(String(reason)));
  }, [onError]);

  return installLocation;
}
