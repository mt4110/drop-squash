use std::sync::mpsc::{Receiver, SyncSender};

use dropsquash_core::{FrameSize, OutputSize};
use dropsquash_platform::{
    accessibility_observations_from_native, start_attested_native_strict_recording,
    temporal_observations_from_native, vision_observations_from_native, SckCaptureTarget,
    SckCaptureTargetKind,
};

use super::super::finalize::{recording, SecureShareRecordingDto};
use super::super::paths::RecordingPaths;
use super::super::publication::PublicationPermit;
use super::super::selection::{NativeWindowSelection, SecureShareWindowSelectionDto};
use dimensions::dimensions;
use events::{wait_started, wait_stopped};

mod dimensions;
mod events;
mod report;

pub(super) fn start(
    selection: SecureShareWindowSelectionDto,
    output_size: OutputSize,
    paths: RecordingPaths,
    permit: PublicationPermit,
    stop: Receiver<()>,
    ready: SyncSender<Result<(), String>>,
    finished: SyncSender<Result<SecureShareRecordingDto, String>>,
) {
    std::thread::spawn(move || {
        let result = run(
            selection.into_native(),
            output_size,
            &paths,
            &permit,
            stop,
            ready,
        );
        let _ = finished.send(result);
    });
}

fn run(
    selection: NativeWindowSelection,
    output_size: OutputSize,
    paths: &RecordingPaths,
    permit: &PublicationPermit,
    stop: Receiver<()>,
    ready: SyncSender<Result<(), String>>,
) -> Result<SecureShareRecordingDto, String> {
    let (width, height) = dimensions(selection.frame.width, selection.frame.height, output_size)?;
    let handle = start_attested_native_strict_recording(
        selection.window_id,
        selection.owner_pid,
        selection.frame,
        width,
        height,
        &paths.partial,
    )
    .map_err(|error| startup_failure(&ready, error))?;
    wait_started(&handle.events, &ready)?;
    let (frames, native_vision, native_accessibility, native_temporal) =
        wait_stopped(&handle, stop)?;
    let vision = vision_observations_from_native(
        native_vision,
        &frames,
        dropsquash_core::FrameSize { width, height },
    )
    .map_err(|error| error.user_message())?;
    let accessibility = accessibility_observations_from_native(
        native_accessibility,
        &target(&selection),
        FrameSize { width, height },
        &frames,
    )
    .map_err(|error| error.user_message())?;
    let temporal =
        temporal_observations_from_native(native_temporal, &frames, FrameSize { width, height })
            .map_err(|error| error.user_message())?;
    recording(
        Ok(report::build(report::ReportInput {
            selection,
            width,
            height,
            frames,
            accessibility,
            vision,
            temporal,
            paths,
        })),
        paths.clone(),
        permit.clone(),
    )
}

fn startup_failure(ready: &SyncSender<Result<(), String>>, error: String) -> String {
    let _ = ready.send(Err(error.clone()));
    error
}

fn target(selection: &NativeWindowSelection) -> SckCaptureTarget {
    SckCaptureTarget {
        kind: SckCaptureTargetKind::Window,
        id: selection.window_id,
        frame: selection.frame,
        owner_pid: Some(selection.owner_pid),
    }
}
