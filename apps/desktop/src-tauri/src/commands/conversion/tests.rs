use tokio_util::sync::CancellationToken;

use dropsquash_core::{LicenseState, TrialState};

use super::{ensure_not_cancelled, reject_locked_license};

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
    let error = reject_locked_license(LicenseState::Locked(TrialState {
        successful_conversions: 10,
        limit: 10,
    }))
    .unwrap_err();

    assert!(error.contains("Trial complete"));
}

#[test]
fn accepts_trial_and_pro_before_starting_conversion() {
    assert!(reject_locked_license(LicenseState::Trial(TrialState {
        successful_conversions: 9,
        limit: 10,
    }))
    .is_ok());
    assert!(reject_locked_license(LicenseState::Pro).is_ok());
}
