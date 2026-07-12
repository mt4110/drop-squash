use super::{require_dmg_name, validate_artifact_url, Input, PreparedNotes};

#[test]
fn parses_artifact_and_url() {
    let input = Input::parse(vec![
        "target/release/bundle/dmg/DropSquash.dmg".into(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
    ])
    .unwrap();

    assert_eq!(input.artifact.file_name().unwrap(), "DropSquash.dmg");
}

#[test]
fn rejects_wrong_argument_count() {
    let error = Input::parse(vec!["DropSquash.dmg".into()]).unwrap_err();

    assert!(error.contains("release-notes-prepare"));
}

#[test]
fn rejects_wrong_artifact_name() {
    let error = require_dmg_name(std::path::Path::new("/tmp/Other.dmg")).unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

#[test]
fn accepts_public_artifact_url() {
    validate_artifact_url(
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg",
    )
    .unwrap();
}

#[test]
fn rejects_nested_artifact_url() {
    let error = validate_artifact_url(
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/nested/DropSquash.dmg",
    )
    .unwrap_err();

    assert!(error.contains("Artifact URL"));
}

#[test]
fn renders_prepared_release_notes_fields() {
    let notes = PreparedNotes {
        version: "0.1.0".into(),
        artifact_url:
            "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
        commit: "abc1234".into(),
    };
    let text = notes.lines().join("\n");

    assert!(text.contains("- Version: v0.1.0"));
    assert!(text.contains("- Artifact: DropSquash.dmg"));
    assert!(text.contains("- Git commit: abc1234"));
    assert!(text.contains("GitHub Release checksum"));
}
