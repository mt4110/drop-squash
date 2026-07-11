use std::io::Write;

use super::check_file;

#[test]
fn accepts_artifact_without_nix_store_reference() {
    let (_directory, path) = write_artifact(b"DropSquash artifact");

    assert!(check_file(&path).is_ok());
}

#[test]
fn rejects_artifact_with_nix_store_reference() {
    let (_directory, path) = write_artifact(b"linked to /nix/store/abc-drop-squash");
    let error = check_file(&path).unwrap_err();

    assert!(error.contains("/nix/store"));
}

#[test]
fn rejects_directories() {
    let directory = tempfile::tempdir().unwrap();
    let error = check_file(directory.path()).unwrap_err();

    assert!(error.contains("not a file"));
}

#[test]
fn rejects_empty_artifacts() {
    let (_directory, path) = write_artifact(b"");
    let error = check_file(&path).unwrap_err();

    assert!(error.contains("empty"));
}

#[test]
fn rejects_non_dmg_artifacts() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.zip");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(b"not a dmg")
        .unwrap();

    let error = check_file(&path).unwrap_err();

    assert!(error.contains("must be a DMG"));
}

fn write_artifact(bytes: &[u8]) -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.dmg");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(bytes)
        .unwrap();
    (directory, path)
}
