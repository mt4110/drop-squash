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
  DropZoneState,
  OutputSize,
  Profile,
  SavedConfig,
  SourceActionDecision,
  SourcePolicy,
} from "./lib/commands";
import { initialState } from "./lib/initialState";
import type { QueueEntry } from "./lib/queue";
import {
  blockQueued,
  clearFinished,
  entriesForInputPaths,
  isCancelReason,
  markFailed,
  markRunning,
  markSourceAction,
  markSucceeded,
  nextQueued,
} from "./lib/queue";
import { savedConfigWith } from "./lib/settings";

export function App() {
  const [state, setState] = useState<DropZoneState>(initialState);
  const [result, setResult] = useState<ConversionSummary>();
  const [error, setError] = useState<string>();
  const [isBusy, setIsBusy] = useState(false);
  const [isDragging, setIsDragging] = useState(false);
  const [isHelpOpen, setIsHelpOpen] = useState(false);
  const [inputPath, setInputPath] = useState<string>();
  const [progress, setProgress] = useState<number>();
  const [queue, setQueue] = useState<QueueEntry[]>([]);
  const activeQueueId = useRef<number>();
  const nextQueueId = useRef(1);

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

  const enqueueInputs = useCallback((inputPaths: string[]) => {
    if (!isTauri() || state.isLocked) {
      return;
    }

    const queued = entriesForInputPaths(inputPaths, nextQueueId.current);
    nextQueueId.current = queued.nextId;
    const { entries } = queued;
    if (entries.length > 0) {
      setError(undefined);
      setResult(undefined);
      setQueue((current) => [...current, ...entries]);
    }
  }, [state.isLocked]);

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
      const summary = await invoke<ConversionSummary>("convert", {
        request: {
          inputPath: entry.inputPath,
          outputDir: state.outputDir,
          profile: state.profile,
          outputSize: state.outputSize,
          sourcePolicy: state.sourcePolicy,
          writePrivacyReceipt: state.writePrivacyReceipt,
        },
      });
      setResult(summary);
      setQueue((current) => markSucceeded(current, entry.id, summary));
      await refreshState();
    } catch (reason) {
      const status = isCancelReason(reason) ? "cancelled" : "failed";
      const message = String(reason);
      setQueue((current) => markFailed(current, entry.id, status, message));
      if (status === "failed") {
        setError(message);
      }
    } finally {
      activeQueueId.current = undefined;
      setIsBusy(false);
      setProgress(undefined);
    }
  }, [isBusy, refreshState, state.isLocked, state.outputDir, state.outputSize, state.profile, state.sourcePolicy, state.writePrivacyReceipt]);
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
      setQueue((current) => (
        blockQueued(current, "Trial complete. Enter a license key to continue.")
      ));
    }
  }, [state.isLocked]);

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
      setState((current) => ({ ...current, outputDir }));
      await persistSettings(savedConfigWith(state, { outputDir }));
    }
  }, [persistSettings, state]);

  const changeProfile = useCallback((profile: Profile) => {
    setState((current) => ({ ...current, profile }));
    void persistSettings(savedConfigWith(state, { profile }));
  }, [persistSettings, state]);

  const changeOutputSize = useCallback((outputSize: OutputSize) => {
    setState((current) => ({ ...current, outputSize }));
    void persistSettings(savedConfigWith(state, { outputSize }));
  }, [persistSettings, state]);

  const changeSourcePolicy = useCallback((sourcePolicy: SourcePolicy) => {
    setState((current) => ({ ...current, sourcePolicy }));
    void persistSettings(savedConfigWith(state, { sourcePolicy }));
  }, [persistSettings, state]);

  const changeWritePrivacyReceipt = useCallback((writePrivacyReceipt: boolean) => {
    setState((current) => ({ ...current, writePrivacyReceipt }));
    void persistSettings(savedConfigWith(state, { writePrivacyReceipt }));
  }, [persistSettings, state]);

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

  const deactivateLicense = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    try {
      const nextState = await invoke<DropZoneState>("deactivate_license");
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
      />
      <LicensePanel
        isPro={state.isPro}
        isLocked={state.isLocked}
        onActivate={activateLicense}
        onDeactivate={deactivateLicense}
      />
      <DropZone
        isBusy={isBusy}
        isDragging={isDragging}
        isLocked={state.isLocked}
        progress={progress}
        result={result}
        error={error}
        inputPath={inputPath}
        inputExtensions={state.inputExtensions}
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
        onClearFinished={() => setQueue((current) => clearFinished(current))}
        onRevealOutput={(outputPath) => void revealOutput(outputPath)}
        onRevealReceipt={(receiptPath) => void revealOutput(receiptPath)}
      />
      {isHelpOpen && <HelpPopover onClose={() => setIsHelpOpen(false)} />}
    </main>
  );
}
