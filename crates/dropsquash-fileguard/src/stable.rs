use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use dropsquash_core::{AppError, Result};
use tokio::fs;
use tokio::time::{Instant, MissedTickBehavior};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilityOptions {
    pub timeout: Duration,
    pub interval: Duration,
    pub stable_samples: usize,
}

impl Default for StabilityOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            interval: Duration::from_millis(250),
            stable_samples: 3,
        }
    }
}

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
    validate_options(&options)?;

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

fn validate_options(options: &StabilityOptions) -> Result<()> {
    if options.timeout.is_zero() {
        return Err(AppError::InvalidConfig(
            "file stability timeout must be greater than zero".to_string(),
        ));
    }

    if options.interval.is_zero() {
        return Err(AppError::InvalidConfig(
            "file stability interval must be greater than zero".to_string(),
        ));
    }

    if options.stable_samples == 0 {
        return Err(AppError::InvalidConfig(
            "stable_samples must be greater than zero".to_string(),
        ));
    }

    Ok(())
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
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    fn options() -> StabilityOptions {
        StabilityOptions {
            timeout: Duration::from_secs(2),
            interval: Duration::from_millis(5),
            stable_samples: 2,
        }
    }

    #[tokio::test]
    async fn returns_when_size_and_modified_time_are_stable() {
        let mut file = NamedTempFile::new().expect("temp file");
        file.write_all(b"recording").expect("write fixture");
        file.flush().expect("flush fixture");

        let result = wait_until_stable(file.path(), options(), CancellationToken::new())
            .await
            .unwrap();

        assert_eq!(result.path, file.path());
        assert_eq!(result.size, 9);
        assert!(result.sample_count >= 2);
    }

    #[tokio::test]
    async fn returns_cancelled_when_token_is_cancelled() {
        let file = NamedTempFile::new().expect("temp file");
        let cancel = CancellationToken::new();
        cancel.cancel();

        let error = wait_until_stable(file.path(), options(), cancel)
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::Cancelled));
    }

    #[tokio::test]
    async fn rejects_invalid_options() {
        let file = NamedTempFile::new().expect("temp file");
        let invalid = StabilityOptions {
            timeout: Duration::ZERO,
            ..options()
        };

        let error = wait_until_stable(file.path(), invalid, CancellationToken::new())
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::InvalidConfig(_)));
    }
}
