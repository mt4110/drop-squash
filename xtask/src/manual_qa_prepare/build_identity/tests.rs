use super::{read_version, BuildIdentity};

#[test]
fn formats_manual_qa_app_build() {
    let identity = BuildIdentity::from_parts("0.1.0".to_string(), "abc1234".to_string()).unwrap();

    assert_eq!(identity.app_build(), "DropSquash 0.1.0 git abc1234");
}

#[test]
fn rejects_non_semver_version() {
    let error = BuildIdentity::from_parts("0.1".to_string(), "abc1234".to_string()).unwrap_err();

    assert!(error.contains("major.minor.patch"));
}

#[test]
fn rejects_non_hex_commit() {
    let error = BuildIdentity::from_parts("0.1.0".to_string(), "revision".to_string()).unwrap_err();

    assert!(error.contains("hexadecimal"));
}

#[test]
fn reads_tauri_version() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("tauri.conf.json");
    std::fs::write(&path, r#"{ "version": "0.1.0" }"#).unwrap();

    assert_eq!(read_version(&path).unwrap(), "0.1.0");
}
