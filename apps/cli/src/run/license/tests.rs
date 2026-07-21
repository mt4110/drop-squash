use dropsquash_core::{LicenseState, LockedReason, TrialState};
use dropsquash_license::{license_key_fingerprint, LicenseCache};

use super::{
    diagnostics::format_cache_diagnostics, forget_at_path, forget_lines, format_state, state,
};

#[test]
fn formats_pro_state_without_trial_count() {
    assert_eq!(format_state(LicenseState::Pro), vec!["license state: Pro"]);
}

#[test]
fn formats_trial_state_with_usage() {
    assert_eq!(
        format_state(LicenseState::Trial(TrialState {
            successful_conversions: 3,
            limit: 20,
        })),
        vec![
            "license state: Trial".to_string(),
            "trial: 3/20 successful conversions used".to_string(),
        ]
    );
}

#[test]
fn formats_locked_state_with_usage() {
    assert_eq!(
        format_state(LicenseState::Locked {
            reason: LockedReason::TrialComplete,
            trial: TrialState {
                successful_conversions: 20,
                limit: 20,
            },
        }),
        vec![
            "license state: Locked".to_string(),
            "trial: 20/20 successful conversions used".to_string(),
        ]
    );
}

#[test]
fn forget_clears_only_local_license_cache() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("license.json");
    LicenseCache {
        valid: true,
        instance_name: Some("DropSquash CLI".to_string()),
        instance_id: Some("instance-123".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint("LS-SECRET-RAW-KEY")),
        activation_id: Some("activation-123".to_string()),
        validated_at_unix: Some(1),
        offline_grace_until_unix: Some(2),
    }
    .save_to_path(&path)
    .unwrap();

    forget_at_path(&path).unwrap();

    assert!(!path.exists());
    assert_eq!(
        LicenseCache::load_or_default(&path).unwrap(),
        LicenseCache::default()
    );
}

#[test]
fn forget_output_says_server_activation_is_unchanged() {
    assert_eq!(
        forget_lines(),
        vec![
            "local license cache forgotten".to_string(),
            "server-side license activation unchanged".to_string(),
        ]
    );
}

#[tokio::test]
async fn state_reads_custom_cache_path() {
    let directory = tempfile::tempdir().unwrap();
    let history = directory.path().join("history.jsonl");
    let path = directory.path().join("license.json");
    std::fs::write(&history, "").unwrap();
    LicenseCache {
        valid: true,
        instance_name: Some("DropSquash CLI".to_string()),
        instance_id: Some("instance-123".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint("LS-SECRET-RAW-KEY")),
        activation_id: None,
        validated_at_unix: Some(1),
        offline_grace_until_unix: Some(u64::MAX),
    }
    .save_to_path(&path)
    .unwrap();

    let state = state(&history, &path).await.unwrap();

    assert_eq!(state, LicenseState::Pro);
}

#[test]
fn cache_diagnostics_report_present_identity_and_active_grace() {
    let cache = LicenseCache {
        valid: true,
        instance_name: Some("DropSquash CLI".to_string()),
        instance_id: Some("instance-123".to_string()),
        license_key_fingerprint: Some(license_key_fingerprint("LS-SECRET-RAW-KEY")),
        activation_id: None,
        validated_at_unix: Some(100),
        offline_grace_until_unix: Some(200),
    };

    assert_eq!(
        format_cache_diagnostics(&cache, 150),
        vec![
            "raw license key persisted: no".to_string(),
            "license cache identity: present".to_string(),
            "license cache fingerprint: present 64-character lowercase hex".to_string(),
            "license cache instance_id: present".to_string(),
            "offline grace: active until unix 200".to_string(),
        ]
    );
}

#[test]
fn cache_diagnostics_report_missing_identity_and_expired_grace() {
    let cache = LicenseCache {
        valid: true,
        offline_grace_until_unix: Some(200),
        ..LicenseCache::default()
    };

    assert_eq!(
        format_cache_diagnostics(&cache, 201),
        vec![
            "raw license key persisted: no".to_string(),
            "license cache identity: missing".to_string(),
            "license cache fingerprint: missing".to_string(),
            "license cache instance_id: missing".to_string(),
            "offline grace: expired at unix 200".to_string(),
        ]
    );
}

#[test]
fn cache_diagnostics_report_absent_grace() {
    assert_eq!(
        format_cache_diagnostics(&LicenseCache::default(), 1),
        vec![
            "raw license key persisted: no".to_string(),
            "license cache identity: missing".to_string(),
            "license cache fingerprint: missing".to_string(),
            "license cache instance_id: missing".to_string(),
            "offline grace: absent".to_string(),
        ]
    );
}

#[test]
fn cache_diagnostics_reject_uppercase_fingerprint_shape() {
    let cache = LicenseCache {
        license_key_fingerprint: Some("A".repeat(64)),
        instance_id: Some("instance-123".to_string()),
        ..LicenseCache::default()
    };

    assert_eq!(
        format_cache_diagnostics(&cache, 1),
        vec![
            "raw license key persisted: no".to_string(),
            "license cache identity: missing".to_string(),
            "license cache fingerprint: missing".to_string(),
            "license cache instance_id: present".to_string(),
            "offline grace: absent".to_string(),
        ]
    );
}
