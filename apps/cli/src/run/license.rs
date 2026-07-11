use std::time::{SystemTime, UNIX_EPOCH};

use dropsquash_core::{default_license_cache_path, TRIAL_CONVERSION_LIMIT};
use dropsquash_license::{LicenseCache, LicenseGate};

pub fn gate() -> dropsquash_core::Result<LicenseGate> {
    let cache = LicenseCache::load_or_default(&default_license_cache_path())?;
    Ok(LicenseGate {
        trial_limit: TRIAL_CONVERSION_LIMIT,
        has_valid_license: cache.permits_pro(now_unix()),
    })
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}
