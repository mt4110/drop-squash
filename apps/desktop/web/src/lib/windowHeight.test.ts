import {
  WINDOW_MIN_HEIGHT,
  nextWindowHeight,
} from "./windowHeight.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

assert(
  nextWindowHeight(200, [240, 260], 250) === WINDOW_MIN_HEIGHT,
  "short content should keep the minimum height",
);
assert(
  nextWindowHeight(320, [402], 360) === 520,
  "a lower control should grow the window enough to stay visible",
);
assert(
  nextWindowHeight(340, [360], 421) === 520,
  "scroll height should protect content that extends past the last measured child",
);
assert(
  nextWindowHeight(320, [450], 360) === 546,
  "taller lower panels should add enough breathing room instead of stopping at a cramped fit",
);
