export type InputGate = {
  markReady: (flush: (paths: string[]) => void) => void;
  submit: (paths: string[], flush: (paths: string[]) => void) => void;
};

export function createInputGate(): InputGate {
  let isReady = false;
  const pending: string[][] = [];
  return {
    markReady(flush) {
      isReady = true;
      pending.splice(0).forEach(flush);
    },
    submit(paths, flush) {
      if (isReady) {
        flush(paths);
        return;
      }
      pending.push(paths);
    },
  };
}
