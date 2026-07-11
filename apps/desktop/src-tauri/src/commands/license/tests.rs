use async_trait::async_trait;
use dropsquash_core::{AppError, Result};
use dropsquash_license::LicenseProvider;
use dropsquash_license::{license_key_fingerprint, LicenseActivation, LicenseCache};

use super::write_activation_cache;

struct FakeProvider {
    fail: bool,
}

#[async_trait]
impl LicenseProvider for FakeProvider {
    async fn activate(&self, license_key: &str, instance_id: &str) -> Result<LicenseActivation> {
        if self.fail {
            return Err(AppError::License("activation failed".to_string()));
        }
        Ok(LicenseActivation {
            license_key_fingerprint: license_key_fingerprint(license_key),
            instance_id: format!("remote-{instance_id}"),
            valid: true,
        })
    }

    async fn validate(&self, _license_key: &str, _instance_id: &str) -> Result<bool> {
        Ok(false)
    }

    async fn deactivate(&self, _license_key: &str, _instance_id: &str) -> Result<()> {
        Ok(())
    }
}

#[tokio::test]
async fn activation_failure_does_not_write_partial_cache() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");

    let result = write_activation_cache(
        "LS-SECRET-RAW-KEY",
        &path,
        &FakeProvider { fail: true },
        100,
    )
    .await;

    assert!(result.is_err());
    assert!(!path.exists());
}

#[tokio::test]
async fn activation_success_writes_fingerprint_without_raw_key() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");

    write_activation_cache(
        "LS-SECRET-RAW-KEY",
        &path,
        &FakeProvider { fail: false },
        100,
    )
    .await
    .unwrap();

    let json = std::fs::read_to_string(&path).unwrap();
    let cache = LicenseCache::load_or_default(&path).unwrap();
    assert!(!json.contains("LS-SECRET-RAW-KEY"));
    assert!(cache.valid);
    assert_eq!(cache.validated_at_unix, Some(100));
    assert_eq!(cache.offline_grace_until_unix, Some(2_592_100));
}
