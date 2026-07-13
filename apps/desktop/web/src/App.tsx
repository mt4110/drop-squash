import { useCallback, useEffect, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { DropZone } from "./components/DropZone";
import { HelpPopover } from "./components/HelpPopover";
import { LicensePanel } from "./components/LicensePanel";
import { QueuePanel } from "./components/QueuePanel";
import { SettingsDrawer } from "./components/SettingsDrawer";
import { TrialBanner } from "./components/TrialBanner";
import { useConversionProgress } from "./hooks/useConversionProgress";
import { useRecordingDropEvents } from "./hooks/useRecordingDropEvents";
import type {
  ConversionSummary,
  ConvertRequest,
  DropZoneState,
  OutputSize,
  Profile,
  SavedConfig,
  SourceActionDecision,
  SourcePolicy,
} from "./lib/commands";
import { initialState } from "./lib/initialState";
import {
  blockQueuedJobs,
  cancelActiveQueueJob,
  cancelQueuedJob,
  clearCompletedQueueJobs,
  enqueueFiles,
  failActiveQueueJob,
  finishActiveQueueJob,
  startNextQueueJob,
} from "./lib/queueCommands";
import type { QueueEntry } from "./lib/queue";
import {
  blockQueued,
  cancelQueued,
  clearFinished,
  isCancelReason,
  markFailed,
  markRunning,
  markSourceAction,
  markSucceeded,
  nextQueued,
} from "./lib/queue";
import {
  encodeResultFromSummary,
  isFinishedQueueEvent,
  queueEntryFromRustItem,
  requestFromRustItem,
} from "./lib/queueWire";
import { savedConfigFromState, stateWithSavedConfigPatch } from "./lib/settings";

function lockedMessage(reason: DropZoneState["lockedReason"]) {
  return reason === "license-refresh-required"
    ? "Reconnect once with your license key to refresh Pro."
    : "Enter a license key to continue.";
}

export function App() {
  const [state, setState] = useState<DropZoneState>(initialState);
  const [result, setResult] = useState<ConversionSummary>();
  const [error, setError] = useState<string>();
  const [isBusy, setIsBusy] = useState(false);
  const [isDragging, setIsDragging] = useState(false);
  const [isHelpOpen, setIsHelpOpen] = useState(false);
  const [isTrashingOriginal, setIsTrashingOriginal] = useState(false);
  const [inputPath, setInputPath] = useState<string>();
  const [progress, setProgress] = useState<number>();
  const [queue, setQueue] = useState<QueueEntry[]>([]);
  const stateRef = useRef<DropZoneState>(initialState);
  const activeQueueId = useRef<number>();
  const currentLockedMessage = lockedMessage(state.lockedReason);

  useEffect(() => {
    stateRef.current = state;
  }, [state]);

  const refreshState = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    const nextState = await invoke<DropZoneState>("load_state");
    setState(nextState);
  }, []);

  const persistSettings = useCallback(async (settings: SavedConfig) => {
    if (!isTauri()) {
      return;
    }

    try {
      await invoke<SavedConfig>("save_config", settings);
    } catch (reason) {
      setError(String(reason));
    }
  }, []);

  const updateConfig = useCallback((patch: Partial<SavedConfig>) => {
    const nextState = stateWithSavedConfigPatch(stateRef.current, patch);
    stateRef.current = nextState;
    setState(nextState);
    void persistSettings(savedConfigFromState(nextState));
  }, [persistSettings]);

  const requestForInput = useCallback((path: string): ConvertRequest => ({
    inputPath: path,
    outputDir: stateRef.current.outputDir,
    profile: stateRef.current.profile,
    outputSize: stateRef.current.outputSize,
    sourcePolicy: stateRef.current.sourcePolicy,
    writePrivacyReceipt: stateRef.current.writePrivacyReceipt,
  }), []);

  const enqueueInputs = useCallback((inputPaths: string[]) => {
    if (!isTauri() || state.isLocked) {
      return;
    }

    void (async () => {
      const requests = inputPaths.map(requestForInput);
      const events = await enqueueFiles(requests);
      const entries = events.flatMap((event, index) => {
        if ("Enqueued" in event) {
          return [{
            ...queueEntryFromRustItem(event.Enqueued),
            writePrivacyReceipt: requests[index]?.writePrivacyReceipt,
          }];
        }
        return [];
      });
      if (entries.length > 0) {
        setError(undefined);
        setResult(undefined);
        setQueue((current) => [...current, ...entries]);
      }
    })().catch((reason) => setError(String(reason)));
  }, [requestForInput, state.isLocked]);

  const runQueued = useCallback(async (entry: QueueEntry) => {
    if (!isTauri() || isBusy || state.isLocked || activeQueueId.current) {
      return;
    }

    activeQueueId.current = entry.id;
    setInputPath(entry.inputPath);
    setIsBusy(true);
    setProgress(0);
    setError(undefined);
    setResult(undefined);
    setQueue((current) => markRunning(current, entry.id));
    try {
      const started = await startNextQueueJob();
      if (!started || !("Started" in started) || started.Started.id !== entry.id) {
        throw new Error("Queue state changed before conversion could start.");
      }
      const request = requestFromRustItem(
        started.Started,
        entry.writePrivacyReceipt ?? stateRef.current.writePrivacyReceipt,
      );
      const summary = await invoke<ConversionSummary>("convert", {
        request,
      });
      setResult(summary);
      const finished = await finishActiveQueueJob(encodeResultFromSummary(summary, request.profile));
      if (!isFinishedQueueEvent(finished, entry.id)) {
        throw new Error("Queue state did not finish the active conversion.");
      }
      setQueue((current) => markSucceeded(current, entry.id, summary));
      await refreshState();
    } catch (reason) {
      const status = isCancelReason(reason) ? "cancelled" : "failed";
      const message = String(reason);
      if (status === "cancelled") {
        await cancelActiveQueueJob();
      } else {
        await failActiveQueueJob(message);
      }
      setQueue((current) => markFailed(current, entry.id, status, message));
      if (status === "failed") {
        setError(message);
      }
    } finally {
      activeQueueId.current = undefined;
      setIsBusy(false);
      setProgress(undefined);
    }
  }, [isBusy, refreshState, state.isLocked]);
  useEffect(() => {
    if (isBusy || state.isLocked) {
      return;
    }

    const next = nextQueued(queue);
    if (next) {
      void runQueued(next);
    }
  }, [isBusy, queue, runQueued, state.isLocked]);

  useEffect(() => {
    if (state.isLocked) {
      void blockQueuedJobs(currentLockedMessage).catch((reason) => {
        setError(String(reason));
      });
      setQueue((current) => blockQueued(current, currentLockedMessage));
    }
  }, [currentLockedMessage, state.isLocked]);

  useConversionProgress({ activeQueueId, setProgress, setQueue });

  useRecordingDropEvents({
    enqueueInputs,
    refreshState,
    setError,
    setIsDragging,
  });

  const chooseRecording = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    const inputPath = await open({
      filters: [{ name: "Screen recordings", extensions: state.inputExtensions }],
      multiple: false,
      title: "Choose a recording",
    });
    if (typeof inputPath === "string") {
      enqueueInputs([inputPath]);
    }
  }, [enqueueInputs, state.inputExtensions]);

  const chooseOutputDirectory = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    const outputDir = await open({
      defaultPath: state.outputDir,
      directory: true,
      multiple: false,
      title: "Choose an output folder",
    });
    if (typeof outputDir === "string") {
      updateConfig({ outputDir });
    }
  }, [state.outputDir, updateConfig]);

  const changeProfile = useCallback((profile: Profile) => {
    updateConfig({ profile });
  }, [updateConfig]);

  const changeOutputSize = useCallback((outputSize: OutputSize) => {
    updateConfig({ outputSize });
  }, [updateConfig]);

  const changeSourcePolicy = useCallback((sourcePolicy: SourcePolicy) => {
    updateConfig({ sourcePolicy });
  }, [updateConfig]);

  const changeWritePrivacyReceipt = useCallback((writePrivacyReceipt: boolean) => {
    updateConfig({ writePrivacyReceipt });
  }, [updateConfig]);

  const revealOutput = useCallback(async (outputPath: string) => {
    if (!isTauri()) {
      return;
    }

    try {
      await revealItemInDir(outputPath);
    } catch (reason) {
      setError(String(reason));
    }
  }, []);

  const cancelConversion = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    try {
      await invoke<boolean>("cancel_conversion");
    } catch (reason) {
      setError(String(reason));
    }
  }, []);

  const trashOriginal = useCallback(async (sourcePath: string, outputPath: string) => {
    if (!isTauri()) {
      return;
    }

    setIsTrashingOriginal(true);
    try {
      const decision = await invoke<SourceActionDecision>("trash_original", {
        sourcePath,
        outputPath,
      });
      if (decision.action !== "move-original-to-trash") {
        setError(decision.reason);
        return;
      }
      setResult((current) => current ? { ...current, sourceAction: decision.action } : current);
      setQueue((current) => markSourceAction(current, outputPath, decision.action));
    } catch (reason) {
      setError(String(reason));
    } finally {
      setIsTrashingOriginal(false);
    }
  }, []);

  const activateLicense = useCallback(async (licenseKey: string) => {
    if (!isTauri()) {
      return;
    }

    try {
      const nextState = await invoke<DropZoneState>("activate_license", { licenseKey });
      setState(nextState);
      setError(undefined);
    } catch (reason) {
      setError(String(reason));
    }
  }, []);

  const forgetLicense = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    try {
      const nextState = await invoke<DropZoneState>("forget_license");
      setState(nextState);
      setError(undefined);
    } catch (reason) {
      setError(String(reason));
    }
  }, []);

  return (
    <main className={`shell${queue.length > 0 ? " has-queue" : ""}`}>
      <button aria-label="Show quick tips" className="help-button" title="Show quick tips" type="button" onClick={() => setIsHelpOpen(true)}>?</button>
      <TrialBanner
        successfulConversions={state.successfulConversions}
        trialLimit={state.trialLimit}
        isPro={state.isPro}
        isLocked={state.isLocked}
        lockedMessage={currentLockedMessage}
      />
      <LicensePanel
        isPro={state.isPro}
        onActivate={activateLicense}
        onForget={forgetLicense}
      />
      <DropZone
        isBusy={isBusy}
        isDragging={isDragging}
        isLocked={state.isLocked}
        lockedMessage={currentLockedMessage}
        progress={progress}
        result={result}
        error={error}
        inputPath={inputPath}
        inputExtensions={state.inputExtensions}
        isTrashingOriginal={isTrashingOriginal}
        onPick={() => void chooseRecording()}
        onCancel={() => void cancelConversion()}
        onRevealOutput={(outputPath) => void revealOutput(outputPath)}
        onRevealReceipt={(receiptPath) => void revealOutput(receiptPath)}
        onTrashOriginal={(sourcePath, outputPath) => void trashOriginal(sourcePath, outputPath)}
      />
      <SettingsDrawer
        outputDir={state.outputDir}
        profile={state.profile}
        outputSize={state.outputSize}
        sourcePolicy={state.sourcePolicy}
        writePrivacyReceipt={state.writePrivacyReceipt}
        profiles={state.profiles}
        outputSizes={state.outputSizes}
        sourcePolicies={state.sourcePolicies}
        onChooseOutput={() => void chooseOutputDirectory()}
        onProfileChange={changeProfile}
        onOutputSizeChange={changeOutputSize}
        onSourcePolicyChange={changeSourcePolicy}
        onWritePrivacyReceiptChange={changeWritePrivacyReceipt}
      />
      <QueuePanel
        items={queue}
        onCancelQueued={(id) => {
          void cancelQueuedJob(id)
            .then((event) => {
              if (event && "Cancelled" in event) {
                setQueue((current) => cancelQueued(current, id));
              }
            })
            .catch((reason) => setError(String(reason)));
        }}
        onClearFinished={() => {
          void clearCompletedQueueJobs()
            .then(() => setQueue((current) => clearFinished(current)))
            .catch((reason) => setError(String(reason)));
        }}
        onRevealOutput={(outputPath) => void revealOutput(outputPath)}
        onRevealReceipt={(receiptPath) => void revealOutput(receiptPath)}
      />
      {isHelpOpen && <HelpPopover onClose={() => setIsHelpOpen(false)} />}
    </main>
  );
}
