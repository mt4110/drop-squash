use async_trait::async_trait;
use dropsquash_core::{AppError, Result};
use dropsquash_license::LicenseProvider;
use dropsquash_license::{license_key_fingerprint, LicenseActivation, LicenseCache};

use super::{forget_license_at_path, write_activation_cache};

struct FakeProvider {
    fail: bool,
    expected_key: Option<&'static str>,
    valid: bool,
}

#[async_trait]
impl LicenseProvider for FakeProvider {
    async fn activate(&self, license_key: &str, instance_id: &str) -> Result<LicenseActivation> {
        if let Some(expected_key) = self.expected_key {
            assert_eq!(license_key, expected_key);
        }
        if self.fail {
            return Err(AppError::License("activation failed".to_string()));
        }
        Ok(LicenseActivation {
            license_key_fingerprint: license_key_fingerprint(license_key),
            instance_id: format!("remote-{instance_id}"),
            valid: self.valid,
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
        &FakeProvider {
            fail: true,
            expected_key: None,
            valid: false,
        },
        100,
    )
    .await;

    assert!(result.is_err());
    assert!(!path.exists());
}

#[tokio::test]
async fn activation_failure_preserves_existing_cache() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");
    let existing = LicenseCache {
        instance_name: Some("device-1".to_string()),
        instance_id: Some("remote-device-1".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint("LS-OLD-KEY")),
        validated_at_unix: Some(90),
        offline_grace_until_unix: Some(1000),
        valid: true,
        ..LicenseCache::default()
    };
    existing.save_to_path(&path).unwrap();

    let result = write_activation_cache(
        "LS-NEW-KEY",
        &path,
        &FakeProvider {
            fail: true,
            expected_key: Some("LS-NEW-KEY"),
            valid: false,
        },
        100,
    )
    .await;

    assert!(result.is_err());
    assert_eq!(LicenseCache::load_or_default(&path).unwrap(), existing);
}

#[tokio::test]
async fn activation_success_writes_fingerprint_without_raw_key() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");

    write_activation_cache(
        "LS-SECRET-RAW-KEY",
        &path,
        &FakeProvider {
            fail: false,
            expected_key: None,
            valid: true,
        },
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

#[tokio::test]
async fn invalid_activation_does_not_write_cache() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");

    let result = write_activation_cache(
        "LS-SECRET-RAW-KEY",
        &path,
        &FakeProvider {
            fail: false,
            expected_key: None,
            valid: false,
        },
        100,
    )
    .await;

    assert!(result.unwrap_err().to_string().contains("valid license"));
    assert!(!path.exists());
}

#[tokio::test]
async fn activation_trims_license_key_before_provider_call() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");

    write_activation_cache(
        "  LS-SECRET-RAW-KEY  ",
        &path,
        &FakeProvider {
            fail: false,
            expected_key: Some("LS-SECRET-RAW-KEY"),
            valid: true,
        },
        100,
    )
    .await
    .unwrap();
}

#[test]
fn forget_license_clears_local_cache() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");
    LicenseCache {
        instance_name: Some("device-1".to_string()),
        instance_id: Some("remote-device-1".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint("LS-SECRET-RAW-KEY")),
        validated_at_unix: Some(100),
        offline_grace_until_unix: Some(200),
        valid: true,
        ..LicenseCache::default()
    }
    .save_to_path(&path)
    .unwrap();

    forget_license_at_path(&path).unwrap();

    assert_eq!(
        LicenseCache::load_or_default(&path).unwrap(),
        LicenseCache::default()
    );
}
