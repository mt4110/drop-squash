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

        if self.license_refresh_required {
            LicenseState::Locked {
                trial,
                reason: LockedReason::LicenseRefreshRequired,
            }
        } else if trial.is_locked() {
            LicenseState::Locked {
                trial,
                reason: LockedReason::TrialComplete,
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
mod tests;
