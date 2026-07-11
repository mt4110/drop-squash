use dropsquash_core::{AppConfig, LicenseState, OutputSize, Profile, TRIAL_CONVERSION_LIMIT};

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
    assert_eq!(state.profiles.len(), Profile::DELIVERY.len());
    assert_eq!(state.output_sizes.len(), OutputSize::ALL.len());
    assert_eq!(state.input_extensions, ["mov", "mp4", "m4v"]);
    assert!(!state.is_locked);
}
