import { createInputGate } from "./inputGate.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

function buffersUntilReadyThenFlushesInOrder() {
  const gate = createInputGate();
  const seen: string[] = [];
  const flush = (paths: string[]) => seen.push(paths.join(","));
  gate.submit(["/tmp/one.mov"], flush);
  gate.submit(["/tmp/two.mov", "/tmp/three.mov"], flush);

  assert(seen.length === 0, "gate should buffer inputs before ready");

  gate.markReady(flush);

  assert(seen[0] === "/tmp/one.mov", "first buffered event changed order");
  assert(
    seen[1] === "/tmp/two.mov,/tmp/three.mov",
    "second buffered event changed order",
  );
}

function passesThroughAfterReady() {
  const gate = createInputGate();
  const seen: string[] = [];
  const flush = (paths: string[]) => seen.push(paths.join(","));
  gate.markReady(flush);
  gate.submit(["/tmp/four.mov"], flush);

  assert(seen.length === 1, "ready gate should pass through immediately");
  assert(seen[0] === "/tmp/four.mov", "ready gate changed input");
}

buffersUntilReadyThenFlushesInOrder();
passesThroughAfterReady();
