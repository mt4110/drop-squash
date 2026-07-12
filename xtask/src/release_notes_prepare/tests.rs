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
    assert!(input.markdown_output.is_none());
}

#[test]
fn rejects_wrong_argument_count() {
    let error = Input::parse(vec!["DropSquash.dmg".into()]).unwrap_err();

    assert!(error.contains("release-notes-prepare"));
}

#[test]
fn parses_markdown_output() {
    let input = Input::parse(vec![
        "target/release/bundle/dmg/DropSquash.dmg".into(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        "--markdown-output".into(),
        "/tmp/dropsquash-release-notes-prepared.md".into(),
    ])
    .unwrap();

    assert_eq!(
        input.markdown_output.unwrap(),
        std::path::PathBuf::from("/tmp/dropsquash-release-notes-prepared.md")
    );
}

#[test]
fn rejects_relative_markdown_output() {
    let error = Input::parse(vec![
        "target/release/bundle/dmg/DropSquash.dmg".into(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        "--markdown-output".into(),
        "../release-notes.md".into(),
    ])
    .unwrap_err();

    assert!(error.contains("absolute"));
}

#[test]
fn rejects_non_markdown_output() {
    let error = Input::parse(vec![
        "target/release/bundle/dmg/DropSquash.dmg".into(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        "--markdown-output".into(),
        "/tmp/release-notes.txt".into(),
    ])
    .unwrap_err();

    assert!(error.contains(".md"));
}

#[test]
fn rejects_markdown_output_inside_repository() {
    let error = Input::parse(vec![
        "target/release/bundle/dmg/DropSquash.dmg".into(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        "--markdown-output".into(),
        std::env::current_dir()
            .unwrap()
            .join("release-notes-prepared.md")
            .display()
            .to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("outside the repository"));
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
        markdown_output: None,
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
        artifact_path: "/tmp/DropSquash.dmg".into(),
        artifact_url:
            "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
        commit: "abc1234".into(),
    };
    let text = notes.lines().join("\n");

    assert!(text.contains("## Artifact"));
    assert!(text.contains("## Distribution"));
    assert!(text.contains("- Version: v0.1.0"));
    assert!(text.contains("- Artifact: DropSquash.dmg"));
    assert!(text.contains("- Git commit: abc1234"));
    assert!(text.contains("- GitHub Release URL:"));
    assert!(text.contains("releases/tag/v0.1.0"));
    assert!(text.contains("- SHA256SUMS line:"));
    assert!(text.contains("- GitHub Release checksum:"));
    assert!(text.contains("SHA256SUMS output command:"));
    assert!(text.contains("checksum /tmp/DropSquash.dmg --output SHA256SUMS"));
    assert!(text.contains("pending upload"));
    assert!(!text.contains("GitHub Release checksum after upload"));
    assert!(text.contains("Homebrew cask command"));
    assert!(text.contains("homebrew-cask 0.1.0"));
    assert!(text.contains("Homebrew tap PR evidence draft"));
    assert!(text.contains("versioned DropSquash.dmg"));
    assert!(text
        .contains("with SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"));
    assert!(text.contains("replace this line with reviewed PR evidence"));
    assert!(text.contains("auto_updates false"));
    assert!(text.contains("zap cleanup path"));
    assert!(text.contains("Homebrew install result evidence draft"));
    assert!(text.contains("brew install --cask mt4110/tap/dropsquash"));
    assert!(text.contains("replace this line with observed install evidence"));
    let checksum = text
        .find("GitHub Release checksum")
        .expect("checksum field");
    let release_url = text
        .find("- GitHub Release URL:")
        .expect("release URL field");
    let artifact_heading = text.find("## Artifact").expect("artifact heading");
    let distribution_heading = text.find("## Distribution").expect("distribution heading");
    assert!(artifact_heading < distribution_heading);
    assert!(checksum < release_url);
}

#[test]
fn shell_quotes_checksum_command_path() {
    let notes = PreparedNotes {
        version: "0.1.0".into(),
        artifact_path: "/tmp/drop squash/DropSquash.dmg".into(),
        artifact_url:
            "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
        commit: "abc1234".into(),
    };
    let text = notes.lines().join("\n");

    assert!(text.contains("checksum '/tmp/drop squash/DropSquash.dmg' --output SHA256SUMS"));
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
