import { useCallback, useEffect, useRef, useState } from "react";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { open } from "@tauri-apps/plugin-dialog";
import { revealItemInDir } from "@tauri-apps/plugin-opener";
import { DropZone } from "./components/DropZone";
import { HelpPopover } from "./components/HelpPopover";
import { SettingsDrawer } from "./components/SettingsDrawer";
import type {
  ConversionSummary,
  DropZoneState,
  OutputSize,
  Profile,
} from "./lib/commands";

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
  privacyMode: "local-only",
  successfulConversions: 0,
  trialLimit: 10,
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

  const convert = useCallback(async (inputPath: string) => {
    if (!isTauri() || isBusy || state.isLocked) {
      return;
    }

    setInputPath(inputPath);
    setIsBusy(true);
    setProgress(0);
    setError(undefined);
    setResult(undefined);
    try {
      const summary = await invoke<ConversionSummary>("convert", {
        inputPath,
        outputDir: state.outputDir,
        profile: state.profile,
        outputSize: state.outputSize,
      });
      setResult(summary);
      setState((current) => ({
        ...current,
        successfulConversions: current.successfulConversions + 1,
        isLocked: current.successfulConversions + 1 >= current.trialLimit,
      }));
    } catch (reason) {
      setError(String(reason));
    } finally {
      setIsBusy(false);
      setProgress(undefined);
    }
  }, [isBusy, state.isLocked, state.outputDir, state.outputSize, state.profile]);
  const convertRef = useRef(convert);

  useEffect(() => {
    convertRef.current = convert;
  }, [convert]);

  useEffect(() => {
    if (!isTauri()) {
      return;
    }

    let unlisten: (() => void) | undefined;
    void listen<number>("conversion-progress", (event) => {
      setProgress(event.payload);
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
      await convert(inputPath);
    }
  }, [convert, state.inputExtensions]);

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
    }
  }, [state.outputDir]);

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
        const [inputPath] = event.payload.paths;
        if (inputPath) {
          void convertRef.current(inputPath);
        }
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
    <main className="shell">
      <button aria-label="Show quick tips" className="help-button" title="Show quick tips" type="button" onClick={() => setIsHelpOpen(true)}>?</button>
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
        onRevealOutput={(outputPath) => void revealOutput(outputPath)}
      />
      <SettingsDrawer
        outputDir={state.outputDir}
        profile={state.profile}
        outputSize={state.outputSize}
        profiles={state.profiles}
        outputSizes={state.outputSizes}
        onChooseOutput={() => void chooseOutputDirectory()}
        onProfileChange={(profile: Profile) => setState((current) => ({ ...current, profile }))}
        onOutputSizeChange={(outputSize: OutputSize) => setState((current) => ({ ...current, outputSize }))}
      />
      {isHelpOpen && <HelpPopover onClose={() => setIsHelpOpen(false)} />}
    </main>
  );
}
