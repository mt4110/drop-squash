use std::time::{SystemTime, UNIX_EPOCH};

use std::path::PathBuf;

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
    let records = read_records(&history).await?;
    let metrics = HistoryMetrics::from_records(&records);
    let state = gate()?.state_for_metrics(metrics);
    match state {
        LicenseState::Pro => println!("license state: Pro"),
        LicenseState::Trial(trial) => {
            println!("license state: Trial");
            println!(
                "trial: {}/{} successful conversions used",
                trial.successful_conversions, trial.limit
            );
        }
        LicenseState::Locked(trial) => {
            println!("license state: Locked");
            println!(
                "trial: {}/{} successful conversions used",
                trial.successful_conversions, trial.limit
            );
        }
    }
    println!("license cache: {}", default_license_cache_path().display());
    println!("raw license key persisted: no");
    Ok(())
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}
