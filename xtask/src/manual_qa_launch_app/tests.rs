use super::paths::app_bundle;
use super::{ensure_dmg_isolated, home_dir, parse_args, state_dir};

#[test]
fn parses_optional_event_log_flag() {
    assert_eq!(
        parse_args(vec![
            "--event-log".into(),
            "/tmp/DropSquash.dmg".into(),
            "/tmp/state/Library/Application Support/DropSquash/config.json".into()
        ])
        .unwrap(),
        (
            true,
            false,
            false,
            None,
            5,
            None,
            "/tmp/DropSquash.dmg".into(),
            "/tmp/state/Library/Application Support/DropSquash/config.json".into()
        )
    );
}

#[test]
fn parses_open_panel_flag_order_independently() {
    assert_eq!(
        parse_args(vec![
            "--open-panel".into(),
            "--event-log".into(),
            "--mount-dmg".into(),
            "/tmp/DropSquash.dmg".into(),
            "/tmp/state/Library/Application Support/DropSquash/config.json".into()
        ])
        .unwrap(),
        (
            true,
            true,
            true,
            None,
            5,
            None,
            "/tmp/DropSquash.dmg".into(),
            "/tmp/state/Library/Application Support/DropSquash/config.json".into()
        )
    );
}

#[test]
fn parses_open_file_option() {
    assert_eq!(
        parse_args(vec![
            "--open-file".into(),
            "/tmp/sample.mov".into(),
            "/tmp/DropSquash.app".into(),
            "/tmp/state/Library/Application Support/DropSquash/config.json".into()
        ])
        .unwrap(),
        (
            false,
            false,
            false,
            Some("/tmp/sample.mov".into()),
            5,
            None,
            "/tmp/DropSquash.app".into(),
            "/tmp/state/Library/Application Support/DropSquash/config.json".into()
        )
    );
}

#[test]
fn parses_custom_settle_seconds_option() {
    assert_eq!(
        parse_args(vec![
            "--settle-seconds".into(),
            "9".into(),
            "--open-file".into(),
            "/tmp/sample.mov".into(),
            "/tmp/DropSquash.app".into(),
            "/tmp/state/Library/Application Support/DropSquash/config.json".into()
        ])
        .unwrap(),
        (
            false,
            false,
            false,
            Some("/tmp/sample.mov".into()),
            9,
            None,
            "/tmp/DropSquash.app".into(),
            "/tmp/state/Library/Application Support/DropSquash/config.json".into()
        )
    );
}

#[test]
fn parses_license_api_base_url_option() {
    assert_eq!(
        parse_args(vec![
            "--license-api-base-url".into(),
            "http://127.0.0.1:9/licenses".into(),
            "/tmp/DropSquash.app".into(),
            "/tmp/state/Library/Application Support/DropSquash/config.json".into()
        ])
        .unwrap(),
        (
            false,
            false,
            false,
            None,
            5,
            Some("http://127.0.0.1:9/licenses".into()),
            "/tmp/DropSquash.app".into(),
            "/tmp/state/Library/Application Support/DropSquash/config.json".into()
        )
    );
}

#[test]
fn derives_home_dir_and_sibling_app() {
    let dir = tempfile::tempdir().unwrap();
    let app = dir.path().join("macos/DropSquash.app");
    std::fs::create_dir_all(&app).unwrap();
    let dmg = dir.path().join("dmg/DropSquash.dmg");
    std::fs::create_dir_all(dmg.parent().unwrap()).unwrap();
    std::fs::write(&dmg, []).unwrap();
    assert_eq!(
        home_dir("/tmp/state/Library/Application Support/DropSquash/config.json").unwrap(),
        "/tmp/state"
    );
    assert_eq!(
        state_dir("/tmp/state/Library/Application Support/DropSquash/config.json").unwrap(),
        "/tmp/state/Library/Application Support/DropSquash"
    );
    assert_eq!(
        app_bundle(dmg.to_str().unwrap()).unwrap(),
        app.display().to_string()
    );
}

#[test]
fn allows_app_bundle_without_installed_app_check() {
    assert!(ensure_dmg_isolated("/tmp/DropSquash.app").is_ok());
}
