import { useEffect, useRef } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { userErrorMessage } from "../lib/errorMessage";
import { createInputGate } from "../lib/inputGate";
import { recordManualQaEvent } from "../lib/manualQa";

type UseRecordingDropEventsProps = {
  enqueueInputs: (inputPaths: string[]) => void;
  refreshState: () => Promise<void>;
  setError: (error: string | undefined) => void;
  setIsDragging: (isDragging: boolean) => void;
};

export function useRecordingDropEvents({
  enqueueInputs,
  refreshState,
  setError,
  setIsDragging,
}: UseRecordingDropEventsProps) {
  const enqueueRef = useRef(enqueueInputs);
  const dragStateRef = useRef<string | undefined>(undefined);

  useEffect(() => {
    enqueueRef.current = enqueueInputs;
  }, [enqueueInputs]);

  useEffect(() => {
    if (!isTauri()) {
      return;
    }

    const inputGate = createInputGate();
    void refreshState()
      .catch((reason) => setError(userErrorMessage(reason)))
      .finally(() => inputGate.markReady((paths) => enqueueRef.current(paths)));

    let unlisten: (() => void) | undefined;
    let unlistenOpened: (() => void) | undefined;
    let isDisposed = false;
    void getCurrentWebview().onDragDropEvent((event) => {
      const detail = event.payload.type === "drop"
        ? `${event.payload.type}:${event.payload.paths.length}`
        : event.payload.type;
      if (dragStateRef.current !== detail) {
        dragStateRef.current = detail;
        recordManualQaEvent("drag", detail);
      }
      if (event.payload.type === "enter" || event.payload.type === "over") {
        setIsDragging(true);
      } else {
        setIsDragging(false);
      }

      if (event.payload.type === "drop") {
        inputGate.submit(event.payload.paths, (paths) => enqueueRef.current(paths));
      }
    }).then((listener) => {
      if (isDisposed) {
        listener();
      } else {
        unlisten = listener;
      }
    }).catch((reason) => {
      setError(`Drag and drop is unavailable: ${userErrorMessage(reason)}`);
    });
    void listen<string[]>("native-opened", (event) => {
      recordManualQaEvent("native-opened", String(event.payload.length));
      inputGate.submit(event.payload, (paths) => enqueueRef.current(paths));
    }).then((listener) => {
      if (isDisposed) {
        listener();
      } else {
        unlistenOpened = listener;
      }
    }).catch((reason) => {
      setError(`Open file handling is unavailable: ${userErrorMessage(reason)}`);
    });

    return () => {
      isDisposed = true;
      unlisten?.();
      unlistenOpened?.();
    };
  }, [refreshState, setError, setIsDragging]);
}
