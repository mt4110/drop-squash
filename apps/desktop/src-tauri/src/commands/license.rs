use dropsquash_core::{default_history_path, LicenseState, TRIAL_CONVERSION_LIMIT};
use dropsquash_history::{read_records, HistoryMetrics};
use dropsquash_license::LicenseGate;

pub async fn current_license_state() -> dropsquash_core::Result<LicenseState> {
    let records = read_records(&default_history_path()).await?;
    let metrics = HistoryMetrics::from_records(&records);
    Ok(license_gate().state_for_metrics(metrics))
}

fn license_gate() -> LicenseGate {
    LicenseGate {
        trial_limit: TRIAL_CONVERSION_LIMIT,
        has_valid_license: false,
    }
}
