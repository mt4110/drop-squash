use dropsquash_core::{LicenseState, TrialState};

use super::format_state;

#[test]
fn formats_pro_state_without_trial_count() {
    assert_eq!(format_state(LicenseState::Pro), vec!["license state: Pro"]);
}

#[test]
fn formats_trial_state_with_usage() {
    assert_eq!(
        format_state(LicenseState::Trial(TrialState {
            successful_conversions: 3,
            limit: 10,
        })),
        vec![
            "license state: Trial".to_string(),
            "trial: 3/10 successful conversions used".to_string(),
        ]
    );
}

#[test]
fn formats_locked_state_with_usage() {
    assert_eq!(
        format_state(LicenseState::Locked(TrialState {
            successful_conversions: 10,
            limit: 10,
        })),
        vec![
            "license state: Locked".to_string(),
            "trial: 10/10 successful conversions used".to_string(),
        ]
    );
}
