use std::io::Write;

use super::check;

const SHA256: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const URL: &str = "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/DropSquash.dmg";

#[test]
fn accepts_cask_matching_release_notes() {
    let directory = tempfile::tempdir().unwrap();
    let cask = write(
        &directory.path().join("dropsquash.rb"),
        &cask_text("0.1.0", URL, SHA256),
    );
    let notes = write(
        &directory.path().join("release-notes.md"),
        &notes_text("v0.1.0", URL, SHA256),
    );

    check(&cask, &notes).unwrap();
}

#[test]
fn rejects_version_mismatch() {
    let directory = tempfile::tempdir().unwrap();
    let cask = write(
        &directory.path().join("dropsquash.rb"),
        &cask_text("0.1.1", URL, SHA256),
    );
    let notes = write(
        &directory.path().join("release-notes.md"),
        &notes_text("v0.1.0", URL, SHA256),
    );

    let error = check(&cask, &notes).unwrap_err();

    assert!(error.contains("version"));
}

#[test]
fn rejects_artifact_url_mismatch() {
    let directory = tempfile::tempdir().unwrap();
    let cask = write(
        &directory.path().join("dropsquash.rb"),
        &cask_text("0.1.0", URL, SHA256),
    );
    let notes = write(
        &directory.path().join("release-notes.md"),
        &notes_text(
            "v0.1.0",
            "https://github.com/mt4110/drop-squash/releases/download/v0.1.0/Other.dmg",
            SHA256,
        ),
    );

    let error = check(&cask, &notes).unwrap_err();

    assert!(error.contains("Artifact URL"));
}

#[test]
fn rejects_sha256_mismatch() {
    let directory = tempfile::tempdir().unwrap();
    let cask = write(
        &directory.path().join("dropsquash.rb"),
        &cask_text("0.1.0", URL, SHA256),
    );
    let notes = write(
        &directory.path().join("release-notes.md"),
        &notes_text(
            "v0.1.0",
            URL,
            "1111111111111111111111111111111111111111111111111111111111111111",
        ),
    );

    let error = check(&cask, &notes).unwrap_err();

    assert!(error.contains("SHA-256"));
}

#[test]
fn rejects_cask_without_safety_stanzas() {
    let directory = tempfile::tempdir().unwrap();
    let cask = write(
        &directory.path().join("dropsquash.rb"),
        &cask_text("0.1.0", URL, SHA256).replace("auto_updates false\n\n", ""),
    );
    let notes = write(
        &directory.path().join("release-notes.md"),
        &notes_text("v0.1.0", URL, SHA256),
    );

    let error = check(&cask, &notes).unwrap_err();

    assert!(error.contains("auto_updates false"));
}

fn cask_text(version: &str, url: &str, sha256: &str) -> String {
    format!(
        r#"cask "dropsquash" do
  version "{version}"
  sha256 "{sha256}"

  url "{url}"
  name "DropSquash"
  desc "Local screen recording compressor"
  homepage "https://github.com/mt4110/drop-squash"

  auto_updates false

  app "DropSquash.app"

  zap trash: "~/Library/Application Support/DropSquash"
end"#
    )
}

fn notes_text(version: &str, url: &str, sha256: &str) -> String {
    format!(
        r#"- Artifact URL: {url}
- Version: {version}
- Artifact: DropSquash.dmg
- SHA-256: {sha256}
"#
    )
}

fn write(path: &std::path::Path, text: &str) -> std::path::PathBuf {
    std::fs::File::create(path)
        .unwrap()
        .write_all(text.as_bytes())
        .unwrap();
    path.to_path_buf()
}
