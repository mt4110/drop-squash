use dropsquash_license::LicenseCache;

use super::{fingerprint_label, grace_label, instance_id_label, parse_args, USAGE};

#[test]
fn parses_cache_path() {
    assert_eq!(
        parse_args(vec!["/tmp/license.json".into()]).unwrap(),
        "/tmp/license.json"
    );
}

#[test]
fn rejects_invalid_arguments() {
    assert_eq!(parse_args(vec![]).unwrap_err(), USAGE);
}

#[test]
fn reports_present_identity_fields() {
    let cache = LicenseCache {
        instance_id: Some("instance-1".into()),
        license_key_fingerprint: Some("a".repeat(64)),
        offline_grace_until_unix: Some(123),
        ..LicenseCache::default()
    };

    assert_eq!(
        fingerprint_label(&cache),
        "present 64-character lowercase hex"
    );
    assert_eq!(instance_id_label(&cache), "present");
    assert_eq!(grace_label(&cache), "recorded at unix 123");
}

#[test]
fn reports_missing_identity_fields() {
    let cache = LicenseCache::default();

    assert_eq!(fingerprint_label(&cache), "missing");
    assert_eq!(instance_id_label(&cache), "missing");
    assert_eq!(grace_label(&cache), "absent");
}
