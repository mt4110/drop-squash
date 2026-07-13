use std::io::Write;

use super::{prepare, Request};

#[test]
fn prepares_canonical_signed_dmg_target() {
    let directory = tempfile::tempdir().unwrap();
    let source = write_dmg(directory.path(), "unsigned", b"unsigned");
    let output = directory.path().join("signed");

    let target = prepare(&Request {
        source,
        output_dir: output.clone(),
    })
    .unwrap();

    assert_eq!(target, output.join("DropSquash.dmg"));
    assert!(output.is_dir());
    assert!(!target.exists());
}

#[test]
fn rejects_existing_signed_target() {
    let directory = tempfile::tempdir().unwrap();
    let source = write_dmg(directory.path(), "unsigned", b"unsigned");
    let output = directory.path().join("signed");
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(output.join("DropSquash.dmg"), b"old").unwrap();

    let error = prepare(&Request {
        source,
        output_dir: output,
    })
    .unwrap_err();

    assert!(error.contains("already exists"));
}

#[test]
fn rejects_output_that_would_overwrite_unsigned_input() {
    let directory = tempfile::tempdir().unwrap();
    let source = write_dmg(directory.path(), "", b"unsigned");

    let error = prepare(&Request {
        source,
        output_dir: directory.path().to_path_buf(),
    })
    .unwrap_err();

    assert!(error.contains("must not overwrite"));
}

#[test]
fn rejects_noncanonical_unsigned_input() {
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("Unsigned.dmg");
    std::fs::File::create(&source)
        .unwrap()
        .write_all(&dmg_bytes(b"unsigned"))
        .unwrap();

    let error = prepare(&Request {
        source,
        output_dir: directory.path().join("signed"),
    })
    .unwrap_err();

    assert!(error.contains("DropSquash.dmg"));
}

#[test]
fn rejects_unsigned_input_with_nix_store_reference() {
    let directory = tempfile::tempdir().unwrap();
    let source = write_dmg(directory.path(), "unsigned", b"/nix/store/abc");

    let error = prepare(&Request {
        source,
        output_dir: directory.path().join("signed"),
    })
    .unwrap_err();

    assert!(error.contains("/nix/store"));
}

fn write_dmg(root: &std::path::Path, directory: &str, prefix: &[u8]) -> std::path::PathBuf {
    let parent = if directory.is_empty() {
        root.to_path_buf()
    } else {
        root.join(directory)
    };
    std::fs::create_dir_all(&parent).unwrap();
    let path = parent.join("DropSquash.dmg");
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
