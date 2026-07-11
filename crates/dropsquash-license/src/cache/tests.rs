use super::{license_key_fingerprint, LicenseCache};

#[test]
fn fingerprint_is_stable_and_does_not_include_raw_key() {
    let key = "LS-SECRET-RAW-KEY";
    let fingerprint = license_key_fingerprint(key);

    assert_eq!(fingerprint, license_key_fingerprint(key));
    assert_ne!(fingerprint, key);
}

#[test]
fn serialized_cache_never_contains_raw_license_key() {
    let key = "LS-SECRET-RAW-KEY";
    let cache = LicenseCache {
        instance_name: Some("device-1".to_string()),
        instance_id: Some("device-1".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint(key)),
        activation_id: Some("activation-1".to_string()),
        validated_at_unix: Some(100),
        offline_grace_until_unix: Some(200),
        valid: true,
    };

    let json = serde_json::to_string(&cache).unwrap();

    assert!(!json.contains(key));
    assert!(json.contains("license_key_fingerprint"));
}

#[test]
fn pro_requires_valid_cache_inside_grace_window() {
    let cache = LicenseCache {
        valid: true,
        offline_grace_until_unix: Some(200),
        ..LicenseCache::default()
    };

    assert!(cache.permits_pro(200));
    assert!(!cache.permits_pro(201));
}
