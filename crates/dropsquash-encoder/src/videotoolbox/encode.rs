use dropsquash_core::{AppError, EncodeJob, EncodeResult, Profile, Result, NOT_SMALLER_MESSAGE};
use tokio_util::sync::CancellationToken;

use crate::{verify_output, EncodeProgressReporter};

use super::finalize::finalize_verified_output;
use super::paths::output_path_for;
use super::presets::presets_for;
use super::secure_share::encode_with_reader_writer;
use super::session::{export_session_for, run_export};

pub(super) fn encode_with_avfoundation(
    job: EncodeJob,
    reporter: Option<&dyn EncodeProgressReporter>,
    cancel: CancellationToken,
) -> Result<EncodeResult> {
    if job.secure_share.is_some() {
        return encode_with_secure_share(job);
    }
    validate_job(&job)?;
    std::fs::create_dir_all(&job.output_dir)?;
    let output_path = output_path_for(&job)?;
    let presets = presets_for(&job)?;
    let temporary_directory = tempfile::Builder::new()
        .prefix(".dropsquash-")
        .tempdir_in(&job.output_dir)?;
    let mut last_verification = None;
    for (index, preset) in presets.iter().enumerate() {
        let temporary_output = temporary_directory
            .path()
            .join(format!("output-{index}.mp4"));
        let export_session = export_session_for(&job, &temporary_output, preset)?;

        run_export(&export_session, reporter, &cancel)?;
        if cancel.is_cancelled() {
            return Err(AppError::Cancelled);
        }
        let verification = verify_output(&job.input_path, &temporary_output)?;
        if verification.is_valid_output {
            finalize_verified_output(&temporary_output, &output_path)?;
            reporter.iter().for_each(|reporter| reporter.report(1.0));

            return Ok(EncodeResult {
                input_path: job.input_path,
                output_path,
                profile: job.profile,
                original_bytes: verification.original_bytes,
                output_bytes: verification.output_bytes,
                success: true,
                error_message: None,
            });
        }
        last_verification = Some(verification);
        let _ = std::fs::remove_file(&temporary_output);
    }
    Err(verification_error(last_verification))
}

fn encode_with_secure_share(job: EncodeJob) -> Result<EncodeResult> {
    validate_job(&job)?;
    encode_with_reader_writer(&job)
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

pub(super) fn verification_error(verification: Option<crate::OutputVerification>) -> AppError {
    verification.map_or_else(
        || AppError::Encoder("native export failed without a verification result".to_string()),
        |verification| {
            if verification
                .failure_summary()
                .contains("output is not smaller")
            {
                return AppError::Encoder(NOT_SMALLER_MESSAGE.to_string());
            }
            AppError::Encoder(format!(
                "native export failed output verification: {} ({} bytes -> {} bytes)",
                verification.failure_summary(),
                verification.original_bytes,
                verification.output_bytes,
            ))
        },
    )
}
