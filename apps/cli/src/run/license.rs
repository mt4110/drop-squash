use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use dropsquash_core::{
    default_history_path, default_license_cache_path, LicenseState, TRIAL_CONVERSION_LIMIT,
};
use dropsquash_history::{read_records, HistoryMetrics};
use dropsquash_license::{LicenseCache, LicenseGate};

pub fn gate() -> dropsquash_core::Result<LicenseGate> {
    let cache = LicenseCache::load_or_default(&default_license_cache_path())?;
    Ok(LicenseGate {
        trial_limit: TRIAL_CONVERSION_LIMIT,
        has_valid_license: cache.permits_pro(now_unix()),
    })
}

pub async fn status(history: Option<PathBuf>) -> dropsquash_core::Result<()> {
    let history = history.unwrap_or_else(default_history_path);
    let state = state(&history).await?;
    print_state(state);
    println!("license cache: {}", default_license_cache_path().display());
    println!("raw license key persisted: no");
    Ok(())
}

pub async fn state(history: &Path) -> dropsquash_core::Result<LicenseState> {
    let records = read_records(history).await?;
    let metrics = HistoryMetrics::from_records(&records);
    Ok(gate()?.state_for_metrics(metrics))
}

pub fn print_state(state: LicenseState) {
    for line in format_state(state) {
        println!("{line}");
    }
}

fn format_state(state: LicenseState) -> Vec<String> {
    match state {
        LicenseState::Pro => vec!["license state: Pro".to_string()],
        LicenseState::Trial(trial) => trial_lines("Trial", trial),
        LicenseState::Locked(trial) => trial_lines("Locked", trial),
    }
}

fn trial_lines(label: &str, trial: dropsquash_core::TrialState) -> Vec<String> {
    vec![
        format!("license state: {label}"),
        format!(
            "trial: {}/{} successful conversions used",
            trial.successful_conversions, trial.limit
        ),
    ]
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests;
