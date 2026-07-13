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
    let cache = LicenseCache::load_or_default(&default_license_cache_path())?;
    print_state(state);
    println!("license cache: {}", default_license_cache_path().display());
    for line in format_cache_diagnostics(&cache, now_unix()) {
        println!("{line}");
    }
    Ok(())
}

pub fn forget() -> dropsquash_core::Result<()> {
    forget_at_path(&default_license_cache_path())?;
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

fn format_cache_diagnostics(cache: &LicenseCache, now: u64) -> Vec<String> {
    vec![
        "raw license key persisted: no".to_string(),
        format!("license cache identity: {}", identity_label(cache)),
        format!("offline grace: {}", grace_label(cache, now)),
    ]
}

fn identity_label(cache: &LicenseCache) -> &'static str {
    if has_hex_fingerprint(cache) && has_instance_id(cache) {
        "present"
    } else {
        "missing"
    }
}

fn has_hex_fingerprint(cache: &LicenseCache) -> bool {
    cache
        .license_key_fingerprint
        .as_deref()
        .is_some_and(|value| {
            value.len() == 64 && value.chars().all(|char| char.is_ascii_hexdigit())
        })
}

fn has_instance_id(cache: &LicenseCache) -> bool {
    cache
        .instance_id
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
}

fn grace_label(cache: &LicenseCache, now: u64) -> String {
    match cache.offline_grace_until_unix {
        Some(until) if until >= now => format!("active until unix {until}"),
        Some(until) => format!("expired at unix {until}"),
        None => "absent".to_string(),
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
