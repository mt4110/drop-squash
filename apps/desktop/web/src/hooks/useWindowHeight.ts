import { useLayoutEffect, type RefObject } from "react";
import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import {
  WINDOW_MIN_HEIGHT,
  WINDOW_WIDTH,
} from "../lib/windowHeight";

export function useWindowHeight(
  _targetRef: RefObject<HTMLElement | null>,
  _deps: readonly unknown[],
) {
  useLayoutEffect(() => {
    if (!isTauri()) {
      return;
    }
    void getCurrentWindow()
      .setMinSize(new LogicalSize(WINDOW_WIDTH, WINDOW_MIN_HEIGHT))
      .catch(() => undefined);
  }, []);
}
