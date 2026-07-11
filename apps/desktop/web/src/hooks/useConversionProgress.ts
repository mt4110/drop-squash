import { useEffect, type Dispatch, type MutableRefObject, type SetStateAction } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { QueueEntry } from "../lib/queue";
import { updateProgress } from "../lib/queue";

type UseConversionProgressProps = {
  activeQueueId: MutableRefObject<number | undefined>;
  setProgress: Dispatch<SetStateAction<number | undefined>>;
  setQueue: Dispatch<SetStateAction<QueueEntry[]>>;
};

export function useConversionProgress({
  activeQueueId,
  setProgress,
  setQueue,
}: UseConversionProgressProps) {
  useEffect(() => {
    if (!isTauri()) {
      return;
    }

    let unlisten: (() => void) | undefined;
    void listen<number>("conversion-progress", (event) => {
      setProgress(event.payload);
      const id = activeQueueId.current;
      if (id) {
        setQueue((current) => updateProgress(current, id, event.payload));
      }
    }).then((listener) => {
      unlisten = listener;
    });

    return () => unlisten?.();
  }, [activeQueueId, setProgress, setQueue]);
}
