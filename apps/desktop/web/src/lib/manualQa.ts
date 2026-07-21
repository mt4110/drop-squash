import { invoke, isTauri } from "@tauri-apps/api/core";

export function recordManualQaEvent(kind: string, detail: string) {
  if (!isTauri()) {
    return;
  }

  void invoke("record_manual_qa_event", { kind, detail }).catch(() => undefined);
}
