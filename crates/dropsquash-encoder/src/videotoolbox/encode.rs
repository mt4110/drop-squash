use dropsquash_core::{AppError, EncodeJob, EncodeResult, Profile, Result};
use tokio_util::sync::CancellationToken;

use crate::{verify_output, EncodeProgressReporter};

use super::finalize::finalize_verified_output;
use super::paths::output_path_for;
use super::session::{export_session_for, run_export};

pub(super) fn encode_with_avfoundation(
    job: EncodeJob,
    reporter: Option<&dyn EncodeProgressReporter>,
    cancel: CancellationToken,
) -> Result<EncodeResult> {
    validate_job(&job)?;
    std::fs::create_dir_all(&job.output_dir)?;
    let output_path = output_path_for(&job)?;
    let temporary_directory = tempfile::Builder::new()
        .prefix(".dropsquash-")
        .tempdir_in(&job.output_dir)?;
    let temporary_output = temporary_directory.path().join("output.mp4");
    let export_session = export_session_for(&job, &temporary_output)?;

    run_export(&export_session, reporter, &cancel)?;
    if cancel.is_cancelled() {
        return Err(AppError::Cancelled);
    }
    let verification = verify_output(&job.input_path, &temporary_output)?;
    if !verification.is_valid_output {
        return Err(AppError::Encoder(format!(
            "native export failed output verification: {} ({} bytes -> {} bytes)",
            verification.failure_summary(),
            verification.original_bytes,
            verification.output_bytes,
        )));
    }

    finalize_verified_output(&temporary_output, &output_path)?;
    reporter.iter().for_each(|reporter| reporter.report(1.0));

    Ok(EncodeResult {
        input_path: job.input_path,
        output_path,
        profile: job.profile,
        original_bytes: verification.original_bytes,
        output_bytes: verification.output_bytes,
        success: true,
        error_message: None,
    })
}

fn validate_job(job: &EncodeJob) -> Result<()> {
    if !job.input_path.is_file() {
        return Err(AppError::FileNotFound(job.input_path.clone()));
    }
    if job.profile == Profile::Privacy {
        return Err(AppError::Encoder(
            "Privacy profile requires the dedicated metadata and audio pipeline".to_string(),
        ));
    }
    Ok(())
}
