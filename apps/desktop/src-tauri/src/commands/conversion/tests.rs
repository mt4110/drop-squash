use tokio_util::sync::CancellationToken;

use dropsquash_core::AppError;

use super::{ensure_not_cancelled, gate::reject_locked_license, is_not_smaller_error};
use dropsquash_core::{LicenseState, LockedReason, TrialState};

#[test]
fn accepts_uncancelled_token() {
    let cancel = CancellationToken::new();

    assert!(ensure_not_cancelled(&cancel).is_ok());
}

#[test]
fn rejects_cancelled_token_before_postprocessing() {
    let cancel = CancellationToken::new();
    cancel.cancel();

    let error = ensure_not_cancelled(&cancel).unwrap_err();

    assert!(error.contains("cancelled"));
}

#[test]
fn rejects_locked_trial_before_starting_conversion() {
    let error = reject_locked_license(LicenseState::Locked {
        reason: LockedReason::TrialComplete,
        trial: TrialState {
            successful_conversions: 20,
            limit: 20,
        },
    })
    .unwrap_err();

    assert!(error.contains("20 successful conversions"));
}

#[test]
fn reports_expired_license_before_starting_conversion() {
    let error = reject_locked_license(LicenseState::Locked {
        reason: LockedReason::LicenseRefreshRequired,
        trial: TrialState {
            successful_conversions: 3,
            limit: 20,
        },
    })
    .unwrap_err();

    assert!(error.contains("refresh Pro"));
}

#[test]
fn accepts_trial_and_pro_before_starting_conversion() {
    assert!(reject_locked_license(LicenseState::Trial(TrialState {
        successful_conversions: 9,
        limit: 20,
    }))
    .is_ok());
    assert!(reject_locked_license(LicenseState::Pro).is_ok());
}

#[test]
fn recognizes_raw_not_smaller_encoder_errors() {
    assert!(is_not_smaller_error(&AppError::Encoder(
        "native export failed output verification: output is not smaller (1 bytes -> 2 bytes)"
            .to_string(),
    )));
}

#[test]
fn recognizes_friendly_not_smaller_encoder_errors() {
    assert!(is_not_smaller_error(&AppError::Encoder(
        "This recording could not be made smaller. It may already be small, so DropSquash kept the original and did not count the attempt. Try a smaller Size setting for this clip."
            .to_string(),
    )));
}
