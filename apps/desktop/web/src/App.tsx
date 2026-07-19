import { useCallback, useEffect, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { DropZone } from "./components/DropZone";
import { HelpPopover } from "./components/HelpPopover";
import { InstallNotice } from "./components/InstallNotice";
import { LicensePanel } from "./components/LicensePanel";
import { QueuePanel } from "./components/QueuePanel";
import { SettingsDrawer } from "./components/SettingsDrawer";
import { SecureShareAlpha } from "./components/SecureShareAlpha";
import { TrialBanner } from "./components/TrialBanner";
import { useApplicationsInstall } from "./hooks/useApplicationsInstall";
import { useConversionProgress } from "./hooks/useConversionProgress";
import { useRecordingDropEvents } from "./hooks/useRecordingDropEvents";
import { useWindowHeight } from "./hooks/useWindowHeight";
import type {
  ConversionOutcome,
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
  unchangedActiveQueueJob,
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
  markUnchanged,
  nextQueued,
} from "./lib/queue";
import {
  encodeResultFromSummary,
  isFinishedQueueEvent,
  queueEntryFromRustItem,
  requestFromRustItem,
} from "./lib/queueWire";
import { isLockedMessage, lockedMessage } from "./lib/licenseLock";
import { savedConfigFromState, stateWithSavedConfigPatch } from "./lib/settings";
import { convertedSummary, keptOriginalMessage } from "./lib/conversionOutcomeWire";
import { normalizeConversionSummary } from "./lib/conversionSummaryWire";
import { applySourceActionDecision, sourceActionError } from "./lib/sourceAction";
import { isNotSmallerMessage, userErrorMessage } from "./lib/errorMessage";

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
  const shellRef = useRef<HTMLElement>(null);
  const stateRef = useRef<DropZoneState>(initialState);
  const activeQueueId = useRef<number | undefined>(undefined);
  const showError = useCallback((reason: unknown) => {
    setError(userErrorMessage(reason));
  }, []);
  const currentLockedMessage = lockedMessage(state.lockedReason);
  const applicationsInstall = useApplicationsInstall(showError, () => setError(undefined));
  useWindowHeight(shellRef, [
    state.isPro,
    state.isLocked,
    applicationsInstall.shouldShowNotice,
    queue.length,
    Boolean(error),
    Boolean(result?.receiptPath ?? result?.privacyReceiptPath),
    result?.sourceAction ?? "keep-original",
  ]);

  useEffect(() => {
    stateRef.current = state;
  }, [state]);

  const loadState = useCallback(async () => {
    if (!isTauri()) {
      return undefined;
    }

    const nextState = await invoke<DropZoneState>("load_state");
    setState(nextState);
    return nextState;
  }, []);

  const refreshState = useCallback(async () => { await loadState(); }, [loadState]);
  const persistSettings = useCallback(async (settings: SavedConfig) => {
    if (!isTauri()) {
      return;
    }

    try {
      await invoke<SavedConfig>("save_config", settings);
    } catch (reason) {
      showError(reason);
    }
  }, [showError]);

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
    })().catch(showError);
  }, [requestForInput, showError, state.isLocked]);

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
      const outcome = await invoke<ConversionOutcome>("convert", {
        request,
      });
      const unchangedMessage = keptOriginalMessage(outcome);
      if (unchangedMessage) {
        await unchangedActiveQueueJob(unchangedMessage);
        setQueue((current) => markUnchanged(current, entry.id, unchangedMessage));
        await refreshState();
        return;
      }
      const summary = convertedSummary(outcome);
      setResult(summary);
      const finished = await finishActiveQueueJob(encodeResultFromSummary(summary, request.profile));
      if (!isFinishedQueueEvent(finished, entry.id)) {
        throw new Error("Queue state did not finish the active conversion.");
      }
      setQueue((current) => markSucceeded(current, entry.id, summary));
      await refreshState();
    } catch (reason) {
      const status = isCancelReason(reason) ? "cancelled" : "failed";
      const message = userErrorMessage(reason);
      if (status === "cancelled") {
        await cancelActiveQueueJob();
        setQueue((current) => markFailed(current, entry.id, status, message));
      } else {
        await failActiveQueueJob(message);
        const nextState = await loadState();
        const queueMessage = nextState?.isLocked ? lockedMessage(nextState.lockedReason) : message;
        await (isLockedMessage(queueMessage) ? blockQueuedJobs(queueMessage) : Promise.resolve());
        setQueue((current) => {
          const failed = markFailed(current, entry.id, status, message);
          return isLockedMessage(queueMessage) ? blockQueued(failed, queueMessage) : failed;
        });
        if (isLockedMessage(queueMessage)) {
          setError(undefined);
          return;
        }
        setError(isNotSmallerMessage(message) ? undefined : message);
      }
    } finally {
      activeQueueId.current = undefined;
      setIsBusy(false);
      setProgress(undefined);
    }
  }, [isBusy, loadState, state.isLocked]);
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
      void blockQueuedJobs(currentLockedMessage).catch(showError);
      setQueue((current) => blockQueued(current, currentLockedMessage));
    }
  }, [currentLockedMessage, showError, state.isLocked]);

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
      title: "録画を選ぶ / Choose a recording",
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
      title: "保存先を選ぶ / Choose an output folder",
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
      await invoke("reveal_finder_item", { path: outputPath });
    } catch (reason) {
      showError(reason);
    }
  }, [showError]);

  const cancelConversion = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    try {
      await invoke<boolean>("cancel_conversion");
    } catch (reason) {
      showError(reason);
    }
  }, [showError]);

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
      setResult((current) => applySourceActionDecision(current, outputPath, decision));
      setQueue((current) => markSourceAction(current, outputPath, decision.action));
      const decisionError = sourceActionError(decision);
      if (decisionError) {
        setError(decisionError);
        return;
      }
    } catch (reason) {
      showError(reason);
    } finally {
      setIsTrashingOriginal(false);
    }
  }, [showError]);

  const activateLicense = useCallback(async (licenseKey: string) => {
    if (!isTauri()) {
      return;
    }

    try {
      const nextState = await invoke<DropZoneState>("activate_license", { licenseKey });
      setState(nextState);
      setError(undefined);
    } catch (reason) {
      showError(reason);
    }
  }, [showError]);

  const forgetLicense = useCallback(async () => {
    if (!isTauri()) {
      return;
    }

    try {
      const nextState = await invoke<DropZoneState>("forget_license");
      setState(nextState);
      setError(undefined);
    } catch (reason) {
      showError(reason);
    }
  }, [showError]);

  const shellClassName = ["shell", state.isPro ? "is-pro" : "", queue.length > 0 ? "has-queue" : "", applicationsInstall.shouldShowNotice ? "has-install-notice" : ""].filter(Boolean).join(" ");
  const hasQueue = queue.length > 0;
  const queuePanel = (
    <QueuePanel
      items={queue}
      onCancelQueued={(id) => {
        void cancelQueuedJob(id)
          .then((event) => {
            if (event && "Cancelled" in event) {
              setQueue((current) => cancelQueued(current, id));
            }
          })
          .catch(showError);
      }}
      onClearFinished={() => {
        void clearCompletedQueueJobs()
          .then(() => setQueue((current) => clearFinished(current)))
          .catch(showError);
      }}
      onRevealOutput={(outputPath) => void revealOutput(outputPath)}
      onRevealReceipt={(receiptPath) => void revealOutput(receiptPath)}
    />
  );

  return (
    <main ref={shellRef} className={shellClassName}>
      <button aria-label="使い方 / Quick tips" className="help-button" title="使い方 / Quick tips" type="button" onClick={() => setIsHelpOpen(true)}>?</button>
      <TrialBanner
        successfulConversions={state.successfulConversions}
        trialLimit={state.trialLimit}
        isPro={state.isPro}
        isLocked={state.isLocked}
        lockedReason={state.lockedReason}
      />
      <LicensePanel
        isPro={state.isPro}
        lockedReason={state.lockedReason}
        onActivate={activateLicense}
        onForget={forgetLicense}
      />
      {applicationsInstall.shouldShowNotice && applicationsInstall.appPath && (
        <InstallNotice
          appPath={applicationsInstall.appPath}
          didCopyToApplications={applicationsInstall.didCopyToApplications}
          didOpenInstalledApp={applicationsInstall.didOpenInstalledApp}
          installedPath={applicationsInstall.installedPath}
          isQuittingAfterEject={applicationsInstall.isQuittingAfterEject}
          isMoving={applicationsInstall.isMoving}
          isOpeningInstalledApp={applicationsInstall.isOpeningInstalledApp}
          shouldOfferInstallerVolumeEject={applicationsInstall.shouldOfferInstallerVolumeEject}
          onDismiss={applicationsInstall.dismissNotice}
          onQuitAfterInstallerVolumeEject={() =>
            void applicationsInstall.quitAfterInstallerVolumeEject()
          }
          onMove={() => void applicationsInstall.moveToApplications()}
          onOpenInstalledApp={() => void applicationsInstall.openInstalledApp()}
          onQuitCurrentApp={() => void applicationsInstall.quitCurrentApp()}
        />
      )}
      <DropZone
        isBusy={isBusy}
        isDragging={isDragging}
        isLocked={state.isLocked}
        lockedReason={state.lockedReason}
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
      <SecureShareAlpha disabled={isBusy} />
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
      {hasQueue && queuePanel}
      {isHelpOpen && <HelpPopover onClose={() => setIsHelpOpen(false)} />}
    </main>
  );
}
