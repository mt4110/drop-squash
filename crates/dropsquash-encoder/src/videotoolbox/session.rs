use dropsquash_core::{AppError, EncodeJob, Result};
use std::sync::mpsc::{sync_channel, Receiver, RecvTimeoutError};
use std::time::Duration;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2_av_foundation::{AVAssetExportSession, AVFileTypeMPEG4, AVURLAsset};
use objc2_foundation::NSURL;
use tokio_util::sync::CancellationToken;

use crate::EncodeProgressReporter;

mod status;

pub(super) fn export_session_for(
    job: &EncodeJob,
    temporary_output: &std::path::Path,
    preset: &objc2_foundation::NSString,
) -> Result<Retained<AVAssetExportSession>> {
    let input_url = file_url(&job.input_path)?;
    let output_url = file_url(temporary_output)?;
    let asset = unsafe { AVURLAsset::URLAssetWithURL_options(&input_url, None) };
    let export_session =
        unsafe { AVAssetExportSession::exportSessionWithAsset_presetName(&asset, preset) }
            .ok_or_else(|| {
                AppError::Encoder("no compatible Apple export preset was found".to_string())
            })?;
    configure_export_session(&export_session, &output_url)?;
    Ok(export_session)
}

pub(super) fn run_export(
    export_session: &AVAssetExportSession,
    reporter: Option<&dyn EncodeProgressReporter>,
    cancel: &CancellationToken,
) -> Result<()> {
    let (completion_sender, completion_receiver) = sync_channel(1);
    let completion = RcBlock::new(move || {
        let _ = completion_sender.send(());
    });
    unsafe { export_session.exportAsynchronouslyWithCompletionHandler(&completion) };
    wait_for_export(export_session, reporter, completion_receiver, cancel)
}

fn configure_export_session(
    export_session: &AVAssetExportSession,
    output_url: &NSURL,
) -> Result<()> {
    let output_file_type = unsafe { AVFileTypeMPEG4 }.ok_or_else(|| {
        AppError::Encoder("MPEG-4 output is unavailable on this system".to_string())
    })?;
    unsafe {
        export_session.setOutputFileType(Some(output_file_type));
        export_session.setOutputURL(Some(output_url));
        export_session.setShouldOptimizeForNetworkUse(true);
        export_session.setAllowsParallelizedExport(true);
    }
    Ok(())
}

pub(super) fn file_url(path: &std::path::Path) -> Result<Retained<NSURL>> {
    NSURL::from_file_path(path).ok_or_else(|| {
        AppError::Encoder(format!(
            "could not create a file URL for {}",
            path.display()
        ))
    })
}

fn wait_for_export(
    export_session: &AVAssetExportSession,
    reporter: Option<&dyn EncodeProgressReporter>,
    completion_receiver: Receiver<()>,
    cancel: &CancellationToken,
) -> Result<()> {
    loop {
        if cancel.is_cancelled() {
            unsafe { export_session.cancelExport() };
            return Err(AppError::Cancelled);
        }
        match completion_receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(()) => break,
            Err(RecvTimeoutError::Timeout) => {
                reporter.iter().for_each(|reporter| {
                    reporter.report(unsafe { export_session.progress() }.clamp(0.0, 1.0));
                });
            }
            Err(RecvTimeoutError::Disconnected) => {
                return Err(AppError::Encoder(
                    "Apple export completed without a completion signal".to_string(),
                ));
            }
        }
    }
    status::ensure_completed(export_session)
}
