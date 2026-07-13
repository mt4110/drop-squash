use dropsquash_core::{LicenseState, LockedReason, TrialState};
use dropsquash_history::HistoryMetrics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LicenseGate {
    pub trial_limit: u32,
    pub has_valid_license: bool,
    pub license_refresh_required: bool,
}

impl LicenseGate {
    pub fn state_for_metrics(self, metrics: HistoryMetrics) -> LicenseState {
        if self.has_valid_license {
            return LicenseState::Pro;
        }

        let trial = TrialState {
            successful_conversions: metrics.successful_conversion_count,
            limit: self.trial_limit,
        };

        if trial.is_locked() {
            LicenseState::Locked {
                trial,
                reason: LockedReason::TrialComplete,
            }
        } else if self.license_refresh_required {
            LicenseState::Locked {
                trial,
                reason: LockedReason::LicenseRefreshRequired,
            }
        } else {
            LicenseState::Trial(trial)
        }
    }
}

pub fn trial_state_from_history(metrics: HistoryMetrics, trial_limit: u32) -> TrialState {
    TrialState {
        successful_conversions: metrics.successful_conversion_count,
        limit: trial_limit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trial_counts_only_successful_metrics() {
        let metrics = HistoryMetrics {
            successful_conversion_count: 9,
            ..HistoryMetrics::default()
        };

        let state = trial_state_from_history(metrics, 10);

        assert_eq!(state.successful_conversions, 9);
        assert_eq!(state.remaining(), 1);
    }

    #[test]
    fn locks_when_successful_count_reaches_limit() {
        let metrics = HistoryMetrics {
            successful_conversion_count: 10,
            ..HistoryMetrics::default()
        };
        let gate = LicenseGate {
            trial_limit: 10,
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
            trial_limit: 10,
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
            trial_limit: 10,
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
}
