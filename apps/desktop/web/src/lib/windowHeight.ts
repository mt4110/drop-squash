export const WINDOW_WIDTH = 480;
export const WINDOW_MIN_HEIGHT = 520;
const WINDOW_PADDING = 96;
const WINDOW_SCREEN_MARGIN = 20;

export function nextWindowHeight(
  rootHeight: number,
  childBottoms: readonly number[],
  scrollHeight: number,
  screenHeight = Number.POSITIVE_INFINITY,
) {
  const contentHeight = Math.max(rootHeight, ...childBottoms, scrollHeight);
  const desiredHeight = Math.max(WINDOW_MIN_HEIGHT, contentHeight + WINDOW_PADDING);
  const availableHeight = Math.max(WINDOW_MIN_HEIGHT, screenHeight - WINDOW_SCREEN_MARGIN);
  return Math.ceil(Math.min(desiredHeight, availableHeight));
}
