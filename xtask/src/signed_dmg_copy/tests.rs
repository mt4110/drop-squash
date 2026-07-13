use std::io::Write;

use super::{copy, Request};

#[test]
fn copies_unsigned_dmg_to_canonical_signed_target() {
    let directory = tempfile::tempdir().unwrap();
    let unsigned = write_dmg(&directory.path().join("unsigned"), b"unsigned");
    let output = directory.path().join("signed");

    let target = copy(&Request {
        unsigned: unsigned.clone(),
        output_dir: output.clone(),
    })
    .unwrap();

    assert_eq!(target, output.join("DropSquash.dmg"));
    assert_eq!(
        std::fs::read(target).unwrap(),
        std::fs::read(unsigned).unwrap()
    );
}

#[test]
fn rejects_existing_signed_target() {
    let directory = tempfile::tempdir().unwrap();
    let unsigned = write_dmg(&directory.path().join("unsigned"), b"unsigned");
    let output = directory.path().join("signed");
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(output.join("DropSquash.dmg"), b"old").unwrap();

    let error = copy(&Request {
        unsigned,
        output_dir: output,
    })
    .unwrap_err();

    assert!(error.contains("already exists"));
}

#[test]
fn rejects_output_that_would_overwrite_unsigned_input() {
    let directory = tempfile::tempdir().unwrap();
    let unsigned = write_dmg(directory.path(), b"unsigned");

    let error = copy(&Request {
        unsigned,
        output_dir: directory.path().to_path_buf(),
    })
    .unwrap_err();

    assert!(error.contains("must not overwrite"));
}

#[test]
fn rejects_unsigned_input_with_nix_store_reference() {
    let directory = tempfile::tempdir().unwrap();
    let unsigned = write_dmg(&directory.path().join("unsigned"), b"/nix/store/abc");

    let error = copy(&Request {
        unsigned,
        output_dir: directory.path().join("signed"),
    })
    .unwrap_err();

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
