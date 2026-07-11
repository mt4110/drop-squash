use dropsquash_core::{LicenseState, TrialState};
use dropsquash_license::{license_key_fingerprint, LicenseCache};

use super::{forget_at_path, forget_lines, format_state};

#[test]
fn formats_pro_state_without_trial_count() {
    assert_eq!(format_state(LicenseState::Pro), vec!["license state: Pro"]);
}

#[test]
fn formats_trial_state_with_usage() {
    assert_eq!(
        format_state(LicenseState::Trial(TrialState {
            successful_conversions: 3,
            limit: 10,
        })),
        vec![
            "license state: Trial".to_string(),
            "trial: 3/10 successful conversions used".to_string(),
        ]
    );
}

#[test]
fn formats_locked_state_with_usage() {
    assert_eq!(
        format_state(LicenseState::Locked(TrialState {
            successful_conversions: 10,
            limit: 10,
        })),
        vec![
            "license state: Locked".to_string(),
            "trial: 10/10 successful conversions used".to_string(),
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
