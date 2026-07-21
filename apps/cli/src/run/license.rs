use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use dropsquash_core::{
    default_history_path, default_license_cache_path, LicenseState, TRIAL_CONVERSION_LIMIT,
};
use dropsquash_history::{read_records, HistoryMetrics};
use dropsquash_license::{LicenseCache, LicenseGate};

mod diagnostics;

pub fn gate() -> dropsquash_core::Result<LicenseGate> {
    let cache = LicenseCache::load_or_default(&default_license_cache_path())?;
    let now = now_unix();
    Ok(LicenseGate {
        trial_limit: TRIAL_CONVERSION_LIMIT,
        has_valid_license: cache.permits_pro(now),
        license_refresh_required: cache.requires_license_refresh(now),
    })
}

pub async fn status(
    history: Option<PathBuf>,
    cache_path: Option<PathBuf>,
) -> dropsquash_core::Result<()> {
    let history = history.unwrap_or_else(default_history_path);
    let cache_path = cache_path.unwrap_or_else(default_license_cache_path);
    let state = state(&history, &cache_path).await?;
    let cache = LicenseCache::load_or_default(&cache_path)?;
    print_state(state);
    println!("license cache: {}", cache_path.display());
    for line in diagnostics::format_cache_diagnostics(&cache, now_unix()) {
        println!("{line}");
    }
    Ok(())
}

pub fn forget(cache_path: Option<PathBuf>) -> dropsquash_core::Result<()> {
    forget_at_path(&cache_path.unwrap_or_else(default_license_cache_path))?;
    for line in forget_lines() {
        println!("{line}");
    }
    Ok(())
}

fn forget_at_path(path: &Path) -> dropsquash_core::Result<()> {
    LicenseCache::forget_path(path)
}

fn forget_lines() -> Vec<String> {
    vec![
        "local license cache forgotten".to_string(),
        "server-side license activation unchanged".to_string(),
    ]
}

pub async fn state(history: &Path, cache_path: &Path) -> dropsquash_core::Result<LicenseState> {
    let records = read_records(history).await?;
    let metrics = HistoryMetrics::from_records(&records);
    let cache = LicenseCache::load_or_default(cache_path)?;
    Ok(license_gate(&cache, now_unix()).state_for_metrics(metrics))
}

fn license_gate(cache: &LicenseCache, now: u64) -> LicenseGate {
    LicenseGate {
        trial_limit: TRIAL_CONVERSION_LIMIT,
        has_valid_license: cache.permits_pro(now),
        license_refresh_required: cache.requires_license_refresh(now),
    }
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
        LicenseState::Locked { trial, .. } => trial_lines("Locked", trial),
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
