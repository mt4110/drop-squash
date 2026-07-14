use std::io::Write;

use super::{checksum_line, run};

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

#[test]
fn nix_store_references_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("DropSquash.dmg");
    std::fs::File::create(&path)
        .unwrap()
        .write_all(&dmg_bytes(b"/nix/store/abc"))
        .unwrap();

    let error = checksum_line(&path).unwrap_err();

    assert!(error.contains("/nix/store"));
}

#[test]
fn writes_checksum_output_once() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("SHA256SUMS");
    std::fs::File::create(&artifact)
        .unwrap()
        .write_all(&dmg_bytes(b"dropsquash"))
        .unwrap();

    run(vec![
        artifact.display().to_string(),
        "--output".into(),
        output.display().to_string(),
    ])
    .unwrap();
    let text = std::fs::read_to_string(output).unwrap();

    assert!(text.ends_with("  DropSquash.dmg\n"));
}

#[test]
fn creates_checksum_output_parent_directory() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("qa-output").join("SHA256SUMS");
    std::fs::File::create(&artifact)
        .unwrap()
        .write_all(&dmg_bytes(b"dropsquash"))
        .unwrap();

    run(vec![
        artifact.display().to_string(),
        "--output".into(),
        output.display().to_string(),
    ])
    .unwrap();
    let text = std::fs::read_to_string(output).unwrap();

    assert!(text.ends_with("  DropSquash.dmg\n"));
}

#[test]
fn rejects_existing_checksum_output() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("SHA256SUMS");
    std::fs::File::create(&artifact)
        .unwrap()
        .write_all(&dmg_bytes(b"dropsquash"))
        .unwrap();
    std::fs::write(&output, "existing").unwrap();

    let error = run(vec![
        artifact.display().to_string(),
        "--output".into(),
        output.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("failed to create checksum output"));
}

#[test]
fn rejects_noncanonical_checksum_output_name() {
    let directory = tempfile::tempdir().unwrap();
    let artifact = directory.path().join("DropSquash.dmg");
    let output = directory.path().join("checksums.txt");
    std::fs::File::create(&artifact)
        .unwrap()
        .write_all(&dmg_bytes(b"dropsquash"))
        .unwrap();

    let error = run(vec![
        artifact.display().to_string(),
        "--output".into(),
        output.display().to_string(),
    ])
    .unwrap_err();

    assert!(error.contains("SHA256SUMS"));
}

#[test]
fn rejects_output_with_multiple_artifacts() {
    let error = run(vec![
        "first.dmg".into(),
        "second.dmg".into(),
        "--output".into(),
        "SHA256SUMS".into(),
    ])
    .unwrap_err();

    assert!(error.contains("usage: checksum"));
}

fn dmg_bytes(prefix: &[u8]) -> Vec<u8> {
    let mut bytes = prefix.to_vec();
    let mut trailer = vec![0; 512];
    trailer[..4].copy_from_slice(b"koly");
    bytes.extend(trailer);
    bytes
}
