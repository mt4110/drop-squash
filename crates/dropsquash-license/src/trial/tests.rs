use dropsquash_core::{LicenseState, LockedReason};
use dropsquash_history::HistoryMetrics;

use super::{trial_state_from_history, LicenseGate};

#[test]
fn trial_counts_only_successful_metrics() {
    let metrics = HistoryMetrics {
        successful_conversion_count: 9,
        ..HistoryMetrics::default()
    };

    let state = trial_state_from_history(metrics, 20);

    assert_eq!(state.successful_conversions, 9);
    assert_eq!(state.remaining(), 11);
}

#[test]
fn locks_when_successful_count_reaches_limit() {
    let metrics = HistoryMetrics {
        successful_conversion_count: 20,
        ..HistoryMetrics::default()
    };
    let gate = LicenseGate {
        trial_limit: 20,
        has_valid_license: false,
        license_refresh_required: false,
    };

    assert!(matches!(
        gate.state_for_metrics(metrics),
        LicenseState::Locked {
            reason: LockedReason::TrialComplete,
            ..
        }
    ));
}

#[test]
fn valid_license_unlocks_regardless_of_trial_count() {
    let metrics = HistoryMetrics {
        successful_conversion_count: 99,
        ..HistoryMetrics::default()
    };
    let gate = LicenseGate {
        trial_limit: 20,
        has_valid_license: true,
        license_refresh_required: true,
    };

    assert_eq!(gate.state_for_metrics(metrics), LicenseState::Pro);
}

#[test]
fn expired_license_cache_locks_before_trial_is_exhausted() {
    let metrics = HistoryMetrics {
        successful_conversion_count: 3,
        ..HistoryMetrics::default()
    };
    let gate = LicenseGate {
        trial_limit: 20,
        has_valid_license: false,
        license_refresh_required: true,
    };

    assert!(matches!(
        gate.state_for_metrics(metrics),
        LicenseState::Locked {
            reason: LockedReason::LicenseRefreshRequired,
            ..
        }
    ));
}

#[test]
fn expired_license_cache_takes_priority_over_trial_complete() {
    let metrics = HistoryMetrics {
        successful_conversion_count: 20,
        ..HistoryMetrics::default()
    };
    let gate = LicenseGate {
        trial_limit: 20,
        has_valid_license: false,
        license_refresh_required: true,
    };

    assert!(matches!(
        gate.state_for_metrics(metrics),
        LicenseState::Locked {
            reason: LockedReason::LicenseRefreshRequired,
            ..
        }
    ));
}
