use std::io::Write;

use super::checksum_line;

#[test]
fn checksum_line_uses_sha256sum_format() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.dmg");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&dmg_bytes(b"dropsquash"))
        .unwrap();

    let line = checksum_line(&path).unwrap();

    assert!(!line.ends_with("  "));
    assert!(line.contains("  "));
    assert!(line.ends_with("  DropSquash.dmg"));
    assert_eq!(line.split("  ").next().unwrap().len(), 64);
}

#[test]
fn checksum_line_omits_parent_directories() {
    let directory = tempfile::tempdir().unwrap();
    let nested = directory.path().join("bundle").join("dmg");
    std::fs::create_dir_all(&nested).unwrap();
    let path = nested.join("DropSquash.dmg");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&dmg_bytes(b"dropsquash"))
        .unwrap();

    let line = checksum_line(&path).unwrap();

    assert!(line.ends_with("  DropSquash.dmg"));
    assert!(!line.contains("bundle/dmg"));
}

#[test]
fn directories_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let error = checksum_line(directory.path()).unwrap_err();

    assert!(error.contains("not a file"));
}

#[test]
fn empty_files_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("empty.dmg");
    std::fs::File::create(&path).unwrap();

    let error = checksum_line(&path).unwrap_err();

    assert!(error.contains("empty"));
}

#[test]
fn non_udif_dmg_files_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.dmg");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(b"not really a dmg")
        .unwrap();

    let error = checksum_line(&path).unwrap_err();

    assert!(error.contains("UDIF DMG"));
}

#[test]
fn non_dmg_files_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.zip");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(b"dropsquash")
        .unwrap();

    let error = checksum_line(&path).unwrap_err();

    assert!(error.contains("must be a DMG"));
}

#[test]
fn noncanonical_dmg_names_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("Other.dmg");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&dmg_bytes(b"dropsquash"))
        .unwrap();

    let error = checksum_line(&path).unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}
