use std::time::{SystemTime, UNIX_EPOCH};

use dropsquash_core::{
    default_history_path, default_license_cache_path, LicenseState, TRIAL_CONVERSION_LIMIT,
};
use dropsquash_history::{read_records, HistoryMetrics};
use dropsquash_license::{LicenseCache, LicenseGate};

pub async fn current_license_state() -> dropsquash_core::Result<LicenseState> {
    let records = read_records(&default_history_path()).await?;
    let metrics = HistoryMetrics::from_records(&records);
    let cache = LicenseCache::load_or_default(&default_license_cache_path())?;
    Ok(license_gate(cache.permits_pro(now_unix())).state_for_metrics(metrics))
}

fn license_gate(has_valid_license: bool) -> LicenseGate {
    LicenseGate {
        trial_limit: TRIAL_CONVERSION_LIMIT,
        has_valid_license,
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}
