import { useEffect, useRef } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";

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

  useEffect(() => {
    enqueueRef.current = enqueueInputs;
  }, [enqueueInputs]);

  useEffect(() => {
    if (!isTauri()) {
      return;
    }

    void refreshState().catch((reason) => setError(String(reason)));

    let unlisten: (() => void) | undefined;
    let isDisposed = false;
    void getCurrentWebview().onDragDropEvent((event) => {
      if (event.payload.type === "enter" || event.payload.type === "over") {
        setIsDragging(true);
      } else {
        setIsDragging(false);
      }

      if (event.payload.type === "drop") {
        enqueueRef.current(event.payload.paths);
      }
    }).then((listener) => {
      if (isDisposed) {
        listener();
      } else {
        unlisten = listener;
      }
    }).catch((reason) => {
      setError(`Drag and drop is unavailable: ${String(reason)}`);
    });

    return () => {
      isDisposed = true;
      unlisten?.();
    };
  }, [refreshState, setError, setIsDragging]);
}
