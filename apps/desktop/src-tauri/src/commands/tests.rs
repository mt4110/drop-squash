use dropsquash_core::{
    AppConfig, LicenseState, LockedReason, OutputSize, Profile, TRIAL_CONVERSION_LIMIT,
};

use super::dto::drop_zone_state;

#[test]
fn exposes_placeholder_drop_zone_state() {
    let state = drop_zone_state(
        &AppConfig::default(),
        LicenseState::Trial(dropsquash_core::TrialState {
            successful_conversions: 0,
            limit: TRIAL_CONVERSION_LIMIT,
        }),
        vec!["mov".to_string(), "mp4".to_string(), "m4v".to_string()],
    );
    assert_eq!(state.product_name, "DropSquash");
    assert_eq!(state.privacy_mode, "local-only");
    assert_eq!(state.output_size, OutputSize::Auto);
    assert!(state.write_privacy_receipt);
    assert_eq!(state.profiles.len(), Profile::DELIVERY.len());
    assert_eq!(state.output_sizes.len(), OutputSize::ALL.len());
    assert_eq!(state.input_extensions, ["mov", "mp4", "m4v"]);
    assert!(!state.is_locked);
    assert_eq!(state.locked_reason, None);
}

#[test]
fn exposes_license_refresh_lock_reason() {
    let state = drop_zone_state(
        &AppConfig::default(),
        LicenseState::Locked {
            trial: dropsquash_core::TrialState {
                successful_conversions: 3,
                limit: TRIAL_CONVERSION_LIMIT,
            },
            reason: LockedReason::LicenseRefreshRequired,
        },
        vec!["mov".to_string()],
    );

    assert!(state.is_locked);
    assert_eq!(state.locked_reason, Some("license-refresh-required"));
}
