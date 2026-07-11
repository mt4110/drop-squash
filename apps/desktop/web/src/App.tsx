import { useCallback, useEffect, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open } from "@tauri-apps/plugin-dialog";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { DropZone } from "./components/DropZone";
import { HelpPopover } from "./components/HelpPopover";
import { LicensePanel } from "./components/LicensePanel";
import { QueuePanel } from "./components/QueuePanel";
import { SettingsDrawer } from "./components/SettingsDrawer";
import { TrialBanner } from "./components/TrialBanner";
import type {
  ConversionSummary,
  DropZoneState,
  OutputSize,
  Profile,
  SavedConfig,
  SourceActionDecision,
  SourcePolicy,
} from "./lib/commands";
import type { QueueEntry } from "./lib/queue";
import { isCancelReason } from "./lib/queue";

const initialState: DropZoneState = {
  productName: "DropSquash",
  outputDir: "~/Movies/DropSquash",
  profile: "auto",
  outputSize: "auto",
  profiles: [
    { value: "auto", label: "Auto" },
    { value: "slack", label: "Slack" },
    { value: "teams", label: "Microsoft Teams" },
    { value: "discord", label: "Discord" },
    { value: "chatwork", label: "Chatwork" },
    { value: "line", label: "LINE" },
    { value: "whatsapp", label: "WhatsApp" },
    { value: "docs", label: "Docs" },
  ],
  outputSizes: [
    { value: "auto", label: "Auto" },
    { value: "1080p", label: "1920 x 1080" },
    { value: "720p", label: "1280 x 720" },
    { value: "480p", label: "640 x 480" },
  ],
  inputExtensions: ["mov", "mp4", "m4v"],
  sourcePolicy: "ask",
  sourcePolicies: [
    { value: "ask", label: "Ask after saving" },
    { value: "keep", label: "Keep original" },
    { value: "trash", label: "Move to Trash" },
  ],
  privacyMode: "local-only",
  successfulConversions: 0,
  trialLimit: 10,
  isPro: false,
  isLocked: false,
};

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

  const persistSettings = useCallback(async (settings: {
    outputDir: string;
    profile: Profile;
    outputSize: OutputSize;
    sourcePolicy: SourcePolicy;
  }) => {
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

    const entries = inputPaths.map((inputPath) => ({
      id: nextQueueId.current++,
      inputPath,
      status: "queued" as const,
    }));
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
    setQueue((current) => current.map((item) => (
      item.id === entry.id ? { ...item, status: "running", progress: 0 } : item
    )));
    try {
      const summary = await invoke<ConversionSummary>("convert", {
        inputPath: entry.inputPath,
        outputDir: state.outputDir,
        profile: state.profile,
        outputSize: state.outputSize,
        sourcePolicy: state.sourcePolicy,
      });
      setResult(summary);
      setQueue((current) => current.map((item) => (
        item.id === entry.id ? { ...item, status: "succeeded", progress: 100, result: summary } : item
      )));
      setState((current) => ({
        ...current,
        successfulConversions: current.successfulConversions + 1,
        isLocked: current.successfulConversions + 1 >= current.trialLimit,
      }));
    } catch (reason) {
      const status = isCancelReason(reason) ? "cancelled" : "failed";
      const message = String(reason);
      setQueue((current) => current.map((item) => (
        item.id === entry.id ? { ...item, status, error: message } : item
      )));
      if (status === "failed") {
        setError(message);
      }
    } finally {
      activeQueueId.current = undefined;
      setIsBusy(false);
      setProgress(undefined);
    }
  }, [isBusy, state.isLocked, state.outputDir, state.outputSize, state.profile, state.sourcePolicy]);
  const enqueueRef = useRef(enqueueInputs);

  useEffect(() => {
    enqueueRef.current = enqueueInputs;
  }, [enqueueInputs]);

  useEffect(() => {
    if (isBusy || state.isLocked) {
      return;
    }

    const next = queue.find((item) => item.status === "queued");
    if (next) {
      void runQueued(next);
    }
  }, [isBusy, queue, runQueued, state.isLocked]);

  useEffect(() => {
    if (!isTauri()) {
      return;
    }

    let unlisten: (() => void) | undefined;
    void listen<number>("conversion-progress", (event) => {
      setProgress(event.payload);
      const id = activeQueueId.current;
      if (id) {
        setQueue((current) => current.map((item) => (
          item.id === id ? { ...item, progress: event.payload } : item
        )));
      }
    }).then((listener) => {
      unlisten = listener;
    });

    return () => unlisten?.();
  }, []);
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
      await persistSettings({
        outputDir,
        profile: state.profile,
        outputSize: state.outputSize,
        sourcePolicy: state.sourcePolicy,
      });
    }
  }, [persistSettings, state.outputDir, state.outputSize, state.profile, state.sourcePolicy]);

  const changeProfile = useCallback((profile: Profile) => {
    setState((current) => ({ ...current, profile }));
    void persistSettings({
      outputDir: state.outputDir,
      profile,
      outputSize: state.outputSize,
      sourcePolicy: state.sourcePolicy,
    });
  }, [persistSettings, state.outputDir, state.outputSize, state.sourcePolicy]);

  const changeOutputSize = useCallback((outputSize: OutputSize) => {
    setState((current) => ({ ...current, outputSize }));
    void persistSettings({
      outputDir: state.outputDir,
      profile: state.profile,
      outputSize,
      sourcePolicy: state.sourcePolicy,
    });
  }, [persistSettings, state.outputDir, state.profile, state.sourcePolicy]);

  const changeSourcePolicy = useCallback((sourcePolicy: SourcePolicy) => {
    setState((current) => ({ ...current, sourcePolicy }));
    void persistSettings({
      outputDir: state.outputDir,
      profile: state.profile,
      outputSize: state.outputSize,
      sourcePolicy,
    });
  }, [persistSettings, state.outputDir, state.outputSize, state.profile]);

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
      setQueue((current) => current.map((item) => (
        item.result?.outputPath === outputPath
          ? { ...item, result: { ...item.result, sourceAction: decision.action } }
          : item
      )));
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

  useEffect(() => {
    if (!isTauri()) {
      return;
    }

    void invoke<DropZoneState>("load_state").then(setState).catch((reason) => setError(String(reason)));

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
        onTrashOriginal={(sourcePath, outputPath) => void trashOriginal(sourcePath, outputPath)}
      />
      <SettingsDrawer
        outputDir={state.outputDir}
        profile={state.profile}
        outputSize={state.outputSize}
        sourcePolicy={state.sourcePolicy}
        profiles={state.profiles}
        outputSizes={state.outputSizes}
        sourcePolicies={state.sourcePolicies}
        onChooseOutput={() => void chooseOutputDirectory()}
        onProfileChange={changeProfile}
        onOutputSizeChange={changeOutputSize}
        onSourcePolicyChange={changeSourcePolicy}
      />
      <QueuePanel
        items={queue}
        onRevealOutput={(outputPath) => void revealOutput(outputPath)}
      />
      {isHelpOpen && <HelpPopover onClose={() => setIsHelpOpen(false)} />}
    </main>
  );
}
