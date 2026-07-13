use std::io::Write;

use super::{command, Request};

#[test]
fn prints_gatekeeper_assessment_command() {
    let directory = tempfile::tempdir().unwrap();
    let target = write_dmg(directory.path(), b"signed");

    let line = command(&Request { target }).unwrap();

    assert!(line.starts_with("spctl --assess --type open --verbose=4 "));
    assert!(line.ends_with("DropSquash.dmg"));
}

#[test]
fn quotes_spaces_in_target_path() {
    let directory = tempfile::tempdir().unwrap();
    let target = write_dmg(&directory.path().join("drop squash"), b"signed");

    let text = command(&Request { target }).unwrap();

    assert!(text.contains("'"));
    assert!(text.contains("drop squash/DropSquash.dmg"));
}

#[test]
fn rejects_noncanonical_target() {
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("Other.dmg");
    std::fs::File::create(&target)
        .unwrap()
        .write_all(&dmg_bytes(b"signed"))
        .unwrap();

    let error = command(&Request { target }).unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

#[test]
fn rejects_target_with_nix_store_reference() {
    let directory = tempfile::tempdir().unwrap();
    let target = write_dmg(directory.path(), b"/nix/store/abc");

    let error = command(&Request { target }).unwrap_err();

    assert!(error.contains("/nix/store"));
}

fn write_dmg(directory: &std::path::Path, prefix: &[u8]) -> std::path::PathBuf {
    std::fs::create_dir_all(directory).unwrap();
    let path = directory.join("DropSquash.dmg");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&dmg_bytes(prefix))
        .unwrap();
    path
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}
