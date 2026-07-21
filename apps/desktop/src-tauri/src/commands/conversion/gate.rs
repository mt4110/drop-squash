use dropsquash_core::{LicenseState, LockedReason};

use super::format_error;
use crate::commands::license::current_license_state;

const TRIAL_COMPLETE_MESSAGE: &str =
    "You used 20 successful conversions. Upgrade once to keep squashing locally.";

pub(super) async fn ensure_trial_open() -> Result<(), String> {
    reject_locked_license(current_license_state().await.map_err(format_error)?)
}

pub(super) fn reject_locked_license(state: LicenseState) -> Result<(), String> {
    match state {
        LicenseState::Locked { reason, .. } => Err(locked_message(reason).to_string()),
        LicenseState::Trial(_) | LicenseState::Pro => Ok(()),
    }
}

fn locked_message(reason: LockedReason) -> &'static str {
    match reason {
        LockedReason::TrialComplete => TRIAL_COMPLETE_MESSAGE,
        LockedReason::LicenseRefreshRequired => {
            "Reconnect once with your license key to refresh Pro."
        }
    }
}
