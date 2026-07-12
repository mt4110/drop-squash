use std::time::Duration;

use dropsquash_core::{AppError, Result};

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

pub(super) fn validate(options: &StabilityOptions) -> Result<()> {
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
