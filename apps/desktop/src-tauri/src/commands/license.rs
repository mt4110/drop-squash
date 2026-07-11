use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use dropsquash_core::AppError;
use dropsquash_core::{
    default_history_path, default_license_cache_path, LicenseState, TRIAL_CONVERSION_LIMIT,
};
use dropsquash_history::{read_records, HistoryMetrics};
use dropsquash_license::{LemonSqueezyProvider, LicenseCache, LicenseGate, LicenseProvider};

const OFFLINE_GRACE_SECONDS: u64 = 60 * 60 * 24 * 30;

pub async fn activate_license(license_key: String) -> dropsquash_core::Result<LicenseState> {
    if license_key.trim().is_empty() {
        return Err(AppError::License("Enter a license key.".to_string()));
    }
    let path = default_license_cache_path();
    write_activation_cache(
        &license_key,
        &path,
        &LemonSqueezyProvider::default(),
        now_unix(),
    )
    .await?;
    current_license_state().await
}

async fn write_activation_cache<P: LicenseProvider>(
    license_key: &str,
    path: &Path,
    provider: &P,
    now: u64,
) -> dropsquash_core::Result<()> {
    let license_key = license_key.trim();
    let mut cache = LicenseCache::load_or_default(path)?;
    let instance_name = cache
        .instance_name
        .clone()
        .unwrap_or_else(generate_instance_id);
    let activation = provider.activate(license_key, &instance_name).await?;
    cache.instance_name = Some(instance_name);
    cache.instance_id = Some(activation.instance_id);
    cache.license_key_fingerprint = Some(activation.license_key_fingerprint);
    cache.activation_id = None;
    cache.validated_at_unix = Some(now);
    cache.offline_grace_until_unix = Some(now + OFFLINE_GRACE_SECONDS);
    cache.valid = activation.valid;
    cache.save_to_path(path)
}

fn generate_instance_id() -> String {
    format!("dropsquash-{}-{}", std::process::id(), now_unix())
}

pub async fn forget_license() -> dropsquash_core::Result<LicenseState> {
    forget_license_at_path(&default_license_cache_path())?;
    current_license_state().await
}

fn forget_license_at_path(path: &Path) -> dropsquash_core::Result<()> {
    LicenseCache::default().save_to_path(path)
}

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

#[cfg(test)]
mod tests;
