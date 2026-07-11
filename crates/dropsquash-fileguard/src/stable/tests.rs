use std::io::Write;
use std::time::Duration;

use dropsquash_core::AppError;
use tempfile::NamedTempFile;
use tokio_util::sync::CancellationToken;

use super::{wait_until_stable, StabilityOptions};

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
