use std::path::PathBuf;

use super::*;

#[test]
fn missing_config_loads_defaults() {
    let directory = tempfile::tempdir().unwrap();
    let config = AppConfig::load_or_default(&directory.path().join("missing.json")).unwrap();

    assert_eq!(config, AppConfig::default());
}

#[test]
fn saves_and_loads_config() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("nested").join("config.json");
    let config = AppConfig {
        output_dir: PathBuf::from("/tmp/DropSquash"),
        default_profile: Profile::Docs,
        default_output_size: OutputSize::P720,
        source_policy: SourcePolicy::Ask,
        write_privacy_receipt: false,
        trial_conversion_limit: TRIAL_CONVERSION_LIMIT,
    };

    config.save_to_path(&path).unwrap();

    assert_eq!(AppConfig::load_or_default(&path).unwrap(), config);
}

#[test]
fn save_does_not_leave_temp_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("config.json");

    AppConfig::default().save_to_path(&path).unwrap();

    assert!(std::fs::read_dir(directory.path())
        .unwrap()
        .all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .ends_with(".tmp")));
}

#[test]
fn missing_fields_fall_back_to_defaults() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("config.json");
    std::fs::write(
        &path,
        r#"{
  "default_profile": "docs",
  "default_output_size": "720p"
}"#,
    )
    .unwrap();

    let config = AppConfig::load_or_default(&path).unwrap();

    assert_eq!(config.default_profile, Profile::Docs);
    assert_eq!(config.default_output_size, OutputSize::P720);
    assert_eq!(config.output_dir, default_output_dir());
    assert_eq!(config.source_policy, SourcePolicy::Ask);
    assert!(config.write_privacy_receipt);
    assert_eq!(config.trial_conversion_limit, TRIAL_CONVERSION_LIMIT);
}
