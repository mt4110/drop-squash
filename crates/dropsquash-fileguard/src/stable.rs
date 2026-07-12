use std::path::{Path, PathBuf};
use std::time::SystemTime;

use dropsquash_core::{AppError, Result};
use tokio::fs;
use tokio::time::{Instant, MissedTickBehavior};
use tokio_util::sync::CancellationToken;

mod options;

pub use options::StabilityOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilityResult {
    pub path: PathBuf,
    pub size: u64,
    pub modified_at: SystemTime,
    pub checked_at: SystemTime,
    pub sample_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sample {
    size: u64,
    modified_at: SystemTime,
}

pub async fn wait_until_stable(
    path: &Path,
    options: StabilityOptions,
    cancel: CancellationToken,
) -> Result<StabilityResult> {
    options::validate(&options)?;

    let deadline = Instant::now() + options.timeout;
    let mut ticker = tokio::time::interval(options.interval);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let mut previous = None;
    let mut stable_count = 0usize;
    let mut sample_count = 0usize;

    loop {
        if cancel.is_cancelled() {
            return Err(AppError::Cancelled);
        }

        if Instant::now() >= deadline {
            return Err(AppError::FileStabilityTimeout(path.to_path_buf()));
        }

        ticker.tick().await;

        let sample = sample(path).await?;
        sample_count += 1;

        if previous == Some(sample) {
            stable_count += 1;
        } else {
            stable_count = 1;
            previous = Some(sample);
        }

        if stable_count >= options.stable_samples {
            fs::File::open(path).await?;
            return Ok(StabilityResult {
                path: path.to_path_buf(),
                size: sample.size,
                modified_at: sample.modified_at,
                checked_at: SystemTime::now(),
                sample_count,
            });
        }
    }
}

async fn sample(path: &Path) -> Result<Sample> {
    let metadata = fs::metadata(path)
        .await
        .map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => AppError::FileNotFound(path.to_path_buf()),
            _ => AppError::Io(error),
        })?;

    Ok(Sample {
        size: metadata.len(),
        modified_at: metadata.modified()?,
    })
}

#[cfg(test)]
mod tests;
