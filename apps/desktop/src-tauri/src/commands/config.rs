use std::path::PathBuf;

use dropsquash_core::{
    default_config_path, AppConfig, AppError, LicenseState, OutputSize, Profile, SourcePolicy,
};
use dropsquash_encoder::EncoderBackend;

use crate::state::AppState;

use super::dto::{drop_zone_state, DropZoneState, SavedConfig};
use super::format_error;
use super::license::current_license_state;

#[cfg(target_os = "macos")]
use dropsquash_encoder::AppleNativeEncoder as NativeEncoder;
#[cfg(target_os = "linux")]
use dropsquash_encoder::GStreamerEncoder as NativeEncoder;
#[cfg(target_os = "windows")]
use dropsquash_encoder::MediaFoundationEncoder as NativeEncoder;

pub async fn load_state(
    app_state: tauri::State<'_, AppState>,
) -> std::result::Result<DropZoneState, String> {
    let config = load_config(&app_state)?;
    let license_state = current_license_state().await.map_err(format_error)?;
    let capabilities = NativeEncoder.probe_capabilities().map_err(format_error)?;
    Ok(drop_zone_state(
        &config,
        license_state,
        capabilities.input_extensions,
    ))
}

pub async fn state_for_license(
    license_state: LicenseState,
) -> std::result::Result<DropZoneState, String> {
    let config = AppConfig::load_or_default(&default_config_path()).map_err(format_error)?;
    let capabilities = NativeEncoder.probe_capabilities().map_err(format_error)?;
    Ok(drop_zone_state(
        &config,
        license_state,
        capabilities.input_extensions,
    ))
}

pub fn save_config(
    app_state: tauri::State<'_, AppState>,
    output_dir: String,
    profile: Profile,
    output_size: OutputSize,
    source_policy: SourcePolicy,
    write_privacy_receipt: bool,
) -> std::result::Result<SavedConfig, String> {
    let output_dir = PathBuf::from(output_dir);
    if output_dir.as_os_str().is_empty() {
        return Err(format_error(AppError::InvalidConfig(
            "output directory cannot be empty".to_string(),
        )));
    }

    let _guard = app_state.config_lock.lock().map_err(lock_error)?;
    let config_path = default_config_path();
    let mut config = AppConfig::load_or_default(&config_path).map_err(format_error)?;
    config.output_dir = output_dir;
    config.default_profile = profile.desktop_profile();
    config.default_output_size = output_size;
    config.source_policy = source_policy;
    config.write_privacy_receipt = write_privacy_receipt;
    config.save_to_path(&config_path).map_err(format_error)?;
    Ok(saved_config(config))
}

fn load_config(app_state: &tauri::State<'_, AppState>) -> Result<AppConfig, String> {
    let _guard = app_state.config_lock.lock().map_err(lock_error)?;
    AppConfig::load_or_default(&default_config_path()).map_err(format_error)
}

fn lock_error<T>(_error: T) -> String {
    format_error(AppError::InvalidConfig(
        "config lock was poisoned".to_string(),
    ))
}

fn saved_config(config: AppConfig) -> SavedConfig {
    SavedConfig {
        output_dir: config.output_dir.display().to_string(),
        profile: config.default_profile.desktop_profile(),
        output_size: config.default_output_size,
        source_policy: config.source_policy,
        write_privacy_receipt: config.write_privacy_receipt,
    }
}
