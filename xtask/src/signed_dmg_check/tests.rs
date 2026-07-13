use std::io::Write;

use super::{check, Request};

#[test]
fn accepts_signed_candidate_that_differs_from_unsigned_input() {
    let directory = tempfile::tempdir().unwrap();
    let unsigned = write_dmg(&directory.path().join("unsigned"), b"unsigned");
    let signed = write_dmg(&directory.path().join("signed"), b"signed");

    assert!(check(&Request { signed, unsigned }).is_ok());
}

#[test]
fn rejects_signed_candidate_with_same_path_as_unsigned_input() {
    let directory = tempfile::tempdir().unwrap();
    let unsigned = write_dmg(directory.path(), b"unsigned");

    let error = check(&Request {
        signed: unsigned.clone(),
        unsigned,
    })
    .unwrap_err();

    assert!(error.contains("unsigned input path"));
}

#[test]
fn rejects_signed_candidate_with_identical_bytes() {
    let directory = tempfile::tempdir().unwrap();
    let unsigned = write_dmg(&directory.path().join("unsigned"), b"same");
    let signed = write_dmg(&directory.path().join("signed"), b"same");

    let error = check(&Request { signed, unsigned }).unwrap_err();

    assert!(error.contains("must differ"));
}

#[test]
fn rejects_noncanonical_signed_candidate_name() {
    let directory = tempfile::tempdir().unwrap();
    let unsigned = write_dmg(&directory.path().join("unsigned"), b"unsigned");
    let signed = write_named_dmg(&directory.path().join("signed"), "Other.dmg", b"signed");

    let error = check(&Request { signed, unsigned }).unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

#[test]
fn rejects_signed_candidate_with_nix_store_reference() {
    let directory = tempfile::tempdir().unwrap();
    let unsigned = write_dmg(&directory.path().join("unsigned"), b"unsigned");
    let signed = write_dmg(&directory.path().join("signed"), b"/nix/store/abc");

    let error = check(&Request { signed, unsigned }).unwrap_err();

    assert!(error.contains("/nix/store"));
}

fn write_dmg(directory: &std::path::Path, prefix: &[u8]) -> std::path::PathBuf {
    write_named_dmg(directory, "DropSquash.dmg", prefix)
}

fn write_named_dmg(directory: &std::path::Path, name: &str, prefix: &[u8]) -> std::path::PathBuf {
    std::fs::create_dir_all(directory).unwrap();
    let path = directory.join(name);
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
