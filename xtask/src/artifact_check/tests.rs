use std::io::Write;

use super::check_file;

#[test]
fn accepts_artifact_without_nix_store_reference() {
    let (_directory, path) = write_artifact(&dmg_bytes(b"DropSquash artifact"));

    assert!(check_file(&path).is_ok());
}

#[test]
fn rejects_artifact_with_nix_store_reference() {
    let (_directory, path) = write_artifact(&dmg_bytes(b"linked to /nix/store/abc"));
    let error = check_file(&path).unwrap_err();

    assert!(error.contains("/nix/store"));
}

#[test]
fn rejects_artifact_with_utf16_nix_store_reference() {
    let reference = "/nix/store/abc"
        .encode_utf16()
        .flat_map(|value| value.to_le_bytes())
        .collect::<Vec<_>>();
    let (_directory, path) = write_artifact(&dmg_bytes(&reference));

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
fn rejects_non_udif_dmg_artifacts() {
    let (_directory, path) = write_artifact(b"not really a dmg");
    let error = check_file(&path).unwrap_err();

    assert!(error.contains("UDIF DMG"));
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

#[test]
fn rejects_noncanonical_dmg_name() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("Other.dmg");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&dmg_bytes(b"other artifact"))
        .unwrap();

    let error = check_file(&path).unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
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
