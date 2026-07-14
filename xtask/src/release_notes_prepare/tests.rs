use super::{git_state::clean_status, require_dmg_name, url, Input, PreparedNotes};
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
fn rejects_duplicate_markdown_output() {
    let error = Input::parse(vec![
        "target/release/bundle/dmg/DropSquash.dmg".into(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        "--markdown-output".into(),
        "/tmp/first.md".into(),
        "--markdown-output".into(),
        "/tmp/second.md".into(),
    ])
    .unwrap_err();

    assert!(error.contains("at most once"));
}

#[test]
fn rejects_unknown_options() {
    let error = Input::parse(vec![
        "target/release/bundle/dmg/DropSquash.dmg".into(),
        "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        "--draft".into(),
    ])
    .unwrap_err();

    assert!(error.contains("unknown release-notes-prepare argument"));
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
fn detects_dirty_git_status() {
    assert!(clean_status(""));
    assert!(clean_status("\n"));
    assert!(!clean_status(" M docs/release.md\n"));
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
    assert!(text.contains("## macOS Verification"));
    assert!(text.contains("macOS verification commands:"));
    assert!(text.contains("codesign --verify --deep --strict --verbose=4 /tmp/DropSquash.dmg"));
    assert!(text.contains("codesign -dv --verbose=4 /tmp/DropSquash.dmg"));
    assert!(text.contains("spctl --assess --type open --verbose=4 /tmp/DropSquash.dmg"));
    assert!(text.contains("xcrun stapler validate /tmp/DropSquash.dmg"));
    assert!(text.contains(
        "- `codesign`: pending codesign --verify and codesign -dv Developer ID verification"
    ));
    assert!(text.contains("- `spctl`: pending spctl --assess --type open Gatekeeper assessment"));
    assert!(text.contains("- `stapler`: pending stapled ticket validation"));
    assert!(text.contains("- Apple notary log: pending notarytool accepted log"));
    assert!(text.contains("- Gatekeeper clean-machine open: pending clean-machine open test"));
    assert!(text.contains("signed, notarized, stapled, and no warning"));
    assert!(text.contains("## Productization Evidence"));
    assert!(text.contains("- Public website URL: pending production deployment"));
    assert!(text.contains("- Pricing URL: pending final pricing"));
    assert!(text.contains("- Refund policy URL: pending final refund policy"));
    assert!(text.contains("- Live checkout URL: pending live checkout"));
    assert!(text.contains("- GitHub Release URL:"));
    assert!(text.contains("releases/tag/v0.1.0"));
    assert!(text.contains("- SHA256SUMS line:"));
    assert!(text.contains("- GitHub Release checksum:"));
    assert!(text.contains("public https://github.com/mt4110/drop-squash/releases/tag/v0.1.0"));
    assert!(text.contains("the exact Artifact URL"));
    assert!(text.contains("GitHub Release URL above"));
    assert!(text.contains("Artifact URL above"));
    assert!(text.contains("with lowercase SHA-256"));
    assert!(text.contains("SHA256SUMS output command:"));
    assert!(text.contains("checksum /tmp/DropSquash.dmg --output /tmp/SHA256SUMS"));
    assert!(text.contains("GitHub Release command plan:"));
    assert!(text.contains(
        "github-release-plan v0.1.0 /tmp/DropSquash.dmg /tmp/SHA256SUMS /tmp/dropsquash-release-notes.md"
    ));
    assert!(text.contains("pending upload"));
    assert!(!text.contains("GitHub Release checksum after upload"));
    assert!(text.contains("Homebrew cask command"));
    assert!(text.contains("homebrew-cask 0.1.0"));
    assert!(text.contains("> packaging/homebrew/Casks/dropsquash.rb"));
    assert!(text.contains("Homebrew cask check command"));
    assert!(text.contains(
        "homebrew-cask-check packaging/homebrew/Casks/dropsquash.rb /tmp/dropsquash-release-notes.md"
    ));
    assert!(text.contains("Homebrew tap PR evidence draft"));
    assert!(text.contains("- Homebrew tap PR URL: pending tap PR"));
    assert!(text.contains("versioned DropSquash.dmg"));
    assert!(text.contains("Homebrew tap PR URL above"));
    assert!(text.contains("reviewed public PR evidence"));
    assert!(text.contains("uses the Artifact URL above"));
    assert!(text.contains(
        "with lowercase SHA-256 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
    ));
    assert!(text.contains("replace this line with reviewed public PR evidence"));
    assert!(text.contains("auto_updates false"));
    assert!(text.contains("zap cleanup path"));
    assert!(text.contains("Homebrew install result evidence draft"));
    assert!(text.contains("brew install --cask mt4110/tap/dropsquash"));
    assert!(text.contains("brew uninstall --cask mt4110/tap/dropsquash"));
    assert!(text.contains("from the Homebrew tap PR URL above installs"));
    assert!(text.contains("from the Artifact URL above"));
    assert!(text.contains("replace this line with observed install and uninstall evidence"));
    let checksum = text
        .find("GitHub Release checksum")
        .expect("checksum field");
    let release_url = text
        .find("- GitHub Release URL:")
        .expect("release URL field");
    let artifact_heading = text.find("## Artifact").expect("artifact heading");
    let productization_heading = text
        .find("## Productization Evidence")
        .expect("productization heading");
    let verification_heading = text
        .find("## macOS Verification")
        .expect("verification heading");
    let distribution_heading = text.find("## Distribution").expect("distribution heading");
    assert!(artifact_heading < verification_heading);
    assert!(verification_heading < productization_heading);
    assert!(productization_heading < distribution_heading);
    assert!(checksum < release_url);
}

#[test]
fn prepared_public_url_drafts_are_not_release_evidence() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("release-notes.md");
    let notes = PreparedNotes {
        version: "0.1.0".into(),
        artifact_path: "/tmp/DropSquash.dmg".into(),
        artifact_url:
            "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
        commit: "abc1234".into(),
    };
    std::fs::write(&path, notes.lines().join("\n")).unwrap();

    let error = crate::release_notes_check::check_file(&path).unwrap_err();

    assert!(error.contains("Public website URL"));
    assert!(error.contains("Pricing URL"));
    assert!(error.contains("Refund policy URL"));
    assert!(error.contains("Live checkout URL"));
}

#[test]
fn prepared_macos_verification_drafts_are_not_release_evidence() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("release-notes.md");
    let notes = PreparedNotes {
        version: "0.1.0".into(),
        artifact_path: "/tmp/DropSquash.dmg".into(),
        artifact_url:
            "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
        commit: "abc1234".into(),
    };
    std::fs::write(&path, notes.lines().join("\n")).unwrap();
    let error = crate::release_notes_check::check_file(&path).unwrap_err();

    assert!(error.contains("`codesign`"));
    assert!(error.contains("Apple notary log"));
    assert!(error.contains("Gatekeeper clean-machine open"));
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

    assert!(text.contains(
        "checksum '/tmp/drop squash/DropSquash.dmg' --output '/tmp/drop squash/SHA256SUMS'"
    ));
}

#[test]
fn shell_quotes_macos_verification_command_paths() {
    let notes = PreparedNotes {
        version: "0.1.0".into(),
        artifact_path: "/tmp/drop squash/DropSquash.dmg".into(),
        artifact_url:
            "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg".into(),
        sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".into(),
        commit: "abc1234".into(),
    };
    let text = notes.lines().join("\n");

    assert!(text.contains(
        "codesign --verify --deep --strict --verbose=4 '/tmp/drop squash/DropSquash.dmg'"
    ));
    assert!(text.contains("codesign -dv --verbose=4 '/tmp/drop squash/DropSquash.dmg'"));
    assert!(
        text.contains("spctl --assess --type open --verbose=4 '/tmp/drop squash/DropSquash.dmg'")
    );
    assert!(text.contains("xcrun stapler validate '/tmp/drop squash/DropSquash.dmg'"));
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
