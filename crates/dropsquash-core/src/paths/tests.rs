use super::*;

#[test]
fn state_dir_override_controls_state_files() {
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("DROP_SQUASH_APP_STATE_DIR", dir.path());
    assert_eq!(default_config_path(), dir.path().join("config.json"));
    assert_eq!(default_history_path(), dir.path().join("history.jsonl"));
    assert_eq!(
        default_license_cache_path(),
        dir.path().join("license.json")
    );
    std::env::remove_var("DROP_SQUASH_APP_STATE_DIR");
}

#[test]
fn home_override_controls_default_output_dir() {
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("DROP_SQUASH_HOME", dir.path());
    assert_eq!(
        default_output_dir(),
        dir.path().join("Movies").join("DropSquash")
    );
    std::env::remove_var("DROP_SQUASH_HOME");
}
