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
fn saves_and_loads_cache_without_temp_leftover() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");
    let cache = LicenseCache {
        instance_id: Some("instance-1".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint("LS-SECRET-RAW-KEY")),
        valid: true,
        ..LicenseCache::default()
    };

    cache.save_to_path(&path).unwrap();

    assert_eq!(LicenseCache::load_or_default(&path).unwrap(), cache);
    assert!(std::fs::read_dir(directory.path())
        .unwrap()
        .all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .ends_with(".tmp")));
}

#[test]
fn save_replaces_existing_cache() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");
    LicenseCache {
        valid: false,
        ..LicenseCache::default()
    }
    .save_to_path(&path)
    .unwrap();

    LicenseCache {
        instance_id: Some("instance-2".to_string()),
        valid: true,
        ..LicenseCache::default()
    }
    .save_to_path(&path)
    .unwrap();

    let loaded = LicenseCache::load_or_default(&path).unwrap();
    assert!(loaded.valid);
    assert_eq!(loaded.instance_id.as_deref(), Some("instance-2"));
}

#[test]
fn pro_requires_valid_cache_inside_grace_window() {
    let cache = LicenseCache {
        instance_id: Some("instance-1".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint("LS-SECRET-RAW-KEY")),
        valid: true,
        offline_grace_until_unix: Some(200),
        ..LicenseCache::default()
    };

    assert!(cache.permits_pro(200));
    assert!(!cache.permits_pro(201));
}

#[test]
fn pro_requires_activation_identity() {
    let cache = LicenseCache {
        valid: true,
        offline_grace_until_unix: Some(200),
        ..LicenseCache::default()
    };

    assert!(!cache.permits_pro(100));
}

#[test]
fn pro_requires_valid_flag_even_with_identity_and_grace() {
    let cache = LicenseCache {
        instance_id: Some("instance-1".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint("LS-SECRET-RAW-KEY")),
        valid: false,
        offline_grace_until_unix: Some(200),
        ..LicenseCache::default()
    };

    assert!(!cache.permits_pro(100));
}

#[test]
fn pro_requires_offline_grace_window() {
    let cache = LicenseCache {
        instance_id: Some("instance-1".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint("LS-SECRET-RAW-KEY")),
        valid: true,
        offline_grace_until_unix: None,
        ..LicenseCache::default()
    };

    assert!(!cache.permits_pro(100));
}

#[test]
fn pro_rejects_incomplete_activation_identity() {
    let mut cache = LicenseCache {
        instance_id: Some("instance-1".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint("LS-SECRET-RAW-KEY")),
        valid: true,
        offline_grace_until_unix: Some(200),
        ..LicenseCache::default()
    };

    cache.instance_id = None;
    assert!(!cache.permits_pro(100));

    cache.instance_id = Some("instance-1".to_string());
    cache.license_key_fingerprint = None;
    assert!(!cache.permits_pro(100));
}

#[test]
fn pro_rejects_blank_activation_identity() {
    let cache = LicenseCache {
        instance_id: Some("   ".to_string()),
        license_key_fingerprint: Some("   ".to_string()),
        valid: true,
        offline_grace_until_unix: Some(200),
        ..LicenseCache::default()
    };

    assert!(!cache.permits_pro(100));
}

#[test]
fn pro_rejects_malformed_fingerprint() {
    let mut cache = LicenseCache {
        instance_id: Some("instance-1".to_string()),
        license_key_fingerprint: Some("short".to_string()),
        valid: true,
        offline_grace_until_unix: Some(200),
        ..LicenseCache::default()
    };

    assert!(!cache.permits_pro(100));

    cache.license_key_fingerprint = Some("z".repeat(64));
    assert!(!cache.permits_pro(100));
}
