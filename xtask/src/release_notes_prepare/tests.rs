use super::{require_dmg_name, url, Input, PreparedNotes};
use std::io::Write;

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
    url::validate(
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg",
        "0.1.0",
    )
    .unwrap();
}

#[test]
fn rejects_nested_artifact_url() {
    let error = url::validate(
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/nested/DropSquash.dmg",
        "0.1.0",
    )
    .unwrap_err();

    assert!(error.contains("Artifact URL"));
}

#[test]
fn rejects_artifact_with_nix_store_reference() {
    let (_directory, artifact) = write_dmg(b"linked to /nix/store/abc");
    let input = Input {
        artifact,
        artifact_url:
            "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
    };

    let error = match PreparedNotes::current(&input) {
        Ok(_) => panic!("artifact with /nix/store reference should fail"),
        Err(error) => error,
    };

    assert!(error.contains("/nix/store"));
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
    assert!(text.contains("- GitHub Release URL:"));
    assert!(text.contains("releases/tag/v0.1.0"));
    assert!(text.contains("- SHA256SUMS line:"));
    assert!(text.contains("GitHub Release checksum"));
    assert!(text.contains("after upload"));
    assert!(text.contains("Homebrew cask command"));
    assert!(text.contains("homebrew-cask 0.1.0"));
    let checksum = text
        .find("GitHub Release checksum")
        .expect("checksum field");
    let release_url = text
        .find("- GitHub Release URL:")
        .expect("release URL field");
    assert!(checksum < release_url);
}

fn write_dmg(prefix: &[u8]) -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.dmg");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&dmg_bytes(prefix))
        .unwrap();
    (directory, path)
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}
