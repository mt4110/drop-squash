use dropsquash_core::Result;

mod activation_watch;
mod ax_event_watch;
mod finish;
mod lifecycle_watch;
mod prepare;
pub(super) mod request;
pub(super) mod revalidation;
mod watch;
use super::SckRecordingProbeReport;
use prepare::prepare;
use request::RecordingSessionRequest;

pub(super) fn record_until_stopped_from_content(
    request: RecordingSessionRequest,
    started: impl FnOnce(std::result::Result<(), String>),
) -> Result<SckRecordingProbeReport> {
    let prepared = prepare(
        &request.selection,
        request.output_path.clone(),
        request.output_size,
        request.policy,
        request.content,
    );
    let (target, frame_size, accessibility, stream) = match prepared {
        Ok(value) => value,
        Err(error) => {
            started(Err(error.user_message()));
            return Err(error);
        }
    };
    let display_epoch = match super::display_watch::start() {
        Ok(epoch) => epoch,
        Err(error) => {
            stream.discard_recording();
            started(Err(error.user_message()));
            return Err(error);
        }
    };
    let attestation = match super::core_graphics_window::attest(&request.selection) {
        Ok(attestation) => attestation,
        Err(error) => {
            stream.discard_recording();
            started(Err(error.user_message()));
            return Err(error);
        }
    };
    let ax_watch = match ax_event_watch::AxEventWatch::start(&request.selection) {
        Ok(watch) => watch,
        Err(error) => {
            stream.discard_recording();
            started(Err(error.user_message()));
            return Err(error);
        }
    };
    let lifecycle_watch = lifecycle_watch::LifecycleWatch::start();
    let activation_watch = activation_watch::ActivationWatch::start(request.selection.owner_pid);
    if let Err(error) = stream.start_capture(request.probe.completion_timeout) {
        stream.discard_recording();
        let _ = activation_watch.finish();
        let _ = lifecycle_watch.finish();
        let _ = ax_watch.finish();
        started(Err(error.user_message()));
        return Err(error);
    }
    started(Ok(()));
    let watch = watch::WindowWatch::start(request.selection.clone(), display_epoch, attestation);
    let stopped = request.stop.recv().is_ok();
    let stop_result = stream.stop_capture(request.probe.completion_timeout);
    let activation_watch_result = activation_watch.finish();
    let lifecycle_watch_result = lifecycle_watch.finish();
    let watch_result = watch.finish();
    let ax_watch_result = ax_watch.finish();
    if !stopped {
        stream.discard_recording();
        return Err(super::recording_probe::fail_closed(
            "stop control disconnected",
        ));
    }
    if let Err(error) = stop_result {
        stream.discard_recording();
        return Err(error);
    }
    if let Err(error) = lifecycle_watch_result {
        stream.discard_recording();
        return Err(error);
    }
    if let Err(error) = activation_watch_result {
        stream.discard_recording();
        return Err(error);
    }
    if let Err(error) = ax_watch_result {
        stream.discard_recording();
        return Err(error);
    }
    if let Err(error) = watch_result {
        stream.discard_recording();
        return Err(error);
    }
    let result = finish::report(
        &stream,
        request.probe,
        &request.selection,
        target,
        frame_size,
        request.output_path,
        accessibility,
    );
    if result.is_err() {
        stream.discard_recording();
    }
    result
}
